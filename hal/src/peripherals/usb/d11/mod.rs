//! USB Device support

use crate::gpio::{
    AlternateG,
    pin::{PA24, PA25, Pin},
};
use atsamd_hal_macros::hal_cfg;

pub use usb_device;

mod buffer;
mod bus;
pub use self::buffer::*;
pub use self::bus::UsbBus;

mod devicedesc;
use self::devicedesc::Descriptors;

/// Emit SOF at 1Khz on this pin when configured as function G
#[hal_cfg(any("usb-d21", "usb-d5x"))]
pub type SofPad = Pin<crate::gpio::pin::PA23, AlternateG>;

/// USB D- is connected here
pub type DmPad = Pin<PA24, AlternateG>;

/// USB D+ is connected here
pub type DpPad = Pin<PA25, AlternateG>;
