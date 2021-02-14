//! # Version 2 of the SERCOM module
//!
//! This module provides a new API for the SERCOM peripherals. So far, only the
//! [`pads`] and [`spi`] modules have been updated, but it is expected that the
//! `uart`, and `i2c` modules will eventually receive updates as well.

use core::ops::Deref;

use paste::paste;
use seq_macro::seq;

use crate::target_device as pac;

#[cfg(feature = "min-samd51g")]
use pac::MCLK as APB_CLK_CTRL;
#[cfg(any(feature = "samd11", feature = "samd21"))]
use pac::PM as APB_CLK_CTRL;

use pac::{sercom0, SERCOM0, SERCOM1};
#[cfg(any(feature = "samd21", feature = "min-samd51g"))]
use pac::{SERCOM2, SERCOM3};
#[cfg(any(feature = "min-samd21g", feature = "min-samd51g"))]
use pac::{SERCOM4, SERCOM5};
#[cfg(feature = "min-samd51n")]
use pac::{SERCOM6, SERCOM7};

#[cfg(any(feature = "samd11", feature = "samd21"))]
pub use crate::common::thumbv6m::sercom::v2::*;

#[cfg(feature = "min-samd51g")]
pub use crate::common::thumbv7em::sercom::v2::*;

use crate::typelevel::Sealed;

pub mod pads;
pub mod spi_future;

//==============================================================================
//  Sercom
//==============================================================================

/// Type-level `enum` representing a Serial Communication Interface (SERCOM)
pub trait Sercom: Sealed + Deref<Target = sercom0::RegisterBlock> {
    /// SERCOM number
    const NUM: usize;
    /// Enable the corresponding APB clock
    fn enable_apb_clock(&mut self, ctrl: &APB_CLK_CTRL);
}

macro_rules! sercom {
    ( $apbmask:ident: ($start:literal, $end:literal) ) => {
        seq!(N in $start..=$end {
            paste! {
                /// Type alias for the corresponding SERCOM instance
                pub type Sercom#N = SERCOM#N;
                impl Sealed for Sercom#N {}
                impl Sercom for Sercom#N {
                    const NUM: usize = N;
                    #[inline]
                    fn enable_apb_clock(&mut self, ctrl: &APB_CLK_CTRL) {
                        ctrl.$apbmask.modify(|_, w| w.[<sercom#N _>]().set_bit());
                    }
                }
            }
        });
    };
}

#[cfg(any(feature = "samd11", feature = "samd21"))]
sercom!(apbcmask: (0, 1));
#[cfg(feature = "samd21")]
sercom!(apbcmask: (2, 3));
#[cfg(feature = "min-samd21g")]
sercom!(apbcmask: (4, 5));

#[cfg(feature = "min-samd51g")]
sercom!(apbamask: (0, 1));
#[cfg(feature = "min-samd51g")]
sercom!(apbbmask: (2, 3));
#[cfg(feature = "min-samd51g")]
sercom!(apbdmask: (4, 5));
#[cfg(feature = "min-samd51n")]
sercom!(apbdmask: (6, 7));
