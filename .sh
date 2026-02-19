#!/usr/bin/env bash
set -uo pipefail
cargo +nightly fmt
cargo clippy
cargo check
# cargo run
./view
