pub mod calibration;
pub mod clock;
pub mod sercom;

#[cfg(feature = "usb")]
pub mod usb;

// This is merely to avoid breaking the public API.
// Added after the PWM module was refactored into common.
pub use crate::pwm;
