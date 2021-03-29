#![no_std]

pub extern crate embedded_hal as hal;

pub use paste;

pub mod typelevel;

#[cfg(feature = "samd11c")]
pub use atsamd11c as target_device;

#[cfg(feature = "samd21e")]
pub use atsamd21e as target_device;

#[cfg(feature = "samd21g")]
pub use atsamd21g as target_device;

#[cfg(feature = "samd21j")]
pub use atsamd21j as target_device;

#[cfg(feature = "samd51g")]
pub use atsamd51g as target_device;

#[cfg(feature = "samd51j")]
pub use atsamd51j as target_device;

#[cfg(feature = "samd51n")]
pub use atsamd51n as target_device;

#[cfg(feature = "samd51p")]
pub use atsamd51p as target_device;

#[cfg(feature = "same51g")]
pub use atsame51g as target_device;

#[cfg(feature = "same51j")]
pub use atsame51j as target_device;

#[cfg(feature = "same51n")]
pub use atsame51n as target_device;

#[cfg(feature = "same53j")]
pub use atsame53j as target_device;

#[cfg(feature = "same53n")]
pub use atsame53n as target_device;

#[cfg(feature = "same54n")]
pub use atsame54n as target_device;

#[cfg(feature = "same54p")]
pub use atsame54p as target_device;

#[cfg(feature = "use_rtt")]
pub use jlink_rtt;

#[cfg(feature = "use_rtt")]
#[macro_export]
macro_rules! dbgprint {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut out = $crate::jlink_rtt::NonBlockingOutput::new();
            writeln!(out, $($arg)*).ok();
        }
    };
}

#[cfg(not(feature = "use_rtt"))]
#[macro_export]
macro_rules! dbgprint {
    ($($arg:tt)*) => {{}};
}

#[macro_use]
pub mod common;
pub use self::common::*;

// The following modules are included purely for backward compatibility reasons.
// Whenever major breaking changes are made to the HAL next, these modules
// should be removed.

#[cfg(feature = "samd51")]
pub mod samd51 {
    #[cfg(feature = "unproven")]
    pub use crate::pwm;
}

#[cfg(feature = "same51")]
pub mod same51 {
    #[cfg(feature = "unproven")]
    pub use crate::pwm;
}

#[cfg(feature = "same53")]
pub mod same53 {
    #[cfg(feature = "unproven")]
    pub use crate::pwm;
}

#[cfg(feature = "same54")]
pub mod same54 {
    #[cfg(feature = "unproven")]
    pub use crate::pwm;
}
