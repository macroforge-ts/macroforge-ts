#!/bin/bash
#
# E2E Test Runner: Zed Extension Startup
#
# This script launches Zed with a test project and verifies
# that extensions load successfully by monitoring the log file.
#
# Requirements:
# - Zed must NOT be running before the test starts
# - The test will start Zed, run checks, and close it when done
#
# Usage:
#   ./tests/e2e/run-zed-test.sh [--verbose] [--timeout SECONDS]
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
TEST_PROJECT="$SCRIPT_DIR/fixtures/ts-project"
ZED_LOG="$HOME/Library/Logs/Zed/Zed.log"

# Default options
VERBOSE=false
TIMEOUT=30
DRY_RUN=false

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --verbose|-v)
            VERBOSE=true
            shift
            ;;
        --timeout|-t)
            TIMEOUT="$2"
            shift 2
            ;;
        --dry-run|-n)
            DRY_RUN=true
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  -v, --verbose    Show detailed log output"
            echo "  -t, --timeout N  Maximum wait time in seconds (default: 30)"
            echo "  -n, --dry-run    Check prerequisites without launching Zed"
            echo "  -h, --help       Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[PASS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[FAIL]${NC} $1"
}

# Check if Zed is running (process name is lowercase "zed")
is_zed_running() {
    pgrep -x "zed" > /dev/null 2>&1
}

# Get Zed PID if running
get_zed_pid() {
    pgrep -x "zed" 2>/dev/null | head -1
}

# Kill Zed process gracefully
kill_zed() {
    if ! is_zed_running; then
        return 0
    fi

    local pid
    pid=$(get_zed_pid)
    if [[ -z "$pid" ]]; then
        return 0
    fi

    log_info "Stopping Zed (PID: $pid)..."

    # Try graceful shutdown first (SIGTERM)
    kill -TERM "$pid" 2>/dev/null || true

    # Wait up to 5 seconds for graceful shutdown
    local wait_count=0
    while is_zed_running && [[ $wait_count -lt 10 ]]; do
        sleep 0.5
        ((wait_count++))
    done

    # Force kill if still running
    if is_zed_running; then
        log_warn "Zed didn't exit gracefully, force killing..."
        pkill -9 -x "zed" 2>/dev/null || true
        sleep 1
    fi

    if is_zed_running; then
        log_error "Failed to kill Zed"
        return 1
    fi

    log_info "Zed stopped successfully"
    return 0
}

# Check prerequisites
check_prerequisites() {
    if ! command -v zed &> /dev/null; then
        log_error "Zed is not installed or not in PATH"
        exit 1
    fi

    if [[ ! -d "$TEST_PROJECT" ]]; then
        log_error "Test project not found at $TEST_PROJECT"
        exit 1
    fi

    if [[ ! -f "$ZED_LOG" ]]; then
        log_warn "Zed log file not found at $ZED_LOG (will be created on first run)"
    fi
}

# Get current log file size
get_log_offset() {
    if [[ -f "$ZED_LOG" ]]; then
        stat -f%z "$ZED_LOG" 2>/dev/null || echo "0"
    else
        echo "0"
    fi
}

# Read new log entries since offset
read_new_logs() {
    local offset=$1
    if [[ -f "$ZED_LOG" ]]; then
        local current_size
        current_size=$(get_log_offset)
        if [[ $current_size -gt $offset ]]; then
            tail -c +$((offset + 1)) "$ZED_LOG" 2>/dev/null
        fi
    fi
}

