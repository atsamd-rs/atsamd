use atsamd_hal_macros::{hal_cfg, hal_module};

#[cfg(feature = "device")]
pub mod adc;

#[hal_module(
    any("nvmctrl-d11", "nvmctrl-d21") => "calibration/d11.rs",
    any("nvmctrl-d5x", "nvmctrl-pic32cxsg") => "calibration/d5x.rs",
)]
pub mod calibration {}

#[hal_module(
    any("clock-d11", "clock-d21") => "timer/d11.rs",
    any("clock-d5x", "clock-pic32cxsg") => "timer/d5x.rs",
)]
pub mod timer {}

#[cfg(feature = "device")]
pub mod eic;

#[cfg(feature = "usb")]
#[hal_module(
    any("usb-d11", "usb-d21") => "usb/d11/mod.rs",
    any("usb-d5x", "usb-pic32cxsg") => "usb/d5x/mod.rs",
)]
pub mod usb {}

#[hal_module(
    any("clock-d11", "clock-d21") => "pwm/d11.rs",
    any("clock-d5x", "clock-pic32cxsg") => "pwm/d5x.rs",
)]
pub mod pwm {}

#[hal_module(
    any("clock-d11", "clock-d21") => "clock/d11.rs",
    any("clock-d5x", "clock-pic32cxsg") => "clock/d5x/mod.rs",
)]
pub mod clock {}

#[hal_module("aes")]
pub mod aes {}

#[hal_module(any("dsu-d5x", "dsu-pic32cxsg"))]
pub mod dsu {}

#[hal_module("pukcc")]
pub mod pukcc {}

#[hal_module("qspi")]
pub mod qspi {}

#[hal_module("trng")]
pub mod trng {}

#[hal_module("icm")]
pub mod icm {}

#[hal_module(any("nvmctrl-d5x", "nvmctrl-pic32cxsg"))]
pub mod nvm {}

#[cfg(feature = "can")]
#[hal_module(any("can0", "can1"))]
pub mod can {}

#[hal_module("wdt")]
pub mod watchdog {}

#[hal_module(any("pm-d11", "pm-d21", "rstc-d5x", "rstc-pic32cxsg"))]
mod reset_cause {}

#[hal_cfg(any("pm-d11", "pm-d21", "rstc-d5x", "rstc-pic32cxsg"))]
pub use reset_cause::*;

#[hal_module("serial-numbers")]
mod serial_number {}

#[hal_cfg("serial-numbers")]
pub use serial_number::*;
