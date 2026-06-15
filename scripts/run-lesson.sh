#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 <cargo-package-name>"
  echo "Example: $0 lesson_01_foundation"
  exit 1
fi

cargo run -p "$1"
