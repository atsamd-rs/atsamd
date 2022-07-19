#![no_std]
#![recursion_limit = "1024"]

pub use atsamd_hal as hal;
pub use hal::pac;
pub use pins::*;

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub mod devices;
pub mod pins;
