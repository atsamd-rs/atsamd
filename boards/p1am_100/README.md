# Facts Engineering P1AM-100 Board Support Crate

This crate provides a type-safe API for working with the [Facts Engineering P1AM-100
board](https://facts-engineering.github.io/modules/P1AM-100/P1AM-100.html).

## Prerequisites
* Install the cross compile toolchain `rustup target add thumbv6m-none-eabi`

## Uploading an example
Check out the repository for examples:

https://github.com/atsamd-rs/atsamd/tree/master/boards/p1am_100/examples

* Be in this directory `cd boards/p1am_100`
* Put your device in bootloader mode usually by hitting the reset button twice.
* Build using `cargo build --example blinky_basic`
* Upload using the `objcopy` and `bossa` tools as described in the [atsamd crate documentation](https://github.com/atsamd-rs/atsamd#getting-code-onto-the-device-gemma-m0).
