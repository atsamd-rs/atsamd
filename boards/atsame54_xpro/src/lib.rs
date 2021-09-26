#![no_std]
#![recursion_limit = "1024"]

/// Pin definitions
pub mod pins;

pub use atsamd_hal as hal;

pub use hal::common::*;
pub use hal::pac;
pub use hal::same54::*;

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;
pub use pins::Pins;
