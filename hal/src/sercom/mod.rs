//! # Configure the SERCOM peripherals
//!
//! The SERCOM module is used to configure the SERCOM peripherals as USART, SPI
//! or I2C interfaces.
#![cfg_attr(
    feature = "thumbv7",
    doc = "
# Undocumented features
 
The ATSAMx5x chips contain certain features that aren't documented in the datasheet. 
These features are implemented in the HAL based on experimentation with certain boards
which have verifiably demonstrated that those features work as intended.

* [`UndocIoSet1`](pad::UndocIoSet1): Implement an undocumented `IoSet` for PA16, PA17,
PB22 & PB23 configured for [`Sercom1`]. The pygamer & feather_m4 use this combination,
but it is not listed as valid in the datasheet.

* [`UndocIoSet2`](pad::UndocIoSet2): Implement an undocumented `IoSet` for PA00, PA01,
PB22 & PB23 configured for [`Sercom1`]. The itsybitsy_m4 uses this combination, but it is
not listed as valid in the datasheet.

* [`PB02`] is I2C-capable according to metro_m4. As such, [`PB02`]
implements [`IsI2cPad`].

* [`PB03`] is I2C-capable according to metro_m4. As such, [`PB03`]
implements [`IsI2cPad`](pad::IsI2cPad).

[`PB02`]: crate::gpio::pin::PB02
[`PB03`]: crate::gpio::pin::PB03
[`IsI2cPad`]: pad::IsI2cPad
"
)]

use core::ops::Deref;

use paste::paste;
use seq_macro::seq;

use crate::pac;
use pac::sercom0;

#[cfg(feature = "thumbv7")]
use pac::MCLK as APB_CLK_CTRL;
#[cfg(feature = "thumbv6")]
use pac::PM as APB_CLK_CTRL;

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
    ( $apbmask:ident: ($start:literal, $end:literal) ) => {
        seq!(N in $start..=$end {
            paste! {
                #[cfg(feature = "has-" sercom~N)]
                use pac::SERCOM~N;
                /// Type alias for the corresponding SERCOM instance
                #[cfg(feature = "has-" sercom~N)]
                pub type Sercom~N = SERCOM~N;
                #[cfg(feature = "has-" sercom~N)]
                impl Sealed for Sercom~N {}
                #[cfg(feature = "has-" sercom~N)]
                impl Sercom for Sercom~N {
                    const NUM: usize = N;
                    #[cfg(feature = "dma")]
                    const DMA_RX_TRIGGER: TriggerSource = TriggerSource::[<SERCOM~N _RX>];
                    #[cfg(feature = "dma")]
                    const DMA_TX_TRIGGER: TriggerSource = TriggerSource::[<SERCOM~N _TX>];
                    #[inline]
                    fn enable_apb_clock(&mut self, ctrl: &APB_CLK_CTRL) {
                        ctrl.$apbmask.modify(|_, w| w.[<sercom~N _>]().set_bit());
                    }
                }
            }
        });
    };
}

#[cfg(feature = "thumbv6")]
sercom!(apbcmask: (0, 5));

#[cfg(feature = "thumbv7")]
sercom!(apbamask: (0, 1));
#[cfg(feature = "thumbv7")]
sercom!(apbbmask: (2, 3));
#[cfg(feature = "thumbv7")]
sercom!(apbdmask: (4, 7));
