# Round robin
```
[2026-03-06 05:59:24.443]    0.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 05:59:26.445]    2.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 05:59:28.448]    4.0s | :8080=1r/0w  :8081=1r/0w  :8082=2r/0w  :8083=1r/0w
[2026-03-06 05:59:30.450]    6.0s | :8080=1r/0w  :8081=3r/1w  :8082=1r/0w  :8083=3r/2w
[2026-03-06 05:59:32.450]    8.0s | :8080=2r/1w  :8081=1r/0w  :8082=2r/1w  :8083=2r/1w
[2026-03-06 05:59:34.452]   10.0s | :8080=1r/0w  :8081=2r/0w  :8082=3r/1w  :8083=1r/0w
[2026-03-06 05:59:36.453]   12.0s | :8080=3r/1w  :8081=3r/1w  :8082=3r/2w  :8083=3r/2w
[2026-03-06 05:59:38.455]   14.0s | :8080=4r/2w  :8081=2r/0w  :8082=2r/1w  :8083=2r/1w
[2026-03-06 05:59:40.457]   16.0s | :8080=2r/1w  :8081=2r/1w  :8082=2r/1w  :8083=2r/1w
[2026-03-06 05:59:42.459]   18.0s | :8080=4r/2w  :8081=1r/0w  :8082=1r/0w  :8083=1r/1w
[2026-03-06 05:59:44.461]   20.0s | :8080=2r/0w  :8081=2r/0w  :8082=3r/2w  :8083=1r/0w
[2026-03-06 05:59:46.461]   22.0s | :8080=3r/1w  :8081=1r/0w  :8082=1r/0w  :8083=4r/2w
[2026-03-06 05:59:48.463]   24.0s | :8080=2r/1w  :8081=2r/1w  :8082=2r/1w  :8083=3r/1w
[2026-03-06 05:59:50.465]   26.0s | :8080=2r/1w  :8081=2r/1w  :8082=2r/1w  :8083=0r/0w
[2026-03-06 05:59:52.467]   28.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 05:59:54.470]   30.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 05:59:56.470]   32.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 05:59:58.472]   34.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:00:00.474]   36.0s | :8080=3r/1w  :8081=1r/0w  :8082=5r/2w  :8083=3r/1w
[2026-03-06 06:00:02.475]   38.0s | :8080=6r/1w  :8081=2r/0w  :8082=2r/2w  :8083=2r/1w
[2026-03-06 06:00:04.477]   40.0s | :8080=3r/1w  :8081=1r/0w  :8082=3r/1w  :8083=3r/1w
[2026-03-06 06:00:06.478]   42.0s | :8080=3r/1w  :8081=2r/0w  :8082=3r/1w  :8083=3r/1w
[2026-03-06 06:00:08.480]   44.0s | :8080=4r/2w  :8081=2r/0w  :8082=4r/2w  :8083=4r/2w
[2026-03-06 06:00:10.482]   46.0s | :8080=2r/1w  :8081=2r/1w  :8082=2r/2w  :8083=2r/1w
[2026-03-06 06:00:12.483]   48.0s | :8080=1r/0w  :8081=5r/2w  :8082=5r/3w  :8083=3r/1w
[2026-03-06 06:00:14.485]   50.0s | :8080=4r/0w  :8081=3r/1w  :8082=3r/2w  :8083=2r/2w
[2026-03-06 06:00:16.486]   52.0s | :8080=3r/1w  :8081=2r/1w  :8082=5r/2w  :8083=1r/0w
[2026-03-06 06:00:18.488]   54.0s | :8080=2r/0w  :8081=3r/1w  :8082=5r/2w  :8083=1r/0w
[2026-03-06 06:00:20.489]   56.0s | :8080=5r/2w  :8081=2r/1w  :8082=2r/1w  :8083=2r/0w
[2026-03-06 06:00:22.491]   58.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:00:24.493]   60.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:00:26.495]   62.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:00:28.498]   64.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:00:30.500]   66.1s | :8080=1r/0w  :8081=5r/0w  :8082=1r/0w  :8083=2r/2w
[2026-03-06 06:00:32.502]   68.1s | :8080=6r/3w  :8081=1r/0w  :8082=5r/3w  :8083=2r/0w
[2026-03-06 06:00:34.503]   70.1s | :8080=3r/1w  :8081=5r/1w  :8082=6r/3w  :8083=3r/0w
[2026-03-06 06:00:36.505]   72.1s | :8080=2r/1w  :8081=2r/0w  :8082=0r/0w  :8083=4r/0w
[2026-03-06 06:00:38.507]   74.1s | :8080=2r/1w  :8081=0r/0w  :8082=9r/4w  :8083=6r/2w
[2026-03-06 06:00:40.509]   76.1s | :8080=7r/3w  :8081=3r/1w  :8082=2r/0w  :8083=3r/2w
[2026-03-06 06:00:42.509]   78.1s | :8080=0r/0w  :8081=2r/1w  :8082=2r/1w  :8083=0r/0w
[2026-03-06 06:00:44.511]   80.1s | :8080=12r/6w  :8081=1r/0w  :8082=3r/1w  :8083=2r/1w
[2026-03-06 06:00:46.513]   82.1s | :8080=7r/1w  :8081=4r/2w  :8082=1r/0w  :8083=5r/2w
[2026-03-06 06:00:48.515]   84.1s | :8080=10r/4w  :8081=1r/0w  :8082=2r/0w  :8083=1r/0w
[2026-03-06 06:00:50.517]   86.1s | :8080=3r/1w  :8081=4r/2w  :8082=4r/2w  :8083=5r/2w
[2026-03-06 06:00:52.519]   88.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:00:54.521]   90.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:00:56.521]   92.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:00:58.523]   94.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:01:00.525]   96.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:01:02.527]   98.1s | :8080=3r/1w  :8081=8r/3w  :8082=5r/2w  :8083=2r/2w
[2026-03-06 06:01:04.529]  100.1s | :8080=7r/2w  :8081=3r/1w  :8082=5r/1w  :8083=4r/1w
[2026-03-06 06:01:06.531]  102.1s | :8080=11r/5w  :8081=9r/4w  :8082=2r/0w  :8083=2r/0w
[2026-03-06 06:01:08.533]  104.1s | :8080=6r/1w  :8081=5r/2w  :8082=6r/2w  :8083=8r/4w
[2026-03-06 06:01:10.535]  106.1s | :8080=3r/1w  :8081=9r/4w  :8082=2r/1w  :8083=7r/2w
[2026-03-06 06:01:12.535]  108.1s | :8080=7r/4w  :8081=6r/2w  :8082=1r/0w  :8083=2r/0w
[2026-03-06 06:01:14.537]  110.1s | :8080=7r/2w  :8081=5r/2w  :8082=0r/0w  :8083=2r/2w
[2026-03-06 06:01:16.539]  112.1s | :8080=4r/2w  :8081=8r/4w  :8082=0r/0w  :8083=1r/1w
[2026-03-06 06:01:18.541]  114.1s | :8080=6r/2w  :8081=3r/0w  :8082=5r/2w  :8083=7r/3w
[2026-03-06 06:01:20.543]  116.1s | :8080=6r/2w  :8081=6r/1w  :8082=2r/0w  :8083=4r/2w
[2026-03-06 06:01:22.545]  118.1s | :8080=4r/1w  :8081=8r/3w  :8082=2r/0w  :8083=8r/3w
[2026-03-06 06:01:24.547]  120.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:01:26.549]  122.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:01:28.551]  124.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:01:30.553]  126.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:01:32.555]  128.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:01:34.557]  130.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:01:36.559]  132.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:01:38.561]  134.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:01:40.563]  136.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:01:42.565]  138.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:01:44.567]  140.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:01:46.569]  142.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:01:48.569]  144.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:01:50.572]  146.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:01:52.573]  148.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:01:54.575]  150.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:01:56.577]  152.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
```

