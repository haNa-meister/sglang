# Routing & Gateway Research Plan — Next Experiments

## Infrastructure
- **Embedding model**: Qwen3-4B Embedding on 4x H100 (--is-embedding, --chunked-prefill-size -1)
- **Chat model**: Qwen3-4B Instruct on 4x H100
- **Gateway**: sgl-model-gateway with lightweight load server on port+1000
- **Benchmark**: llm-inference-benchmarking with X-SMG-Routing-Key support
- **Monitoring**: monitor_load.py / monitor_token.py at 2s intervals

---

## Theme 1: Prefill-Decode Disaggregation (PD)

### 1a. Interleave Problem in Aggregated Servers

**Goal**: Quantify how large prefills degrade ongoing decode latency in aggregated (non-PD) servers during multi-turn conversations.

**Hypothesis**: When a 50K+ token prefill lands on a worker actively decoding for other requests, ITL spikes due to GPU preemption. PD disaggregation should eliminate this.

**Experiment Design**:

| Parameter | Value |
|---|---|
| Model | Qwen3-4B Instruct |
| Workers | 4 aggregated vs 2P+2D disaggregated |
| Dataset | 60 conversations, 5-15 turns |
| User message | Normal(mean=5000, var=640000) |
| max_tokens | 512 |
| Concurrency | 16 |
| Policies | cache_aware (aggregated), cache_aware with PD policies (disaggregated) |

**Key Metrics**:
- ITL P99 and ITL variance (prefill preemption signal)
- TTFT by turn number (does late-turn prefill get slower?)
- TPOT stability over time (does it spike during large prefills?)

**Run Plan**:
1. Aggregated: `--policy cache_aware` with 4 regular workers
2. PD: `--prefill-policy cache_aware --decode-policy round_robin` with 2P+2D
3. Compare ITL distributions, not just means

---

### 1b. Optimal Prefill/Decode Worker Ratio

**Goal**: Find the best P:D ratio for chat workloads with mixed prompt lengths.

**Experiment Design**:

| Parameter | Value |
|---|---|
| Model | Qwen3-4B Instruct |
| Workers | 4 total: test 1P:3D, 2P:2D, 3P:1D |
| Dataset | 200 single-turn requests |
| Input | Bimodal: 50% Normal(mean=1000, var=10000), 50% Normal(mean=10000, var=1000000) |
| max_tokens | 256 |
| Concurrency | 20, 40, 60 |

**Key Metrics**:
- Throughput at each ratio
- TTFT (prefill bottleneck indicator)
- TPOT (decode bottleneck indicator)
- Worker utilization (are prefill or decode workers idle?)

---

## Theme 2: Load-Aware for Generation Workloads

### 2a. Completions with Variable Output Length

**Goal**: Test load-aware routing on generation workloads where output length varies significantly (unlike embedding where output is fixed).

**Hypothesis**: Load-aware should outperform RR because workers with long-running generations accumulate higher `num_reqs`, allowing the router to route around them.

**Experiment Design**:

| Parameter | Value |
|---|---|
| Model | Qwen3-4B Instruct |
| Workers | 4 regular |
| Dataset | 2000 single-turn completions |
| Input | Normal(mean=500, var=25000) — short prompts |
| max_tokens | Bimodal: 50% = 64 (short), 50% = 1024 (long) |
| Concurrency | 15, 25, 40 |
| Policies | round_robin, load_aware (num_reqs weighted), power_of_two |
| Poll interval | 500ms (load_aware), 500ms (PO2) |

**Key Metrics**:
- Throughput
- E2E P50 and P99 (does load-aware reduce tail from long-output requests?)
- Load distribution (do long-output requests create worker hotspots under RR?)

**Prediction**: Load-aware wins at C=25-40 where RR stacks multiple long-output requests on one worker.

---

### 2b. Mixed Workload: Embedding + Chat Through Same Gateway

**Goal**: Test routing when fast (embedding ~50ms) and slow (chat ~5s) requests share the same workers.

**Experiment Design**:

| Parameter | Value |
|---|---|
| Model | Qwen3-4B Instruct (not --is-embedding, so both APIs work) |
| Workers | 4 regular |
| Dataset | 50% embedding (input Normal(mean=1000, var=10000)), 50% chat (input Normal(mean=2000, var=100000), max_tokens=256) |
| Total requests | 4000 (2000 embed + 2000 chat, interleaved) |
| Concurrency | 20 |
| Policies | round_robin, load_aware |

**Key Metrics**:
- Embedding E2E P99 (are embeddings delayed by chat requests?)
- Chat TTFT and TPOT
- Per-API-type throughput

**Prediction**: Load-aware routes embeddings to workers not currently doing long chat prefills.

---

## Theme 3: Multi-Instance Cache Limitations

### 3a. Cache-Aware Accuracy Measurement

**Goal**: Measure how accurately the router's radix tree approximation matches the actual server-side cache state.

**Experiment Design**:

| Parameter | Value |
|---|---|
| Model | Qwen3-4B Instruct |
| Workers | 4 regular |
| Dataset | 100 conversations, 4 turns, shared system prompt (2000 tokens) |
| User message | Normal(mean=1000, var=10000) |
| max_tokens | 128 |
| Concurrency | 12 |
| Policy | cache_aware |

**Instrumentation needed**:
- Add logging to sglang server: for each request, log `cached_tokens` from `get_load` response
- Compare: router's expected cache match vs actual server-reported cached tokens
- Calculate: accuracy = requests where router's chosen worker had the best cache / total requests

**Key Metrics**:
- Cache routing accuracy (% correct)
- Accuracy by turn number (does it degrade over time?)
- Impact of incorrect routing on TTFT

