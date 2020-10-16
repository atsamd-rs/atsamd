// This is merely to avoid breaking the public API.
// Added after the PWM module was refactored into common.
#[cfg(feature = "unproven")]
pub use crate::pwm;
