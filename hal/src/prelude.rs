//! Import the prelude to gain convenient access to helper traits
pub use crate::gpio::GpioExt as _atsamd21_hal_gpio_GpioExt;
pub use crate::time::U32Ext as _atsamd21_hal_time_U32Ext;

// embedded-hal doesn’t yet have v2 in its prelude, so we need to
// export it ourselves
pub use hal::digital::v2::OutputPin as _atsamd_hal_embedded_hal_digital_v2_OutputPin;
#[cfg(feature = "unproven")]
pub use hal::digital::v2::InputPin as _atsamd_hal_embedded_hal_digital_v2_InputPin;
#[cfg(feature = "unproven")]
pub use hal::digital::v2::ToggleableOutputPin as _atsamd_hal_embedded_hal_digital_v2_ToggleableOutputPin;

pub use hal::prelude::*;
