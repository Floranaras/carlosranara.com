#!/bin/bash
set -e

echo "Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

echo "Adding WASM target..."
rustup target add wasm32-unknown-unknown

echo "Installing Trunk..."
cargo install trunk

# Trunk 0.21 downloads its own wasm-opt to ~/.cache/trunk/ and calls it without
# --enable-bulk-memory, rejecting Rust's bulk-memory and nontrapping-float-to-int
# opcodes. First pass primes the cache; we then wrap the downloaded binary to
# inject --all-features before the real build.
echo "Priming Trunk tool cache..."
trunk build --release || true

CACHE_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/trunk"
TRUNK_WASM_OPT=$(find "$CACHE_DIR" -name "wasm-opt" -not -name "*.real" -type f 2>/dev/null | head -1)
if [ -n "$TRUNK_WASM_OPT" ]; then
    echo "Patching $TRUNK_WASM_OPT to inject --all-features..."
    cp "$TRUNK_WASM_OPT" "${TRUNK_WASM_OPT}.real"
    printf '#!/bin/sh\nexec "%s.real" --all-features "$@"\n' "$TRUNK_WASM_OPT" > "$TRUNK_WASM_OPT"
    chmod +x "$TRUNK_WASM_OPT"
else
    echo "Warning: Trunk wasm-opt not found in $CACHE_DIR, build may fail."
fi

echo "Building frontend..."
trunk build --release

echo "Build complete. Output in dist/"
