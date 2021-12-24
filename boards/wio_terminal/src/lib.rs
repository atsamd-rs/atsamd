//! `wio-terminal` is a Board Support Package (BSP) which provides a type-safe
//! API for the Seeed Studio [Wio Terminal].
//!
//! This crate is essentially a thin wrapper for [atsamd-hal], and re-exports
//! it along with some of its members.
//!
//! [Wio Terminal]: https://www.seeedstudio.com/Wio-Terminal-p-4509.html
//! [atsamd-hal]: https://github.com/atsamd-rs/atsamd

#![no_std]
#![allow(warnings)]

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

// Re-export the HAL and the PAC to give the user lower-level access to the
// device should they need it.
pub use atsamd_hal::{self as hal, pac};

// The accelerometer crate contains a number of traits and types which may be
// useful to the user.
pub use lis3dh::accelerometer;

// `prelude` is the only module from this crate which is public, as the
// remaining have their members exposed via the Sets struct.
pub mod prelude;

mod buttons;
mod display;
mod pins;
mod sensors;
mod serial;
mod sound;
mod storage;

pub use buttons::*;
pub use display::*;
pub use pins::Pins;
pub use pins::*;
pub use sensors::*;
pub use serial::*;
pub use sound::*;
pub use storage::*;

#[cfg(feature = "wifi")]
mod wifi;
#[cfg(feature = "wifi")]
pub use wifi::{rpcs as wifi_rpcs, wifi_prelude, Wifi, WifiPins};
#[cfg(feature = "wifi")]
pub mod wifi_types {
    pub use seeed_erpc::{BssType, IPInfo, L3Interface, Security, WifiMode, BSSID, SSID};
}
#[cfg(feature = "wifi-fw-before-212")]
pub const WIFI_UART_BAUD: u32 = 1843200;
#[cfg(not(feature = "wifi-fw-before-212"))]
pub const WIFI_UART_BAUD: u32 = 614400;
