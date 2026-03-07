#!/usr/bin/env python3
"""Monitor worker load via /get_load HTTP API and log to CSV for graphing."""

import argparse
import csv
import json
import sys
import time
from datetime import datetime
from urllib.request import urlopen, Request
from urllib.error import URLError


def fetch_load(url, timeout=2):
    """Fetch load from a single worker. Returns dict or None on failure."""
    try:
        req = Request(f"{url}/get_load")
        with urlopen(req, timeout=timeout) as resp:
            data = json.loads(resp.read().decode())
            if isinstance(data, list) and len(data) > 0:
                return data[0]
    except (URLError, json.JSONDecodeError, Exception):
        return None


def main():
    parser = argparse.ArgumentParser(description="Monitor sglang worker loads")
    parser.add_argument(
        "--workers",
        nargs="+",
        required=True,
        help="Worker URLs (e.g., http://localhost:8080 http://localhost:8081)",
    )
    parser.add_argument(
        "--interval",
        type=float,
        default=0.5,
        help="Polling interval in seconds (default: 0.5)",
    )
    parser.add_argument(
        "--duration",
        type=float,
        default=60,
        help="Total monitoring duration in seconds (default: 60)",
    )
    parser.add_argument(
        "--output",
        type=str,
        default="load_monitor.csv",
        help="Output CSV file (default: load_monitor.csv)",
    )
    args = parser.parse_args()

    # CSV header
    fieldnames = ["timestamp", "elapsed_s"]
    for url in args.workers:
        name = url.split(":")[-1]  # use port as short name
        fieldnames.extend([
            f"w{name}_num_reqs",
            f"w{name}_num_waiting_reqs",
            f"w{name}_num_tokens",
        ])

    with open(args.output, "w", newline="") as csvfile:
        writer = csv.DictWriter(csvfile, fieldnames=fieldnames)
        writer.writeheader()

        start_time = time.time()
        print(f"Monitoring {len(args.workers)} workers for {args.duration}s, "
              f"interval={args.interval}s, output={args.output}")
        print(f"Workers: {args.workers}")
        print("-" * 80)

        try:
            while time.time() - start_time < args.duration:
                now = time.time()
                elapsed = now - start_time
                row = {
                    "timestamp": datetime.now().strftime("%Y-%m-%d %H:%M:%S.%f")[:-3],
                    "elapsed_s": f"{elapsed:.3f}",
                }

                status_parts = []
                for url in args.workers:
                    name = url.split(":")[-1]
                    data = fetch_load(url)
                    if data:
                        num_reqs = data.get("num_reqs", -1)
                        num_waiting = data.get("num_waiting_reqs", -1)
                        num_tokens = data.get("num_tokens", -1)
                        row[f"w{name}_num_reqs"] = num_reqs
                        row[f"w{name}_num_waiting_reqs"] = num_waiting
                        row[f"w{name}_num_tokens"] = num_tokens
                        status_parts.append(f":{name}={num_reqs}r/{num_waiting}w")
                    else:
                        row[f"w{name}_num_reqs"] = -1
                        row[f"w{name}_num_waiting_reqs"] = -1
                        row[f"w{name}_num_tokens"] = -1
                        status_parts.append(f":{name}=ERR")

                writer.writerow(row)
                csvfile.flush()

                print(f"[{row['timestamp']}] {elapsed:6.1f}s | {'  '.join(status_parts)}")

                sleep_time = args.interval - (time.time() - now)
                if sleep_time > 0:
                    time.sleep(sleep_time)

        except KeyboardInterrupt:
            print("\nStopped by user.")

    print(f"\nDone. Data saved to {args.output}")


if __name__ == "__main__":
    main()
