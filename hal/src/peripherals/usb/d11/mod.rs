//! USB Device support

use crate::gpio::{
    pin::{Pin, PA23, PA24, PA25},
    AlternateG,
};

pub use usb_device;

mod bus;
pub use self::bus::UsbBus;

mod devicedesc;
use self::devicedesc::Descriptors;

/// Emit SOF at 1Khz on this pin when configured as function G
pub type SofPad = Pin<PA23, AlternateG>;

/// USB D- is connected here
pub type DmPad = Pin<PA24, AlternateG>;

/// USB D+ is connected here
pub type DpPad = Pin<PA25, AlternateG>;
