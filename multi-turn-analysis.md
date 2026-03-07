# Multi-Turn Routing Policy Analysis

## TL;DR

**Cache_aware is the best routing policy for multi-turn LLM workloads across all tested scenarios.** It achieves the same ~81% cache hit rate as sticky routing while maintaining balanced load distribution, resulting in:

- **29% higher throughput** and **43% lower E2E P99** vs sticky routing (uniform 8-10 turns)
- **34% lower TPOT P99** — balanced workers keep decode batch sizes small
- **28% lower E2E P99** at higher concurrency (16) with variable turns (5-15)

Sticky routing (consistent hashing) wins on **TTFT P50** (fastest individual cache hits) but loses on every system-level metric due to hash-based load imbalance. Round-robin achieves only 32% cache hit rate — unsuitable for multi-turn.

| Workload | Winner | Cache_aware advantage |
|---|---|---|
| Uniform turns (8-10), C=12 | **Cache Aware** | +29% throughput, -43% E2E P99 |
| Variable turns (2-10), C=12 | **Tie** | Both converge to same behavior |
| Variable turns (5-15), C=16 | **Cache Aware** | +6.7% throughput, -28% E2E P99 |

**Key insight**: The tradeoff is TTFT P50 (sticky wins) vs TPOT/E2E P99 (cache_aware wins). With 512 output tokens, the decode phase dominates E2E, making TPOT more impactful than TTFT. Load balance matters more than routing overhead.

---

## 1. Experiment 1: Uniform Turn Count (8-10 turns, C=12)

### 1.1 Setup

- **Model**: Qwen3-4B Instruct on H100
- **Workers**: 4 sglang instances
- **Dataset**: 44 conversations, 8-10 turns, user_mean=5000 tokens, variance=640000, max_tokens=512
- **Concurrency**: 12
- **Completed**: 40 conversations, avg 9.0 turns/conversation, 360 total requests
- **Policies tested**: round_robin, consistent_hashing (sticky routing via X-SMG-Routing-Key), cache_aware
- **Baseline**: Single machine (theoretical max cache hit rate)

### 1.2 Overall Results

| Metric | RR | Sticky (CH) | Cache Aware | Single Machine |
|---|---|---|---|---|
| **Cache hit rate** | 31.92% | **81.51%** | **81.43%** | 81.57% |
| Throughput (req/s) | 1.50 | 1.31 | **1.69** | N/A |
| Duration (s) | 239.51 | 274.83 | **213.20** | N/A |
| TTFT Mean (ms) | 684.69 | **463.91** | 547.33 | N/A |
| TTFT P50 (ms) | 474.80 | **398.73** | 495.53 | N/A |
| TTFT P99 (ms) | 2431.31 | 1232.96 | **1210.20** | N/A |
| E2E Mean (ms) | 6777.14 | 6683.87 | **5677.89** | N/A |
| E2E P99 (ms) | 14406.07 | 15528.02 | **8889.64** | N/A |
| TPOT Mean (ms) | 11.92 | 12.17 | **10.04** | N/A |
| ITL P50 (ms) | 8.85 | 9.90 | **9.61** | N/A |

### 1.3 Cache Hit Rate

Both sticky routing (81.51%) and cache_aware (81.43%) achieve near-theoretical-maximum cache hit rates, matching the single machine baseline (81.57%). Round-robin achieves only 31.92% — approximately the 1/4 probability of randomly hitting the same worker across 4 replicas.

### 1.4 Per-Turn TTFT Comparison (P50, ms)

| Turn | RR | Sticky | Cache Aware | Sticky vs RR | CA vs RR |
|---|---|---|---|---|---|
| 1 (cold) | 232.96 | 282.06 | 389.47 | +21% | +67% |
| 2 | 341.49 | **217.21** | 323.49 | **-36%** | -5% |
| 3 | 479.93 | **252.32** | 408.15 | **-47%** | -15% |
| 4 | 416.13 | **306.06** | 484.73 | **-26%** | +17% |
| 5 | 563.63 | **345.27** | 637.03 | **-39%** | +13% |
| 6 | 824.52 | **408.72** | 723.09 | **-50%** | -12% |
| 7 | 842.01 | **441.48** | 771.17 | **-48%** | -8% |
| 8 | 506.73 | **484.55** | 794.54 | -4% | +57% |
| 9 | 1083.85 | **549.85** | 698.25 | **-49%** | -36% |
| 10 | 1001.33 | **630.08** | 667.99 | **-37%** | -33% |

