# atsamd & atsame support for Rust

This repository holds various crates that support/enable working with Atmel `samd11`, `samd21`, `samd51`, and `same54` based devices using Rust.

[![Build Status](https://travis-ci.org/atsamd-rs/atsamd.svg?branch=master)](https://travis-ci.org/atsamd-rs/atsamd)

The **P**eripheral **A**ccess **C**rates (**PAC**s) are automatically generated, and provide low-level access to the peripherals specified by a device's SVD file.

The **H**ardware **A**bstraction **L**ayer (**HAL**) is the result of reading the datasheet for the device and encoding a type-safe layer over the raw PACs. This crate implements traits specified by the [embedded-hal](https://github.com/rust-embedded/embedded-hal) project, making it compatible with various drivers in the embedded rust ecosystem.

In addition to the PACs and HAL, there numerous **B**oard **S**upport **P**ackages (**BSP**s) for popular development boards. They aim to rename pins to match silk screens or Arduino pin assignments, add helpers for initialization, and re-export the `atsamd-hal` crate. These BSPs are listed beside their respective PACs below.

| Crate | Version | Board Support Packages |
|:------|:--------|:-----------------------|
| [atsamd11c14a](https://docs.rs/atsamd11c14a/) | [![Crates.io](https://img.shields.io/crates/v/atsamd11c14a.svg)](https://crates.io/crates/atsamd11c14a) |  |
| [atsamd21e18a](https://docs.rs/atsamd21e18a/) | [![Crates.io](https://img.shields.io/crates/v/atsamd21e18a.svg)](https://crates.io/crates/atsamd21e18a) | [Gemma M0][gemma_m0], [Trinket M0][trinket_m0], [Serpente][serpente] |
| [atsamd21g18a](https://docs.rs/atsamd21g18a/) | [![Crates.io](https://img.shields.io/crates/v/atsamd21g18a.svg)](https://crates.io/crates/atsamd21g18a) | [Circuit Playground Express][circuit_playground_express], [Feather M0][feather_m0], [Metro M0][metro_m0], [MKR ZERO][arduino_mkrzero], [SAMD21 Mini][samd21_mini], [SODAQ ONE][sodaq_one] |
| [atsamd21j18a](https://docs.rs/atsamd21j18a/) | [![Crates.io](https://img.shields.io/crates/v/atsamd21j18a.svg)](https://crates.io/crates/atsamd21j18a) | [SODAQ SARA AFF][sodaq_sara_aff] |
| [atsamd51g19a](https://docs.rs/atsamd51g19a/) | [![Crates.io](https://img.shields.io/crates/v/atsamd51g19a.svg)](https://crates.io/crates/atsamd51g19a) | [ItsyBitsy M4][itsybitsy_m4], [Trellis M4][trellis_m4] |
| [atsamd51j19a](https://docs.rs/atsamd51j19a/) | [![Crates.io](https://img.shields.io/crates/v/atsamd51j19a.svg)](https://crates.io/crates/atsamd51j19a) | [EdgeBadge][edgebadge], [Feather M4][feather_m4], [Metro M4][metro_m4] |
| [atsamd51j20a](https://docs.rs/atsamd51j20a/) | [![Crates.io](https://img.shields.io/crates/v/atsamd51j20a.svg)](https://crates.io/crates/atsamd51j20a) | [PyPortal][pyportal] |
| [atsamd51p19a](https://docs.rs/atsamd51p19a/) | [![Crates.io](https://img.shields.io/crates/v/atsamd51p19a.svg)](https://crates.io/crates/atsamd51p19a) |  |
| [atsame54p20a](https://docs.rs/atsame54p20a/) | [![Crates.io](https://img.shields.io/crates/v/atsame54p20a.svg)](https://crates.io/crates/atsame54p20a) | [PathfinderZA Proto1][pfza_proto1] |
| [atsamd-hal](https://docs.rs/atsamd_hal/) | [![Crates.io](https://img.shields.io/crates/v/atsamd_hal.svg)](https://crates.io/crates/atsamd_hal) |  |

[arduino_mkrzero]: https://github.com/atsamd-rs/atsamd/tree/master/boards/arduino_mkrzero/
[circuit_playground_express]: https://github.com/atsamd-rs/atsamd/tree/master/boards/circuit_playground_express/
[edgebadge]: https://github.com/atsamd-rs/atsamd/tree/master/boards/edgebadge
[feather_m0]: https://github.com/atsamd-rs/atsamd/tree/master/boards/feather_m0/
[feather_m4]: https://github.com/atsamd-rs/atsamd/tree/master/boards/feather_m4/
[gemma_m0]: https://github.com/atsamd-rs/atsamd/tree/master/boards/gemma_m0/
[itsybitsy_m0]: https://github.com/atsamd-rs/atsamd/tree/master/boards/itsybitsy_m0/
[itsybitsy_m4]: https://github.com/atsamd-rs/atsamd/tree/master/boards/itsybitsy_m4/
[metro_m0]: https://github.com/atsamd-rs/atsamd/tree/master/boards/metro_m0/
[metro_m4]: https://github.com/atsamd-rs/atsamd/tree/master/boards/metro_m4/
[pfza_proto1]: https://github.com/atsamd-rs/atsamd/tree/master/boards/pfza_proto1/
[pygamer]: https://github.com/atsamd-rs/atsamd/tree/master/boards/pygamer/
[pyportal]: https://github.com/atsamd-rs/atsamd/tree/master/boards/pyportal/
[samd21_mini]: https://github.com/atsamd-rs/atsamd/tree/master/boards/samd21_mini/
[sodaq_one]: https://github.com/atsamd-rs/atsamd/tree/master/boards/sodaq_one/
[sodaq_sara_aff]: https://github.com/atsamd-rs/atsamd/tree/master/boards/sodaq_sara_aff/
[trellis_m4]: https://github.com/atsamd-rs/atsamd/tree/master/boards/trellis_m4/
[trinket_m0]: https://github.com/atsamd-rs/atsamd/tree/master/boards/trinket_m0/
[serpente]: https://github.com/atsamd-rs/atsamd/tree/master/boards/serpente/

## Building

Make sure that you have a new enough version of the gcc toolchain; the one installable even on recent versions of Ubuntu can fail to correctly link the vector table:

```bash
$ sudo add-apt-repository ppa:team-gcc-arm-embedded/ppa -y
$ sudo apt update
$ sudo apt install gcc-arm-embedded
```

You'll need to add the proper compilation target prior to building as well:

```bash
$ # for samd11, samd21:
$ rustup target add thumbv6m-none-eabi
$ # for samd51, same54:
$ rustup target add thumbv7em-none-eabihf
```

Since a number of different MCUs are used, building the examples requires changing directory into one of the board support package directories prior to building. For example:

```bash
$ cd metro_m0
$ cargo build --examples
$ cd ../gemma_m0
$ cargo build --examples
```

### Building everything locally

If you'd like to build all the same things that the CI would build but on your local system, you can run:

```bash
$ python -m venv /tmp/atsamd-virtualenv
$ source /tmp/atsamd-virtualenv/bin/activate
$ pip install -r requirements.txt
$ ./build-all.py
```

## Getting code onto the device: Gemma M0

If you want to flash the device using the tools that come with the Adafruit arduino support package:

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

This same technique should work for all of the Adafruit M0/M4 boards, as they all ship with a bossac compatible  bootloader. Note that M0 devices may need `-o 0x2000` and M4 devices may need `-o 0x4000` added to the `bossac`  parameter lists.

## Getting code onto the device: JLink

If you have a board with a SWD debug header, such as the [Metro M0][metro_m0], or if you attached the header yourself, you can  use your JLink together with gdb. @wez prefers using the JLinkGDBServer, but you can also use OpenOCD.

In one window, run `JLinkGDBServer -if SWD -device ATSAMD21G18`, then in another, run these commands from the root   of this repo so that you pick up its `.gdbinit` file:

```bash
$ cargo build --manifest-path metro_m0/Cargo.toml --example blinky_basic
$ arm-none-eabi-gdb metro_m0/target/thumbv6m-none-eabi/debug/examples/blinky_basic
```

If you prefer or otherwise need to use OpenOCD, then you'd run it in place of the JLinkGDBServer and then modify the `.gdbinit` file to comment out the JLink section and uncomment the OpenOCD section.

### Semihosting

If you want to enable semihosting to be able to see debugging messages, this will enable them in some of the example crates. Note that when you enable semihosting, the resultant firmware will only run when a debugger is  attached to your board; it will fault the MCU if the debugger is absent:

```bash
$ cargo build --manifest-path metro_m0/Cargo.toml \
    --example blinky_basic --features use_semihosting
```

## Getting code onto the devices with bootloaders: hf2-rs

[hf2-rs](https://github.com/jacobrosenthal/hf2-rs) implements [Microsofts HID Flashing Format (HF2)](https://github.com/microsoft/uf2/blob/86e101e3a282553756161fe12206c7a609975e70/hf2.md) to upload firmware to UF2 bootloaders. UF2 is factory programmed extensively by [Microsoft MakeCode](https://www.microsoft.com/en-us/makecode) and [Adafruit](https://www.adafruit.com/) hardware.

The `cargo-hf2` crate replaces the `cargo build` command to include flashing over USB to connected UF2 devices, using hf2 flashing over HID protocol.

```bash
$ cargo install cargo-hf2
$ cargo hf2 --manifest-path metro_m0/Cargo.toml \
    --example blinky_basic --features unproven --release
```

For more information, refer to the `README` files for each crate:
* [hf2 library (`hf2`)](https://github.com/jacobrosenthal/hf2-rs/tree/master/hf2)
* [hf2 binary (`hf2-cli`)](https://github.com/jacobrosenthal/hf2-rs/tree/master/hf2-cli)
* [hf2 cargo subcommand (`hf2-cargo`)](https://github.com/jacobrosenthal/hf2-rs/tree/master/cargo-hf2)

## Getting code onto the devices with bootloaders: uf2conv-rs

[uf2conv](https://github.com/sajattack/uf2conv-rs) adds a uf2 header [Microsofts HID Flashing Format (UF2)](https://github.com/microsoft/uf2/blob/86e101e3a282553756161fe12206c7a609975e70/README.md) for copying over to UF2 bootloader mass storage devices. UF2 is factory programmed extensively by [Microsoft MakeCode](https://www.microsoft.com/en-us/makecode) and [Adafruit](https://www.adafruit.com/) hardware.
[cargo-binutils](https://github.com/rust-embedded/cargo-binutils) replaces the `cargo build` command to find and convert elf files into binary. 

Install the dependencies
```bash
$ rustup component add llvm-tools-preview
$ cargo install uf2conv cargo-binutils
```

Then for say, metro_m0 examples
```bash
$ cargo objcopy --example blinky_basic --features unproven --release -- -O binary blinky_basic.bin
$ uf2conv blinky_basic.bin --base 0x2000 --output blinky_basic.uf2
$ cp blinky_basic.uf2 /Volumes/PYGAMERBOOT/
```

For more information, refer to the `README` files for each crate:
* [uf2conv (`uf2conv-rs`)](https://github.com/sajattack/uf2conv-rs)
* [cargo-binutils (`cargo-binutils`)](https://github.com/rust-embedded/cargo-binutils)

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
