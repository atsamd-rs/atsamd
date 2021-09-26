#![no_std]
#![recursion_limit = "1024"]

#[cfg(feature = "unproven")]
pub mod buttons;

pub mod pins;
pub use atsamd_hal as hal;

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use pins::Pins;

use hal::*;

pub use hal::common::*;
pub use hal::samd51::*;
pub use hal::pac;
