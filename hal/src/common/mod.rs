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
#[cfg(feature = "device")]
pub mod spi_common;
pub mod time;
pub mod timer_params;
pub mod timer_traits;

#[cfg(feature = "unproven")]
#[cfg(feature = "dma")]
pub mod dmac;

#[cfg(any(feature = "samd11", feature = "samd21"))]
mod thumbv6m;
#[cfg(any(feature = "samd11", feature = "samd21"))]
pub use thumbv6m::*;

#[cfg(feature = "min-samd51g")]
mod thumbv7em;
#[cfg(feature = "min-samd51g")]
pub use thumbv7em::*;

// This module maintains backwards compatibility for the v1 SERCOM pads API
#[cfg(feature = "device")]
pub mod pad {
    pub use crate::sercom::v1::pads::PadPin;
}
