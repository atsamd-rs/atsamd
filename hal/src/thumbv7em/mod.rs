pub mod calibration;
pub mod clock;
pub mod eic;
pub mod pukcc;
pub mod qspi;
pub(crate) mod sercom;
pub mod timer;
pub mod trng;

#[cfg(feature = "usb")]
pub mod usb;

mod reset_cause;
pub use reset_cause::*;

mod serial_number;
pub use serial_number::*;

#[cfg(feature = "unproven")]
pub mod adc;

#[cfg(feature = "unproven")]
pub mod pwm;

#[cfg(feature = "unproven")]
pub mod watchdog;
