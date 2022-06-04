#![no_std]

pub use embedded_hal as ehal;

pub use paste;

pub mod typelevel;

#[cfg(not(any(feature = "library", feature = "device")))]
compile_error!(
    "The HAL is usually built for a specific target device, selected using a \
    feature.  If atsamd-hal is being built as a library, bypass this check by \
    specifying the `library` feature"
);

#[cfg(feature = "samd11c")]
pub use atsamd11c as pac;

#[cfg(feature = "samd11d")]
pub use atsamd11d as pac;

#[cfg(feature = "samd21e")]
pub use atsamd21e as pac;

#[cfg(feature = "samd21g")]
pub use atsamd21g as pac;

#[cfg(feature = "samd21j")]
pub use atsamd21j as pac;

#[cfg(feature = "samd51g")]
pub use atsamd51g as pac;

#[cfg(feature = "samd51j")]
pub use atsamd51j as pac;

#[cfg(feature = "samd51n")]
pub use atsamd51n as pac;

#[cfg(feature = "samd51p")]
pub use atsamd51p as pac;

#[cfg(feature = "same51g")]
pub use atsame51g as pac;

#[cfg(feature = "same51j")]
pub use atsame51j as pac;

#[cfg(feature = "same51n")]
pub use atsame51n as pac;

#[cfg(feature = "same53j")]
pub use atsame53j as pac;

#[cfg(feature = "same53n")]
pub use atsame53n as pac;

#[cfg(feature = "same54n")]
pub use atsame54n as pac;

#[cfg(feature = "same54p")]
pub use atsame54p as pac;

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

#[cfg(feature = "device")]
pub mod delay;
#[cfg(feature = "device")]
pub mod gpio;
#[cfg(feature = "device")]
pub mod prelude;
#[cfg(feature = "device")]
pub mod rtc;
#[cfg(feature = "device")]
pub mod sercom;
pub mod sleeping_delay;
pub mod time;
pub mod timer_params;
pub mod timer_traits;

#[cfg(all(feature = "unproven", feature = "dma"))]
pub mod dmac;

#[cfg(all(feature = "usb", feature = "samd11"))]
compile_error!("'usb' is enabled, but USB isn't supported on SAMD11");

#[cfg(all(
    feature = "usb",
    not(any(feature = "samd21", feature = "min-samd51g", feature = "library"))
))]
compile_error!("The 'usb' feature is enabled, but not a chip with USB support");

#[cfg(any(feature = "samd11", feature = "samd21"))]
pub mod thumbv6m;
#[cfg(any(feature = "samd11", feature = "samd21"))]
pub use crate::thumbv6m::*;

#[cfg(feature = "min-samd51g")]
pub mod thumbv7em;
#[cfg(feature = "min-samd51g")]
pub use crate::thumbv7em::*;

#[macro_use]
mod bsp_peripherals_macro;
pub use bsp_peripherals_macro::*;
