# Adafruit Feather M4 CAN Express Board Support Crate

This crate provides a type-safe API for working with the 
[Adafruit Feather M4 CAN Express board](https://www.adafruit.com/product/4759).

## Prerequisites
* Install the cross compile toolchain `rustup target add thumbv7em-none-eabihf`
* Install [cargo-hf2 the hf2 bootloader flasher tool](https://crates.io/crates/cargo-hf2) however your platform requires

## Uploading an example
Check out the repository for examples:

https://github.com/atsamd-rs/atsamd/tree/master/boards/feather_m4_can/examples

* Be in this directory `cd boards/feather_m4_can`
* Put your device in bootloader mode usually by hitting the reset button twice.
* Build and upload in one step
```
$ cargo hf2 --release --example blinky_basic
    Finished release [optimized + debuginfo] target(s) in 15.80s
    Searching for a connected device with known vid/pid pair.
    Trying  Ok(Some("Adafruit Industries")) Ok(Some("Feather M4 CAN Express"))
    Flashing ".../atsamd-rs/atsamd/boards/feather_m4_can/target/thumbv7em-none-eabihf/release/examples/blinky_basic"
    Finished in 0.104s
$
```
