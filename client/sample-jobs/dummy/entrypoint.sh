#!/usr/bin/env bash

set -euo pipefail

echo "Hello, stdout!"
echo "Hello, stderr!" >&2
echo "Hello, hello.txt!" > hello.txt