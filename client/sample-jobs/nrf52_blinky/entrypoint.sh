#!/usr/bin/env bash

set -e

echo "-=-=-= START NRF SAMPLE JOB =-=-=-"

rustup target add thumbv7em-none-eabi
cargo build --release

if [[ -z "${HILLTOP_PROBE_SELECTOR:-}" ]]; then
    echo "HILLTOP_PROBE_SELECTOR was not provided by the runner" >&2
    exit 1
fi

if [[ -z "${PROBE_RS_CHIP:-}" ]]; then
    echo "PROBE_RS_CHIP was not provided in job.json's env" >&2
    exit 1
fi

set +e
timeout 20s probe-rs run \
    --chip "$PROBE_RS_CHIP" \
    --probe "$HILLTOP_PROBE_SELECTOR" \
    target/thumbv7em-none-eabi/release/nrf52_blinky
status=$?
set -e

# A continuously running blinky is expected to be terminated by timeout.
# Any other non-zero status is a real programming/test failure.
if [[ "$status" -ne 0 && "$status" -ne 124 ]]; then
    exit "$status"
fi

echo "-=-=-= END NRF SAMPLE JOB =-=-=-"
