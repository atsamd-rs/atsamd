#[cfg(feature = "unproven")]
#[cfg_attr(feature = "thumbv6", path = "adc/d11.rs")]
#[cfg_attr(feature = "thumbv7", path = "adc/d5x.rs")]
pub mod adc;

#[cfg_attr(feature = "thumbv6", path = "calibration/d11.rs")]
#[cfg_attr(feature = "thumbv7", path = "calibration/d5x.rs")]
pub mod calibration;

#[cfg_attr(feature = "thumbv6", path = "timer/d11.rs")]
#[cfg_attr(feature = "thumbv7", path = "timer/d5x.rs")]
pub mod timer;

#[cfg_attr(feature = "thumbv6", path = "eic/d11/mod.rs")]
#[cfg_attr(feature = "thumbv7", path = "eic/d5x/mod.rs")]
pub mod eic;

#[cfg(all(feature = "usb", feature = "has-usb"))]
#[cfg_attr(feature = "thumbv6", path = "usb/d11/mod.rs")]
#[cfg_attr(feature = "thumbv7", path = "usb/d5x/mod.rs")]
pub mod usb;

#[cfg(feature = "unproven")]
#[cfg_attr(feature = "thumbv6", path = "pwm/d11.rs")]
#[cfg_attr(feature = "thumbv7", path = "pwm/d5x.rs")]
pub mod pwm;

#[cfg_attr(feature = "thumbv6", path = "clock/d11.rs")]
#[cfg_attr(feature = "thumbv7", path = "clock/d5x/mod.rs")]
pub mod clock;

#[cfg(feature = "thumbv7")]
pub mod aes;

#[cfg(feature = "thumbv7")]
pub mod dsu;

#[cfg(feature = "thumbv7")]
pub mod pukcc;

#[cfg(feature = "thumbv7")]
pub mod qspi;

#[cfg(feature = "thumbv7")]
pub mod trng;

#[cfg(feature = "unproven")]
#[cfg(feature = "thumbv7")]
pub mod icm;

#[cfg(feature = "thumbv7")]
pub mod nvm;

#[cfg(all(
    any(feature = "has-can0", feature = "has-can1"),
    feature = "can",
    feature = "thumbv7"
))]
pub mod can;

#[cfg(feature = "unproven")]
#[cfg_attr(feature = "thumbv6", path = "watchdog/d11.rs")]
#[cfg_attr(feature = "thumbv7", path = "watchdog/d5x.rs")]
pub mod watchdog;

#[cfg_attr(feature = "thumbv6", path = "reset_cause/d11.rs")]
#[cfg_attr(feature = "thumbv7", path = "reset_cause/d5x.rs")]
mod reset_cause;

pub use reset_cause::*;

#[cfg_attr(feature = "thumbv6", path = "serial_number/d11.rs")]
#[cfg_attr(feature = "thumbv7", path = "serial_number/d5x.rs")]
mod serial_number;

pub use serial_number::*;
