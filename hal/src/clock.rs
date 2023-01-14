//! # Clocking API
//!
//! Users are encouraged to use [`v2`] variant of an API because of the richer
//! feature set and safety.

#[cfg(feature = "thumbv6")]
#[path = "clock/v1_thumbv6m.rs"]
pub mod v1;

#[cfg(feature = "thumbv7")]
#[path = "clock/v1_thumbv7em.rs"]
pub mod v1;

pub use v1::*;

pub mod v2;
