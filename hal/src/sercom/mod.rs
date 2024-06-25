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
use pac::Peripherals;

#[hal_cfg("sercom0-d5x")]
use pac::MCLK as APB_CLK_CTRL;
#[hal_cfg(any("sercom0-d11", "sercom0-d21"))]
use pac::PM as APB_CLK_CTRL;

#[cfg(feature = "dma")]
use crate::dmac::TriggerSource;

use crate::typelevel::Sealed;

pub mod pad;
pub use pad::*;

pub mod i2c;
pub mod spi;

#[deprecated(
    since = "0.17.0",
    note = "spi_future is deprecated and will be removed in a later version of atsamd_hal. Consider using the `async` APIs available in the `spi` module as a replacement."
)]
pub mod spi_future;
pub mod uart;

#[cfg(feature = "dma")]
pub mod dma;

#[cfg(all(feature = "dma", feature = "async"))]
mod async_dma;

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

    #[cfg(feature = "async")]
    type Interrupt: crate::async_hal::interrupts::InterruptSource;

    /// Enable the corresponding APB clock
    fn enable_apb_clock(&mut self, ctrl: &APB_CLK_CTRL);

    /// Get a reference to the sercom from a
    /// [`Peripherals`] block
    fn reg_block(peripherals: &mut Peripherals) -> &crate::pac::sercom0::RegisterBlock;

    /// Get a reference to this [`Sercom`]'s associated RX Waker
    #[cfg(feature = "async")]
    #[inline]
    fn rx_waker() -> &'static embassy_sync::waitqueue::AtomicWaker {
        &crate::sercom::async_api::RX_WAKERS[Self::NUM]
    }

    /// Get a reference to this [`Sercom`]'s associated TX Waker
    #[cfg(feature = "async")]
    #[inline]
    fn tx_waker() -> &'static embassy_sync::waitqueue::AtomicWaker {
        &crate::sercom::async_api::TX_WAKERS[Self::NUM]
    }
}

macro_rules! sercom {
    ( $apbmask:ident, $N:expr, $alias:ident, $pac_type:ident, $pac_rx:ident, $pac_tx:ident, $pac_modify:ident) => {
        use pac::$pac_type;
        /// Type alias for the corresponding SERCOM instance
        pub type $alias = $pac_type;
        impl Sealed for $alias {}
        impl Sercom for $alias {
            const NUM: usize = $N;
            #[cfg(feature = "dma")]
            const DMA_RX_TRIGGER: TriggerSource = TriggerSource::$pac_rx;
            #[cfg(feature = "dma")]
            const DMA_TX_TRIGGER: TriggerSource = TriggerSource::$pac_tx;

            #[cfg(feature = "async")]
            #[hal_cfg(any("sercom0-d11", "sercom0-d21"))]
            type Interrupt = crate::async_hal::interrupts::$pac_type;

            #[cfg(feature = "async")]
            #[hal_cfg("sercom0-d5x")]
            type Interrupt = crate::async_hal::interrupts::$pac_type;

            #[inline]
            fn enable_apb_clock(&mut self, ctrl: &APB_CLK_CTRL) {
                ctrl.$apbmask.modify(|_, w| w.$pac_modify().set_bit());
            }

            #[inline]
            fn reg_block(peripherals: &mut Peripherals) -> &crate::pac::sercom0::RegisterBlock {
                &*peripherals.$pac_type
            }
        }
    };
}

// d11 and d21 families
#[hal_cfg(any("sercom0-d11", "sercom0-d21"))]
sercom!(apbcmask, 0, Sercom0, SERCOM0, SERCOM0_RX, SERCOM0_TX, sercom0_);

#[hal_cfg(any("sercom1-d11", "sercom1-d21"))]
sercom!(apbcmask, 1, Sercom1, SERCOM1, SERCOM1_RX, SERCOM1_TX, sercom1_);

#[hal_cfg(any("sercom2-d11", "sercom2-d21"))]
sercom!(apbcmask, 2, Sercom2, SERCOM2, SERCOM2_RX, SERCOM2_TX, sercom2_);

#[hal_cfg("sercom3-d21")]
sercom!(apbcmask, 3, Sercom3, SERCOM3, SERCOM3_RX, SERCOM3_TX, sercom3_);

#[hal_cfg("sercom4-d21")]
sercom!(apbcmask, 4, Sercom4, SERCOM4, SERCOM4_RX, SERCOM4_TX, sercom4_);

#[hal_cfg("sercom5-d21")]
sercom!(apbcmask, 5, Sercom5, SERCOM5, SERCOM5_RX, SERCOM5_TX, sercom5_);

// d5x family
#[hal_cfg("sercom0-d5x")]
sercom!(apbamask, 0, Sercom0, SERCOM0, SERCOM0_RX, SERCOM0_TX, sercom0_);

#[hal_cfg("sercom1-d5x")]
sercom!(apbamask, 1, Sercom1, SERCOM1, SERCOM1_RX, SERCOM1_TX, sercom1_);

#[hal_cfg("sercom2-d5x")]
sercom!(apbbmask, 2, Sercom2, SERCOM2, SERCOM2_RX, SERCOM2_TX, sercom2_);

#[hal_cfg("sercom3-d5x")]
sercom!(apbbmask, 3, Sercom3, SERCOM3, SERCOM3_RX, SERCOM3_TX, sercom3_);

#[hal_cfg("sercom4-d5x")]
sercom!(apbdmask, 4, Sercom4, SERCOM4, SERCOM4_RX, SERCOM4_TX, sercom4_);

#[hal_cfg("sercom5-d5x")]
sercom!(apbdmask, 5, Sercom5, SERCOM5, SERCOM5_RX, SERCOM5_TX, sercom5_);

#[hal_cfg("sercom6-d5x")]
sercom!(apbdmask, 6, Sercom6, SERCOM6, SERCOM6_RX, SERCOM6_TX, sercom6_);

#[hal_cfg("sercom7-d5x")]
sercom!(apbdmask, 7, Sercom7, SERCOM7, SERCOM7_RX, SERCOM7_TX, sercom7_);

// Reserve space for the max number of SERCOM peripherals based on chip type,
// even though some wakers may not be used on some chips if they actually don't
// exist on variant's hardware
#[hal_cfg("sercom0-d11")]
#[cfg(feature = "async")]
const NUM_SERCOM: usize = 3;

#[hal_cfg("sercom0-d21")]
#[cfg(feature = "async")]
const NUM_SERCOM: usize = 6;

#[hal_cfg("sercom0-d5x")]
#[cfg(feature = "async")]
const NUM_SERCOM: usize = 8;

#[cfg(feature = "async")]
pub(super) mod async_api {
    use embassy_sync::waitqueue::AtomicWaker;

    #[allow(clippy::declare_interior_mutable_const)]
    const NEW_WAKER: AtomicWaker = AtomicWaker::new();
    /// Waker for a RX event. By convention, if a SERCOM has only one type of
    /// event (ie, I2C), this the waker to be used.
    pub(super) static RX_WAKERS: [AtomicWaker; super::NUM_SERCOM] = [NEW_WAKER; super::NUM_SERCOM];
    /// Waker for a TX event.
    pub(super) static TX_WAKERS: [AtomicWaker; super::NUM_SERCOM] = [NEW_WAKER; super::NUM_SERCOM];
}
