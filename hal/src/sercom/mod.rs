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
use pac::Mclk as APB_CLK_CTRL;
#[hal_cfg(any("sercom0-d11", "sercom0-d21"))]
use pac::Pm as APB_CLK_CTRL;

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
    fn enable_apb_clock(&mut self, ctrl: &APB_CLK_CTRL);
}

macro_rules! sercom {
    ( $apbmask:ident, $N:expr, $pac_type:ident, $pac_rx:ident, $pac_tx:ident, $pac_modify:ident) => {
        // use pac::$pac_type;
        /// Type alias for the corresponding SERCOM instance
        pub type $pac_type = $crate::pac::$pac_type;
        impl Sealed for $pac_type {}
        impl Sercom for $pac_type {
            const NUM: usize = $N;
            #[cfg(feature = "dma")]
            const DMA_RX_TRIGGER: TriggerSource = TriggerSource::$pac_rx;
            #[cfg(feature = "dma")]
            const DMA_TX_TRIGGER: TriggerSource = TriggerSource::$pac_tx;
            #[inline]
            fn enable_apb_clock(&mut self, ctrl: &APB_CLK_CTRL) {
                ctrl.$apbmask().modify(|_, w| w.$pac_modify().set_bit());
            }
        }
    };
}

// d11 and d21 families
#[hal_cfg(any("sercom0-d11", "sercom0-d21"))]
sercom!(apbcmask, 0, Sercom0, Sercom0Rx, Sercom0Tx, sercom0_);

#[hal_cfg(any("sercom1-d11", "sercom1-d21"))]
sercom!(apbcmask, 1, Sercom1, Sercom1Rx, Sercom1Tx, sercom1_);

#[hal_cfg(any("sercom2-d11", "sercom2-d21"))]
sercom!(apbcmask, 2, Sercom2, Sercom2Rx, Sercom2Tx, sercom2_);

#[hal_cfg("sercom3-d21")]
sercom!(apbcmask, 3, Sercom3, SERCOM3_RX, SERCOM3_TX, sercom3_);

#[hal_cfg("sercom4-d21")]
sercom!(apbcmask, 4, Sercom4, SERCOM4_RX, SERCOM4_TX, sercom4_);

#[hal_cfg("sercom5-d21")]
sercom!(apbcmask, 5, Sercom5, SERCOM5_RX, SERCOM5_TX, sercom5_);

// d5x family
#[hal_cfg("sercom0-d5x")]
sercom!(apbamask, 0, Sercom0, SERCOM0_RX, SERCOM0_TX, sercom0_);

#[hal_cfg("sercom1-d5x")]
sercom!(apbamask, 1, Sercom1, SERCOM1_RX, SERCOM1_TX, sercom1_);

#[hal_cfg("sercom2-d5x")]
sercom!(apbbmask, 2, Sercom2, SERCOM2_RX, SERCOM2_TX, sercom2_);

#[hal_cfg("sercom3-d5x")]
sercom!(apbbmask, 3, Sercom3, SERCOM3_RX, SERCOM3_TX, sercom3_);

#[hal_cfg("sercom4-d5x")]
sercom!(apbdmask, 4, Sercom4, SERCOM4_RX, SERCOM4_TX, sercom4_);

#[hal_cfg("sercom5-d5x")]
sercom!(apbdmask, 5, Sercom5, SERCOM5_RX, SERCOM5_TX, sercom5_);

#[hal_cfg("sercom6-d5x")]
sercom!(apbdmask, 6, Sercom6, SERCOM6_RX, SERCOM6_TX, sercom6_);

#[hal_cfg("sercom7-d5x")]
sercom!(apbdmask, 7, Sercom7, SERCOM7_RX, SERCOM7_TX, sercom7_);
