//! Import the prelude to gain convenient access to helper traits
pub use crate::eic::pin::EicPin;
pub use crate::time::U32Ext as _atsamd21_hal_time_U32Ext;
pub use crate::timer_traits::InterruptDrivenTimer as _atsamd_hal_timer_traits_InterruptDrivenTimer;

// embedded-hal doesn’t yet have v2 in its prelude, so we need to
// export it ourselves
#[cfg(feature = "unproven")]
pub use crate::ehal::digital::v2::InputPin as _atsamd_hal_embedded_hal_digital_v2_InputPin;
pub use crate::ehal::digital::v2::OutputPin as _atsamd_hal_embedded_hal_digital_v2_OutputPin;
#[cfg(feature = "unproven")]
pub use crate::ehal::digital::v2::ToggleableOutputPin as _atsamd_hal_embedded_hal_digital_v2_ToggleableOutputPin;

pub use crate::ehal::prelude::*;

pub use nb;
