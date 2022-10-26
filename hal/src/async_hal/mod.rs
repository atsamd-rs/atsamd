//! Async APIs

// #[cfg(feature = "samd11")]
// #[path = "irqs_samd11.rs"]
// mod irqs;
//
// #[cfg(feature = "samd21")]
// #[path = "irqs_samd21.rs"]
// mod irqs;
//
// #[cfg(feature = "samd51")]
// #[path = "irqs_thumbv7em.rs"]
// mod irqs;
//
// pub mod interrupt {
//     pub use super::irqs::*;
//     pub use cortex_m::interrupt::{CriticalSection, Mutex};
//     pub use embassy::interrupt::{declare, take, Interrupt};
//
//     #[cfg(any(feature = "samd11", feature = "samd21"))]
//     /// thumbv6m targets have 4 different interrupt priority levels
//     pub use embassy_hal_common::interrupt::Priority2 as Priority;
//
//     #[cfg(feature = "min-samd51g")]
//     /// thumbv7em targets have 8 different interrupt priority levels
//     pub use embassy_hal_common::interrupt::Priority3 as Priority;
// }

pub mod timer;
