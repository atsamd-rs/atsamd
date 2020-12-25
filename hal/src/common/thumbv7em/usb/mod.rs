//! USB Device support

use crate::gpio;

pub use usb_device;

mod bus;
pub use self::bus::UsbBus;

mod devicedesc;
use self::devicedesc::Descriptors;


use gpio::v2::*;

/// Emit SOF at 1Khz on this pin when configured as function H
pub type SofPad = Pin<PA23, AlternateH>;

/// USB D- is connected here
pub type DmPad = Pin<PA24, AlternateH>;

/// USB D+ is connected here
pub type DpPad = Pin<PA25, AlternateH>;
