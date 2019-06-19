#![no_std]
#![cfg_attr(feature = "usb", feature(align_offset, ptr_offset_from))]

#[cfg(feature = "samd21e18a")]
pub use atsamd21e18a as target_device;

#[cfg(feature = "samd21g18a")]
pub use atsamd21g18a as target_device;

#[cfg(feature = "samd51g19a")]
pub use atsamd51g19a as target_device;

#[cfg(feature = "samd51j19a")]
pub use atsamd51j19a as target_device;

#[cfg(feature = "samd51j20a")]
pub use atsamd51j20a as target_device;

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

pub extern crate embedded_hal as hal;
pub use paste;

#[cfg(not(feature="samd51"))]
mod calibration;

#[cfg(not(feature = "samd51"))]
pub mod clock;

#[cfg(feature = "samd51")]
#[path = "clock51.rs"]
pub mod clock;

#[macro_use]
mod pad;

#[cfg(not(feature = "samd51"))]
pub mod sercom;

#[cfg(feature = "samd51")]
#[path = "sercom51/mod.rs"]
pub mod sercom;

#[cfg(feature = "samd51")]
#[path = "timer51.rs"]
pub mod timer;

#[cfg(not(feature = "samd51"))]
pub mod timer;

#[cfg(feature="samd51")]
#[path = "pwm51.rs"]
pub mod pwm;

#[cfg(not(feature="samd51"))]
pub mod pwm;

pub mod delay;
pub mod gpio;
pub mod prelude;
pub mod time;

#[cfg(feature = "usb")]
pub mod usb;
