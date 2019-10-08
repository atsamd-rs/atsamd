//! USB Device support

use crate::gpio;

pub use usb_device;

mod bus;
pub use self::bus::UsbBus;

mod devicedesc;
use self::devicedesc::Descriptors;

/// Emit SOF at 1Khz on this pin when configured as function H
pub type SofPad = gpio::Pa23<gpio::PfH>;

/// USB D- is connected here
pub type DmPad = gpio::Pa24<gpio::PfH>;

/// USB D+ is connected here
pub type DpPad = gpio::Pa25<gpio::PfH>;
