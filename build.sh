#!/bin/bash
set -e

echo "Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

echo "Adding WASM target..."
rustup target add wasm32-unknown-unknown

echo "Installing Trunk..."
cargo install trunk

echo "Building frontend..."
trunk build --release

echo "Build complete. Output in dist/"