```
Loading config from: embedding.json
Loaded 4 configs

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
Worker 0 (PID 178754) started
  Connecting to: http://localhost:8000
Worker 0 connected to server
Worker 0 terminated

Benchmark completed in 22.85s
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
  Request Throughput:  262.56 req/s
  Input Tok/s:         0.00 tok/s
  Output Tok/s:        0.00 tok/s
  Duration:            22.85s

----------------------------------------------------------------------
Latency (milliseconds):

  Metric    Mean      P50       P90       P95       P99       Count
  --------------------------------------------------------------------
  TTFT      N/A       N/A       N/A       N/A       N/A       0
  TPOT      N/A       N/A       N/A       N/A       N/A       0
  ITL       N/A       N/A       N/A       N/A       N/A       0
  E2E       56.66     54.52     70.72     77.10     89.45     6000

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
Worker 0 (PID 178776) started
  Connecting to: http://localhost:8000
Worker 0 connected to server
Worker 0 terminated

Benchmark completed in 22.50s
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
  Request Throughput:  266.69 req/s
  Input Tok/s:         0.00 tok/s
  Output Tok/s:        0.00 tok/s
  Duration:            22.50s

----------------------------------------------------------------------
Latency (milliseconds):

  Metric    Mean      P50       P90       P95       P99       Count
  --------------------------------------------------------------------
  TTFT      N/A       N/A       N/A       N/A       N/A       0
  TPOT      N/A       N/A       N/A       N/A       N/A       0
  ITL       N/A       N/A       N/A       N/A       N/A       0
  E2E       74.24     70.34     98.15     108.48    133.62    6000

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
Worker 0 (PID 178798) started
  Connecting to: http://localhost:8000
Worker 0 connected to server
Worker 0 terminated

Benchmark completed in 22.40s
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
  Request Throughput:  267.82 req/s
  Input Tok/s:         0.00 tok/s
  Output Tok/s:        0.00 tok/s
  Duration:            22.40s

----------------------------------------------------------------------
Latency (milliseconds):

  Metric    Mean      P50       P90       P95       P99       Count
  --------------------------------------------------------------------
  TTFT      N/A       N/A       N/A       N/A       N/A       0
  TPOT      N/A       N/A       N/A       N/A       N/A       0
  ITL       N/A       N/A       N/A       N/A       N/A       0
  E2E       92.25     82.80     137.95    162.36    201.28    6000

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
Worker 0 (PID 178818) started
  Connecting to: http://localhost:8000
Worker 0 connected to server
Worker 0 terminated

Benchmark completed in 22.05s
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
  Request Throughput:  272.09 req/s
  Input Tok/s:         0.00 tok/s
  Output Tok/s:        0.00 tok/s
  Duration:            22.05s

----------------------------------------------------------------------
Latency (milliseconds):

  Metric    Mean      P50       P90       P95       P99       Count
  --------------------------------------------------------------------
  TTFT      N/A       N/A       N/A       N/A       N/A       0
  TPOT      N/A       N/A       N/A       N/A       N/A       0
  ITL       N/A       N/A       N/A       N/A       N/A       0
  E2E       108.79    104.34    150.13    168.45    210.11    6000

======================================================================
```

