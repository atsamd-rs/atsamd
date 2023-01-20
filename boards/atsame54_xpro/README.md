# SAM E54 Xplained Pro Evaluation Kit Board Support Crate

This crate provides a type-safe Rust API for working with the
[SAM E54 Xplained Pro Evaluation Kit](https://www.microchip.com/developmenttools/productdetails/atsame54_xpro).

## Board Features

- Microchip [ATSAME54P] Cortex-M4F microcontroller
  - 1MB Flash
  - 256kB SRAM (128kB if ECCRAM is enabled)
  - 8MB SPI Flash chip

## Prerequisites

* Install the cross-compilation target
    * `$ rustup target add thumbv7em-none-eabihf`
* Install the [`cargo-embed`](https://github.com/probe-rs/probe-rs/tree/master/cargo-embed)
    * `$ cargo install cargo-embed`

## Running an example

* Checkout the atsamd repository
* Go to directory `boards/atsame54_xpro`
* Build and flash the device
    * eg. `cargo embed --release --example blinky_rtic`
