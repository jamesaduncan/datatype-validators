#!/bin/bash

# ABOUTME: Build script that compiles all WASM validators and organizes them into build/
# ABOUTME: Creates a clean directory structure with compiled WASM files as index.wasm

set -e  # Exit on any error

echo "🔨 Building all validators..."

# Create build directory structure
echo "📁 Creating build directory structure..."
rm -rf build
mkdir -p build/Text
mkdir -p build/URL
mkdir -p build/Boolean
mkdir -p build/Integer
mkdir -p build/FloatingPoint
mkdir -p build/DateTime
mkdir -p build/Date
mkdir -p build/Time

# Text Validator
echo "📝 Building Text validator..."
cd Text/text-validator
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/text_validator.wasm ../../build/Text/index.wasm
cd ../..

# URL Validator
echo "🔗 Building URL validator..."
cd URL/url-validator
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/url_validator.wasm ../../build/URL/index.wasm
cd ../..

# Boolean Validator
echo "✓ Building Boolean validator..."
cd Boolean/boolean-validator
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/boolean_validator.wasm ../../build/Boolean/index.wasm
cd ../..

# Integer Validator
echo "🔢 Building Integer validator..."
cd Number/integer-validator
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/integer_validator.wasm ../../build/Integer/index.wasm
cd ../..

# FloatingPoint Validator
echo "🔢 Building FloatingPoint validator..."
cd Number/floatingpoint-validator
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/floatingpoint_validator.wasm ../../build/FloatingPoint/index.wasm
cd ../..

# DateTime Validator
echo "📅 Building DateTime validator..."
cd DateTime/datetime-validator
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/datetime_validator.wasm ../../build/DateTime/index.wasm
cd ../..

# Date Validator
echo "📆 Building Date validator..."
cd DateTime/date-validator
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/date_validator.wasm ../../build/Date/index.wasm
cd ../..

# Time Validator
echo "🕐 Building Time validator..."
cd DateTime/time-validator
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/time_validator.wasm ../../build/Time/index.wasm
cd ../..

echo "✅ Build complete!"
echo ""
echo "📊 Build summary:"
echo "  • Text validator       → build/Text/index.wasm"
echo "  • URL validator        → build/URL/index.wasm"
echo "  • Boolean validator    → build/Boolean/index.wasm"
echo "  • Integer validator    → build/Integer/index.wasm"
echo "  • FloatingPoint validator → build/FloatingPoint/index.wasm"
echo "  • DateTime validator   → build/DateTime/index.wasm"
echo "  • Date validator       → build/Date/index.wasm"
echo "  • Time validator       → build/Time/index.wasm"
echo ""

# Show file sizes
echo "📏 File sizes:"
ls -lh build/*/index.wasm | awk '{print "  • " $9 ": " $5}'