# Load aware
```
Monitoring 4 workers for 300.0s, interval=2.0s, output=load_monitor.csv
Workers: ['http://localhost:8080', 'http://localhost:8081', 'http://localhost:8082', 'http://localhost:8083']
--------------------------------------------------------------------------------
[2026-03-06 06:04:12.177]    0.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:04:14.179]    2.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:04:16.181]    4.0s | :8080=ERR  :8081=1r/0w  :8082=0r/0w  :8083=2r/2w
[2026-03-06 06:04:18.181]    6.0s | :8080=2r/1w  :8081=2r/1w  :8082=2r/0w  :8083=2r/0w
[2026-03-06 06:04:20.183]    8.0s | :8080=2r/0w  :8081=2r/0w  :8082=2r/1w  :8083=2r/1w
[2026-03-06 06:04:22.185]   10.0s | :8080=3r/1w  :8081=1r/1w  :8082=0r/0w  :8083=2r/1w
[2026-03-06 06:04:24.187]   12.0s | :8080=1r/0w  :8081=3r/1w  :8082=2r/2w  :8083=2r/1w
[2026-03-06 06:04:26.189]   14.0s | :8080=ERR  :8081=2r/2w  :8082=2r/1w  :8083=2r/0w
[2026-03-06 06:04:28.190]   16.0s | :8080=3r/1w  :8081=3r/1w  :8082=1r/0w  :8083=2r/1w
[2026-03-06 06:04:30.192]   18.0s | :8080=3r/2w  :8081=2r/1w  :8082=2r/1w  :8083=2r/0w
[2026-03-06 06:04:32.193]   20.0s | :8080=3r/1w  :8081=1r/0w  :8082=1r/0w  :8083=2r/1w
[2026-03-06 06:04:34.195]   22.0s | :8080=3r/1w  :8081=2r/0w  :8082=2r/0w  :8083=0r/0w
[2026-03-06 06:04:36.197]   24.0s | :8080=1r/0w  :8081=4r/2w  :8082=3r/1w  :8083=1r/1w
[2026-03-06 06:04:38.199]   26.0s | :8080=3r/2w  :8081=1r/1w  :8082=5r/3w  :8083=1r/0w
[2026-03-06 06:04:40.200]   28.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:04:42.202]   30.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:04:44.204]   32.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:04:46.206]   34.0s | :8080=1r/0w  :8081=1r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:04:48.208]   36.0s | :8080=3r/0w  :8081=3r/0w  :8082=ERR  :8083=2r/1w
[2026-03-06 06:04:50.209]   38.0s | :8080=5r/2w  :8081=1r/0w  :8082=4r/2w  :8083=2r/1w
[2026-03-06 06:04:52.210]   40.0s | :8080=1r/0w  :8081=2r/2w  :8082=1r/1w  :8083=1r/0w
[2026-03-06 06:04:54.212]   42.0s | :8080=4r/2w  :8081=2r/1w  :8082=4r/1w  :8083=3r/1w
[2026-03-06 06:04:56.212]   44.0s | :8080=3r/1w  :8081=3r/1w  :8082=1r/0w  :8083=4r/2w
[2026-03-06 06:04:58.214]   46.0s | :8080=2r/1w  :8081=4r/1w  :8082=5r/2w  :8083=3r/0w
[2026-03-06 06:05:00.216]   48.0s | :8080=3r/0w  :8081=1r/1w  :8082=4r/2w  :8083=2r/0w
[2026-03-06 06:05:02.218]   50.0s | :8080=3r/1w  :8081=4r/1w  :8082=0r/0w  :8083=ERR
[2026-03-06 06:05:04.219]   52.0s | :8080=2r/0w  :8081=4r/2w  :8082=3r/1w  :8083=8r/3w
[2026-03-06 06:05:06.221]   54.0s | :8080=3r/0w  :8081=ERR  :8082=2r/2w  :8083=2r/0w
[2026-03-06 06:05:08.221]   56.0s | :8080=1r/0w  :8081=3r/2w  :8082=2r/1w  :8083=3r/1w
[2026-03-06 06:05:10.223]   58.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:05:12.224]   60.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:05:14.226]   62.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:05:16.228]   64.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:05:18.229]   66.1s | :8080=5r/1w  :8081=6r/0w  :8082=2r/1w  :8083=5r/3w
[2026-03-06 06:05:20.229]   68.1s | :8080=2r/0w  :8081=2r/1w  :8082=ERR  :8083=2r/0w
[2026-03-06 06:05:22.229]   70.1s | :8080=2r/1w  :8081=2r/0w  :8082=3r/1w  :8083=ERR
[2026-03-06 06:05:24.231]   72.1s | :8080=7r/3w  :8081=3r/0w  :8082=3r/1w  :8083=ERR
[2026-03-06 06:05:26.233]   74.1s | :8080=1r/0w  :8081=6r/2w  :8082=ERR  :8083=ERR
[2026-03-06 06:05:28.235]   76.1s | :8080=1r/0w  :8081=2r/1w  :8082=ERR  :8083=1r/0w
[2026-03-06 06:05:30.237]   78.1s | :8080=2r/1w  :8081=1r/0w  :8082=1r/1w  :8083=ERR
[2026-03-06 06:05:32.239]   80.1s | :8080=7r/2w  :8081=4r/2w  :8082=ERR  :8083=2r/0w
[2026-03-06 06:05:34.240]   82.1s | :8080=6r/3w  :8081=7r/3w  :8082=3r/1w  :8083=2r/0w
[2026-03-06 06:05:36.241]   84.1s | :8080=4r/2w  :8081=4r/2w  :8082=5r/2w  :8083=3r/0w
[2026-03-06 06:05:38.243]   86.1s | :8080=2r/0w  :8081=4r/1w  :8082=5r/0w  :8083=0r/0w
[2026-03-06 06:05:40.245]   88.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:05:42.246]   90.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:05:44.248]   92.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:05:46.248]   94.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:05:48.250]   96.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:05:50.252]   98.1s | :8080=2r/1w  :8081=3r/0w  :8082=3r/1w  :8083=7r/3w
[2026-03-06 06:05:52.254]  100.1s | :8080=6r/2w  :8081=ERR  :8082=8r/4w  :8083=3r/0w
[2026-03-06 06:05:54.256]  102.1s | :8080=3r/1w  :8081=ERR  :8082=0r/0w  :8083=ERR
[2026-03-06 06:05:56.257]  104.1s | :8080=6r/1w  :8081=7r/1w  :8082=4r/0w  :8083=6r/1w
[2026-03-06 06:05:58.259]  106.1s | :8080=8r/3w  :8081=3r/1w  :8082=4r/0w  :8083=6r/2w
[2026-03-06 06:06:00.261]  108.1s | :8080=1r/0w  :8081=4r/1w  :8082=3r/0w  :8083=11r/6w
[2026-03-06 06:06:02.261]  110.1s | :8080=5r/1w  :8081=4r/1w  :8082=ERR  :8083=ERR
[2026-03-06 06:06:04.263]  112.1s | :8080=2r/0w  :8081=ERR  :8082=3r/0w  :8083=ERR
[2026-03-06 06:06:06.264]  114.1s | :8080=ERR  :8081=ERR  :8082=5r/1w  :8083=11r/6w
[2026-03-06 06:06:08.266]  116.1s | :8080=3r/1w  :8081=3r/2w  :8082=3r/2w  :8083=5r/1w
[2026-03-06 06:06:10.268]  118.1s | :8080=ERR  :8081=ERR  :8082=4r/1w  :8083=5r/1w
[2026-03-06 06:06:12.270]  120.1s | :8080=ERR  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:06:14.272]  122.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:06:16.273]  124.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:06:18.274]  126.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
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
Worker 0 (PID 179299) started
  Connecting to: http://localhost:8000
Worker 0 connected to server
Worker 0 terminated

Benchmark completed in 23.24s
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
  Request Throughput:  258.17 req/s
  Input Tok/s:         0.00 tok/s
  Output Tok/s:        0.00 tok/s
  Duration:            23.24s

----------------------------------------------------------------------
Latency (milliseconds):

  Metric    Mean      P50       P90       P95       P99       Count
  --------------------------------------------------------------------
  TTFT      N/A       N/A       N/A       N/A       N/A       0
  TPOT      N/A       N/A       N/A       N/A       N/A       0
  ITL       N/A       N/A       N/A       N/A       N/A       0
  E2E       57.61     54.46     75.84     83.26     100.69    6000

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
Worker 0 (PID 179318) started
  Connecting to: http://localhost:8000
Worker 0 connected to server
Worker 0 terminated

Benchmark completed in 22.88s
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
  Request Throughput:  262.23 req/s
  Input Tok/s:         0.00 tok/s
  Output Tok/s:        0.00 tok/s
  Duration:            22.88s

----------------------------------------------------------------------
Latency (milliseconds):

  Metric    Mean      P50       P90       P95       P99       Count
  --------------------------------------------------------------------
  TTFT      N/A       N/A       N/A       N/A       N/A       0
  TPOT      N/A       N/A       N/A       N/A       N/A       0
  ITL       N/A       N/A       N/A       N/A       N/A       0
  E2E       75.52     71.16     104.33    115.60    145.04    6000

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
Worker 0 (PID 179332) started
  Connecting to: http://localhost:8000
Worker 0 connected to server
Worker 0 terminated

Benchmark completed in 22.92s
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
  Request Throughput:  261.81 req/s
  Input Tok/s:         0.00 tok/s
  Output Tok/s:        0.00 tok/s
  Duration:            22.92s

----------------------------------------------------------------------
Latency (milliseconds):

  Metric    Mean      P50       P90       P95       P99       Count
  --------------------------------------------------------------------
  TTFT      N/A       N/A       N/A       N/A       N/A       0
  TPOT      N/A       N/A       N/A       N/A       N/A       0
  ITL       N/A       N/A       N/A       N/A       N/A       0
  E2E       94.42     88.54     130.99    146.65    187.95    6000

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
Worker 0 (PID 179349) started
  Connecting to: http://localhost:8000
Worker 0 connected to server
Worker 0 terminated

Benchmark completed in 22.35s
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
  Request Throughput:  268.46 req/s
  Input Tok/s:         0.00 tok/s
  Output Tok/s:        0.00 tok/s
  Duration:            22.35s

----------------------------------------------------------------------
Latency (milliseconds):

  Metric    Mean      P50       P90       P95       P99       Count
  --------------------------------------------------------------------
  TTFT      N/A       N/A       N/A       N/A       N/A       0
  TPOT      N/A       N/A       N/A       N/A       N/A       0
  ITL       N/A       N/A       N/A       N/A       N/A       0
  E2E       110.13    105.67    163.22    182.76    231.51    6000

======================================================================
```

