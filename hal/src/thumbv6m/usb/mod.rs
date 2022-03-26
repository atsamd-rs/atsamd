//! USB Device support

use crate::gpio::v2::{AlternateG, Pin, PA23, PA24, PA25};

pub use usb_device;

mod bus;
pub use self::bus::UsbBus;

mod devicedesc;
use self::devicedesc::Descriptors;

/// Emit SOF at 1Khz on this pin
pub type SofPad = Pin<PA23, AlternateG>;

/// USB D-
pub type DmPad = Pin<PA24, AlternateG>;

/// USB D+
pub type DpPad = Pin<PA25, AlternateG>;
