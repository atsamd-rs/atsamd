pub mod delay;
pub mod sleeping_delay;
#[macro_use]
pub mod pad;
pub mod prelude;
pub mod spi_common;
pub mod time;
pub mod timer_traits;

#[cfg(any(feature = "samd11", feature = "samd21"))]
mod thumbv6m;
#[cfg(any(feature = "samd11", feature = "samd21"))]
pub use thumbv6m::*;

#[cfg(any(feature = "samd51", feature = "same54"))]
mod thumbv7em;
#[cfg(any(feature = "samd51", feature = "same54"))]
pub use thumbv7em::*;
