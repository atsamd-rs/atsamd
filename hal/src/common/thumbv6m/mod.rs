pub mod eic;

mod reset_cause;
pub use reset_cause::*;

mod serial_number;
pub use serial_number::*;

#[cfg(feature = "unproven")]
pub mod adc;

#[cfg(all(feature = "unproven", feature = "samd21"))]
pub mod ptc;

#[cfg(feature = "unproven")]
pub mod pwm;

#[cfg(feature = "unproven")]
pub mod watchdog;

pub(crate) mod sercom;
