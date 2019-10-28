#!/usr/bin/env bash -i

command rm ./target/debug/temp
RUSTFLAGS="-A warnings" cargo build
export RUST_LOG=DEBUG
id | ./target/debug/temp