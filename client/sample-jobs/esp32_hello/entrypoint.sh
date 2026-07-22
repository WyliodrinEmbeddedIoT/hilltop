#!/usr/bin/env bash

set -e

echo "Hello, World!"
echo "stderr sample message" >&2
echo "file contents sample yey" > ./sample.txt

cargo run

timeout 10 cat /dev/ttyUSB0 || true
