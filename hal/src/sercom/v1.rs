//! # Version 1 of the SERCOM module
//!
//! This module retains the previous SERCOM API. The [`pads`] module provides a
//! compatibility shim that uses the new [`v2::pad`] module to implement the
//! old API. This API will eventually be deprecated and removed.
//!
//! [`v2::pad`]: super::v2::pad

pub mod pads;
pub use pads::*;

#[cfg(any(feature = "samd11", feature = "samd21"))]
pub use crate::common::thumbv6m::sercom::v1::*;

#[cfg(feature = "min-samd51g")]
pub use crate::common::thumbv7em::sercom::v1::*;
