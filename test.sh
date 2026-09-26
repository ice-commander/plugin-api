#!/bin/sh
# The contract and its examples. The checker that loads them lives in the closed
# tests checkout now, so building the libraries here is what lets its own tests
# run: see tests/plugin-check.
set -e
cd "$(dirname "$0")"
cargo build --workspace
./build-c.sh
cargo test --workspace