# PO2
```
--------------------------------------------------------------------------------
[2026-03-06 06:09:00.690]    0.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:09:02.692]    2.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:09:04.694]    4.0s | :8080=2r/2w  :8081=1r/0w  :8082=4r/1w  :8083=2r/0w
[2026-03-06 06:09:06.696]    6.0s | :8080=0r/0w  :8081=0r/0w  :8082=3r/1w  :8083=8r/4w
[2026-03-06 06:09:08.697]    8.0s | :8080=0r/0w  :8081=0r/0w  :8082=5r/1w  :8083=9r/4w
[2026-03-06 06:09:10.699]   10.0s | :8080=0r/0w  :8081=8r/2w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:09:12.701]   12.0s | :8080=0r/0w  :8081=5r/2w  :8082=4r/0w  :8083=3r/0w
[2026-03-06 06:09:14.701]   14.0s | :8080=1r/0w  :8081=0r/0w  :8082=5r/2w  :8083=5r/1w
[2026-03-06 06:09:16.703]   16.0s | :8080=0r/0w  :8081=0r/0w  :8082=9r/3w  :8083=0r/0w
[2026-03-06 06:09:18.705]   18.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=8r/5w
[2026-03-06 06:09:20.707]   20.0s | :8080=0r/0w  :8081=3r/0w  :8082=2r/1w  :8083=2r/0w
[2026-03-06 06:09:22.709]   22.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=4r/0w
[2026-03-06 06:09:24.711]   24.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=7r/0w
[2026-03-06 06:09:26.711]   26.0s | :8080=0r/0w  :8081=1r/1w  :8082=9r/4w  :8083=4r/1w
[2026-03-06 06:09:28.714]   28.0s | :8080=0r/0w  :8081=0r/0w  :8082=7r/1w  :8083=0r/0w
[2026-03-06 06:09:30.716]   30.0s | :8080=1r/0w  :8081=0r/0w  :8082=5r/2w  :8083=3r/0w
[2026-03-06 06:09:32.718]   32.0s | :8080=2r/1w  :8081=0r/0w  :8082=6r/4w  :8083=6r/2w
[2026-03-06 06:09:34.720]   34.0s | :8080=0r/0w  :8081=1r/0w  :8082=7r/3w  :8083=6r/1w
[2026-03-06 06:09:36.721]   36.0s | :8080=1r/0w  :8081=5r/2w  :8082=0r/0w  :8083=2r/0w
[2026-03-06 06:09:38.723]   38.0s | :8080=4r/2w  :8081=10r/4w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:09:40.725]   40.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:09:42.727]   42.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:09:44.730]   44.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:09:46.732]   46.0s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:09:48.734]   48.0s | :8080=0r/0w  :8081=0r/0w  :8082=5r/1w  :8083=2r/0w
[2026-03-06 06:09:50.736]   50.0s | :8080=2r/0w  :8081=0r/0w  :8082=6r/4w  :8083=1r/0w
[2026-03-06 06:09:52.737]   52.0s | :8080=6r/1w  :8081=5r/0w  :8082=1r/0w  :8083=4r/2w
[2026-03-06 06:09:54.739]   54.0s | :8080=2r/1w  :8081=0r/0w  :8082=0r/0w  :8083=9r/3w
[2026-03-06 06:09:56.741]   56.1s | :8080=0r/0w  :8081=10r/6w  :8082=0r/0w  :8083=6r/4w
[2026-03-06 06:09:58.743]   58.1s | :8080=8r/4w  :8081=5r/1w  :8082=0r/0w  :8083=1r/0w
[2026-03-06 06:10:00.745]   60.1s | :8080=3r/0w  :8081=0r/0w  :8082=0r/0w  :8083=10r/4w
[2026-03-06 06:10:02.747]   62.1s | :8080=0r/0w  :8081=3r/2w  :8082=0r/0w  :8083=9r/4w
[2026-03-06 06:10:04.749]   64.1s | :8080=0r/0w  :8081=0r/0w  :8082=8r/3w  :8083=0r/0w
[2026-03-06 06:10:06.751]   66.1s | :8080=1r/0w  :8081=0r/0w  :8082=13r/8w  :8083=1r/0w
[2026-03-06 06:10:08.753]   68.1s | :8080=2r/1w  :8081=3r/0w  :8082=2r/0w  :8083=0r/0w
[2026-03-06 06:10:10.755]   70.1s | :8080=0r/0w  :8081=2r/1w  :8082=9r/0w  :8083=7r/3w
[2026-03-06 06:10:12.756]   72.1s | :8080=0r/0w  :8081=1r/1w  :8082=16r/6w  :8083=0r/0w
[2026-03-06 06:10:14.758]   74.1s | :8080=0r/0w  :8081=1r/0w  :8082=6r/1w  :8083=7r/2w
[2026-03-06 06:10:16.760]   76.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:10:18.762]   78.1s | :8080=0r/0w  :8081=2r/1w  :8082=1r/0w  :8083=12r/6w
[2026-03-06 06:10:20.764]   80.1s | :8080=0r/0w  :8081=1r/0w  :8082=8r/4w  :8083=9r/2w
[2026-03-06 06:10:22.766]   82.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:10:24.767]   84.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:10:26.768]   86.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:10:28.770]   88.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:10:30.772]   90.1s | :8080=0r/0w  :8081=2r/0w  :8082=9r/4w  :8083=2r/0w
[2026-03-06 06:10:32.774]   92.1s | :8080=0r/0w  :8081=1r/0w  :8082=17r/10w  :8083=0r/0w
[2026-03-06 06:10:34.776]   94.1s | :8080=0r/0w  :8081=0r/0w  :8082=9r/0w  :8083=0r/0w
[2026-03-06 06:10:36.776]   96.1s | :8080=0r/0w  :8081=1r/0w  :8082=11r/0w  :8083=1r/0w
[2026-03-06 06:10:38.778]   98.1s | :8080=7r/0w  :8081=0r/0w  :8082=1r/0w  :8083=0r/0w
[2026-03-06 06:10:40.780]  100.1s | :8080=2r/1w  :8081=5r/3w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:10:42.781]  102.1s | :8080=0r/0w  :8081=22r/11w  :8082=1r/1w  :8083=0r/0w
[2026-03-06 06:10:44.783]  104.1s | :8080=0r/0w  :8081=15r/7w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:10:46.785]  106.1s | :8080=0r/0w  :8081=11r/5w  :8082=1r/1w  :8083=5r/2w
[2026-03-06 06:10:48.787]  108.1s | :8080=0r/0w  :8081=5r/1w  :8082=13r/3w  :8083=0r/0w
[2026-03-06 06:10:50.789]  110.1s | :8080=0r/0w  :8081=0r/0w  :8082=12r/5w  :8083=1r/0w
[2026-03-06 06:10:52.791]  112.1s | :8080=2r/0w  :8081=7r/3w  :8082=8r/5w  :8083=0r/0w
[2026-03-06 06:10:54.793]  114.1s | :8080=0r/0w  :8081=4r/1w  :8082=14r/7w  :8083=0r/0w
[2026-03-06 06:10:56.795]  116.1s | :8080=0r/0w  :8081=8r/2w  :8082=3r/0w  :8083=4r/3w
[2026-03-06 06:10:58.797]  118.1s | :8080=0r/0w  :8081=6r/0w  :8082=2r/1w  :8083=7r/2w
[2026-03-06 06:11:00.798]  120.1s | :8080=0r/0w  :8081=0r/0w  :8082=7r/1w  :8083=2r/1w
[2026-03-06 06:11:02.800]  122.1s | :8080=0r/0w  :8081=2r/1w  :8082=3r/0w  :8083=13r/3w
[2026-03-06 06:11:04.802]  124.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:11:06.802]  126.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:11:08.803]  128.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:11:10.805]  130.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:11:12.807]  132.1s | :8080=6r/0w  :8081=3r/3w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:11:14.809]  134.1s | :8080=3r/3w  :8081=0r/0w  :8082=5r/1w  :8083=7r/1w
[2026-03-06 06:11:16.811]  136.1s | :8080=0r/0w  :8081=18r/6w  :8082=0r/0w  :8083=1r/0w
[2026-03-06 06:11:18.813]  138.1s | :8080=0r/0w  :8081=19r/7w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:11:20.814]  140.1s | :8080=2r/1w  :8081=0r/0w  :8082=19r/6w  :8083=0r/0w
[2026-03-06 06:11:22.814]  142.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=18r/10w
[2026-03-06 06:11:24.815]  144.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=19r/7w
[2026-03-06 06:11:26.817]  146.1s | :8080=0r/0w  :8081=0r/0w  :8082=28r/11w  :8083=0r/0w
[2026-03-06 06:11:28.819]  148.1s | :8080=2r/1w  :8081=0r/0w  :8082=24r/9w  :8083=1r/0w
[2026-03-06 06:11:30.821]  150.1s | :8080=9r/4w  :8081=0r/0w  :8082=14r/8w  :8083=0r/0w
[2026-03-06 06:11:32.823]  152.1s | :8080=0r/0w  :8081=17r/5w  :8082=7r/3w  :8083=0r/0w
[2026-03-06 06:11:34.825]  154.1s | :8080=2r/0w  :8081=26r/11w  :8082=0r/0w  :8083=0r/0w
[2026-03-06 06:11:36.825]  156.1s | :8080=2r/0w  :8081=0r/0w  :8082=4r/2w  :8083=15r/7w
[2026-03-06 06:11:38.827]  158.1s | :8080=2r/2w  :8081=0r/0w  :8082=0r/0w  :8083=5r/5w
[2026-03-06 06:11:40.829]  160.1s | :8080=2r/0w  :8081=0r/0w  :8082=5r/1w  :8083=13r/2w
[2026-03-06 06:11:42.830]  162.1s | :8080=16r/1w  :8081=0r/0w  :8082=1r/1w  :8083=0r/0w
[2026-03-06 06:11:44.832]  164.1s | :8080=0r/0w  :8081=0r/0w  :8082=0r/0w  :8083=13r/2w
[2026-03-06 06:11:46.833]  166.1s | :8080=0r/0w  :8081=0r/0w  :8082=11r/2w  :8083=0r/0w
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
Worker 0 (PID 179821) started
  Connecting to: http://localhost:8000
Worker 0 connected to server
Worker 0 terminated

Benchmark completed in 36.02s
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
  Request Throughput:  166.56 req/s
  Input Tok/s:         0.00 tok/s
  Output Tok/s:        0.00 tok/s
  Duration:            36.02s

----------------------------------------------------------------------
Latency (milliseconds):

  Metric    Mean      P50       P90       P95       P99       Count
  --------------------------------------------------------------------
  TTFT      N/A       N/A       N/A       N/A       N/A       0
  TPOT      N/A       N/A       N/A       N/A       N/A       0
  ITL       N/A       N/A       N/A       N/A       N/A       0
  E2E       89.19     80.84     156.30    178.83    220.95    6000

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
Worker 0 (PID 179846) started
  Connecting to: http://localhost:8000
Worker 0 connected to server
Worker 0 terminated

Benchmark completed in 34.38s
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
  Request Throughput:  174.51 req/s
  Input Tok/s:         0.00 tok/s
  Output Tok/s:        0.00 tok/s
  Duration:            34.38s

----------------------------------------------------------------------
Latency (milliseconds):

  Metric    Mean      P50       P90       P95       P99       Count
  --------------------------------------------------------------------
  TTFT      N/A       N/A       N/A       N/A       N/A       0
  TPOT      N/A       N/A       N/A       N/A       N/A       0
  ITL       N/A       N/A       N/A       N/A       N/A       0
  E2E       113.37    99.54     210.47    242.38    290.82    6000

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
Worker 0 (PID 179873) started
  Connecting to: http://localhost:8000
Worker 0 connected to server
Worker 0 terminated

Benchmark completed in 34.11s
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
  Request Throughput:  175.88 req/s
  Input Tok/s:         0.00 tok/s
  Output Tok/s:        0.00 tok/s
  Duration:            34.11s

----------------------------------------------------------------------
Latency (milliseconds):

  Metric    Mean      P50       P90       P95       P99       Count
  --------------------------------------------------------------------
  TTFT      N/A       N/A       N/A       N/A       N/A       0
  TPOT      N/A       N/A       N/A       N/A       N/A       0
  ITL       N/A       N/A       N/A       N/A       N/A       0
  E2E       140.26    112.09    275.17    319.81    424.87    6000

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
Worker 0 (PID 179889) started
  Connecting to: http://localhost:8000
Worker 0 connected to server
Worker 0 terminated

Benchmark completed in 35.63s
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
  Request Throughput:  168.41 req/s
  Input Tok/s:         0.00 tok/s
  Output Tok/s:        0.00 tok/s
  Duration:            35.63s

----------------------------------------------------------------------
Latency (milliseconds):

  Metric    Mean      P50       P90       P95       P99       Count
  --------------------------------------------------------------------
  TTFT      N/A       N/A       N/A       N/A       N/A       0
  TPOT      N/A       N/A       N/A       N/A       N/A       0
  ITL       N/A       N/A       N/A       N/A       N/A       0
  E2E       175.67    128.80    374.42    407.80    488.43    6000

======================================================================
```