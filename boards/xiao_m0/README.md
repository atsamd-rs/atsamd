# Seeeduino XIAO Board Support Crate

This crate provides a type-safe API for working with the [Seeed Studio
Seeeduino XIAO](http://wiki.seeedstudio.com/Seeeduino-XIAO/).

## Prerequisites
* Install the cross compile toolchain `rustup target add thumbv6m-none-eabi`
* Install the [cargo-hf2 tool](https://crates.io/crates/cargo-hf2) however your
  platform requires

## Uploading an example
Check out [the
repository](https://github.com/atsamd-rs/atsamd/tree/master/boards/xiao_m0/examples)
for examples.

* Be in this directory `cd boards/xiao_m0`
* Put your device in bootloader mode by bridging the `RST` pads _twice_ in
  quick succession. The orange LED will pulse when the device is in bootloader
  mode.
* Build and upload in one step: `cargo hf2 --release --example blink`
  * Note that if you're using an older `cargo-hf2` that you'll need to specify
    the VID/PID when flashing: `cargo hf2 --vid 0x2886 --pid 0x002f --release
    --example blink`
