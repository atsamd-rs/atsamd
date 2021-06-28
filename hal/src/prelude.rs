//! Import the prelude to gain convenient access to helper traits
pub use crate::eic::pin::EicPin;
#[allow(deprecated)]
pub use crate::gpio::v1::GpioExt as _atsamd21_hal_gpio_GpioExt;
pub use crate::spi_common::CommonSpi as _atsamd_hal_spi_common_CommonSpi;
pub use crate::time::U32Ext as _atsamd21_hal_time_U32Ext;
pub use crate::timer_traits::InterruptDrivenTimer as _atsamd_hal_timer_traits_InterruptDrivenTimer;

// embedded-hal doesn’t yet have v2 in its prelude, so we need to
// export it ourselves
#[cfg(feature = "unproven")]
pub use hal::digital::v2::InputPin as _atsamd_hal_embedded_hal_digital_v2_InputPin;
pub use hal::digital::v2::OutputPin as _atsamd_hal_embedded_hal_digital_v2_OutputPin;
#[cfg(feature = "unproven")]
pub use hal::digital::v2::ToggleableOutputPin as _atsamd_hal_embedded_hal_digital_v2_ToggleableOutputPin;

pub use hal::prelude::*;

pub use nb;
