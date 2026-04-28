#!/bin/bash
set -e

echo "Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

echo "Adding WASM target..."
rustup target add wasm32-unknown-unknown

echo "Installing Trunk..."
cargo install trunk

echo "Installing wasm-opt..."
npm install -g binaryen

echo "Building frontend..."
trunk build --release

echo "Optimizing WASM..."
WASM_FILE=$(find dist -name "*_bg.wasm" | head -1)
if [ -n "$WASM_FILE" ]; then
    wasm-opt --all-features -Oz -o "$WASM_FILE" "$WASM_FILE"
    echo "WASM optimized: $WASM_FILE"
fi

echo "Build complete. Output in dist/"
