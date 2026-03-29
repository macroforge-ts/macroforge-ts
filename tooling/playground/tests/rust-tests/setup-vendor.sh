#!/usr/bin/env bash
# Creates vendor symlinks for rust-tests crate dependencies.
# Reads paths from ../../../.env (relative to this script's location).
#
# Usage:  bash setup-vendor.sh   (from anywhere — uses BASH_SOURCE)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ENV_FILE="$SCRIPT_DIR/../../../.env"

if [[ ! -f "$ENV_FILE" ]]; then
    echo "error: $ENV_FILE not found" >&2
    exit 1
fi

# shellcheck source=/dev/null
source "$ENV_FILE"

VENDOR_DIR="$SCRIPT_DIR/vendor"
mkdir -p "$VENDOR_DIR"

ln -sfn "$MACROFORGE_TS_CRATE" "$VENDOR_DIR/macroforge_ts"
ln -sfn "$MACROFORGE_TS_QUOTE_CRATE" "$VENDOR_DIR/macroforge_ts_quote"
ln -sfn "$MACROFORGE_TS_SYN_CRATE" "$VENDOR_DIR/macroforge_ts_syn"
ln -sfn "$MACROFORGE_TS_MACROS_CRATE" "$VENDOR_DIR/macroforge_ts_macros"
