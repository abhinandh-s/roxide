#!/usr/bin/env bash

cargo update --verbose
cargo clippy
cargo build --release --verbose
cargo test --verbose

