//! USB Device support

use crate::gpio;

pub use usb_device;

mod bus;
pub use self::bus::UsbBus;

mod devicedesc;
use self::devicedesc::Descriptors;

/// Default SOF pad
#[allow(deprecated)]
pub type SofPad = gpio::v1::Pa23<gpio::v1::PfH>;
/// Default USB D- pad
#[allow(deprecated)]
pub type DmPad = gpio::v1::Pa24<gpio::v1::PfH>;
/// Default USB D+ pad
#[allow(deprecated)]
pub type DpPad = gpio::v1::Pa25<gpio::v1::PfH>;
