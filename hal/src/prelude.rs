//! Import the prelude to gain convenient access to helper traits
pub use crate::eic::pin::EicPin;
pub use crate::timer_traits::InterruptDrivenTimer as _atsamd_hal_timer_traits_InterruptDrivenTimer;
pub use fugit::ExtU32 as _;
pub use fugit::RateExtU32 as _;

// embedded-hal doesn’t yet have v2 in its prelude, so we need to
// export it ourselves
//
// We only have embedded-hal v0.2 items inour prelude if v1 is not enabled.
#[cfg(all(not(feature = "embedded-hal-1"), feature = "unproven"))]
pub use crate::ehal::digital::v2::InputPin as _atsamd_hal_embedded_hal_digital_v2_InputPin;
#[cfg(not(feature = "embedded-hal-1"))]
pub use crate::ehal::digital::v2::OutputPin as _atsamd_hal_embedded_hal_digital_v2_OutputPin;
#[cfg(all(not(feature = "embedded-hal-1"), feature = "unproven"))]
pub use crate::ehal::digital::v2::ToggleableOutputPin as _atsamd_hal_embedded_hal_digital_v2_ToggleableOutputPin;

#[cfg(not(feature = "embedded-hal-1"))]
pub use crate::ehal::prelude::*;

#[cfg(feature = "embedded-hal-1")]
pub use crate::ehal::digital::{Error as _, InputPin as _, OutputPin as _, StatefulOutputPin as _};

pub use nb;
