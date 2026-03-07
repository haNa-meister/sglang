# Load aware with waiting punish
```
Workers: ['http://localhost:8080', 'http://localhost:8081', 'http://localhost:8082', 'http://localhost:8083']
--------------------------------------------------------------------------------
[2026-03-06 07:01:26.241]    0.0s | :8080=3r/1w  :8081=2r/1w  :8082=3r/1w  :8083=1r/0w
[2026-03-06 07:01:28.243]    2.0s | :8080=3r/2w  :8081=0r/0w  :8082=4r/2w  :8083=1r/0w
[2026-03-06 07:01:30.245]    4.0s | :8080=3r/1w  :8081=1r/1w  :8082=1r/0w  :8083=2r/1w
[2026-03-06 07:01:32.247]    6.0s | :8080=2r/0w  :8081=3r/1w  :8082=2r/1w  :8083=2r/1w
[2026-03-06 07:01:34.249]    8.0s | :8080=3r/2w  :8081=1r/0w  :8082=0r/0w  :8083=3r/2w
[2026-03-06 07:01:36.251]   10.0s | :8080=3r/2w  :8081=1r/1w  :8082=4r/2w  :8083=1r/0w
[2026-03-06 07:01:38.253]   12.0s | :8080=1r/1w  :8081=2r/1w  :8082=2r/0w  :8083=ERR
[2026-03-06 07:01:40.253]   14.0s | :8080=1r/0w  :8081=2r/1w  :8082=4r/2w  :8083=2r/1w
[2026-03-06 07:01:42.255]   16.0s | :8080=0r/0w  :8081=1r/0w  :8082=2r/0w  :8083=1r/0w
[2026-03-06 07:01:44.256]   18.0s | :8080=4r/1w  :8081=0r/0w  :8082=0r/0w  :8083=ERR
[2026-03-06 07:01:46.257]   20.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 07:01:48.259]   22.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 07:01:50.261]   24.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 07:01:52.263]   26.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 07:01:54.265]   28.0s | :8080=1r/0w  :8081=1r/0w  :8082=4r/0w  :8083=ERR
[2026-03-06 07:01:56.266]   30.0s | :8080=1r/1w  :8081=3r/2w  :8082=1r/1w  :8083=1r/0w
[2026-03-06 07:01:58.268]   32.0s | :8080=4r/2w  :8081=2r/0w  :8082=4r/2w  :8083=2r/1w
[2026-03-06 07:02:00.270]   34.0s | :8080=6r/3w  :8081=3r/0w  :8082=2r/0w  :8083=4r/3w
[2026-03-06 07:02:02.271]   36.0s | :8080=4r/2w  :8081=3r/2w  :8082=5r/1w  :8083=3r/1w
[2026-03-06 07:02:04.273]   38.0s | :8080=6r/2w  :8081=5r/1w  :8082=ERR  :8083=3r/1w
[2026-03-06 07:02:06.273]   40.0s | :8080=3r/1w  :8081=2r/1w  :8082=ERR  :8083=2r/1w
[2026-03-06 07:02:08.275]   42.0s | :8080=3r/0w  :8081=3r/1w  :8082=ERR  :8083=3r/1w
[2026-03-06 07:02:10.277]   44.0s | :8080=5r/2w  :8081=4r/2w  :8082=1r/0w  :8083=4r/1w
[2026-03-06 07:02:12.278]   46.0s | :8080=2r/0w  :8081=3r/3w  :8082=5r/2w  :8083=1r/0w
[2026-03-06 07:02:14.280]   48.0s | :8080=2r/0w  :8081=1r/0w  :8082=ERR  :8083=1r/1w
[2026-03-06 07:02:16.282]   50.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 07:02:18.284]   52.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 07:02:20.285]   54.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 07:02:22.287]   56.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 07:02:24.288]   58.0s | :8080=1r/0w  :8081=0r/0w  :8082=0r/0w  :8083=ERR
[2026-03-06 07:02:26.289]   60.0s | :8080=5r/1w  :8081=7r/4w  :8082=1r/0w  :8083=5r/2w
[2026-03-06 07:02:28.291]   62.0s | :8080=3r/1w  :8081=1r/0w  :8082=6r/3w  :8083=3r/1w
[2026-03-06 07:02:30.293]   64.1s | :8080=5r/2w  :8081=3r/1w  :8082=1r/1w  :8083=3r/1w
[2026-03-06 07:02:32.295]   66.1s | :8080=3r/1w  :8081=ERR  :8082=3r/1w  :8083=5r/3w
[2026-03-06 07:02:34.297]   68.1s | :8080=8r/3w  :8081=5r/3w  :8082=3r/0w  :8083=4r/1w
[2026-03-06 07:02:36.299]   70.1s | :8080=2r/1w  :8081=ERR  :8082=4r/1w  :8083=4r/2w
[2026-03-06 07:02:38.301]   72.1s | :8080=ERR  :8081=ERR  :8082=3r/0w  :8083=5r/3w
[2026-03-06 07:02:40.303]   74.1s | :8080=2r/0w  :8081=2r/0w  :8082=7r/2w  :8083=ERR
[2026-03-06 07:02:42.305]   76.1s | :8080=ERR  :8081=4r/1w  :8082=2r/2w  :8083=5r/3w
[2026-03-06 07:02:44.307]   78.1s | :8080=ERR  :8081=ERR  :8082=2r/1w  :8083=3r/1w
[2026-03-06 07:02:46.309]   80.1s | :8080=ERR  :8081=1r/0w  :8082=5r/3w  :8083=3r/1w
[2026-03-06 07:02:48.311]   82.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 07:02:50.312]   84.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 07:02:52.314]   86.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 07:02:54.314]   88.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 07:02:56.316]   90.1s | :8080=ERR  :8081=ERR  :8082=8r/4w  :8083=6r/2w
[2026-03-06 07:02:58.317]   92.1s | :8080=3r/2w  :8081=3r/0w  :8082=10r/6w  :8083=ERR
[2026-03-06 07:03:00.319]   94.1s | :8080=ERR  :8081=3r/0w  :8082=4r/2w  :8083=ERR
[2026-03-06 07:03:02.321]   96.1s | :8080=4r/0w  :8081=4r/2w  :8082=8r/0w  :8083=3r/0w
[2026-03-06 07:03:04.323]   98.1s | :8080=0r/0w  :8081=ERR  :8082=ERR  :8083=ERR
[2026-03-06 07:03:06.325]  100.1s | :8080=3r/3w  :8081=4r/4w  :8082=ERR  :8083=ERR
[2026-03-06 07:03:08.326]  102.1s | :8080=3r/1w  :8081=6r/3w  :8082=2r/0w  :8083=5r/2w
[2026-03-06 07:03:10.327]  104.1s | :8080=ERR  :8081=ERR  :8082=6r/2w  :8083=5r/2w
[2026-03-06 07:03:12.328]  106.1s | :8080=2r/0w  :8081=8r/5w  :8082=3r/1w  :8083=ERR
[2026-03-06 07:03:14.330]  108.1s | :8080=3r/2w  :8081=5r/2w  :8082=ERR  :8083=ERR
[2026-03-06 07:03:16.332]  110.1s | :8080=2r/0w  :8081=6r/2w  :8082=6r/4w  :8083=4r/2w
[2026-03-06 07:03:18.334]  112.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
```

