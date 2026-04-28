#!/bin/bash
set -e

echo "Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

echo "Adding WASM target..."
rustup target add wasm32-unknown-unknown

echo "Installing Trunk..."
cargo install trunk

# Wrap system wasm-opt to inject --all-features before Trunk calls it.
# Rust WASM uses bulk-memory (memory.copy/fill) and nontrapping-float-to-int
# opcodes that wasm-opt rejects without these flags.
WASM_OPT_BIN=$(which wasm-opt 2>/dev/null || true)
if [ -n "$WASM_OPT_BIN" ]; then
    echo "Wrapping wasm-opt at $WASM_OPT_BIN to inject --all-features..."
    cp "$WASM_OPT_BIN" "${WASM_OPT_BIN}.real"
    printf '#!/bin/bash\nexec "%s.real" --all-features "$@"\n' "$WASM_OPT_BIN" > "$WASM_OPT_BIN"
    chmod +x "$WASM_OPT_BIN"
fi

echo "Building frontend..."
trunk build --release

echo "Build complete. Output in dist/"
