//! # Version 2 of the SERCOM module
//!
//! This module provides a new API for the SERCOM peripherals. So far, only the
//! [`pads`] module has been updated, but it is expected that the `uart`, `spi`
//! and `i2c` modules will receive updates as well.

pub mod pads;
pub use pads::*;

#[cfg(any(feature = "samd11", feature = "samd21"))]
pub use crate::common::thumbv6m::sercom::v2::*;

#[cfg(feature = "min-samd51g")]
pub use crate::common::thumbv7em::sercom::v2::*;
