#!/bin/bash
# Stop all PD sglang servers and router
PID_FILE="pd_pids.txt"

cd "$(dirname "${BASH_SOURCE[0]}")"

if [ ! -f "$PID_FILE" ]; then
    echo "No PID file found. Nothing to stop."
    exit 0
fi

echo "Stopping PD servers and router..."
while read -r pid port role; do
    if kill -0 "$pid" 2>/dev/null; then
        kill "$pid"
        echo "  Killed $role on port $port (pid $pid)"
    else
        echo "  $role on port $port (pid $pid) already stopped"
    fi
done < "$PID_FILE"

rm -f "$PID_FILE"
echo "Done."