# Main test
main() {
    local test_passed=true
    local zed_started_by_test=false

    log_info "=== Zed Extension E2E Test ==="
    log_info "Test project: $TEST_PROJECT"
    log_info "Log file: $ZED_LOG"
    log_info "Timeout: ${TIMEOUT}s"
    if [[ "$DRY_RUN" == "true" ]]; then
        log_info "Mode: DRY RUN (prerequisites check only)"
    fi
    echo ""

    check_prerequisites

    # Check if Zed is already running - FAIL if so
    if is_zed_running; then
        log_error "Zed is already running!"
        log_error "Please close Zed before running this test."
        log_info "You can close Zed with: pkill -x Zed"
        exit 1
    fi

    if [[ "$DRY_RUN" == "true" ]]; then
        log_success "Prerequisites check passed"
        log_info "Zed path: $(which zed)"
        log_info "Zed is not running: OK"
        log_info "Test project exists: $TEST_PROJECT"
        log_info "Log file: $([[ -f "$ZED_LOG" ]] && echo "exists" || echo "will be created")"
        echo ""
        log_success "=== Dry Run Complete ==="
        exit 0
    fi

    # Record initial log position
    local initial_offset
    initial_offset=$(get_log_offset)
    log_info "Initial log offset: $initial_offset bytes"

    # Launch Zed
    log_info "Starting Zed..."
    zed "$TEST_PROJECT"

    # Wait for Zed.app to actually start (the CLI exits immediately)
    local start_wait=0
    while ! is_zed_running && [[ $start_wait -lt 15 ]]; do
        sleep 1
        ((start_wait++))
        if [[ $((start_wait % 3)) -eq 0 ]]; then
            log_info "Waiting for Zed to start... (${start_wait}s)"
        fi
    done

    if ! is_zed_running; then
        log_error "Zed failed to start after ${start_wait}s"
        exit 1
    fi

    zed_started_by_test=true
    local zed_pid
    zed_pid=$(get_zed_pid)
    log_info "Zed started with PID: $zed_pid"

    # Give Zed time to initialize extensions
    log_info "Waiting for Zed to initialize..."
    sleep 5

    # Monitor logs for startup indicators
    log_info "Monitoring logs for startup indicators..."

    local elapsed=0
    local extensions_loaded=false
    local vtsls_started=false
    local new_logs=""

    while [[ $elapsed -lt $TIMEOUT ]]; do
        new_logs=$(read_new_logs "$initial_offset")

        if [[ -n "$new_logs" ]]; then
            if [[ "$VERBOSE" == "true" ]]; then
                echo "$new_logs" | grep -E "(extension|vtsls|language server)" | tail -10 || true
            fi

            # Check for extensions loaded
            if echo "$new_logs" | grep -q "extensions updated. loading"; then
                extensions_loaded=true
            fi

            # Check for vtsls starting
            if echo "$new_logs" | grep -q "starting language server process.*vtsls"; then
                vtsls_started=true
            fi

            # Early exit if both conditions met
            if [[ "$extensions_loaded" == "true" && "$vtsls_started" == "true" ]]; then
                log_info "vtsls started, waiting 5s to check stability..."
                sleep 5
                # Re-read logs to check for crashes that happened during stabilization
                new_logs=$(read_new_logs "$initial_offset")
                break
            fi
        fi

        sleep 1
        ((elapsed++))

        # Show progress every 5 seconds
        if [[ $((elapsed % 5)) -eq 0 ]]; then
            log_info "Waiting... (${elapsed}s) extensions=$extensions_loaded vtsls=$vtsls_started"
        fi
    done

    echo ""
    log_info "=== Results ==="

    # Final log read for error checking
    new_logs=$(read_new_logs "$initial_offset")

    # Report results
    if [[ "$extensions_loaded" == "true" ]]; then
        log_success "Extensions loaded successfully"
    else
        log_error "Extensions did not load within timeout"
        test_passed=false
    fi

    if [[ "$vtsls_started" == "true" ]]; then
        log_success "vtsls language server started"
    else
        log_warn "vtsls language server did not start"
    fi

    # Save diagnostics to file
    DIAGNOSTICS_FILE="$SCRIPT_DIR/zed-diagnostics.log"
    echo "$new_logs" > "$DIAGNOSTICS_FILE"
    log_info "Zed diagnostics saved to: $DIAGNOSTICS_FILE"

    # Check for critical errors in logs
    if [[ -n "$new_logs" ]]; then
        local ts_macro_errors
        ts_macro_errors=$(echo "$new_logs" | grep -c "No extension manifest found for extension ts-" 2>/dev/null || echo "0")
        ts_macro_errors="${ts_macro_errors//[^0-9]/}"
        ts_macro_errors="${ts_macro_errors:-0}"

        if [[ "$ts_macro_errors" -gt 0 ]]; then
            log_warn "macroforge extension manifest not found (extension may not be installed)"
        fi

        # Check for vtsls crashes - this indicates the tsserver-plugin-macroforge crashed vtsls
        local vtsls_crashes
        vtsls_crashes=$(echo "$new_logs" | grep -c "Server reset the connection" 2>/dev/null || echo "0")
        vtsls_crashes="${vtsls_crashes//[^0-9]/}"
        vtsls_crashes="${vtsls_crashes:-0}"

        local lsp_header_errors
        lsp_header_errors=$(echo "$new_logs" | grep -c "cannot read LSP message headers" 2>/dev/null || echo "0")
        lsp_header_errors="${lsp_header_errors//[^0-9]/}"
        lsp_header_errors="${lsp_header_errors:-0}"

        if [[ "$vtsls_crashes" -gt 0 ]] || [[ "$lsp_header_errors" -gt 0 ]]; then
            log_error "vtsls crashed! ($vtsls_crashes connection resets, $lsp_header_errors header errors)"
            log_error "This indicates the tsserver-plugin-macroforge is crashing vtsls"
            if [[ "$VERBOSE" == "true" ]]; then
                echo ""
                echo "=== vtsls crash details ==="
                echo "$new_logs" | grep -E "(Server reset|cannot read LSP|vtsls failed)" | head -20
                echo "==========================="
            fi
            test_passed=false
        else
            log_success "vtsls running stable (no crashes detected)"
        fi
    fi

    echo ""

    # Cleanup: Close Zed if we started it
    if [[ "$zed_started_by_test" == "true" ]]; then
        log_info "Cleaning up: closing Zed..."
        kill_zed
    fi

    echo ""

    if [[ "$test_passed" == "true" ]]; then
        log_success "=== E2E Test PASSED ==="
        exit 0
    else
        log_error "=== E2E Test FAILED ==="
        exit 1
    fi
}

# Ensure cleanup on script exit (in case of errors)
cleanup_on_exit() {
    if is_zed_running; then
        log_info "Cleanup: ensuring Zed is closed..."
        kill_zed
    fi
}

trap cleanup_on_exit EXIT

main