---

### 3b. Shared System Prompt Cache Contention

**Goal**: Test cache_aware when many users share the same system prompt — does it overload one worker?

**Experiment Design**:

| Parameter | Value |
|---|---|
| Model | Qwen3-4B Instruct |
| Workers | 4 regular |
| Dataset | 200 single-turn requests, ALL with same 5000-token system prompt |
| User message | Normal(mean=1000, var=10000) — unique per request |
| max_tokens | 256 |
| Concurrency | 20, 40 |
| Policies | round_robin, cache_aware |

**Hypothesis**: cache_aware routes all requests to the same worker (best system prompt cache match), creating a hotspot. Its load balancing threshold should kick in and spread load — but does it?

**Key Metrics**:
- Load distribution (does cache_aware create a hotspot?)
- Cache hit rate (does spreading reduce cache hit rate?)
- TTFT and TPOT (is the hotspot worth the cache benefit?)

**Expected outcome**: cache_aware's `balance_abs_threshold` and `balance_rel_threshold` parameters control this tradeoff. Test with default values and then tune.

---

### 3c. Cache Eviction Under Memory Pressure

**Goal**: Measure cache_aware routing accuracy when KV cache is near capacity and eviction is active.

**Experiment Design**:

| Parameter | Value |
|---|---|
| Model | Qwen3-4B Instruct |
| Workers | 4 regular |
| Dataset | 80 conversations, 10-15 turns |
| User message | Normal(mean=5000, var=640000) |
| max_tokens | 512 |
| Concurrency | 16 (to push KV cache to ~80%+ capacity) |
| Policy | cache_aware |

**Key Metrics**:
- Cache hit rate over time (does it drop when eviction starts?)
- TTFT by turn number (later turns should be slower if cache evicted)
- Monitor `max_total_num_tokens` utilization per worker

**Expected outcome**: At ~80% KV utilization, earlier conversations get evicted. The router's radix tree still thinks the cache exists, leading to routing mismatches and TTFT degradation.

---

## Theme 4: Fault Tolerance

### 4a. Worker Failure During Multi-Turn

**Goal**: Test routing behavior when a worker dies mid-benchmark.

**Experiment Design**:

| Parameter | Value |
|---|---|
| Model | Qwen3-4B Instruct |
| Workers | 4 regular (kill one at t=60s) |
| Dataset | 60 conversations, 8-10 turns |
| User message | Normal(mean=5000, var=640000) |
| max_tokens | 512 |
| Concurrency | 12 |
| Policies | round_robin, cache_aware, consistent_hashing |

**Procedure**:
1. Start benchmark
2. At t=60s, `kill -9` one sglang server process
3. Measure: error rate, recovery time, throughput before/after
4. For cache_aware: does it gracefully reroute conversations?
5. For sticky: all conversations on killed worker fail permanently

**Key Metrics**:
- Error rate during failure window
- Time to detect failure (circuit breaker)
- Throughput recovery time
- Cache hit rate after recovery (does rerouting kill cache?)

---

## Theme 5: Advanced Routing

### 5a. Hybrid: Cache-Aware + Load-Aware

**Goal**: Combine prefix cache affinity with real-time load feedback from the lightweight load server.

**Implementation**:
- Modify cache_aware to use `num_tokens` from load server as a tiebreaker
- When multiple workers have similar cache match quality, pick the one with lowest `num_tokens`
- Current cache_aware uses `worker.load()` (request count) — replace with polled `num_tokens`

**Experiment Design**:

| Parameter | Value |
|---|---|
| Model | Qwen3-4B Instruct |
| Workers | 4 regular |
| Dataset | 60 conversations, 5-15 turns |
| User message | Normal(mean=5000, var=640000) |
| max_tokens | 512 |
| Concurrency | 16 |
| Policies | cache_aware (baseline), hybrid_cache_load_aware (new) |

**Hypothesis**: Hybrid should match cache_aware's cache hit rate while improving TPOT via better load-informed tiebreaking.

---

### 5b. Predictive Routing by Input Length

**Goal**: Use input token count (estimated from text length) to predict processing time and route proactively.

**Implementation**:
- In load_aware policy, weight `estimated_tokens` more heavily for longer inputs
- Route long prompts to the least loaded worker even if its delta is slightly higher
- Threshold: if estimated_tokens > 2x mean, apply 2x weight to load score

**Experiment Design**:

| Parameter | Value |
|---|---|
| Model | Qwen3-4B Embedding |
| Workers | 4 regular |
| Dataset | 6000 embeddings |
| Input | Bimodal: 70% Normal(mean=500, var=2500), 30% Normal(mean=5000, var=250000) |
| Concurrency | 20, 40, 60 |
| Policies | round_robin, load_aware (current), predictive_load_aware (new) |

**Hypothesis**: Predictive routing avoids stacking two 5000-token requests on the same worker, reducing P99 vs standard load_aware.

---

## Priority & Sequencing

| Phase | Experiments | Time Estimate | Dependencies |
|---|---|---|---|
| **Phase 1** (immediate) | 2a (completion load-aware), 3b (shared prompt) | 1-2 days | None — uses existing code |
| **Phase 2** (next week) | 1a (PD interleave), 3a (cache accuracy) | 2-3 days | Need sglang instrumentation for 3a |
| **Phase 3** (following) | 5a (hybrid policy), 1b (P:D ratio) | 3-4 days | Requires code changes for 5a |
| **Phase 4** (later) | 4a (fault tolerance), 3c (eviction), 2b (mixed workload) | 2-3 days | 4a needs kill script |
| **Phase 5** (stretch) | 5b (predictive routing) | 2-3 days | Requires policy implementation |
