use atsamd_hal_macros::{hal_cfg, hal_module};

#[cfg(feature = "unproven")]
#[hal_module(
    any("adc-d11", "adc-d21") => "adc/d11.rs",
    "adc-d5x" => "adc/d5x.rs",
)]
pub mod adc {}

#[hal_module(
    any("nvmctrl-d11", "nvmctrl-d21") => "calibration/d11.rs",
    "nvmctrl-d5x" => "calibration/d5x.rs",
)]
pub mod calibration {}

#[hal_module(
    any("clock-d11", "clock-d21") => "timer/d11.rs",
    "clock-d5x" => "timer/d5x.rs",
)]
pub mod timer {}

#[hal_module(
    any("eic-d11", "eic-d21") => "eic/d11/mod.rs",
    "eic-d5x" => "eic/d5x/mod.rs",
)]
pub mod eic {}

#[cfg(feature = "usb")]
#[hal_module(
    any("usb-d11", "usb-d21") => "usb/d11/mod.rs",
    "usb-d5x" => "usb/d5x/mod.rs",
)]
pub mod usb {}

#[cfg(feature = "unproven")]
#[hal_module(
    any("clock-d11", "clock-d21") => "pwm/d11.rs",
    "clock-d5x" => "pwm/d5x.rs",
)]
pub mod pwm {}

#[hal_module(
    any("clock-d11", "clock-d21") => "clock/d11.rs",
    "clock-d5x" => "clock/d5x/mod.rs",
)]
pub mod clock {}

#[hal_module("aes")]
pub mod aes {}

#[hal_module("dsu-d5x")]
pub mod dsu {}

#[hal_module("pukcc")]
pub mod pukcc {}

#[hal_module("qspi")]
pub mod qspi {}

#[hal_module("trng")]
pub mod trng {}

#[cfg(feature = "unproven")]
#[hal_module("icm")]
pub mod icm {}

#[hal_module("nvmctrl-d5x")]
pub mod nvm {}

#[cfg(feature = "can")]
#[hal_module(any("can0", "can1"))]
pub mod can {}

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
