//! # Clocking API
//!
//! Users are encouraged to use [`v2`] variant of an API because of the richer
//! feature set and safety.

pub mod v1;
pub use v1::*;

pub mod v2;
