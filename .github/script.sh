#!/bin/bash
set -euxo pipefail

echo "===> Setting up Rust"
rustup update stable
rustup default stable

echo "===> Running cargo build"
cargo build --verbose

echo "===> Running cargo test"
cargo test --verbose -- --nocapture