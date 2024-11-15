#![no_std]

use embedded_hal_02 as ehal_02;
pub use embedded_hal_1 as ehal;
pub use embedded_hal_nb as ehal_nb;
pub use embedded_io;
pub use fugit;
pub use nb;
pub use paste;

#[cfg(feature = "async")]
pub use embedded_hal_async as ehal_async;

#[cfg(feature = "async")]
pub use embedded_io_async;

pub mod typelevel;
mod util;

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

#[cfg(feature = "async")]
pub mod async_hal;

#[cfg(feature = "device")]
pub mod delay;
#[cfg(feature = "device")]
pub mod gpio;
#[cfg(feature = "device")]
pub mod interrupt;
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

#[cfg(feature = "rtic")]
pub use rtic_time;

#[doc(hidden)]
mod peripherals;
#[doc(inline)]
#[allow(unused_imports)]
pub use crate::peripherals::*;

#[macro_use]
mod bsp_peripherals_macro;
