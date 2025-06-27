//! Import the prelude to gain convenient access to helper traits
pub use crate::eic::EicPin;
pub use crate::timer_traits::InterruptDrivenTimer;
pub use fugit::ExtU32 as _;
pub use fugit::RateExtU32 as _;

#[cfg(feature = "rtic")]
pub use rtic_time::Monotonic as _;

#[cfg(feature = "rtic")]
pub use fugit::{ExtU64, ExtU64Ceil};
