# Load aware using token
```
Monitoring 4 workers for 300.0s, interval=2.0s, output=token_monitor.csv
Workers: ['http://localhost:8080', 'http://localhost:8081', 'http://localhost:8082', 'http://localhost:8083']
--------------------------------------------------------------------------------
[2026-03-06 06:30:44.628]    0.0s | :8080=3325t/2r  :8081=3698t/1r  :8082=2114t/2r  :8083=3255t/0r  | total=12392t
[2026-03-06 06:30:46.629]    2.0s | :8080=1186t/1r  :8081=1306t/1r  :8082=1822t/3r  :8083=1613t/1r  | total=5927t
[2026-03-06 06:30:48.631]    4.0s | :8080=2151t/0r  :8081=2972t/3r  :8082=2016t/3r  :8083=995t/0r  | total=8134t
[2026-03-06 06:30:50.633]    6.0s | :8080=1807t/2r  :8081=2481t/2r  :8082=2276t/2r  :8083=1145t/1r  | total=7709t
[2026-03-06 06:30:52.635]    8.0s | :8080=1723t/3r  :8081=2063t/4r  :8082=3136t/2r  :8083=2992t/3r  | total=9914t
[2026-03-06 06:30:54.637]   10.0s | :8080=2233t/2r  :8081=2444t/2r  :8082=2976t/1r  :8083=1747t/4r  | total=9400t
[2026-03-06 06:30:56.639]   12.0s | :8080=1556t/4r  :8081=ERR  :8082=1154t/1r  :8083=ERR  | total=2710t
[2026-03-06 06:30:58.641]   14.0s | :8080=1799t/2r  :8081=2112t/3r  :8082=1649t/2r  :8083=1162t/1r  | total=6722t
[2026-03-06 06:31:00.643]   16.0s | :8080=2488t/5r  :8081=1465t/1r  :8082=ERR  :8083=1835t/1r  | total=5788t
[2026-03-06 06:31:02.644]   18.0s | :8080=3363t/3r  :8081=1878t/3r  :8082=1019t/2r  :8083=2143t/2r  | total=8403t
[2026-03-06 06:31:04.646]   20.0s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:31:06.646]   22.0s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:31:08.647]   24.0s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:31:10.648]   26.0s | :8080=ERR  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:31:12.650]   28.0s | :8080=4058t/2r  :8081=736t/3r  :8082=1988t/5r  :8083=ERR  | total=6782t
[2026-03-06 06:31:14.651]   30.0s | :8080=2380t/2r  :8081=ERR  :8082=ERR  :8083=1879t/3r  | total=4259t
[2026-03-06 06:31:16.653]   32.0s | :8080=3215t/3r  :8081=662t/2r  :8082=2260t/3r  :8083=ERR  | total=6137t
[2026-03-06 06:31:18.655]   34.0s | :8080=1758t/3r  :8081=ERR  :8082=3017t/4r  :8083=3794t/4r  | total=8569t
[2026-03-06 06:31:20.657]   36.0s | :8080=3793t/1r  :8081=ERR  :8082=1888t/1r  :8083=1666t/2r  | total=7347t
[2026-03-06 06:31:22.659]   38.0s | :8080=2040t/3r  :8081=871t/1r  :8082=0t/4r  :8083=1115t/3r  | total=4026t
[2026-03-06 06:31:24.659]   40.0s | :8080=ERR  :8081=ERR  :8082=4010t/4r  :8083=1806t/3r  | total=5816t
[2026-03-06 06:31:26.661]   42.0s | :8080=ERR  :8081=ERR  :8082=2491t/4r  :8083=2271t/1r  | total=4762t
[2026-03-06 06:31:28.663]   44.0s | :8080=ERR  :8081=ERR  :8082=ERR  :8083=ERR  | total=0t
[2026-03-06 06:31:30.665]   46.0s | :8080=ERR  :8081=2368t/3r  :8082=2260t/4r  :8083=1078t/4r  | total=5706t
[2026-03-06 06:31:32.667]   48.0s | :8080=ERR  :8081=ERR  :8082=1126t/1r  :8083=700t/2r  | total=1826t
[2026-03-06 06:31:34.669]   50.0s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:31:36.669]   52.0s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:31:38.671]   54.0s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:31:40.673]   56.0s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:31:42.673]   58.0s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:31:44.676]   60.0s | :8080=ERR  :8081=2404t/3r  :8082=4226t/4r  :8083=5433t/1r  | total=12063t
[2026-03-06 06:31:46.678]   62.0s | :8080=0t/4r  :8081=4486t/5r  :8082=1945t/3r  :8083=3702t/5r  | total=10133t
[2026-03-06 06:31:48.678]   64.0s | :8080=3396t/2r  :8081=3589t/3r  :8082=809t/3r  :8083=4113t/7r  | total=11907t
[2026-03-06 06:31:50.680]   66.1s | :8080=4411t/0r  :8081=4116t/3r  :8082=1911t/7r  :8083=712t/4r  | total=11150t
[2026-03-06 06:31:52.681]   68.1s | :8080=3477t/0r  :8081=1131t/2r  :8082=4127t/0r  :8083=2063t/4r  | total=10798t
[2026-03-06 06:31:54.683]   70.1s | :8080=3851t/1r  :8081=ERR  :8082=1567t/5r  :8083=981t/4r  | total=6399t
[2026-03-06 06:31:56.685]   72.1s | :8080=3165t/2r  :8081=3470t/5r  :8082=4114t/3r  :8083=3241t/4r  | total=13990t
[2026-03-06 06:31:58.687]   74.1s | :8080=2175t/2r  :8081=1505t/2r  :8082=3186t/6r  :8083=1950t/7r  | total=8816t
[2026-03-06 06:32:00.689]   76.1s | :8080=1819t/4r  :8081=3158t/4r  :8082=ERR  :8083=3219t/3r  | total=8196t
[2026-03-06 06:32:02.689]   78.1s | :8080=3714t/3r  :8081=4010t/5r  :8082=1087t/5r  :8083=ERR  | total=8811t
[2026-03-06 06:32:04.691]   80.1s | :8080=3505t/5r  :8081=1785t/2r  :8082=5568t/7r  :8083=5141t/1r  | total=15999t
[2026-03-06 06:32:06.693]   82.1s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:32:08.695]   84.1s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:32:10.697]   86.1s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:32:12.699]   88.1s | :8080=0t/0r  :8081=ERR  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:32:14.701]   90.1s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:32:16.703]   92.1s | :8080=1250t/2r  :8081=2145t/1r  :8082=2353t/7r  :8083=6409t/2r  | total=12157t
[2026-03-06 06:32:18.705]   94.1s | :8080=3883t/5r  :8081=2537t/3r  :8082=ERR  :8083=3467t/7r  | total=9887t
[2026-03-06 06:32:20.705]   96.1s | :8080=3382t/5r  :8081=1981t/2r  :8082=2150t/8r  :8083=2724t/5r  | total=10237t
[2026-03-06 06:32:22.706]   98.1s | :8080=ERR  :8081=697t/4r  :8082=895t/3r  :8083=6417t/2r  | total=8009t
[2026-03-06 06:32:24.708]  100.1s | :8080=3275t/4r  :8081=3930t/6r  :8082=879t/4r  :8083=ERR  | total=8084t
[2026-03-06 06:32:26.709]  102.1s | :8080=4590t/7r  :8081=2183t/3r  :8082=2144t/4r  :8083=5431t/6r  | total=14348t
[2026-03-06 06:32:28.711]  104.1s | :8080=2063t/4r  :8081=1997t/6r  :8082=1707t/4r  :8083=5484t/5r  | total=11251t
[2026-03-06 06:32:30.712]  106.1s | :8080=4797t/2r  :8081=5067t/4r  :8082=2879t/4r  :8083=4399t/2r  | total=17142t
[2026-03-06 06:32:32.714]  108.1s | :8080=4499t/2r  :8081=2805t/1r  :8082=5925t/2r  :8083=4559t/2r  | total=17788t
[2026-03-06 06:32:34.716]  110.1s | :8080=4078t/5r  :8081=ERR  :8082=ERR  :8083=3792t/5r  | total=7870t
[2026-03-06 06:32:36.718]  112.1s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:32:38.720]  114.1s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
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
Worker 0 (PID 181789) started
  Connecting to: http://localhost:8000
Worker 0 connected to server
Worker 0 terminated

Benchmark completed in 23.05s
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
  Request Throughput:  260.30 req/s
  Input Tok/s:         0.00 tok/s
  Output Tok/s:        0.00 tok/s
  Duration:            23.05s

----------------------------------------------------------------------
Latency (milliseconds):

  Metric    Mean      P50       P90       P95       P99       Count
  --------------------------------------------------------------------
  TTFT      N/A       N/A       N/A       N/A       N/A       0
  TPOT      N/A       N/A       N/A       N/A       N/A       0
  ITL       N/A       N/A       N/A       N/A       N/A       0
  E2E       57.09     54.95     72.54     79.01     94.08     6000

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
Worker 0 (PID 181811) started
  Connecting to: http://localhost:8000
Worker 0 connected to server
Worker 0 terminated

Benchmark completed in 22.53s
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
  Request Throughput:  266.36 req/s
  Input Tok/s:         0.00 tok/s
  Output Tok/s:        0.00 tok/s
  Duration:            22.53s

----------------------------------------------------------------------
Latency (milliseconds):

  Metric    Mean      P50       P90       P95       P99       Count
  --------------------------------------------------------------------
  TTFT      N/A       N/A       N/A       N/A       N/A       0
  TPOT      N/A       N/A       N/A       N/A       N/A       0
  ITL       N/A       N/A       N/A       N/A       N/A       0
  E2E       74.30     71.01     97.07     106.06    125.23    6000

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
Worker 0 (PID 181829) started
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
  Request Throughput:  270.49 req/s
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
  E2E       91.34     87.76     122.07    135.45    163.63    6000

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
Worker 0 (PID 181847) started
  Connecting to: http://localhost:8000
Worker 0 connected to server
Worker 0 terminated

Benchmark completed in 22.02s
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
  Request Throughput:  272.52 req/s
  Input Tok/s:         0.00 tok/s
  Output Tok/s:        0.00 tok/s
  Duration:            22.02s

----------------------------------------------------------------------
Latency (milliseconds):

  Metric    Mean      P50       P90       P95       P99       Count
  --------------------------------------------------------------------
  TTFT      N/A       N/A       N/A       N/A       N/A       0
  TPOT      N/A       N/A       N/A       N/A       N/A       0
  ITL       N/A       N/A       N/A       N/A       N/A       0
  E2E       108.62    103.65    150.47    169.63    214.42    6000

======================================================================
```

