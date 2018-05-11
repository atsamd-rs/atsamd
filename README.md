# atsamd21 support for Rust

This repo holds various things that support/enable working with atmel samd21 based
devices, such as the Adafruit Metro M0, Trinket M0 and Gemma M0, using Rust.

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

Since a couple of different MCUs are used, building the examples requires changing
directory into one of the board support crate dirs prior to building:

```bash
$ cd metro_m0
$ cargo build --examples
$ cd ../gemma_m0
$ cargo build --examples
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