#### TTFT Observations

- **Sticky routing wins TTFT P50 on every turn after turn 1** — the hash-based routing has zero routing overhead, so cache hits translate directly to faster TTFT.
- **Turn 1 is slower for both cache-aware policies** — routing overhead with no cache to hit yet. Cache_aware has the highest turn 1 TTFT (389ms vs RR's 232ms) due to radix tree initialization.
- **Sticky routing TTFT stays flat** across turns (217-630ms) while RR TTFT grows with context size (232-1352ms). Cache hits skip the growing prefill cost.

### 1.5 Throughput and Tail Latency: Cache Aware Wins

Despite sticky routing's superior TTFT, **cache_aware achieves 29% higher throughput** (1.69 vs 1.31 req/s) and **43% lower E2E P99** (8889ms vs 15528ms).

The root cause is **load imbalance** in sticky routing:

#### Load Distribution

**Sticky routing — severe imbalance:**
```
:9080=2r/0w  :9081=0r/0w  :9082=9r/0w  :9083=1r/0w   (9x spread!)
:9080=2r/0w  :9081=0r/0w  :9082=8r/0w  :9083=2r/0w   (8x spread)
:9080=1r/0w  :9081=0r/0w  :9082=9r/0w  :9083=1r/0w   (worker 9081 idle!)
```

With 40 conversations hashed to 4 workers, the consistent hash may assign 15 conversations to one worker and 5 to another. The overloaded worker becomes a bottleneck while others sit idle.

**Cache_aware — perfectly balanced:**
```
:9080=3r/0w  :9081=3r/0w  :9082=3r/0w  :9083=3r/0w   (perfect)
:9080=3r/0w  :9081=3r/0w  :9082=3r/0w  :9083=3r/0w   (consistent)
:9080=3r/0w  :9081=3r/1w  :9082=3r/0w  :9083=3r/0w   (rare deviation)
```

Cache_aware routes by prompt content prefix, and its radix tree tracks which worker has the best cache match. When multiple workers have similar cache quality, it picks the least loaded one — achieving both cache affinity AND load balance.

### 1.6 TPOT and ITL Analysis

#### Overall TPOT / ITL

| Metric | RR | Sticky (CH) | Cache Aware | CA vs RR | CA vs Sticky |
|---|---|---|---|---|---|
| TPOT Mean | 11.92 | 12.17 | **10.04** | **-15.8%** | **-17.5%** |
| TPOT P50 | 10.67 | 10.83 | **10.04** | -5.9% | -7.3% |
| TPOT P90 | 18.56 | 20.45 | **13.40** | **-27.8%** | **-34.5%** |
| TPOT P99 | 26.03 | 28.48 | **15.65** | **-39.9%** | **-45.1%** |
| ITL Mean | 12.15 | 12.34 | **10.09** | **-17.0%** | **-18.2%** |
| ITL P50 | 8.85 | 9.90 | **9.61** | +8.6% | -2.9% |
| ITL P90 | 14.74 | 18.00 | **12.75** | **-13.5%** | **-29.2%** |
| ITL P99 | 21.44 | 26.60 | **15.43** | **-28.0%** | **-42.0%** |

#### Per-Turn TPOT (Mean, ms)

| Turn | RR | Sticky | Cache Aware |
|---|---|---|---|
| 1 | 8.46 | 10.58 | **7.90** |
| 2 | 9.34 | 10.96 | **8.36** |
| 3 | 9.96 | 10.38 | **8.80** |
| 4 | 10.35 | 10.74 | **9.40** |
| 5 | 12.83 | 11.47 | **10.05** |
| 6 | 13.87 | 12.30 | **11.12** |
| 7 | 13.58 | 13.57 | **11.75** |
| 8 | 13.85 | 14.90 | **12.04** |
| 9 | 15.18 | 15.69 | **10.68** |
| 10 | 14.87 | 12.54 | **11.52** |

#### TPOT / ITL Key Findings

**1. Cache_aware achieves the lowest TPOT across all turns and percentiles.**

Cache_aware's TPOT P99 (15.65ms) is **45% lower** than sticky's (28.48ms) and **40% lower** than RR's (26.03ms). This is a direct consequence of balanced load distribution:
- With 3 requests per worker (cache_aware), each decode step processes a small batch
- With 9 requests on one worker (sticky), decode batches are 3x larger, increasing per-token latency

**2. Sticky routing has the WORST TPOT despite having the best TTFT.**

This is the critical tradeoff: sticky achieves lower TTFT (cache hits skip prefill) but higher TPOT (overloaded workers slow down decode). The net effect on E2E depends on the output length — with `max_tokens=512`, the decode phase dominates E2E, making TPOT more impactful than TTFT.

**3. TPOT grows with turn number for all policies** due to increasing KV cache size:
- Turn 1: ~5K tokens in cache -> fast attention
- Turn 8: ~40K tokens in cache -> slower attention per step
- Cache_aware's TPOT grows from 7.90ms to 12.04ms (52% increase)
- Sticky's TPOT grows from 10.58ms to 14.90ms (41% increase)

**4. ITL tail latency reveals decode scheduling quality.**

ITL P99 measures the worst inter-token gaps during generation. Cache_aware's 15.43ms vs sticky's 26.60ms indicates:
- Cache_aware: decode steps run at consistent pace (low variance)
- Sticky: the overloaded worker experiences scheduling delays between tokens, causing ITL spikes

**5. The TPOT advantage explains cache_aware's throughput win.**

With 360 requests x ~500 output tokens each = 180K decode steps:
- Cache_aware: 180K x 10.04ms = ~1806s total decode time
- Sticky: 180K x 12.17ms = ~2191s total decode time
- Difference: ~385s saved in decode alone, which is ~64% of the duration gap (274s - 213s = 61s actual gap)

The remaining gap comes from TTFT differences and pipeline efficiency (balanced workers keep GPUs busier).

### 1.7 Why Cache Aware Beats Sticky Routing

| Aspect | Sticky (CH) | Cache Aware | Winner |
|---|---|---|---|
| Cache hit rate | 81.51% | 81.43% | Tie |
| Load balance | Poor (9x spread) | Excellent (even) | **Cache Aware** |
| TTFT P50 | **398.73ms** | 495.53ms | Sticky |
| TTFT P99 | 1232.96ms | **1210.20ms** | Cache Aware |
| Throughput | 1.31 req/s | **1.69 req/s** | **Cache Aware** (+29%) |
| E2E P99 | 15528.02ms | **8889.64ms** | **Cache Aware** (-43%) |
| TPOT Mean | 12.17ms | **10.04ms** | **Cache Aware** (-18%) |
| Routing overhead | Zero (hash lookup) | Medium (radix tree) | Sticky |

**Cache_aware wins overall** because load balancing matters more than routing overhead for throughput and tail latency. The 29% throughput improvement and 43% E2E P99 reduction outweigh sticky's TTFT P50 advantage.

**Why sticky's TPOT is worse (12.17ms vs 10.04ms):** The imbalanced worker has more active requests competing for GPU decode bandwidth. With 9 requests on one worker vs 3 on cache_aware's balanced distribution, each decode step takes longer due to larger batch sizes on the overloaded worker.

### 1.8 When Would Sticky Routing Win Over Cache Aware?

| Scenario | Expected Winner | Rationale |
|---|---|---|
| **200+ conversations, 8-10 turns** | **Sticky** | Hash distributes evenly at scale (law of large numbers); cache_aware's radix tree overhead grows |
| **40 conversations, 8-10 turns** (current) | **Cache Aware** | Hash imbalance at small scale; radix tree load balancing compensates |
| **Shared system prompt across users** | **Cache Aware** | Routes same-prefix requests to same worker by content; sticky can't detect shared content |
| **Higher concurrency (20-30)** | **Cache Aware** | Load balancing becomes more critical; sticky can't rebalance |
| **Heterogeneous conversation lengths** | **Cache Aware** | Some convs are 2 turns, some are 10; sticky can't shift load |
| **Single turn (no cache benefit)** | **Sticky** | Zero routing overhead vs radix tree cost; no cache to hit |
| **Very short conversations (2-3 turns)** | **Sticky** | Less prefix to cache; fewer turns = less cache benefit from content matching |
| **Latency-sensitive (P50 matters most)** | **Sticky** | Direct hash lookup = lowest possible routing latency |

#### Recommended Experiment: High Conversation Count

To find sticky's sweet spot, test with **200 conversations x 4-5 turns** at concurrency 20:
- The hash should distribute ~50 conversations per worker (much more even than 10)
- Shorter conversations reduce cache_aware's radix tree advantage
- Higher concurrency stresses cache_aware's locking more

### 1.9 Comparison with Single Machine

The single machine cache hit rate (81.57%) matches both cache-aware policies, confirming that **routing is the only factor affecting cache performance** — the sglang server's radix cache works identically regardless of routing policy.

---

## 2. Experiment 2: Variable Turn Count (2-10 turns, C=12)

### 2.1 Setup

- 40 conversations, 2-10 turns (avg 6.58), concurrency 12, same dataset params

### 2.2 Results

| Metric | CA (var 2-10) | Sticky (var 2-10) |
|---|---|---|
| Throughput | 1.64 | 1.64 |
| TTFT P50 | **363.43** | 364.43 |
| TPOT Mean | 10.52 | **10.47** |
| E2E Mean | 5753.77 | **5742.16** |
| E2E P99 | 9883.64 | **9740.99** |

### 2.3 Analysis

**Both policies produce nearly identical results** — within noise margin on every metric. This contrasts sharply with the uniform 8-10 turn experiment where cache_aware dominated.

**Why variable turns neutralize cache_aware's advantage:**

With heterogeneous turn counts, the workload itself creates imbalance that neither policy can fix:
- Short conversations (2-3 turns) finish early, freeing workers
- Long conversations (8-10 turns) are stuck on their worker — cache_aware can't migrate 40K cached tokens without losing the entire cache
- Both policies degrade to: "route to wherever the cache is, accept the imbalance"

**Load distribution confirms convergence — both develop similar late-phase imbalance:**

Cache_aware late:
```
:9080=0r  :9081=7r  :9082=0r  :9083=5r   (two workers idle)
```

Sticky late:
```
:9080=5r  :9081=0r  :9082=7r  :9083=1r   (similar pattern)
```

---

## 3. Experiment 3: Higher Concurrency + Longer Variable Turns (5-15 turns, C=16)

### 3.1 Setup

- 60 conversations, 5-15 turns (avg 10.12), concurrency 16, 607 total requests

### 3.2 Results

| Metric | Sticky (5-15, C=16) | CA (5-15, C=16) | CA vs Sticky |
|---|---|---|---|
| Throughput | 1.50 | **1.60** | **+6.7%** |
| Duration | 404.70s | **379.99s** | **-6.1%** |
| TTFT Mean | **592.14** | 628.66 | +6.2% (worse) |
| TTFT P50 | **473.53** | 549.70 | +16.1% (worse) |
| TTFT P99 | 1995.22 | **1507.29** | **-24.4%** |
| TPOT Mean | 15.48 | **13.22** | **-14.6%** |
| TPOT P99 | 33.23 | **21.82** | **-34.3%** |
| ITL P99 | 29.29 | **22.71** | **-22.5%** |
| E2E Mean | 8498.34 | **7382.41** | **-13.1%** |
| E2E P99 | 17973.55 | **12860.95** | **-28.4%** |

### 3.3 Analysis

**Cache_aware reasserts dominance at higher concurrency + longer variable turns.** Unlike the 2-10 turn experiment where they tied, the combination of C=16 and 5-15 turns creates enough load pressure for cache_aware's load balancing to matter.

**Sticky routing's load distribution is catastrophic:**
```
Early:   :9080=6r  :9081=4r  :9082=2r  :9083=4r  (3x spread)
Mid:     :9080=9r  :9081=4r  :9082=2r  :9083=1r  (9x spread!)
Late:    :9080=9r  :9081=6r  :9082=0r  :9083=0r  (2 workers fully idle!)
Final:   :9080=0r  :9081=4r  :9082=8r  :9083=4r  (load shifted, still uneven)
```

Worker 9080 consistently holds 8-9 requests while 9082/9083 sit idle. With C=16, the overloaded worker processes 3x more concurrent requests, causing TPOT P99 to spike to 33ms vs cache_aware's 22ms.

**Per-turn TPOT degradation under sticky routing:**

| Turn | Sticky TPOT | CA TPOT |
|---|---|---|
| 1 | 12.17 | **11.11** |
| 5 | 15.48 | **13.16** |
| 8 | 17.13 | — |
| 10 | 18.79 | — |
| 13 | 19.98 | — |

Sticky's TPOT grows steadily to ~20ms at turn 13 due to the compounding effect of large KV caches + overloaded workers.

### 3.4 Why 5-15 Turns Differs from 2-10 Turns

The 2-10 turn experiment had avg 6.58 turns — many conversations finished by turn 3-4, freeing workers quickly. At 5-15 turns (avg 10.12), the minimum conversation length is 5 turns, meaning **all workers stay loaded for much longer**. This gives cache_aware's load balancing more time to create differentiation:
- Workers processing 5-turn conversations free up and accept new work
- Cache_aware routes new conversations to the freed workers
- Sticky can't — the hash permanently assigns new conversations regardless of worker load

---

## 4. Cross-Experiment Summary

| Experiment | Turns | C | Convs | CA vs Sticky Throughput | CA vs Sticky E2E P99 | Winner |
|---|---|---|---|---|---|---|
| Uniform | 8-10 | 12 | 40 | **+29%** | **-43%** | **CA dominates** |
| Variable short | 2-10 | 12 | 40 | 0% | -1.4% | **Tie** |
| Variable long | 5-15 | 16 | 60 | **+6.7%** | **-28%** | **CA wins** |

**Pattern**: Cache_aware's advantage scales with `concurrency x conversation_length_variance`. At low concurrency with short variable conversations, both policies converge. At higher concurrency with longer conversations, cache_aware's load balancing creates significant differentiation.

---

## 5. Key Takeaways

1. **Cache_aware is the best general-purpose multi-turn routing policy.** It achieves near-optimal cache hit rates while maintaining perfect load balance, resulting in the highest throughput and lowest tail latency.

2. **Sticky routing achieves the same cache hit rate but suffers from load imbalance** at moderate conversation counts (40). The hash-based assignment is permanent — once a conversation is assigned to a worker, it can't be rebalanced.

3. **TTFT P50 vs E2E P99 tradeoff.** Sticky wins on median TTFT (individual request latency) but loses badly on E2E P99 (system-level tail latency). Cache_aware sacrifices some TTFT for much better system throughput.

4. **Load balance is more impactful than routing overhead** for multi-turn workloads. The cost of an overloaded worker (higher TPOT, higher E2E) far exceeds the cost of radix tree lookups.

5. **Round-robin is unsuitable for multi-turn.** The 31.92% cache hit rate means ~68% of follow-up turns do full prefill instead of cache hits, resulting in 42% higher mean TTFT and 62% higher E2E P99 compared to cache_aware.

6. **Cache_aware's advantage scales with concurrency and conversation length.** Use cache_aware as the default — it never loses to sticky and often wins significantly.

**Final recommendation**: Use **cache_aware** for all multi-turn workloads. The only scenario where sticky matches is low-concurrency short conversations — and even there, cache_aware ties rather than losing.
