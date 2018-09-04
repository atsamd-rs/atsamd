#![no_std]

#[cfg(feature = "samd21g18a")]
pub extern crate atsamd21g18a;
#[cfg(feature = "samd21g18a")]
pub use atsamd21g18a as target_device;

#[cfg(feature = "samd21e18a")]
pub extern crate atsamd21e18a;
#[cfg(feature = "samd21e18a")]
pub use atsamd21e18a as target_device;

extern crate cortex_m;
pub extern crate embedded_hal as hal;
pub extern crate mashup;
extern crate nb;
extern crate void;

mod calibration;
pub mod clock;
pub mod delay;
pub mod gpio;
pub mod prelude;
pub mod sercom;
pub mod time;
pub mod timer;
pub mod usb;
