pub mod eic;

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

#[cfg(feature = "unproven")]
#[cfg(feature = "samd21")]
pub mod dmac;

pub(crate) mod sercom;
