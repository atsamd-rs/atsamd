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

macro_rules! define_pac {
    ( $( ($pac:ident, $feat:literal)),+ ) => {
        $(
            #[cfg(feature = $feat)]
            pub use $pac as pac;
        )+
    };
}

define_pac!(
    (atsamd11c, "atsamd11c"),
    (atsamd11d, "atsamd11d"),
    (atsamd21e, "atsamd21e"),
    (atsamd21g, "atsamd21g"),
    (atsamd21j, "atsamd21j"),
    (atsamd51g, "atsamd51g"),
    (atsamd51j, "atsamd51j"),
    (atsamd51n, "atsamd51n"),
    (atsamd51p, "atsamd51p"),
    (atsame51g, "atsame51g"),
    (atsame51j, "atsame51j"),
    (atsame51n, "atsame51n"),
    (atsame53j, "atsame53j"),
    (atsame53n, "atsame53n"),
    (atsame54n, "atsame54n"),
    (atsame54p, "atsame54p")
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
pub use bsp_peripherals_macro::*;
