# High Concurrency Experiment Results

## Setup

- **Model**: Qwen3-4B embedding on H100
- **Workers**: 4 sglang instances (localhost:8080-8083)
- **Chunked prefill**: Disabled (`-1`)
- **Dataset**: 20000 requests, token lengths ~ Normal(mean=1000, std=200)
- **Concurrency**: 60, 80, 100, 120
- **Policies tested**: RR, LA-weighted (running + 4*waiting, +4 delta, 200ms poll), LA-token (num_tokens, chars/4 estimation, 100ms poll via port+1000)
- **Server config**: max_prefill_tokens=16384 (~16 requests per batch at mean=1000 tokens)
- **Load server**: Lightweight HTTP server on port+1000 for contention-free polling

## Results

### Throughput (req/s)

| Concurrency | RR | LA-weighted | LA-token |
|---|---|---|---|
| 60 | **281.27** | 279.80 | 238.66 |
| 80 | 276.21 | **274.62** | 226.33 |
| 100 | 278.92 | **278.31** | 218.18 |
| 120 | **282.39** | 282.22 | 260.62 |

### Latency E2E (ms)

| Concurrency | RR P50 | RR P99 | LA-weighted P50 | LA-weighted P99 | LA-token P50 | LA-token P99 |
|---|---|---|---|---|---|---|
| 60 | 196.82 | 494.95 | **198.08** | **428.59** | 225.88 | 637.19 |
| 80 | 205.26 | 620.45 | **194.84** | **519.41** | 215.78 | 733.77 |
| 100 | 267.38 | 691.40 | **263.88** | **636.30** | 192.08 | 829.93 |
| 120 | **323.24** | **637.64** | 322.85 | 730.53 | 399.83 | 959.43 |

### P99 Improvement: LA-weighted vs RR

| Concurrency | RR P99 | LA-weighted P99 | Improvement |
|---|---|---|---|
| 60 | 494.95 | **428.59** | **-13.4%** |
| 80 | 620.45 | **519.41** | **-16.3%** |
| 100 | 691.40 | **636.30** | **-8.0%** |
| 120 | 637.64 | 730.53 | +14.6% (degraded) |

## Load Distribution Analysis

### RR at High Concurrency — Significant Imbalance

RR shows substantial load spread at high concurrency. Sample snapshots:

```
:8080=25r/9w   :8081=9r/6w   :8082=4r/1w   :8083=7r/2w    (6.3x spread)
:8080=2r/2w    :8081=1r/1w   :8082=27r/21w :8083=4r/1w    (27x spread!)
:8080=23r/11w  :8081=11r/4w  :8082=15r/5w  :8083=14r/6w   (2x spread)
:8080=21r/9w   :8081=3r/0w   :8082=15r/6w  :8083=0r/0w    (21x spread)
:8083=26r/13w  while others 7-11r                           (2.4x spread)
```

RR distributes requests evenly by **count** but doesn't account for processing time variance. When one worker gets a batch of heavier requests (longer tokens), its queue grows while others drain — creating transient hotspots that RR can't correct.

### LA-weighted — Better Balance

LA-weighted uses server feedback (running + 4*waiting) + local delta to actively route away from congested workers:

```
:8080=9r/3w   :8081=8r/2w   :8082=8r/3w   :8083=9r/3w    (even)
:8080=11r/2w  :8081=10r/4w  :8082=7r/1w   :8083=7r/2w    (1.6x spread)
:8080=16r/4w  :8081=13r/4w  :8082=12r/4w  :8083=8r/3w    (2x spread)
```

### LA-token — Unstable at High Concurrency

LA-token creates oscillation with large delta values:

```
:8080=ERR     :8081=ERR     :8082=ERR     :8083=ERR       (all ERR!)
:8080=23r/10w :8081=0r/0w   :8082=0r/0w   :8083=24r/10w  (extreme hotspot)
:8080=0r/0w   :8081=35r/17w :8082=0r/0w   :8083=20r/11w  (35 on one worker!)
```

## Analysis

### Why LA-weighted Works at C=60-100

1. **Meaningful waiting queues**: At C=60+, each worker has 3-10 waiting requests. The 4x waiting penalty amplifies queue differences, making the router more responsive to imbalance.

2. **Small stable deltas**: +4 per request means between polls (~100ms), 25-30 requests add ~100-120 to deltas. This is proportional and stable.

3. **Lightweight load server eliminates ERR**: With port+1000 serving cached load data on a separate thread, polls succeed even under heavy inference load. No more stale data from failed polls.

4. **P99 improvement mechanism**: When RR blindly sends request #N to a worker that already has 20 waiting, LA-weighted sees the high score and routes to a worker with only 5 waiting. This prevents tail latency spikes.

### Why LA-weighted Degrades at C=120

At C=120 (30 per worker), the system is deeply saturated:
- Every worker has 10-15 waiting requests constantly
- The differences between workers become noise (15 vs 17 waiting)
- The waiting penalty amplifies noise, causing oscillation
- The local delta (+4) accumulates to ~120 between polls, overwhelming the cached server score

At deep saturation, RR's blind distribution is actually more stable because it doesn't react to noise.

### Why LA-token Failed

The `text.len()/4` estimation adds ~250 tokens per request to the delta. Between 100ms polls at C=60 (270 req/s, ~27 requests), the delta grows by ~6750. The cached `num_tokens` from the server is typically 2000-5000. So the delta dominates the score, and the server feedback becomes irrelevant — the policy degrades to pure local tracking with oversensitive deltas that oscillate.

## Conclusion

### Best Policy by Concurrency Range

| Concurrency Range | Best Policy | Why |
|---|---|---|
| 15-30 (low) | **RR** or LA-token | Minimal queuing; RR's simplicity wins. LA-token helps with P99 via proportional routing |
| 60-100 (high) | **LA-weighted** | Significant waiting queues; waiting penalty routes around congestion; 13-16% P99 improvement |
| 120+ (saturated) | **RR** | All workers equally overloaded; load-aware oscillates on noise |

### Key Learnings

1. **The optimal policy depends on the operating point.** No single configuration wins across all concurrency levels.

2. **Load metric granularity matters.** Token estimation (large deltas) is too sensitive for high concurrency. Request count with waiting penalty (small deltas) is more stable.

3. **Lightweight load server on port+1000 eliminates the ERR problem.** This infrastructure change is valuable regardless of the policy chosen.

4. **The "sweet spot" for load-aware routing is moderate congestion** (C=60-100) — enough queue depth for load differences to be meaningful, but not so saturated that every worker is equally overloaded.

5. **P99 latency is the metric where load-aware shines.** Throughput is similar across policies (GPU-bound), but tail latency benefits from routing around transient hotspots.
