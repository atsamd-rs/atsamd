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
#[cfg(all(
    feature = "samd21",
    not(any(feature = "samd21el", feature = "samd21gl"))
))]
pub mod usb;
