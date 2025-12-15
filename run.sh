#!/bin/bash
set -e

# Linux Convenience Launcher
# Usage: ./run.sh [policy_path] [target_binary]

POLICY=${1:-"scripting/lua/policy.lua"}
TARGET=${2:-"demo/opaque_client/client"}

if [ ! -f "build/hermes_runner" ]; then
    echo "[INFO] Building HERMES..."
    make all
fi

echo "[HERMES] Launching with Policy: $POLICY"
./build/hermes_runner --policy "$POLICY" --target "$TARGET"
