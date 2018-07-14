# atsamd21 support for Rust

This repo holds various things that support/enable working with atmel samd21 based
devices, such as the Adafruit Metro M0, Trinket M0 and Gemma M0, using Rust.

[![Build Status](https://travis-ci.org/wez/atsamd21-rs.svg?branch=master)](https://travis-ci.org/wez/atsamd21-rs)

There are a couple of crates provided by this repo:

* [`atsamd21g18a`](https://docs.rs/atsamd21g18a/latest/atsamd21g18a/) is an
  auto-generated crate providing access to the peripherals
  specified for this device by its SVD file.  This is the MCU used in the Metro M0,
  Feather M0 and Circuit Playground express boards from Adafruit.
* [`atsamd21e18a`](https://docs.rs/atsamd21e18a/latest/atsamd21e18a/) is an
  auto-generated crate providing access to the peripherals
  specified for this device by its SVD file.  This is the MCU used in the Trinket M0
  and Gemma M0 boards from Adafruit.
* [`atsamd21-hal`](https://docs.rs/atsamd21-hal/latest/atsamd21_hal/) is the result
  of reading the datasheet for the device and encoding
  a type-safe layer over the raw `atsamd21g18a` and `atsamd21e18a` crates.  This crate
  implements traits specified by the `embedded-hal` project, making it compatible with
  various drivers in the embedded rust ecosystem.
* [`metro_m0`](https://docs.rs/metro_m0/latest/metro_m0/) is a board support crate
  for the Adafruit Metro M0 board.  It re-exports the `atsamd21-hal` crate functionality
  using more convenient names; for example, the IO pins are exported using the labels
  printed on the board rather than the more abstract and harder to remember port and
  pin numbers used by the underlying device.
* [`gemma_m0`](https://docs.rs/gemma_m0/latest/gemma_m0/) is a board support crate
  for the Adafruit Gemma M0 board.  Similar to the Metro M0 crate, it re-exports the
  `atsamd21-hal` crate functionality using more convenient names.

## Building

The atsamd21 devices require untagged union support which means that you will
need to be using nightly rust.  You'll also need to install support for
`thumbv6m-none-eabi`:

```bash
$ rustup install nightly-2018-05-16
$ rustup default nightly-2018-05-16
$ rustup target add thumbv6m-none-eabi
```

Since a couple of different MCUs are used, building the examples requires changing
directory into one of the board support crate dirs prior to building:

```bash
$ cd metro_m0
$ cargo build --examples
$ cd ../gemma_m0
$ cargo build --examples
```

## Getting code onto the device: Gemma M0

If you want to flash the device using the tools that come with the adafruit
arduino support package:

```bash
$ cd gemma_m0
$ cargo build --example blinky_basic
$ arm-none-eabi-objcopy -O binary \
  target/thumbv6m-none-eabi/debug/examples/blinky_basic \
  target/thumbv6m-none-eabi/debug/examples/blinky_basic.bin
$ stty -F /dev/ttyACM1 ospeed 1200
$ ~/.arduino15/packages/arduino/tools/bossac/1.7.0/bossac -i -d \
  --port=ttyACM1 -U true -i -e -w -v \
  target/thumbv6m-none-eabi/debug/examples/blinky_basic.bin -R
```

This same technique should work for all of the Adafruit M0 boards, as they
all ship with a bossac compatible bootloader.

## Getting code onto the device: JLink

If you have a board with a SWD debug header, such as the Metro M0, or if you attached
the header yourself, you can use your JLink together with gdb.  @wez prefers using
the JLinkGDBServer, but you can also use OpenOCD.

In one window, run `JLinkGDBServer -if SWD -device ATSAMD21G18`, then in another,
run these commands from the root of this repo so that you pick up its `.gdbinit`
file:

```bash
$ cargo build --manifest-path metro_m0/Cargo.toml --example blinky_basic
$ arm-none-eabi-gdb metro_m0/target/thumbv6m-none-eabi/debug/examples/blinky_basic
```

If you prefer or otherwise need to use OpenOCD, then you'd run it in place of
the JLinkGDBServer and then modify the `.gdbinit` file to comment out the JLink
section and uncomment the OpenOCD section.

### Semihosting

If you want to enable semihosting to be able to see debugging messages, this will
enable them in some of the example crates.  Note that when you enable semihosting,
the resultant firmware will only run when a debugger is attached to your board; it
will fault the MCU if the debugger is absent:

```bash
$ cargo build --manifest-path metro_m0/Cargo.toml \
  --example blinky_basic --features use_semihosting
```


## License

The included SVD files are sourced from http://packs.download.atmel.com/ and
are licensed under the Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
http://www.apache.org/licenses/LICENSE-2.0).

The remainder of the code is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
