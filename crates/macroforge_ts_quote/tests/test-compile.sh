#!/bin/bash
# Test script for isolating infinite compilation loops
# Usage: ./test-compile.sh [timeout_seconds]

TIMEOUT=${1:-60}
TARGET_DIR="/tmp/quote_test_$(date +%s)"

echo "=== Testing compilation with ${TIMEOUT}s timeout ==="
echo "Target dir: $TARGET_DIR"
echo ""

# Run cargo check in background
MF_DEBUG_PARSER=1 CARGO_TARGET_DIR="$TARGET_DIR" cargo check 2>&1 &
PID=$!

# Wait for timeout or completion
ELAPSED=0
while [ $ELAPSED -lt $TIMEOUT ]; do
    if ! ps -p $PID > /dev/null 2>&1; then
        wait $PID
        EXIT_CODE=$?
        echo ""
        echo "=== Completed in ${ELAPSED}s with exit code: $EXIT_CODE ==="
        exit $EXIT_CODE
    fi
    sleep 1
    ELAPSED=$((ELAPSED + 1))

    # Show progress every 10 seconds
    if [ $((ELAPSED % 10)) -eq 0 ]; then
        echo "... still compiling after ${ELAPSED}s"
    fi
done

echo ""
echo "=== TIMEOUT after ${TIMEOUT}s - likely infinite loop! ==="
echo "Killing process $PID..."
kill $PID 2>/dev/null
sleep 1
kill -9 $PID 2>/dev/null

# Show what was being compiled
echo ""
echo "Last cargo output:"
ps aux | grep -E "rustc|cargo" | grep -v grep | head -5

exit 1
