#!/bin/bash
set -xe

cargo build --manifest-path metro_m0/Cargo.toml --example blinky_basic
cargo build --manifest-path gemma_m0/Cargo.toml --examples
cargo build --manifest-path samd21_mini/Cargo.toml --examples
cargo build --manifest-path arduino_mkrzero/Cargo.toml --examples
