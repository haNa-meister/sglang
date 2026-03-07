# Goal
Implement a load aware policy sgl-model gateway.
The key is for a prototyping and confirm that there will be a gain for prefill only traffic compare to po2 and round robin.

## Implementation requirement
1. The load info still come from LoadMonitor, which has num_request which will call /get_load like this:
```
[{"rid":null,"http_worker_ipc":null,"dp_rank":null,"num_reqs":0,"num_waiting_reqs":0,"num_tokens":0,"ts_tic":2359325.135359237}]
```
Confirming if num_reqs means running + waiting request. If no so, use num_reqs + num_waiting_reqs as a condition to filter least load server.

2. Always pick the least load server to send request.

3. Policy name: load-aware

4. The current PO2 policy has an issue for prefill only requests that:
a. The polling interval is too long.
b. During the polling period the router didn't keep the local requests which will cause the load map inaccurate.
So, we set polling interval as 100ms, and have a local thread safe counter when a request sent to sever a, local counter add on it.
But when we poll the load from server with real server load data, we overwrite it.
The interesting point is do we need to decrease the count if the request end for keep the load registry accurate? My instinct is yes, since normal embedding could only have 20ms, so for accuracy of load map, we need to increase and decrease them between poll, however, let's assume the get_load always get the true data and we do the overright (Here is a tricky thing that if the request start from poll 0 and end at poll 1, how should we handle it?).

5. Still keep and enhance the pipeline stage data, I want to make sure the overhead is not overwhelming.