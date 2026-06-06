#!/usr/bin/env sh

set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
SERVER_DIR="$SCRIPT_DIR/server"

if ! command -v cargo >/dev/null 2>&1; then
  echo "Error: cargo is not installed or not available in PATH." >&2
  echo "Install Rust from https://www.rust-lang.org/tools/install before starting the backend." >&2
  exit 1
fi

if [ ! -d "$SERVER_DIR" ]; then
  echo "Error: server directory not found: $SERVER_DIR" >&2
  exit 1
fi

cd "$SERVER_DIR"

if [ ! -f ".env" ] && [ -f ".env.example" ]; then
  cp ".env.example" ".env"
  echo "Created server/.env from server/.env.example."
fi

echo "Starting Rust backend development server..."
cargo run
