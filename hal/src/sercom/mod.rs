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

use crate::pac::{self, Peripherals};

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

#[cfg(all(feature = "dma", feature = "async"))]
mod async_dma;

#[cfg(all(feature = "dma", feature = "async"))]
pub use async_dma::*;

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

    /// Get a reference to the sercom from a
    /// [`Peripherals`](crate::pac::Peripherals) block
    fn reg_block(peripherals: &mut Peripherals) -> &crate::pac::sercom0::RegisterBlock;

    /// Interrupt handler for async I2C operarions
    #[cfg(feature = "async")]
    fn on_interrupt_i2c();

    /// Interrupt handler for async UART operarions
    #[cfg(feature = "async")]
    fn on_interrupt_uart();

    /// Interrupt handler for async SPI operarions
    #[cfg(feature = "async")]
    fn on_interrupt_spi();

    /// Get a reference to this [`Sercom`]'s associated RX Waker
    #[cfg(feature = "async")]
    #[inline]
    fn rx_waker() -> &'static embassy_sync::waitqueue::AtomicWaker {
        &waker::RX_WAKERS[Self::NUM]
    }

    /// Get a reference to this [`Sercom`]'s associated TX Waker
    #[cfg(feature = "async")]
    #[inline]
    fn tx_waker() -> &'static embassy_sync::waitqueue::AtomicWaker {
        &waker::TX_WAKERS[Self::NUM]
    }
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

                    #[inline]
                    fn reg_block(peripherals: &mut Peripherals) -> &crate::pac::sercom0::RegisterBlock {
                        &*peripherals.SERCOM~N
                    }

                    // TODO the ISR gets called twice on every MB request???
                    #[cfg(feature = "async")]
                    #[inline]
                    fn on_interrupt_i2c(){
                        use self::i2c::Flags;
                        let mut peripherals = unsafe { crate::pac::Peripherals::steal() };
                        let i2cm = Self::reg_block(&mut peripherals).i2cm();
                        let flags_to_check = Flags::all();
                        let flags_pending = Flags::from_bits_truncate(i2cm.intflag.read().bits());

                        // Disable interrupts, but don't clear the flags. The future will take care of
                        // clearing flags and re-enabling interrupts when woken.
                        if flags_to_check.contains(flags_pending) {
                            i2cm.intenclr.write(|w| unsafe { w.bits(flags_pending.bits()) } );
                            Self::rx_waker().wake();
                        }
                    }

                    #[cfg(feature = "async")]
                    #[inline]
                    fn on_interrupt_uart(){
                        use self::uart::Flags;
                        unsafe {
                            let mut peripherals = crate::pac::Peripherals::steal();

                            #[cfg(any(feature = "samd11", feature = "samd21"))]
                            let uart = Self::reg_block(&mut peripherals).usart();
                            #[cfg(feature = "min-samd51g")]
                            let uart = Self::reg_block(&mut peripherals).usart_int();

                            let flags_pending = Flags::from_bits_unchecked(uart.intflag.read().bits());
                            let enabled_flags = Flags::from_bits_unchecked(uart.intenset.read().bits());
                            uart.intenclr.write(|w| w.bits(flags_pending.bits()));

                            // Disable interrupts, but don't clear the flags. The future will take care of
                            // clearing flags and re-enabling interrupts when woken.
                            if (Flags::RX & enabled_flags).intersects(flags_pending) {
                                Self::rx_waker().wake();
                            }

                            if (Flags::TX & enabled_flags).intersects(flags_pending) {
                                Self::tx_waker().wake();
                            }
                        }
                    }

                    #[cfg(feature = "async")]
                    #[inline]
                    fn on_interrupt_spi(){
                        use self::spi::{Flags};
                        unsafe {
                            let mut peripherals = crate::pac::Peripherals::steal();

                            let spi = Self::reg_block(&mut peripherals).spi();

                            let flags_pending = Flags::from_bits_truncate(spi.intflag.read().bits());
                            let enabled_flags = Flags::from_bits_truncate(spi.intenset.read().bits());

                            // Disable interrupts, but don't clear the flags. The future will take care of
                            // clearing flags and re-enabling interrupts when woken.
                            if (Flags::RX & enabled_flags).contains(flags_pending) {
                                spi.intenclr.write(|w| w.bits(flags_pending.bits()));
                                Self::rx_waker().wake();
                            }

                            if (Flags::TX & enabled_flags).contains(flags_pending) {
                                spi.intenclr.write(|w| w.bits(flags_pending.bits()));
                                Self::tx_waker().wake();
                            }
                        }
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

#[allow(dead_code)]
#[cfg(feature = "samd11")]
const NUM_SERCOM: usize = 2;

#[allow(dead_code)]
#[cfg(feature = "samd21e")]
const NUM_SERCOM: usize = 4;

#[allow(dead_code)]
#[cfg(any(feature = "min-samd21g", feature = "samd51g", feature = "samd51j"))]
const NUM_SERCOM: usize = 6;

#[allow(dead_code)]
#[cfg(feature = "min-samd51n")]
const NUM_SERCOM: usize = 8;

#[cfg(feature = "async")]
pub(super) mod waker {
    use embassy_sync::waitqueue::AtomicWaker;
    #[allow(clippy::declare_interior_mutable_const)]
    const NEW_WAKER: AtomicWaker = AtomicWaker::new();
    /// Waker for a RX event. By convention, if a SERCOM has only one type of
    /// event (ie, I2C), this the waker to be used.
    pub(super) static RX_WAKERS: [AtomicWaker; super::NUM_SERCOM] = [NEW_WAKER; super::NUM_SERCOM];
    /// Waker for a TX event.
    pub(super) static TX_WAKERS: [AtomicWaker; super::NUM_SERCOM] = [NEW_WAKER; super::NUM_SERCOM];
}
