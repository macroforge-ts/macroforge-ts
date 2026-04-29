#!/usr/bin/env bash
set -euo pipefail

mkdir -p "$HOME/.cargo/bin"

RUSTUP_BIN="$HOME/.pixi/envs/rustup/bin/rustup"

if [ ! -x "$RUSTUP_BIN" ]; then
    echo "Installing global rustup from brads-forge..."
    pixi global install -c https://prefix.dev/brads-forge rustup
fi

for name in cargo rustc rustdoc cargo-clippy cargo-fmt clippy-driver rustfmt rust-gdb rust-lldb; do
    ln -sf "$RUSTUP_BIN" "$HOME/.cargo/bin/$name"
done

if ! grep -q 'RUSTUP_HOME=.*pixi/envs/rustup' "$HOME/.zshenv" 2>/dev/null; then
    echo 'export RUSTUP_HOME="$HOME/.pixi/envs/rustup/.rustup"' >> "$HOME/.zshenv"
fi

echo "Rust toolchain proxies installed."
echo "Open a new shell (or 'source ~/.zshenv') to pick up RUSTUP_HOME."
