# Adafruit QT Py Board Support Crate

This crate provides a type-safe API for working with the [Adafruit QT Py
board](https://www.adafruit.com/product/4600).

## Prerequisites
* Install the cross compile toolchain `rustup target add thumbv6m-none-eabi`
* Install [cargo-hf2 the hf2 bootloader flasher tool](https://crates.io/crates/cargo-hf2) however your platform requires

## Uploading an example

Check out the repository for examples:

[https://github.com/atsamd-rs/atsamd/tree/master/boards/qt_py_m0/examples][]

* Be in this directory `cd boards/qt_py_m0`
* Put your device in bootloader mode by pressing the reset button twice.
* Build and upload in one step
```
$ cargo hf2 --release --example neopixel
   Compiling qt_py_m0 v0.9.0 (/home/gkelly/p/atsamd/boards/qt_py_m0)
    Finished release [optimized] target(s) in 0.48s
    Searching for a connected device with known vid/pid pair.
    Trying  Ok(Some("Adafruit Industries")) Ok(Some("QT Py M0"))
    Flashing "/home/gkelly/p/atsamd/boards/qt_py_m0/target/thumbv6m-none-eabi/release/examples/neopixel"
    Finished in 0.172s
```
