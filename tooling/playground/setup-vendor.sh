#!/usr/bin/env bash
# Creates shared vendor symlinks for playground subprojects.
# Reads paths from ../../.env (relative to this script's location).
#
# Usage:  ./setup-vendor.sh          (from tooling/playground/)
#    or:  bash setup-vendor.sh       (from anywhere — uses BASH_SOURCE)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ENV_FILE="$SCRIPT_DIR/../.env"

if [[ ! -f "$ENV_FILE" ]]; then
    echo "error: $ENV_FILE not found" >&2
    exit 1
fi

# shellcheck source=/dev/null
source "$ENV_FILE"

VENDOR_DIR="$SCRIPT_DIR/vendor"
mkdir -p "$VENDOR_DIR"

# macroforge_ts crate (provides the 'macroforge' npm package)
ln -sfn "$MACROFORGE_TS_CRATE" "$VENDOR_DIR/macroforge_ts"

# NPM packages
ln -sfn "$VITE_PLUGIN_PKG" "$VENDOR_DIR/vite-plugin"
ln -sfn "$SHARED_PKG" "$VENDOR_DIR/shared"
ln -sfn "$SVELTE_PREPROCESSOR_PKG" "$VENDOR_DIR/svelte-preprocessor"

# Add more shared vendor links here as needed:
# ln -sfn "$MACROFORGE_TS_MACROS_CRATE" "$VENDOR_DIR/macroforge_ts_macros"

echo "vendor symlinks created in $VENDOR_DIR"
ls -la "$VENDOR_DIR"
