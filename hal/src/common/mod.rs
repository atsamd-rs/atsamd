pub mod delay;
pub mod gpio;
pub mod prelude;
pub mod sercom;
pub mod sleeping_delay;
pub mod spi_common;
pub mod time;
pub mod timer_params;
pub mod timer_traits;

#[cfg(any(feature = "samd11", feature = "samd21"))]
mod thumbv6m;
#[cfg(any(feature = "samd11", feature = "samd21"))]
pub use thumbv6m::*;

#[cfg(feature = "min-samd51g")]
mod thumbv7em;
#[cfg(feature = "min-samd51g")]
pub use thumbv7em::*;

// This module maintains backwards compatibility for the v1 SERCOM pads API
pub mod pad {
    pub use crate::sercom::v1::pads::PadPin;
}
