#![no_std]
#![recursion_limit = "1024"]

#[cfg(feature = "unproven")]
pub mod buttons;

pub mod pins;
pub use atsamd_hal as hal;

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use pins::Pins;

pub use hal::common::*;
pub use hal::pac;
pub use hal::samd51::*;
