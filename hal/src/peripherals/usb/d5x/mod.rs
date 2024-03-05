//! USB Device support

use crate::gpio::{
    pin::{Pin, PA23, PA24, PA25},
    AlternateH,
};

pub use usb_device;

mod bus;
pub use self::bus::UsbBus;

mod devicedesc;
use self::devicedesc::Descriptors;

/// Default SOF pad
pub type SofPad = Pin<PA23, AlternateH>;

/// Default USB D- pad
pub type DmPad = Pin<PA24, AlternateH>;

/// Default USB D+ pad
pub type DpPad = Pin<PA25, AlternateH>;
