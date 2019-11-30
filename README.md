# atsamd support for Rust

This repo holds various things that support/enable working with atmel samd11, samd21 and samd51 based
devices, such as the Adafruit Metro M0, Trinket M0 and Gemma M0, using Rust.

[![Build Status](https://travis-ci.org/atsamd-rs/atsamd.svg?branch=master)](https://travis-ci.org/atsamd-rs/atsamd)

There are a couple of crates provided by this repo:

* [`atsamd11c14a`](https://docs.rs/atsamd11c14a/) is an auto-generated crate providing access to the peripherals specified for this device by its SVD file.
* [`atsamd21g18a`](https://docs.rs/atsamd21g18a/) is an
  auto-generated crate providing access to the peripherals
  specified for this device by its SVD file.  This is the MCU used in the Metro M0,
  Feather M0, Circuit Playground express boards from Adafruit, the Sparkfun SAMD21 Mini, the Arduino MKR ZERO and the SODAQ ONE board
  from SODAQ.
* [`atsamd21e18a`](https://docs.rs/atsamd21e18a/) is an
  auto-generated crate providing access to the peripherals
  specified for this device by its SVD file.  This is the MCU used in the Trinket M0
  and Gemma M0 boards from Adafruit.
* [`atsamd21j18a`](https://docs.rs/atsamd21j18a/) is an
  auto-generated crate providing access to the peripherals
  specified for this device by its SVD file.  This is the MCU used in the SARA AFF
  boards from Sodaq.
* [`atsamd51j19a`](https://docs.rs/atsamd51j19a/) is an auto-generated crate providing access to the peripherals specified for this device by its SVD file. This is the MCU used in the Metro M4 and Feather M4 boards from Adafruit.
* [`atsamd51j20a`](https://docs.rs/atsamd51j20a/) is an auto-generated crate providing access to the peripherals specified for this device by its SVD file. This is the MCU used in the PyPortal board from Adafruit.
* [`atsamd51g19a`](https://docs.rs/atsamd51g19a/) is an auto-generated crate providing access to the peripherals specified for this device by its SVD file. This is the MCU used in the Trellis M4 and ItsyBitsy M4 boards from Adafruit.
* [`atsamd-hal`](https://docs.rs/atsamd_hal/) is the result
  of reading the datasheet for the device and encoding
  a type-safe layer over the raw `atsamd11c14a`, `atsamd21g18a`, `atsamd21e18a`, `atsamd21j18a`, `atsamd51j19a`, and `atsamd51g19a` crates.  This crate
  implements traits specified by the `embedded-hal` project, making it compatible with
  various drivers in the embedded rust ecosystem.

In addition to the generic crates, there are also crates for popular ATSAMD21/51 based development boards. They aim to rename pins to match silk screens or Arduino pin assignments, add helpers for initialization, and re-export the `atsamd-hal` crate.

* [`arduino_mkrzero`](https://docs.rs/arduino_mkrzero/)
* [`circuit_playground_express`](https://docs.rs/circuit_playground_express/)
* [`feather_m0`](https://docs.rs/feather_m0/)
* [`gemma_m0`](https://docs.rs/gemma_m0/)
* [`itsybitsy_m0`](https://docs.rs/itsybitsy_m0/)
* [`metro_m0`](https://docs.rs/metro_m0/)
* [`samd21_mini`](https://docs.rs/samd21_mini/)
* [`trinket_m0`](https://docs.rs/trinket_m0/)
* [`sodaq_one`](https://docs.rs/sodaq_one/)
* [`metro_m4`](https://docs.rs/metro_m4/)
* [`trellis_m4`](https://docs.rs/trellis_m4/)
* [`feather_m4`](https://docs.rs/feather_m4/)
* [`pyportal`](https://docs.rs/pyportal/)
* [`pygamer`](https://docs.rs/pygamer/)
* [`sodaq_sara_aff`](https://docs.rs/sodaq_sara_aff/)

## Building

 You'll need to install support for
`thumbv6m-none-eabi` if you're targeting samd11 or samd21, or `thumbv7em-none-eabihf` if you're targeting samd51.  Make sure that you have a new enough version of the
gcc toolchain; the one installable even on recent versions of ubuntu can
fail to correctly link the vector table:

```bash
$ sudo add-apt-repository ppa:team-gcc-arm-embedded/ppa -y
$ sudo apt update
$ sudo apt install gcc-arm-embedded
$ rustup target add thumbv6m-none-eabi
$ rustup target add thumbv7em-none-eabihf
```


Since a couple of different MCUs are used, building the examples requires changing
directory into one of the board support crate dirs prior to building:

```bash
$ cd metro_m0
$ cargo build --examples
$ cd ../gemma_m0
$ cargo build --examples
```

## Building everything locally

If you'd like to build all the same things that the CI would build but on
your local system, you can run:

```
$ mkdir -p /tmp/atsamd-virtualenv
$ virtualenv /tmp/atsamd-virtualenv
$ source /tmp/atsamd-virtualenv/bin/activate
$ pip install -r requirements.txt
$ ./build-all.py
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
  --port=ttyACM1 -U -e -w -v \
  target/thumbv6m-none-eabi/debug/examples/blinky_basic.bin -R
```

This same technique should work for all of the Adafruit M0/M4 boards, as they
all ship with a bossac compatible bootloader. Note that M0 devices may need
`-o 0x2000` and M4 devices may need `-o 0x4000` added to the `bossac` parameter
lists.

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

## Adding a new board
See our wiki page for a [complete guide](https://github.com/atsamd-rs/atsamd/wiki/Adding-a-new-board) on adding a new board.

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
