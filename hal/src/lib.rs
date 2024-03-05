#![no_std]

pub use embedded_hal as ehal;
pub use fugit;
pub use paste;
pub mod typelevel;

#[cfg(not(any(feature = "library", feature = "device")))]
compile_error!(
    "The HAL is usually built for a specific target device, selected using a \
    feature.  If atsamd-hal is being built as a library, bypass this check by \
    specifying the `library` feature"
);

#[cfg(all(feature = "library", feature = "device"))]
compile_error!("Cannot combine `library` and `device` features");

#[cfg(all(feature = "library", feature = "dma"))]
compile_error!("Cannot combine `library` and `dma` features");

macro_rules! define_pac {
    ( $( ($pac:ident, $feat:literal)),+ ) => {
        $(
            #[cfg(feature = $feat)]
            pub use $pac as pac;
        )+
    };
}

define_pac!(
    (atsamd11c, "samd11c"),
    (atsamd11d, "samd11d"),
    (atsamd21e, "samd21e"),
    (atsamd21g, "samd21g"),
    (atsamd21j, "samd21j"),
    (atsamd21e, "samd21el"),
    (atsamd21g, "samd21gl"),
    (atsamd51g, "samd51g"),
    (atsamd51j, "samd51j"),
    (atsamd51n, "samd51n"),
    (atsamd51p, "samd51p"),
    (atsame51g, "same51g"),
    (atsame51j, "same51j"),
    (atsame51n, "same51n"),
    (atsame53j, "same53j"),
    (atsame53n, "same53n"),
    (atsame54n, "same54n"),
    (atsame54p, "same54p")
);

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

#[cfg(feature = "dma")]
pub mod dmac;

#[cfg(all(feature = "usb", feature = "device", not(feature = "has-usb")))]
compile_error!("The 'usb' feature is enabled, but this chip does not support USB");

#[cfg(feature = "thumbv6")]
#[doc(hidden)]
pub mod thumbv6m;
#[cfg(feature = "thumbv6")]
#[doc(inline)]
pub use crate::thumbv6m::*;

#[cfg(feature = "thumbv7")]
#[doc(hidden)]
pub mod thumbv7em;
#[cfg(feature = "thumbv7")]
#[doc(inline)]
pub use crate::thumbv7em::*;

#[macro_use]
mod bsp_peripherals_macro;
