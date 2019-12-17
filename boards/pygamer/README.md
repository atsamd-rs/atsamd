# Adafruit PyGamer Board Support Crate

This crate provides a type-safe API for working with the [Adafruit PyGamer
board](https://www.adafruit.com/product/4242).

## Prerequisites
* Add the cross compile toolchain `rustup target add thumbv7em-none-eabihf`
* Install [cargo-hf2 the hf2 bootloader flasher tool](https://crates.io/crates/cargo-hf2) however your platform requires

## Uploading an example
Check out the repository for examples:

https://github.com/atsamd-rs/atsamd/tree/master/boards/pygamer/examples

* Be in this directory `cd boards/pygamer`
* Put your device in bootloader mode usually by hitting the reset button twice.
* Build and upload in one step
```
$ cargo hf2 --release --example blinky_basic
    Finished release [optimized + debuginfo] target(s) in 0.19s
    Searching for a connected device with known vid/pid pair.
    Trying  Ok(Some("Adafruit Industries")) Ok(Some("PyGamer"))
    Flashing "/Users/User/atsamd/boards/pygamer/target/thumbv7em-none-eabihf/release/examples/blinky_basic"
    Finished in 0.079s
$
```

Note some examples will tell you they need more features enabled
```
$ cargo hf2 --release --example neopixel_button
error: target `neopixel_button` in package `pygamer` requires the features: `unproven`
Consider enabling them by passing, e.g., `--features="unproven"`
```
Just follow the instructions to add --features like
```
cargo hf2 --release --example neopixel_button --features="unproven"
    Finished release [optimized + debuginfo] target(s) in 0.09s
    Searching for a connected device with known vid/pid pair.
    Trying  Ok(Some("Adafruit Industries")) Ok(Some("PyGamer"))
    Flashing "/Users/User/atsamd/boards/pygamer/target/thumbv7em-none-eabihf/release/examples/neopixel_button"
    Finished in 0.167s
$
```