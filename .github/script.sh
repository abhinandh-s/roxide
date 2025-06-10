#!/bin/bash
set -euxo pipefail

echo "===> Setting up Rust"
rustup update stable
rustup default stable

mkdir -p ~/.local/share/Trash/files
mkdir -p ~/.local/share/rid
touch ~/.local/share/rid/rid_history.log


echo "===> Running cargo build"
cargo build --verbose

echo "===> Running cargo test"
cargo test --verbose -- --nocapture