#!/usr/bin/env bash

set -e

echo "-=-=-= START NRF SAMPLE JOB =-=-=-"

rustup target add thumbv7em-none-eabi
cargo build --release
timeout 20s cargo run --release || true

echo "-=-=-= END NRF SAMPLE JOB =-=-=-"