//! Import the prelude to gain convenient access to helper traits
pub use crate::eic::EicPin;
pub use crate::timer_traits::InterruptDrivenTimer;
pub use fugit::ExtU32 as _;
pub use fugit::RateExtU32 as _;

// embedded-hal doesn’t yet have v2 in its prelude, so we need to
// export it ourselves
pub use crate::ehal_02::digital::v2::InputPin as _atsamd_hal_embedded_hal_digital_v2_InputPin;
pub use crate::ehal_02::digital::v2::OutputPin as _atsamd_hal_embedded_hal_digital_v2_OutputPin;
pub use crate::ehal_02::digital::v2::ToggleableOutputPin as _atsamd_hal_embedded_hal_digital_v2_ToggleableOutputPin;

pub use crate::ehal_02::prelude::*;

#[cfg(feature = "rtic")]
pub use crate::rtc::rtic::prelude::*;
