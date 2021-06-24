pub mod eic;

mod reset_cause;
pub use reset_cause::*;

mod serial_number;
pub use serial_number::*;

pub mod calibration;
pub mod clock;
pub mod timer;

pub mod adc;

pub mod pwm;

pub mod watchdog;

#[cfg(feature = "usb")]
#[cfg(feature = "samd21")]
pub mod usb;

pub(crate) mod sercom;
