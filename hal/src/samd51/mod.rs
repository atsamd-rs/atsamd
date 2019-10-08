pub mod calibration;
pub mod clock;
pub mod pwm;
pub mod sercom;
pub mod timer;
pub mod trng;

#[cfg(feature="unproven")]
pub mod adc;

#[cfg(feature = "usb")]
pub mod usb;
