//! Async APIs

#[cfg(feature = "samd11")]
#[path = "irqs_samd11.rs"]
mod irqs;

#[cfg(feature = "samd21")]
#[path = "irqs_samd21.rs"]
mod irqs;

#[cfg(feature = "samd51")]
#[path = "irqs_thumbv7em.rs"]
mod irqs;

pub mod interrupt {
    pub use super::irqs::*;
    pub use cortex_m::interrupt::{CriticalSection, Mutex};
    pub use embassy::interrupt::{declare, take, Interrupt};

    // TODO Priority2 seems to only be true for thumbv6m chips
    pub use embassy_hal_common::interrupt::Priority2 as Priority;
}

pub mod timer;
