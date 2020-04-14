# Adafruit ItsyBitsy M4 Express Board Support Crate

This crate provides a type-safe Rust API for working with the
[Adafruit ItsyBitsy M4 Express board](https://www.adafruit.com/product/3800).

## Board Features

- Microchip [ATSAMD51G19A] Cortex-M4 microcontroller @ 120 MHz (32-bit, 3.3V logic and power)
  - 512kB Flash
  - 192kB SRAM
- 2 MB SPI Flash chip

## Prerequisites
* Install the cross compile toolchain `rustup target add thumbv7em-none-eabihf`
* Install [cargo-hf2 the hf2 bootloader flasher tool](https://crates.io/crates/cargo-hf2) however your platform requires

## Uploading an example
Check out the repository for examples:

https://github.com/atsamd-rs/atsamd/tree/master/boards/itsybitsy_m4/examples

* Be in this directory `cd boards/itsybitsy_m4`
* Put your device in bootloader mode usually by hitting the reset button twice.
* Build and upload in one step
```
$ cargo hf2 --release --example blinky_basic
    Finished release [optimized + debuginfo] target(s) in 0.19s
    Searching for a connected device with known vid/pid pair.
    Trying  Ok(Some("Adafruit Industries")) Ok(Some("PyBadge"))
    Flashing "/Users/User/atsamd/boards/itsybitsy_m4/target/thumbv7em-none-eabihf/release/examples/blinky_basic"
    Finished in 0.079s
$
```

Note some examples will tell you they need more features enabled
```
$ cargo hf2 --release --example usb_serial
error: target `usb_serial` in package `itsybitsy_m4` requires the features: `usb`, `use_uart_debug`
Consider enabling them by passing, e.g., `--features="usb use_uart_debug"`
```
Just follow the instructions to add --features like
```
cargo hf2 --release --example usb_serial --features="usb use_uart_debug"
    Finished release [optimized + debuginfo] target(s) in 0.09s
    Searching for a connected device with known vid/pid pair.
    Trying  Ok(Some("Adafruit Industries")) Ok(Some("PyBadge"))
    Flashing "/Users/User/atsamd/boards/itsybitsy_m4/target/thumbv7em-none-eabihf/release/examples/usb_serial"
    Finished in 0.167s
$
```