//! Abstracts out which board we're using for our example
#![no_std]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

#[cfg(feature = "bsp-feather_m0")]
pub use feather_m0 as bsp;
#[cfg(feature = "bsp-feather_m4")]
pub use feather_m4 as bsp;
#[cfg(feature = "bsp-metro_m0")]
pub use metro_m0 as bsp;
#[cfg(feature = "bsp-metro_m4")]
pub use metro_m4 as bsp;
#[cfg(feature = "bsp-pygamer")]
pub use pygamer as bsp;
#[cfg(feature = "bsp-samd11_bare")]
pub use samd11_bare as bsp;
#[cfg(feature = "bsp-wio_terminal")]
pub use wio_terminal as bsp;

#[cfg(all(
    not(feature = "bsp-feather_m0"),
    not(feature = "bsp-feather_m4"),
    not(feature = "bsp-metro_m0"),
    not(feature = "bsp-metro_m4"),
    not(feature = "bsp-pygamer"),
    not(feature = "bsp-samd11_bare"),
    not(feature = "bsp-wio_terminal"),
))]
compile_error!("Must select a board");

#[cfg(any(
    feature = "bsp-feather_m0",
    feature = "bsp-metro_m0",
    feature = "bsp-samd11_bare",
))]
mod thumbv6m;
#[cfg(any(
    feature = "bsp-feather_m0",
    feature = "bsp-metro_m0",
    feature = "bsp-samd11_bare",
))]
pub use crate::thumbv6m::*;

#[cfg(any(
    feature = "bsp-feather_m4",
    feature = "bsp-metro_m4",
    feature = "bsp-pygamer",
    feature = "bsp-wio_terminal",
))]
mod thumbv7em;
#[cfg(any(
    feature = "bsp-feather_m4",
    feature = "bsp-metro_m4",
    feature = "bsp-pygamer",
    feature = "bsp-wio_terminal",
))]
pub use crate::thumbv7em::*;

pub use bsp::hal::usb::{usb_device::bus::UsbBusAllocator, UsbBus};

pub trait Board {
    fn init(
        peripherals: bsp::pac::Peripherals,
    ) -> (
        bsp::hal::clock::GenericClockController,
        bsp::RedLed,
        UsbBusAllocator<UsbBus>,
    );
}
