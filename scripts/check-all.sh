#!/usr/bin/env bash
set -euo pipefail

cargo check --workspace
cargo clippy --workspace -- -D warnings
