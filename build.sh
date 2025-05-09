#!/bin/bash

set -e

source="en_readcomiconline"
target="wasm32-unknown-unknown"
out_file="${source}.wasm"
profile="release"

# Compile the source
echo "Building ${source}..."
cargo build --release --target $target

# Optimize the output WASM
wasm-opt -Oz \
  -o "target/$target/$profile/${out_file}" \
  "target/$target/$profile/${out_file}"

# Inform the user of the result
echo -e "\n✅ Build complete!"
echo "Optimized .wasm located at: target/${target}/${profile}/${out_file}"
echo -e "👉 Please switch to the 'gh-pages' branch and copy the file manually using:\n"
echo "   git checkout gh-pages"
echo "   cp target/${target}/${profile}/${out_file} ."
echo "   git add ${out_file} && git commit -m 'Update WASM' && git push"