```
============================================================
[Config 1/4]
Starting benchmark...
  Protocol: HTTP
  API: embeddings -> /v1/embeddings
  Model: qwen
  Streaming: True
  Endpoint: http://localhost:8000
  Workers: 1
Generated dataset summary: count=6010, min=351, max=1785, mean=998.74, std=199.64
Loaded 6010 items from dataset
Scheduled 6010 requests
Will run 10 warmup + 6000 benchmark requests
Started 1 worker processes
  Concurrency distribution: [15] (total=15)
Running 10 warmup requests...
Warmup complete, 6000 requests remaining for benchmark
Worker 0 (PID 183490) started
  Connecting to: http://localhost:8000
Worker 0 connected to server
Worker 0 terminated

Benchmark completed in 22.96s
Post-processing: counting tokens with tokenizer...
  Tokenized 6000 responses
  Input tokens: 5,991,522, Output tokens: 6,000
Executor stopped

======================================================================
BENCHMARK RESULTS
======================================================================

Configuration:
  Service:       None/None
  Endpoint:      http://localhost:8000
  Schedule Mode: concurrency
  Workers:       1

----------------------------------------------------------------------
Request Summary:
  Total Requests:      6,000
  Successful:          6,000
  Failed:              0
  Success Rate:        100.00%

----------------------------------------------------------------------
Throughput:
  Request Throughput:  261.30 req/s
  Input Tok/s:         0.00 tok/s
  Output Tok/s:        0.00 tok/s
  Duration:            22.96s

----------------------------------------------------------------------
Latency (milliseconds):

  Metric    Mean      P50       P90       P95       P99       Count
  --------------------------------------------------------------------
  TTFT      N/A       N/A       N/A       N/A       N/A       0
  TPOT      N/A       N/A       N/A       N/A       N/A       0
  ITL       N/A       N/A       N/A       N/A       N/A       0
  E2E       56.93     55.07     71.66     77.55     92.19     6000

======================================================================

============================================================
[Config 2/4]
Starting benchmark...
  Protocol: HTTP
  API: embeddings -> /v1/embeddings
  Model: qwen
  Streaming: True
  Endpoint: http://localhost:8000
  Workers: 1
Generated dataset summary: count=6010, min=351, max=1785, mean=998.74, std=199.64
Loaded 6010 items from dataset
Scheduled 6010 requests
Will run 10 warmup + 6000 benchmark requests
Started 1 worker processes
  Concurrency distribution: [20] (total=20)
Running 10 warmup requests...
Warmup complete, 6000 requests remaining for benchmark
Worker 0 (PID 183517) started
  Connecting to: http://localhost:8000
Worker 0 connected to server
Worker 0 terminated

Benchmark completed in 22.71s
Post-processing: counting tokens with tokenizer...
  Tokenized 6000 responses
  Input tokens: 5,991,522, Output tokens: 6,000
Executor stopped

======================================================================
BENCHMARK RESULTS
======================================================================

Configuration:
  Service:       None/None
  Endpoint:      http://localhost:8000
  Schedule Mode: concurrency
  Workers:       1

----------------------------------------------------------------------
Request Summary:
  Total Requests:      6,000
  Successful:          6,000
  Failed:              0
  Success Rate:        100.00%

----------------------------------------------------------------------
Throughput:
  Request Throughput:  264.17 req/s
  Input Tok/s:         0.00 tok/s
  Output Tok/s:        0.00 tok/s
  Duration:            22.71s

----------------------------------------------------------------------
Latency (milliseconds):

  Metric    Mean      P50       P90       P95       P99       Count
  --------------------------------------------------------------------
  TTFT      N/A       N/A       N/A       N/A       N/A       0
  TPOT      N/A       N/A       N/A       N/A       N/A       0
  ITL       N/A       N/A       N/A       N/A       N/A       0
  E2E       74.93     71.10     97.09     105.81    126.46    6000

======================================================================

============================================================
[Config 3/4]
Starting benchmark...
  Protocol: HTTP
  API: embeddings -> /v1/embeddings
  Model: qwen
  Streaming: True
  Endpoint: http://localhost:8000
  Workers: 1
Generated dataset summary: count=6010, min=351, max=1785, mean=998.74, std=199.64
Loaded 6010 items from dataset
Scheduled 6010 requests
Will run 10 warmup + 6000 benchmark requests
Started 1 worker processes
  Concurrency distribution: [25] (total=25)
Running 10 warmup requests...
Warmup complete, 6000 requests remaining for benchmark
Worker 0 (PID 183537) started
  Connecting to: http://localhost:8000
Worker 0 connected to server
Worker 0 terminated

Benchmark completed in 22.18s
Post-processing: counting tokens with tokenizer...
  Tokenized 6000 responses
  Input tokens: 5,991,522, Output tokens: 6,000
Executor stopped

======================================================================
BENCHMARK RESULTS
======================================================================

Configuration:
  Service:       None/None
  Endpoint:      http://localhost:8000
  Schedule Mode: concurrency
  Workers:       1

----------------------------------------------------------------------
Request Summary:
  Total Requests:      6,000
  Successful:          6,000
  Failed:              0
  Success Rate:        100.00%

----------------------------------------------------------------------
Throughput:
  Request Throughput:  270.48 req/s
  Input Tok/s:         0.00 tok/s
  Output Tok/s:        0.00 tok/s
  Duration:            22.18s

----------------------------------------------------------------------
Latency (milliseconds):

  Metric    Mean      P50       P90       P95       P99       Count
  --------------------------------------------------------------------
  TTFT      N/A       N/A       N/A       N/A       N/A       0
  TPOT      N/A       N/A       N/A       N/A       N/A       0
  ITL       N/A       N/A       N/A       N/A       N/A       0
  E2E       91.37     87.79     120.80    131.94    162.30    6000

======================================================================


============================================================
[Config 4/4]
Starting benchmark...
  Protocol: HTTP
  API: embeddings -> /v1/embeddings
  Model: qwen
  Streaming: True
  Endpoint: http://localhost:8000
  Workers: 1
Generated dataset summary: count=6010, min=351, max=1785, mean=998.74, std=199.64
Loaded 6010 items from dataset
Scheduled 6010 requests
Will run 10 warmup + 6000 benchmark requests
Started 1 worker processes
  Concurrency distribution: [30] (total=30)
Running 10 warmup requests...
Warmup complete, 6000 requests remaining for benchmark
Worker 0 (PID 183555) started
  Connecting to: http://localhost:8000
Worker 0 connected to server
Worker 0 terminated

Benchmark completed in 22.39s
Post-processing: counting tokens with tokenizer...
  Tokenized 6000 responses
  Input tokens: 5,991,522, Output tokens: 6,000
Executor stopped

======================================================================
BENCHMARK RESULTS
======================================================================

Configuration:
  Service:       None/None
  Endpoint:      http://localhost:8000
  Schedule Mode: concurrency
  Workers:       1

----------------------------------------------------------------------
Request Summary:
  Total Requests:      6,000
  Successful:          6,000
  Failed:              0
  Success Rate:        100.00%

----------------------------------------------------------------------
Throughput:
  Request Throughput:  267.98 req/s
  Input Tok/s:         0.00 tok/s
  Output Tok/s:        0.00 tok/s
  Duration:            22.39s

----------------------------------------------------------------------
Latency (milliseconds):

  Metric    Mean      P50       P90       P95       P99       Count
  --------------------------------------------------------------------
  TTFT      N/A       N/A       N/A       N/A       N/A       0
  TPOT      N/A       N/A       N/A       N/A       N/A       0
  ITL       N/A       N/A       N/A       N/A       N/A       0
  E2E       110.45    104.63    148.38    171.96    296.02    6000

======================================================================
```