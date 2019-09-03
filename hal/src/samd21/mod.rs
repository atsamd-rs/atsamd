pub mod calibration;
pub mod clock;
pub mod pwm;
pub mod sercom;
pub mod timer; 

#[cfg(feature = "unproven")]
pub mod adc;

#[cfg(feature = "usb")]
pub mod usb;
