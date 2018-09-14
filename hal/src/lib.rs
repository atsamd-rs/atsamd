#![no_std]
#![feature(align_offset)]
#![feature(ptr_offset_from)]
#![recursion_limit = "1024"]

#[cfg(feature = "samd21g18a")]
pub extern crate atsamd21g18a;
#[cfg(feature = "samd21g18a")]
pub use atsamd21g18a as target_device;

#[cfg(feature = "samd21e18a")]
pub extern crate atsamd21e18a;
#[cfg(feature = "samd21e18a")]
pub use atsamd21e18a as target_device;

#[macro_use]
extern crate bitfield;

pub extern crate jlink_rtt;
extern crate vcell;

/*
#[cfg(feature = "use_semihosting")]
extern crate cortex_m_semihosting;
#[cfg(feature = "use_semihosting")]
#[macro_export]
macro_rules! dbgprint {
    ($($arg:tt)*) => {
        {
            use cortex_m_semihosting::hio;
            use core::fmt::Write;
            hio::hstderr().map(|mut stdout| writeln!(stdout, $($arg)*)).ok();
        }
    };
}
#[cfg(not(feature = "use_semihosting"))]
#[macro_export]
macro_rules! dbgprint {
    ($($arg:tt)*) => {{}};
}
*/
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

#[macro_use]
extern crate cortex_m;
pub extern crate embedded_hal as hal;
pub extern crate mashup;
extern crate nb;
pub extern crate usb_device;
extern crate void;

mod calibration;
pub mod clock;
pub mod delay;
pub mod gpio;
pub mod prelude;
pub mod sercom;
pub mod time;
pub mod timer;
pub mod usb;
