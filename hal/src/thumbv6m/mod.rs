pub mod eic;

mod reset_cause;
pub use reset_cause::*;

mod serial_number;
pub use serial_number::*;

pub mod calibration;
pub mod clock;
pub mod timer;

#[cfg(feature = "unproven")]
pub mod adc;

#[cfg(feature = "unproven")]
pub mod pwm;

#[cfg(feature = "unproven")]
pub mod watchdog;

#[cfg(feature = "usb")]
#[cfg(feature = "samd21")]
pub mod usb;

#[cfg(feature = "unproven")]
pub mod ptc;

pub(crate) mod sercom;
