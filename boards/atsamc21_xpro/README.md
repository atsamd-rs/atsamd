# SAM C21 Xplained Pro Evaluation Kit Board Support Crate

This crate provides a type-safe Rust API for working with the
[SAM C21 Xplained Pro Evaluation Kit](https://www.microchip.com/developmenttools/productdetails/atsamc21_xpro).

## Board Features

- Microchip [ATSAMC21J18A] Cortex-M0 microcontroller @ 48 MHz (32-bit, 3.3V or 5V logic and power)
  - 256kB Flash
  - 32kB SRAM
  - CAN and LIN interfaces

## Prerequisites
* Install the cross compile toolchain `rustup target add thumbv6m-none-eabi`

## Uploading an example
Check out the repository for examples:

https://github.com/atsamd-rs/atsamd/tree/master/boards/atsamc21_xpro/examples

* Be in this directory `cd boards/atsamc21_xpro`
* Build and upload in one step
```
$ cargo flash --example blinky_pac --chip-description-path SAMC21.yaml --chip ATSAMC21J18A
    Finished dev [unoptimized + debuginfo] target(s) in 7.56s
    Flashing .../boards/atsamc21_xpro/target/thumbv6m-none-eabi/debug/examples/blinky_pac
     Erasing sectors ✔ [00:00:00] [#######################] 16.00 KiB/16.00 KiB @ 91.94 KiB/s (eta 0s )
 Programming pages   ✔ [00:00:00] [##########################] 2.00 KiB/2.00 KiB @ 5.79 KiB/s (eta 0s )    Finished in 0.593s
$
```