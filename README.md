# atsamd21 support for Rust

This repo holds various things that support/enable working with atmel samd21 based
devices, such as the Adafruit Metro M0, using Rust.

There are a couple of crates provided by this repo:

* [`atsamd21g18a`](https://docs.rs/atsamd21g18a/latest/atsamd21g18a/) is an
  auto-generated crate providing access to the peripherals
  specified for this device by its SVD file.
* [`atsamd21-hal`](https://docs.rs/atsamd21-hal/latest/atsamd21_hal/) is the result
  of reading the datasheet for the device and encoding
  a type-safe layer over the raw `atsamd21g18a` crate.  This crate implements traits
  specified by the `embedded-hal` project, making it compatible with various drivers
  in the embedded rust ecosystem.
* [`metro_m0`](https://docs.rs/metro_m0/latest/metro_m0/) is a board support crate
  for the Adafruit Metro M0 board.  It re-exports the `atsamd21-hal` crate functionality
  using more convenient names; for example, the IO pins are exported using the labels
  printed on the board rather than the more abstract and harder to remember port and
  pin numbers used by the underlying device.

The top level of this repo is a cargo workspace that is used to build everything
in the repo.  The `examples` directory shows examples that execute on the Metro M0.

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
