pub mod calibration;
pub mod clock;
pub mod sercom;
pub mod timer;

#[cfg(feature = "unproven")]
pub mod pwm;

#[cfg(feature = "unproven")]
pub mod adc;

#[cfg(feature = "usb")]
pub mod usb;
