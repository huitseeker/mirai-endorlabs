#!/bin/bash -v
#Copyright (c) Facebook, Inc. and its affiliates.

# This source code is licensed under the MIT license found in the
# LICENSE file in the root directory of this source tree.

# Exit immediately if a command exits with a non-zero status.
set -e

# Check if dynamic link to vcpkg installed Z3 is wanted rather than static linking
if [ "$1" == "vcpkg" ]; then
  FLAGS='--no-default-features --features=vcpkg'
fi

# start clean
cargo clean
cargo update
# Run format checks
cargo fmt --all
# Run lint checks
cargo audit
cargo clippy --no-default-features --all-targets -- -D warnings
# Build MIRAI and regenerate the seed summaries for the current Cargo layout.
./rebuild_std.sh

# Run nextest, starting clean so that the new summary store is used.
cargo clean
cargo build --tests $FLAGS
time cargo nextest run $FLAGS

# Install MIRAI into cargo so that we can use optimized binaries to analyze debug binaries built with special flags
cargo uninstall mirai || true
cargo install --path ./checker $FLAGS

# Run mirai on itself, using the optimized build in cargo as the bootstrap.
cargo clean -p mirai
cargo mirai --no-default-features
