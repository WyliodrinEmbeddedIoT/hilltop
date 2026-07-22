#!/usr/bin/env bash

# Entrypoint script for hilltop runner

set -euo pipefail

SOCKET="/var/run/hilltop.sock"

FIFO=$(mktemp -u)
mkfifo "$FIFO"

# Keep the connection open by holding the FIFO open for writing in the background
socat UNIX-CONNECT:$SOCKET PIPE:$FIFO &
SOCAT_PID=$!

# Open the FIFO on fd 3 to keep the path alive
exec 3>"$FIFO"
# rm "$FIFO"  # unlink the path; fd 3 keeps it alive

send() {
    echo "Sending: $1"
    echo "$1" >&3
}

cleanup() {
    sleep 5
    echo "Cleaning up..."
    exec 3>&-
    # wait $SOCAT_PID || true
    echo "Cleanup complete."
}
trap cleanup EXIT

send '{"status": "container-started"}'

mkdir -p /artifacts
cd /workspace

# Wait for job-data.zip to be copied over
timeout 120s bash -c 'until [[ -f ./job-data.zip ]]; do sleep 0.1; done' || {
    send '{"status": "error", "message": "job-data.zip missing"}' 
    exit 1
}

# Prepare job
unzip ./job-data.zip -d ./

send '{"status": "job-ready"}' 

# Run
send '{"status": "job-started"}'

# Redirect stdout to /artifacts/stdout.log and stderr to /artifacts/stderr.log
./entrypoint.sh > /artifacts/stdout.log 2> /artifacts/stderr.log || true

send '{"status": "job-completed"}'

send '{"status": "container-completed"}'

exit 0
