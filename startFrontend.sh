#!/usr/bin/env sh

set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
CLIENT_DIR="$SCRIPT_DIR/client"

if ! command -v npm >/dev/null 2>&1; then
  echo "Error: npm is not installed or not available in PATH." >&2
  exit 1
fi

if [ ! -d "$CLIENT_DIR" ]; then
  echo "Error: client directory not found: $CLIENT_DIR" >&2
  exit 1
fi

cd "$CLIENT_DIR"

if [ ! -f "package.json" ]; then
  echo "Error: client package.json not found." >&2
  exit 1
fi

echo "Ensuring frontend dependencies..."
npm install

echo "Starting frontend development server..."
echo "Frontend URL: http://127.0.0.1:5173"
npm run dev -- --host 127.0.0.1 --port 5173
