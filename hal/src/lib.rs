#![no_std]
#![cfg_attr(feature = "usb", feature(align_offset, ptr_offset_from))]

#[cfg(feature = "samd21g18a")]
pub extern crate atsamd21g18a;
#[cfg(feature = "samd21g18a")]
pub use atsamd21g18a as target_device;

#[cfg(feature = "samd21e18a")]
pub extern crate atsamd21e18a;
#[cfg(feature = "samd21e18a")]
pub use atsamd21e18a as target_device;

#[cfg(feature = "samd51g19a")]
pub extern crate atsamd51g19a;
#[cfg(feature = "samd51g19a")]
pub use atsamd51g19a as target_device;

#[cfg(feature = "samd51j19a")]
pub extern crate atsamd51j19a;
#[cfg(feature = "samd51j19a")]
pub use atsamd51j19a as target_device;

#[cfg_attr(feature = "usb", macro_use)]
extern crate bitfield;

extern crate vcell;

#[cfg(feature = "use_rtt")]
pub extern crate jlink_rtt;

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

#[cfg_attr(feature = "usb", macro_use)]
extern crate cortex_m;
pub extern crate embedded_hal as hal;
extern crate nb;
pub extern crate paste;

#[cfg(feature = "usb")]
pub extern crate usb_device;

extern crate void;

#[cfg(not(feature="samd51"))]
mod calibration;

#[cfg(not(feature = "samd51"))]
pub mod clock;

#[cfg(feature = "samd51")]
#[path = "clock51.rs"]
pub mod clock;

#[cfg(not(feature = "samd51"))]
pub mod sercom;

#[cfg(feature = "samd51")]
#[path = "timer51.rs"]
pub mod timer;

#[cfg(not(feature = "samd51"))]
pub mod timer;

#[cfg(feature = "samd51")]
#[path = "sercom51/mod.rs"]
pub mod sercom;

#[cfg(all(feature = "unproven", feature="samd51"))]
pub mod pwm51;

#[cfg(feature = "unproven")]
pub mod pwm;

pub mod delay;
pub mod gpio;
pub mod prelude;
pub mod time;

#[cfg(feature = "usb")]
pub mod usb;
