# Adafruit Metro M4 Board Support Crate

This crate provides a type-safe API for working with the [Adafruit Metro M4
board](https://www.adafruit.com/product/3382).

## Prerequisites
* Install the cross compile toolchain `rustup target add thumbv7em-none-eabihf`
* Install [cargo-hf2 the hf2 bootloader flasher tool](https://crates.io/crates/cargo-hf2) however your platform requires

## Uploading an example
Check out the repository for examples:

https://github.com/atsamd-rs/atsamd/tree/master/boards/metro_m4/examples

* Be in this directory `cd boards/metro_m4`
* Put your device in bootloader mode usually by hitting the reset button twice.
* Build and upload in one step
```
$ cargo hf2 --release --example blinky_basic
    Finished release [optimized + debuginfo] target(s) in 0.19s
    Searching for a connected device with known vid/pid pair.
    Trying  Ok(Some("Adafruit Industries")) Ok(Some("PyBadge"))
    Flashing "/Users/User/atsamd/boards/metro_m4/target/thumbv7em-none-eabihf/release/examples/blinky_basic"
    Finished in 0.079s
$
```