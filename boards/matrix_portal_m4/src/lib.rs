#![no_std]

pub use atsamd_hal as hal;
pub use hal::ehal;
pub use hal::pac;

pub mod pins;
pub use pins::*;

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;
