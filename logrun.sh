#!/usr/bin/env bash
RUST_LOG=roxide cargo run --features extra_commands -- "$@"
