//! # Configure the SERCOM peripherals
//!
//! The SERCOM module is used to configure the SERCOM peripherals as USART, SPI
//! or I2C interfaces.
//! # Undocumented features
//!
//! The ATSAMx5x chips contain certain features that aren't documented in the
//! datasheet. These features are implemented in the HAL based on
//! experimentation with certain boards which have verifiably demonstrated that
//! those features work as intended.
//!
//! * [`UndocIoSet1`](pad::UndocIoSet1): Implement an undocumented `IoSet` for
//!   PA16, PA17, PB22 & PB23 configured for [`Sercom1`]. The pygamer &
//!   feather_m4 use this combination, but it is not listed as valid in the
//!   datasheet.
//!
//! * [`UndocIoSet2`](pad::UndocIoSet2): Implement an undocumented `IoSet` for
//!   PA00, PA01, PB22 & PB23 configured for [`Sercom1`]. The itsybitsy_m4 uses
//!   this combination, but it is not listed as valid in the datasheet.
//!
//! * [`PB02`] is I2C-capable according to metro_m4. As such, [`PB02`]
//!   implements [`IsI2cPad`].
//!
//! * [`PB03`] is I2C-capable according to metro_m4. As such, [`PB03`]
//!   implements [`IsI2cPad`](pad::IsI2cPad).
//!
//! [`PB02`]: crate::gpio::pin::PB02
//! [`PB03`]: crate::gpio::pin::PB03
//! [`IsI2cPad`]: pad::IsI2cPad

use atsamd_hal_macros::hal_cfg;

use core::ops::Deref;

use crate::pac;
use pac::sercom0;

#[hal_cfg("sercom0-d5x")]
use pac::Mclk as ApbClkCtrl;
#[hal_cfg(any("sercom0-d11", "sercom0-d21"))]
use pac::Pm as ApbClkCtrl;

#[cfg(feature = "dma")]
use crate::dmac::TriggerSource;

use crate::typelevel::Sealed;

pub mod pad;
pub use pad::*;

pub mod i2c;
pub mod spi;
pub mod spi_future;
pub mod uart;

#[cfg(feature = "dma")]
pub mod dma;

//==============================================================================
//  Sercom
//==============================================================================

/// Type-level `enum` representing a Serial Communication Interface (SERCOM)
pub trait Sercom: Sealed + Deref<Target = sercom0::RegisterBlock> {
    /// SERCOM number
    const NUM: usize;
    /// RX Trigger source for DMA transactions
    #[cfg(feature = "dma")]
    const DMA_RX_TRIGGER: TriggerSource;
    /// TX trigger source for DMA transactions
    #[cfg(feature = "dma")]
    const DMA_TX_TRIGGER: TriggerSource;
    /// Enable the corresponding APB clock
    fn enable_apb_clock(&mut self, ctrl: &ApbClkCtrl);
}

macro_rules! sercom {
    ( $apbmask:ident, $N:expr) => {
        paste::paste! {
            // use pac::$pac_type;
            /// Type alias for the corresponding SERCOM instance
            pub type [< Sercom $N >] = $crate::pac::[< Sercom $N >];
            impl Sealed for [< Sercom $N >] {}
            impl Sercom for [< Sercom $N >] {
                const NUM: usize = $N;
                #[cfg(feature = "dma")]
                const DMA_RX_TRIGGER: TriggerSource = TriggerSource::[< Sercom $N Rx >];
                #[cfg(feature = "dma")]
                const DMA_TX_TRIGGER: TriggerSource = TriggerSource::[< Sercom $N Tx >];
                #[inline]
                fn enable_apb_clock(&mut self, ctrl: &ApbClkCtrl) {
                    ctrl.$apbmask().modify(|_, w| w.[< sercom $N _>]().set_bit());
                }
            }
        }
    };
}

// d11 and d21 families
#[hal_cfg(any("sercom0-d11", "sercom0-d21"))]
sercom!(apbcmask, 0);

#[hal_cfg(any("sercom1-d11", "sercom1-d21"))]
sercom!(apbcmask, 1);

#[hal_cfg(any("sercom2-d11", "sercom2-d21"))]
sercom!(apbcmask, 2);

#[hal_cfg("sercom3-d21")]
sercom!(apbcmask, 3);

#[hal_cfg("sercom4-d21")]
sercom!(apbcmask, 4);

#[hal_cfg("sercom5-d21")]
sercom!(apbcmask, 5);

// d5x family
#[hal_cfg("sercom0-d5x")]
sercom!(apbamask, 0);

#[hal_cfg("sercom1-d5x")]
sercom!(apbamask, 1);

#[hal_cfg("sercom2-d5x")]
sercom!(apbbmask, 2);

#[hal_cfg("sercom3-d5x")]
sercom!(apbbmask, 3);

#[hal_cfg("sercom4-d5x")]
sercom!(apbdmask, 4);

#[hal_cfg("sercom5-d5x")]
sercom!(apbdmask, 5);

#[hal_cfg("sercom6-d5x")]
sercom!(apbdmask, 6);

#[hal_cfg("sercom7-d5x")]
sercom!(apbdmask, 7);
