#!/bin/bash

# Build script for neofetch-rs release version
# This script builds an optimized release version of neofetch-rs

set -e  # Exit on any error

echo "Building neofetch-rs in release mode..."
echo "This will create an optimized binary with:"
echo "  - Maximum optimization level (opt-level = 3)"
echo "  - Link-time optimization (LTO)"
echo "  - Stripped symbols for smaller binary size"
echo "  - Single codegen unit for better optimization"
echo ""

# Build the release version
cargo build --release

echo ""
echo "✅ Release build completed successfully!"
echo ""

# Show binary information
echo "📊 Binary information:"
echo "Release binary: $(ls -lh target/release/neofetch | awk '{print $5, $9}')"
if [ -f target/debug/neofetch ]; then
    echo "Debug binary:   $(ls -lh target/debug/neofetch | awk '{print $5, $9}')"
    echo ""
    echo "📈 Size reduction: $(echo "scale=1; (1 - $(stat -f%z target/release/neofetch) / $(stat -f%z target/debug/neofetch)) * 100" | bc)%"
fi

echo ""
echo "🚀 You can now run the optimized binary with:"
echo "   ./target/release/neofetch"
echo ""
echo "📦 Or install it system-wide with:"
echo "   make install"
