//! # Clocking API
//!
//! Users are encouraged to use [`v2`] variant of an API because of the richer
//! feature set and safety.
use atsamd_hal_macros::{hal_cfg, hal_module};

#[hal_module(
    any("clock-d11", "clock-d21") => "clock/v1_thumbv6m.rs",
    "clock-d5x" => "clock/v1_thumbv7em.rs",
)]
pub mod v1 {}

pub use v1::*;

#[hal_cfg("clock-d5x")]
pub mod v2;
