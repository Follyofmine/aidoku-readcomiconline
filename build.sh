#!/bin/bash
set -e

source="en_readcomiconline"
source_dir="src/rust/en.readcomiconline"
out_dir="src/gh-pages"

# Move to source directory
cd "$source_dir"

# Build WASM
cargo build --release --target wasm32-unknown-unknown

# Ensure the output directory exists
mkdir -p "../../$out_dir"

# Copy the wasm file
cp "target/wasm32-unknown-unknown/release/${source}.wasm" "../../$out_dir/${source}.wasm"