# PO2 using token
```
Workers: ['http://localhost:8080', 'http://localhost:8081', 'http://localhost:8082', 'http://localhost:8083']
--------------------------------------------------------------------------------
[2026-03-06 06:37:08.574]    0.0s | :8080=ERR  :8081=1069t/0r  :8082=0t/0r  :8083=1065t/0r  | total=2134t
[2026-03-06 06:37:10.576]    2.0s | :8080=4468t/2r  :8081=0t/0r  :8082=920t/1r  :8083=1891t/5r  | total=7279t
[2026-03-06 06:37:12.578]    4.0s | :8080=0t/0r  :8081=1847t/1r  :8082=0t/4r  :8083=0t/0r  | total=1847t
[2026-03-06 06:37:14.579]    6.0s | :8080=ERR  :8081=936t/1r  :8082=0t/0r  :8083=1346t/0r  | total=2282t
[2026-03-06 06:37:16.581]    8.0s | :8080=ERR  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:37:18.581]   10.0s | :8080=0t/3r  :8081=936t/2r  :8082=1244t/0r  :8083=765t/2r  | total=2945t
[2026-03-06 06:37:20.583]   12.0s | :8080=3855t/3r  :8081=2310t/1r  :8082=2496t/2r  :8083=0t/2r  | total=8661t
[2026-03-06 06:37:22.585]   14.0s | :8080=6623t/0r  :8081=1446t/1r  :8082=2555t/1r  :8083=0t/0r  | total=10624t
[2026-03-06 06:37:24.586]   16.0s | :8080=1677t/2r  :8081=2254t/1r  :8082=0t/1r  :8083=1049t/3r  | total=4980t
[2026-03-06 06:37:26.588]   18.0s | :8080=607t/1r  :8081=0t/0r  :8082=ERR  :8083=963t/4r  | total=1570t
[2026-03-06 06:37:28.590]   20.0s | :8080=2809t/2r  :8081=1086t/0r  :8082=0t/4r  :8083=1105t/2r  | total=5000t
[2026-03-06 06:37:30.592]   22.0s | :8080=0t/8r  :8081=1201t/0r  :8082=0t/0r  :8083=4098t/2r  | total=5299t
[2026-03-06 06:37:32.593]   24.0s | :8080=2861t/4r  :8081=1883t/1r  :8082=0t/0r  :8083=1951t/1r  | total=6695t
[2026-03-06 06:37:34.595]   26.0s | :8080=0t/3r  :8081=1252t/1r  :8082=2665t/3r  :8083=0t/0r  | total=3917t
[2026-03-06 06:37:36.597]   28.0s | :8080=2349t/1r  :8081=1083t/4r  :8082=2824t/2r  :8083=802t/0r  | total=7058t
[2026-03-06 06:37:38.599]   30.0s | :8080=0t/1r  :8081=3931t/6r  :8082=1041t/0r  :8083=0t/0r  | total=4972t
[2026-03-06 06:37:40.601]   32.0s | :8080=0t/0r  :8081=5175t/4r  :8082=3609t/2r  :8083=2295t/1r  | total=11079t
[2026-03-06 06:37:42.601]   34.0s | :8080=5241t/0r  :8081=1236t/0r  :8082=5183t/1r  :8083=0t/0r  | total=11660t
[2026-03-06 06:37:44.603]   36.0s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:37:46.605]   38.0s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:37:48.607]   40.0s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:37:50.609]   42.0s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:37:52.611]   44.0s | :8080=5639t/7r  :8081=0t/0r  :8082=858t/1r  :8083=3459t/2r  | total=9956t
[2026-03-06 06:37:54.613]   46.0s | :8080=819t/0r  :8081=5769t/7r  :8082=0t/0r  :8083=1488t/1r  | total=8076t
[2026-03-06 06:37:56.615]   48.0s | :8080=1277t/1r  :8081=0t/4r  :8082=4021t/7r  :8083=1803t/1r  | total=7101t
[2026-03-06 06:37:58.617]   50.0s | :8080=8377t/4r  :8081=3159t/2r  :8082=1971t/1r  :8083=989t/0r  | total=14496t
[2026-03-06 06:38:00.619]   52.0s | :8080=0t/0r  :8081=1177t/0r  :8082=ERR  :8083=0t/0r  | total=1177t
[2026-03-06 06:38:02.620]   54.0s | :8080=2276t/10r  :8081=1487t/2r  :8082=2879t/1r  :8083=0t/0r  | total=6642t
[2026-03-06 06:38:04.622]   56.0s | :8080=4821t/0r  :8081=2115t/4r  :8082=1248t/1r  :8083=5085t/1r  | total=13269t
[2026-03-06 06:38:06.624]   58.0s | :8080=0t/1r  :8081=1113t/10r  :8082=3208t/2r  :8083=3362t/3r  | total=7683t
[2026-03-06 06:38:08.626]   60.1s | :8080=2003t/6r  :8081=0t/1r  :8082=0t/0r  :8083=983t/0r  | total=2986t
[2026-03-06 06:38:10.627]   62.1s | :8080=1975t/1r  :8081=1775t/2r  :8082=4909t/6r  :8083=2573t/1r  | total=11232t
[2026-03-06 06:38:12.629]   64.1s | :8080=0t/0r  :8081=855t/0r  :8082=4667t/8r  :8083=0t/0r  | total=5522t
[2026-03-06 06:38:14.631]   66.1s | :8080=0t/0r  :8081=0t/3r  :8082=0t/0r  :8083=1104t/7r  | total=1104t
[2026-03-06 06:38:16.633]   68.1s | :8080=0t/0r  :8081=0t/8r  :8082=5964t/11r  :8083=2886t/2r  | total=8850t
[2026-03-06 06:38:18.633]   70.1s | :8080=4123t/1r  :8081=0t/5r  :8082=0t/0r  :8083=4526t/4r  | total=8649t
[2026-03-06 06:38:20.635]   72.1s | :8080=2773t/2r  :8081=5496t/3r  :8082=0t/0r  :8083=1155t/5r  | total=9424t
[2026-03-06 06:38:22.636]   74.1s | :8080=3889t/4r  :8081=3673t/1r  :8082=0t/10r  :8083=0t/0r  | total=7562t
[2026-03-06 06:38:24.638]   76.1s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:38:26.640]   78.1s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:38:28.642]   80.1s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:38:30.644]   82.1s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:38:32.645]   84.1s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:38:34.647]   86.1s | :8080=5353t/5r  :8081=3301t/7r  :8082=0t/0r  :8083=0t/1r  | total=8654t
[2026-03-06 06:38:36.649]   88.1s | :8080=1924t/1r  :8081=0t/0r  :8082=3906t/9r  :8083=928t/5r  | total=6758t
[2026-03-06 06:38:38.651]   90.1s | :8080=6521t/4r  :8081=0t/5r  :8082=0t/4r  :8083=0t/0r  | total=6521t
[2026-03-06 06:38:40.653]   92.1s | :8080=3543t/1r  :8081=5017t/4r  :8082=0t/7r  :8083=2618t/4r  | total=11178t
[2026-03-06 06:38:42.655]   94.1s | :8080=1058t/1r  :8081=0t/7r  :8082=3739t/7r  :8083=0t/0r  | total=4797t
[2026-03-06 06:38:44.657]   96.1s | :8080=0t/0r  :8081=0t/0r  :8082=ERR  :8083=0t/0r  | total=0t
[2026-03-06 06:38:46.659]   98.1s | :8080=6789t/4r  :8081=0t/0r  :8082=6323t/6r  :8083=941t/3r  | total=14053t
[2026-03-06 06:38:48.660]  100.1s | :8080=2085t/1r  :8081=7240t/5r  :8082=3954t/11r  :8083=0t/0r  | total=13279t
[2026-03-06 06:38:50.661]  102.1s | :8080=5100t/5r  :8081=2443t/1r  :8082=0t/0r  :8083=0t/10r  | total=7543t
[2026-03-06 06:38:52.663]  104.1s | :8080=2864t/4r  :8081=2358t/3r  :8082=0t/0r  :8083=8613t/8r  | total=13835t
[2026-03-06 06:38:54.665]  106.1s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/13r  | total=0t
[2026-03-06 06:38:56.667]  108.1s | :8080=1300t/0r  :8081=3802t/6r  :8082=12863t/5r  :8083=0t/0r  | total=17965t
[2026-03-06 06:38:58.669]  110.1s | :8080=1124t/1r  :8081=5382t/1r  :8082=9359t/15r  :8083=0t/0r  | total=15865t
[2026-03-06 06:39:00.671]  112.1s | :8080=1805t/1r  :8081=1044t/4r  :8082=9653t/11r  :8083=0t/0r  | total=12502t
[2026-03-06 06:39:02.673]  114.1s | :8080=988t/1r  :8081=0t/2r  :8082=3033t/7r  :8083=5374t/11r  | total=9395t
[2026-03-06 06:39:04.675]  116.1s | :8080=1155t/4r  :8081=0t/9r  :8082=1274t/0r  :8083=3693t/2r  | total=6122t
[2026-03-06 06:39:06.677]  118.1s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:39:08.679]  120.1s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:39:10.681]  122.1s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:39:12.683]  124.1s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:39:14.685]  126.1s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:39:16.687]  128.1s | :8080=ERR  :8081=890t/3r  :8082=1393t/0r  :8083=9491t/7r  | total=11774t
[2026-03-06 06:39:18.689]  130.1s | :8080=0t/15r  :8081=3238t/4r  :8082=1127t/0r  :8083=3216t/2r  | total=7581t
[2026-03-06 06:39:20.690]  132.1s | :8080=4683t/5r  :8081=4074t/5r  :8082=7538t/12r  :8083=1336t/2r  | total=17631t
[2026-03-06 06:39:22.692]  134.1s | :8080=0t/0r  :8081=0t/0r  :8082=0t/9r  :8083=2603t/8r  | total=2603t
[2026-03-06 06:39:24.694]  136.1s | :8080=0t/5r  :8081=5793t/17r  :8082=1915t/1r  :8083=0t/0r  | total=7708t
[2026-03-06 06:39:26.696]  138.1s | :8080=8068t/13r  :8081=645t/1r  :8082=3265t/1r  :8083=0t/0r  | total=11978t
[2026-03-06 06:39:28.697]  140.1s | :8080=2205t/5r  :8081=0t/0r  :8082=0t/9r  :8083=5076t/3r  | total=7281t
[2026-03-06 06:39:30.699]  142.1s | :8080=3823t/2r  :8081=1766t/8r  :8082=0t/0r  :8083=5291t/4r  | total=10880t
[2026-03-06 06:39:32.701]  144.1s | :8080=0t/0r  :8081=0t/1r  :8082=0t/13r  :8083=2091t/1r  | total=2091t
[2026-03-06 06:39:34.703]  146.1s | :8080=0t/0r  :8081=3666t/8r  :8082=0t/2r  :8083=10285t/9r  | total=13951t
[2026-03-06 06:39:36.705]  148.1s | :8080=789t/2r  :8081=1720t/6r  :8082=0t/11r  :8083=3665t/3r  | total=6174t
[2026-03-06 06:39:38.706]  150.1s | :8080=2159t/4r  :8081=1936t/3r  :8082=993t/1r  :8083=0t/0r  | total=5088t
[2026-03-06 06:39:40.708]  152.1s | :8080=4007t/4r  :8081=2991t/2r  :8082=0t/4r  :8083=1977t/5r  | total=8975t
[2026-03-06 06:39:42.709]  154.1s | :8080=6093t/6r  :8081=2307t/2r  :8082=0t/7r  :8083=0t/0r  | total=8400t
[2026-03-06 06:39:44.709]  156.1s | :8080=6893t/13r  :8081=0t/0r  :8082=3643t/6r  :8083=2335t/1r  | total=12871t
[2026-03-06 06:39:46.711]  158.1s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
[2026-03-06 06:39:48.712]  160.1s | :8080=0t/0r  :8081=0t/0r  :8082=0t/0r  :8083=0t/0r  | total=0t
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
Worker 0 (PID 182315) started
  Connecting to: http://localhost:8000
Worker 0 connected to server
Worker 0 terminated

Benchmark completed in 35.54s
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
  Request Throughput:  168.83 req/s
  Input Tok/s:         0.00 tok/s
  Output Tok/s:        0.00 tok/s
  Duration:            35.54s

----------------------------------------------------------------------
Latency (milliseconds):

  Metric    Mean      P50       P90       P95       P99       Count
  --------------------------------------------------------------------
  TTFT      N/A       N/A       N/A       N/A       N/A       0
  TPOT      N/A       N/A       N/A       N/A       N/A       0
  ITL       N/A       N/A       N/A       N/A       N/A       0
  E2E       87.98     77.50     158.63    182.62    211.49    6000

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
Worker 0 (PID 182337) started
  Connecting to: http://localhost:8000
Worker 0 connected to server
Worker 0 terminated

Benchmark completed in 32.93s
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
  Request Throughput:  182.20 req/s
  Input Tok/s:         0.00 tok/s
  Output Tok/s:        0.00 tok/s
  Duration:            32.93s

----------------------------------------------------------------------
Latency (milliseconds):

  Metric    Mean      P50       P90       P95       P99       Count
  --------------------------------------------------------------------
  TTFT      N/A       N/A       N/A       N/A       N/A       0
  TPOT      N/A       N/A       N/A       N/A       N/A       0
  ITL       N/A       N/A       N/A       N/A       N/A       0
  E2E       108.41    90.72     207.32    236.59    287.17    6000

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
Worker 0 (PID 182351) started
  Connecting to: http://localhost:8000
Worker 0 connected to server
Worker 0 terminated

Benchmark completed in 32.21s
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
  Request Throughput:  186.28 req/s
  Input Tok/s:         0.00 tok/s
  Output Tok/s:        0.00 tok/s
  Duration:            32.21s

----------------------------------------------------------------------
Latency (milliseconds):

  Metric    Mean      P50       P90       P95       P99       Count
  --------------------------------------------------------------------
  TTFT      N/A       N/A       N/A       N/A       N/A       0
  TPOT      N/A       N/A       N/A       N/A       N/A       0
  ITL       N/A       N/A       N/A       N/A       N/A       0
  E2E       132.34    113.39    253.54    288.62    367.28    6000

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
Worker 0 (PID 182368) started
  Connecting to: http://localhost:8000
Worker 0 connected to server
Worker 0 terminated

Benchmark completed in 32.19s
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
  Request Throughput:  186.37 req/s
  Input Tok/s:         0.00 tok/s
  Output Tok/s:        0.00 tok/s
  Duration:            32.19s

----------------------------------------------------------------------
Latency (milliseconds):

  Metric    Mean      P50       P90       P95       P99       Count
  --------------------------------------------------------------------
  TTFT      N/A       N/A       N/A       N/A       N/A       0
  TPOT      N/A       N/A       N/A       N/A       N/A       0
  ITL       N/A       N/A       N/A       N/A       N/A       0
  E2E       158.26    135.49    288.15    344.85    547.47    6000

======================================================================
```