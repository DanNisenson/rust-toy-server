#!/usr/bin/env bash
set -euo pipefail

URL="${1:-http://127.0.0.1:8080/}"
TOTAL="${2:-5000}"
CONCURRENCY="${3:-3000}"

echo "Sending $TOTAL requests to $URL with concurrency $CONCURRENCY"

start=$(date +%s)

seq 1 "$TOTAL" | xargs -n 1 -P "$CONCURRENCY" -I{} curl -sS -o /dev/null "$URL" >/dev/null

end=$(date +%s)
echo "Done in $((end - start))s"