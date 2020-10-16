pub mod sercom;

#[cfg(feature = "usb")]
pub mod usb;

// This is merely to avoid breaking the public API.
// Added after the PWM module was refactored into common.
#[cfg(feature = "unproven")]
pub use crate::pwm;
