#![no_std]

pub use atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use hal::pac;

pub mod pins;
pub mod sercom;
#[cfg(feature = "usb")]
pub mod usb;
