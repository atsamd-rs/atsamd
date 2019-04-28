#!/bin/bash
set -xe

# Why no workspace?  Each board has a different memory.x and in
# some cases, different core device, so building in a workspace
# will try to link the wrong pieces together.  We have to build
# each board as its own entity.

cargo build --manifest-path boards/metro_m0/Cargo.toml --example blinky_basic --example blinky_rtfm
cargo build --manifest-path boards/feather_m0/Cargo.toml --examples
cargo build --manifest-path boards/gemma_m0/Cargo.toml --examples
cargo build --manifest-path boards/itsybitsy_m0/Cargo.toml --examples
cargo build --manifest-path boards/trinket_m0/Cargo.toml --examples
cargo build --manifest-path boards/samd21_mini/Cargo.toml --examples
cargo build --manifest-path boards/arduino_mkrzero/Cargo.toml --examples
cargo build --manifest-path boards/circuit_playground_express/Cargo.toml --examples
cargo build --manifest-path boards/sodaq_one/Cargo.toml --examples
pushd boards/metro_m4
cargo build --examples
popd
pushd boards/pyportal
cargo build --examples
popd
pushd boards/trellis_m4
cargo build --examples
cargo build --features=keypad-unproven --examples
popd
