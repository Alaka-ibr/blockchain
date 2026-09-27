#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

echo "==> Checking formatting..."
cargo fmt --all -- --check

echo "==> Running clippy..."
cargo clippy --all-targets -- -D warnings

echo "==> Running test suite..."
cargo test --workspace

if ! command -v stellar >/dev/null 2>&1; then
  echo "Error: 'stellar' CLI is not installed or not available in PATH." >&2
  echo "Install it via: cargo install --locked stellar-cli --version ^27" >&2
  exit 1
fi

echo "==> Building contract WASM..."
stellar contract build

# Locate compiled WASM
WASM="target/wasm32v1-none/release/stellar_tickets_ticketing.wasm"
if [ ! -f "$WASM" ]; then
  WASM="target/wasm32-unknown-unknown/release/stellar_tickets_ticketing.wasm"
fi

if [ ! -f "$WASM" ]; then
  echo "Error: Built WASM file not found at '$WASM'." >&2
  exit 1
fi

echo "==> Optimizing contract WASM..."
stellar contract optimize --wasm "$WASM"

OPT_WASM="${WASM%.wasm}.optimized.wasm"
if [ -f "$OPT_WASM" ]; then
  FINAL_WASM="$OPT_WASM"
else
  FINAL_WASM="$WASM"
fi

WASM_HASH=$(sha256sum "$FINAL_WASM" | awk '{print $1}')
echo "=================================================="
echo "Contract build & optimization successful!"
echo "WASM File: $FINAL_WASM"
echo "WASM Hash: $WASM_HASH"
echo "=================================================="
