#!/bin/bash
# Start PD disaggregated sglang servers + router
# Usage: ./start_pd.sh [num_prefill] [num_decode]
# Example: ./start_pd.sh 2 2  (2 prefill on GPU 0,1 + 2 decode on GPU 2,3)

NUM_PREFILL=${1:-2}
NUM_DECODE=${2:-2}
TOTAL_GPUS=$((NUM_PREFILL + NUM_DECODE))

BASE_PORT=8080
ROUTER_PORT=8000
PID_FILE="pd_pids.txt"
LOG_DIR="logs"
MODEL_PATH="/shared/public/elr-models/Qwen/Qwen3-4B"

# Switch to the directory of the current script
cd "$(dirname "${BASH_SOURCE[0]}")"

mkdir -p "$LOG_DIR"
rm -f "$LOG_DIR"/pd_*.log
rm -f "$PID_FILE"

echo "================================================"
echo "PD Disaggregated Setup: ${NUM_PREFILL}P + ${NUM_DECODE}D"
echo "================================================"
echo ""

# Collect URLs for router
PREFILL_URLS=""
DECODE_URLS=""

# --- Start Prefill Workers ---
echo "Starting $NUM_PREFILL prefill worker(s)..."
for (( i=0; i<NUM_PREFILL; i++ )); do
    port=$((BASE_PORT + i))
    gpu_id=$i
    log_file="$LOG_DIR/pd_prefill_${port}.log"

    nohup python -m sglang.launch_server \
        --model-path "$MODEL_PATH" \
        --served-model-name qwen \
        --disaggregation-mode prefill \
        --disaggregation-transfer-backend mooncake \
        --base-gpu-id "$gpu_id" \
        --log-level debug \
        --port "$port" \
        > "$log_file" 2>&1 &
    pid=$!
    echo "$pid $port prefill" >> "$PID_FILE"
    echo "  [PREFILL] GPU $gpu_id -> port $port (pid $pid, log: $log_file)"

    if [ -z "$PREFILL_URLS" ]; then
        PREFILL_URLS="http://localhost:$port"
    else
        PREFILL_URLS="$PREFILL_URLS http://localhost:$port"
    fi
done

# --- Start Decode Workers ---
echo ""
echo "Starting $NUM_DECODE decode worker(s)..."
for (( i=0; i<NUM_DECODE; i++ )); do
    port=$((BASE_PORT + NUM_PREFILL + i))
    gpu_id=$((NUM_PREFILL + i))
    log_file="$LOG_DIR/pd_decode_${port}.log"

    nohup python -m sglang.launch_server \
        --model-path "$MODEL_PATH" \
        --served-model-name qwen \
        --disaggregation-mode decode \
        --disaggregation-transfer-backend mooncake \
        --base-gpu-id "$gpu_id" \
        --log-level debug \
        --port "$port" \
        > "$log_file" 2>&1 &
    pid=$!
    echo "$pid $port decode" >> "$PID_FILE"
    echo "  [DECODE]  GPU $gpu_id -> port $port (pid $pid, log: $log_file)"

    if [ -z "$DECODE_URLS" ]; then
        DECODE_URLS="http://localhost:$port"
    else
        DECODE_URLS="$DECODE_URLS http://localhost:$port"
    fi
done

echo ""
echo "================================================"
echo "Waiting for workers to start..."
echo "================================================"

# Wait for all workers to be healthy
ALL_PORTS=""
for (( i=0; i<TOTAL_GPUS; i++ )); do
    ALL_PORTS="$ALL_PORTS $((BASE_PORT + i))"
done

MAX_WAIT=300
WAITED=0
while [ $WAITED -lt $MAX_WAIT ]; do
    ALL_READY=true
    for port in $ALL_PORTS; do
        if ! curl -s "http://localhost:$port/health" > /dev/null 2>&1; then
            ALL_READY=false
            break
        fi
    done
    if $ALL_READY; then
        echo "All workers are healthy after ${WAITED}s."
        break
    fi
    sleep 5
    WAITED=$((WAITED + 5))
    echo "  Waiting... (${WAITED}s / ${MAX_WAIT}s)"
done

if [ $WAITED -ge $MAX_WAIT ]; then
    echo "WARNING: Timed out waiting for workers. Check logs in $LOG_DIR/"
    echo "Starting router anyway..."
fi

# --- Start Router ---
echo ""
echo "================================================"
echo "Starting PD Router on port $ROUTER_PORT..."
echo "================================================"
echo "  Prefill URLs: $PREFILL_URLS"
echo "  Decode URLs:  $DECODE_URLS"

ROUTER_LOG="$LOG_DIR/pd_router_${ROUTER_PORT}.log"

nohup python -m sglang_router \
    --pd-disaggregation \
    --prefill-urls $PREFILL_URLS \
    --decode-urls $DECODE_URLS \
    --prefill-policy cache_aware \
    --decode-policy round_robin \
    --port "$ROUTER_PORT" \
    > "$ROUTER_LOG" 2>&1 &
router_pid=$!
echo "$router_pid $ROUTER_PORT router" >> "$PID_FILE"
echo "  Router pid: $router_pid, log: $ROUTER_LOG"

# Wait for router
sleep 3
if curl -s "http://localhost:$ROUTER_PORT/v1/models" > /dev/null 2>&1; then
    echo ""
    echo "Router is ready!"
else
    echo ""
    echo "Router may still be starting. Check $ROUTER_LOG"
fi

echo ""
echo "================================================"
echo "PD Setup Complete"
echo "================================================"
echo "  Prefill workers: $NUM_PREFILL (ports $BASE_PORT - $((BASE_PORT + NUM_PREFILL - 1)))"
echo "  Decode workers:  $NUM_DECODE (ports $((BASE_PORT + NUM_PREFILL)) - $((BASE_PORT + TOTAL_GPUS - 1)))"
echo "  Router:          port $ROUTER_PORT"
echo "  Logs:            $LOG_DIR/pd_*.log"
echo "  PID file:        $PID_FILE"
echo ""
echo "Test: curl http://localhost:$ROUTER_PORT/v1/chat/completions -H 'Content-Type: application/json' -d '{\"model\":\"qwen\",\"messages\":[{\"role\":\"user\",\"content\":\"Hello\"}]}'"
echo ""
echo "Stop: ./stop_pd.sh"
