# atsamd & atsame support for Rust

This repository holds various crates that support/enable working with Microchip (nee Atmel) `samd11`, `samd21`, `samd51` and `same5x` based devices using Rust.

![CI Build](https://github.com/atsamd-rs/atsamd/workflows/Build%20BSPs/badge.svg)

![CI Build](https://github.com/atsamd-rs/atsamd/workflows/Build%20HAL/badge.svg)

## HAL - Hardware Abstraction Layer

The Hardware Abstraction Layer (HAL - [![Crates.io](https://img.shields.io/crates/v/atsamd_hal.svg)](https://crates.io/crates/atsamd_hal)) crate encodes a type-safe layer over the raw PACs. This crate implements traits specified by the [embedded-hal](https://github.com/rust-embedded/embedded-hal) project, making it compatible with various drivers in the embedded Rust ecosystem.  Cargo features are used to enable support for specific hardware variations and features.  Online documentation for commonly-used feature sets is provided:

| Chip family | Documented features   |
|:------------|:----------------------|
| [samd11c]   | samd11c               |
| [samd11d]   | samd11d               |
| [samd21g]   | samd21g usb           |
| [samd21j]   | samd21j usb           |
| [samd51g]   | samd51g usb           |
| [samd51j]   | samd51j usb           |
| [samd51n]   | samd51n usb           |
| [samd51p]   | samd51p usb           |

[samd11c]: https://atsamd-rs.github.io/docs/samd11c/thumbv6m-none-eabi/doc/atsamd_hal/index.html
[samd11d]: https://atsamd-rs.github.io/docs/samd11d/thumbv6m-none-eabi/doc/atsamd_hal/index.html
[samd21g]: https://atsamd-rs.github.io/docs/samd21g/thumbv6m-none-eabi/doc/atsamd_hal/index.html
[samd21j]: https://atsamd-rs.github.io/docs/samd21j/thumbv6m-none-eabi/doc/atsamd_hal/index.html
[samd51g]: https://atsamd-rs.github.io/docs/samd51g/thumbv7em-none-eabihf/doc/atsamd_hal/index.html
[samd51j]: https://atsamd-rs.github.io/docs/samd51j/thumbv7em-none-eabihf/doc/atsamd_hal/index.html
[samd51n]: https://atsamd-rs.github.io/docs/samd51n/thumbv7em-none-eabihf/doc/atsamd_hal/index.html
[samd51p]: https://atsamd-rs.github.io/docs/samd51p/thumbv7em-none-eabihf/doc/atsamd_hal/index.html

## PAC and BSP - Peripheral Access Crate and Board Support Package

The Peripheral Access Crates (PACs) are automatically generated from Microchip SVD files, and provide low-level access to the peripherals specified by a device's SVD file.

**B**oard **S**upport **P**ackages (**BSP**s) are provided for popular development boards, which rename pins to match silk screens or Arduino pin assignments, add helpers for initialization, and re-export the `atsamd-hal` crate.  The BSPs included in `atsamd-rs` are separated in to two tiers: Tier 1 BSPs use the latest version of `atsamd-hal`, Tier 2 BSPs use a specific version of `atsamd-hal` that is not necessarily the latest.

| PAC docs | crates.io | Provided BSPs  |
|:---------|:----------|:---------------|
| [atsamd11c](https://docs.rs/atsamd11c/) | [![Crates.io](https://img.shields.io/crates/v/atsamd11c.svg)](https://crates.io/crates/atsamd11c) | [Bare atsamd11][samd11_bare]\* |
| [atsamd21e](https://docs.rs/atsamd21e/) | [![Crates.io](https://img.shields.io/crates/v/atsamd21e.svg)](https://crates.io/crates/atsamd21e) | [Gemma M0][gemma_m0], [Serpente][serpente], [Trinket M0][trinket_m0], [Neo Trinkey][neo_trinkey], [NeoKey Trinkey][neokey_trinkey], [QT Py][qt_py_m0] |
| [atsamd21g](https://docs.rs/atsamd21g/) | [![Crates.io](https://img.shields.io/crates/v/atsamd21g.svg)](https://crates.io/crates/atsamd21g) | [Arduino Nano 33 IOT][arduino_nano33_iot], [Circuit Playground Express][circuit_playground_express], [Feather M0][feather_m0]\*, [ItsyBitsy M0][itsybitsy_m0], [Metro M0][metro_m0]\*, [MKR1000][arduino_mkr1000], [MKR Vidor 4000][arduino_mkr_vidor_4000], [MKR ZERO][arduino_mkrzero], [P1AM-100][p1am_100], [SAMD21 Mini][samd21_mini], [SODAQ ONE][sodaq_one], [Wio Lite MG126][wio_lite_mg126], [Wio Lite W600][wio_lite_w600], [Xiao M0][xiao_m0] |
| [atsamd21j](https://docs.rs/atsamd21j/) | [![Crates.io](https://img.shields.io/crates/v/atsamd21j.svg)](https://crates.io/crates/atsamd21j) | [SODAQ SARA AFF][sodaq_sara_aff] |
| [atsamd51g](https://docs.rs/atsamd51g/) | [![Crates.io](https://img.shields.io/crates/v/atsamd51g.svg)](https://crates.io/crates/atsamd51g) | [ItsyBitsy M4][itsybitsy_m4], [Trellis M4][trellis_m4] |
| [atsamd51j](https://docs.rs/atsamd51j/) | [![Crates.io](https://img.shields.io/crates/v/atsamd51j.svg)](https://crates.io/crates/atsamd51j) | [EdgeBadge][edgebadge], [Feather M4][feather_m4]\*, [Metro M4][metro_m4]\*, [PyPortal][pyportal], [Matrix Portal M4][matrix_portal_m4] |
| [atsame51n](https://docs.rs/atsame51n/) | [![Crates.io](https://img.shields.io/crates/v/atsame51n.svg)](https://crates.io/crates/atsame51n) |  |
| [atsamd51p](https://docs.rs/atsamd51p/) | [![Crates.io](https://img.shields.io/crates/v/atsamd51p.svg)](https://crates.io/crates/atsamd51p) | [Grand Central M4 Express][grand_central_m4], [Wio Terminal][wio_terminal] |
| [atsame51g](https://docs.rs/atsame51g/) | [![Crates.io](https://img.shields.io/crates/v/atsame51g.svg)](https://crates.io/crates/atsame51g) |  |
| [atsame51j](https://docs.rs/atsame51j/) | [![Crates.io](https://img.shields.io/crates/v/atsame51j.svg)](https://crates.io/crates/atsame51j) |  |
| [atsame51n](https://docs.rs/atsame51n/) | [![Crates.io](https://img.shields.io/crates/v/atsame51n.svg)](https://crates.io/crates/atsame51n) |  |
| [atsame53j](https://docs.rs/atsame53j/) | [![Crates.io](https://img.shields.io/crates/v/atsame53j.svg)](https://crates.io/crates/atsame53j) | [pygamer][PyGamer] |
| [atsame53n](https://docs.rs/atsame53n/) | [![Crates.io](https://img.shields.io/crates/v/atsame53n.svg)](https://crates.io/crates/atsame53n) |  |
| [atsame54n](https://docs.rs/atsame54n/) | [![Crates.io](https://img.shields.io/crates/v/atsame54n.svg)](https://crates.io/crates/atsame54n) |  |
| [atsame54p](https://docs.rs/atsame54p/) | [![Crates.io](https://img.shields.io/crates/v/atsame54p.svg)](https://crates.io/crates/atsame54p) | [PathfinderZA Proto1][pfza_proto1], [SAM E54 Xplained Pro Evaluation Kit][atsame54_xpro]\* |

\* Tier 1 BSP

[arduino_mkr1000]: https://github.com/atsamd-rs/atsamd/tree/master/boards/arduino_mkr1000
[arduino_mkr_vidor_4000]: https://github.com/atsamd-rs/atsamd/tree/master/boards/arduino_mkrvidor4000
[arduino_mkrzero]: https://github.com/atsamd-rs/atsamd/tree/master/boards/arduino_mkrzero/
[arduino_nano33_iot]: https://github.com/atsamd-rs/atsamd/tree/master/boards/arduino_nano33iot
[atsame54_xpro]: https://github.com/atsamd-rs/atsamd/tree/master/boards/atsame54_xpro/
[circuit_playground_express]: https://github.com/atsamd-rs/atsamd/tree/master/boards/circuit_playground_express/
[edgebadge]: https://github.com/atsamd-rs/atsamd/tree/master/boards/edgebadge
[feather_m0]: https://github.com/atsamd-rs/atsamd/tree/master/boards/feather_m0/
[feather_m4]: https://github.com/atsamd-rs/atsamd/tree/master/boards/feather_m4/
[gemma_m0]: https://github.com/atsamd-rs/atsamd/tree/master/boards/gemma_m0/
[grand_central_m4]: https://github.com/atsamd-rs/atsamd/tree/master/boards/grand_central_m4/
[itsybitsy_m0]: https://github.com/atsamd-rs/atsamd/tree/master/boards/itsybitsy_m0/
[itsybitsy_m4]: https://github.com/atsamd-rs/atsamd/tree/master/boards/itsybitsy_m4/
[matrix_portal_m4]: https://github.com/atsamd-rs/atsamd/tree/master/boards/matrix_portal_m4/
[metro_m0]: https://github.com/atsamd-rs/atsamd/tree/master/boards/metro_m0/
[metro_m4]: https://github.com/atsamd-rs/atsamd/tree/master/boards/metro_m4/
[neo_trinkey]: https://github.com/atsamd-rs/atsamd/tree/master/boards/neo_trinkey
[neokey_trinkey]: https://github.com/atsamd-rs/atsamd/tree/master/boards/neokey_trinkey
[p1am_100]: https://github.com/atsamd-rs/atsamd/tree/master/boards/p1am_100
[pfza_proto1]: https://github.com/atsamd-rs/atsamd/tree/master/boards/pfza_proto1/
[pygamer]: https://github.com/atsamd-rs/atsamd/tree/master/boards/pygamer/
[pyportal]: https://github.com/atsamd-rs/atsamd/tree/master/boards/pyportal/
[qt_py_m0]: https://github.com/atsamd-rs/atsamd/tree/master/boards/qt_py_m0
[samd11_bare]: https://github.com/atsamd-rs/atsamd/tree/master/boards/samd11_bare
[samd21_mini]: https://github.com/atsamd-rs/atsamd/tree/master/boards/samd21_mini/
[serpente]: https://github.com/atsamd-rs/atsamd/tree/master/boards/serpente/
[sodaq_one]: https://github.com/atsamd-rs/atsamd/tree/master/boards/sodaq_one/
[sodaq_sara_aff]: https://github.com/atsamd-rs/atsamd/tree/master/boards/sodaq_sara_aff/
[trellis_m4]: https://github.com/atsamd-rs/atsamd/tree/master/boards/trellis_m4/
[trinket_m0]: https://github.com/atsamd-rs/atsamd/tree/master/boards/trinket_m0/
[wio_lite_mg126]: https://github.com/atsamd-rs/atsamd/tree/master/boards/wio_lite_mg126
[wio_lite_w600]: https://github.com/atsamd-rs/atsamd/tree/master/boards/wio_lite_w600
[wio_terminal]: https://github.com/atsamd-rs/atsamd/tree/master/boards/wio_terminal
[xiao_m0]: https://github.com/atsamd-rs/atsamd/tree/master/boards/xiao_m0


### `async` APIs

[`atsamd_hal`](https://crates.io/crates/atsamd-hal) provides APIs for using `async`/`await` constructs with some of its peripherals. To enable `async` support, use the `async` Cargo feature.
Detailed documentation is provided in the `atsamd_hal::async_hal` module. The [metro_m4](https://github.com/atsamd-rs/atsamd/tree/master/boards/metro_m4/examples) and
[feather_m0](https://github.com/atsamd-rs/atsamd/tree/master/boards/feather_m0/examples) feature complete examples showing how to use async APIs.

Please note that you must bring your own executor runtime such as [`embassy-executor`](https://crates.io/crates/embassy-executor) or [`rtic`](https://crates.io/crates/rtic) to be able to
use the async APIs.

#### Supported peripherals

* SPI
* I2C
* USART
* DMAC
* EIC (GPIO interrupts)
* Timers

### Examples

The BSPs include examples to quickly get up and running with the board. Building the examples
requires changing directory into one of the board support package directories, and some examples
will require additional features:

```bash
$ cd boards/metro_m0
$ cargo build --examples --features="usb"
```

A new firmware can be made from one of the examples:
  1. Create a new Cargo package for the firmware `cargo new my_firmware`, `cd my_firmware`
  2. Copy the BSP example source file `cp feather_m0/examples/blinky_basic.rs src/main.rs`
  3. Copy Cargo config and memory layout `cp -R feather_m0/.cargo feather_m0/memory.x .`
  4. Add the BSP and any other required dependencies to `Cargo.toml`:
``` TOML
[dependencies]
feather_m0 = "0.13"
panic-halt = "0.2"
```
  5. `cargo build` should create an ELF in `target/thumbv6m-none-eabi/debug/my_firmware`

## Building

You'll need to add the proper compilation target prior to building:

```bash
$ # for samd11, samd21:
$ rustup target add thumbv6m-none-eabi
$ # for samd51, same51, same53, same54:
$ rustup target add thumbv7em-none-eabihf
```

### CI

If you'd like to build all the same things that the CI would build but on your local system, you can run:

```bash
$ ./build-all.py
```

## Running and debugging firmware on target hardware

See our wiki page about [loading code onto the device](https://github.com/atsamd-rs/atsamd/wiki/Loading-code-onto-the-device).

## Adding a new board

See our wiki page about [adding a new board](https://github.com/atsamd-rs/atsamd/wiki/Adding-a-new-board).

## License

The included SVD files are sourced from http://packs.download.atmel.com/ and
are licensed under the Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
http://www.apache.org/licenses/LICENSE-2.0).

The remainder of the code is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
