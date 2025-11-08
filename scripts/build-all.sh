#!/bin/bash
# Build binaries for all platforms (where possible)
# Note: macOS requires a macOS machine or CI

set -e

echo "Building Linux binary..."
cargo +nightly build --release
echo "✅ Linux: target/release/nust"

if command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo "Building Windows binary..."
    cargo +nightly build --release --target x86_64-pc-windows-gnu
    echo "✅ Windows: target/x86_64-pc-windows-gnu/release/nust.exe"
else
    echo "⚠️  Skipping Windows build (mingw-w64 not installed)"
    echo "   Install with: sudo apt install gcc-mingw-w64-x86-64"
fi

echo ""
echo "📦 Binaries ready for testing!"
echo "   Linux: target/release/nust"
if [ -f "target/x86_64-pc-windows-gnu/release/nust.exe" ]; then
    echo "   Windows: target/x86_64-pc-windows-gnu/release/nust.exe"
fi
