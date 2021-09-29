#![deny(missing_docs)]
#![deny(warnings)]
//! # Pclk - Peripheral Channel (Clock)
//!
//! Peripheral clocks serve as a last element in a chain within a clocking
//! system and are directly associated with a specific peripheral in a 1:1
//! manner. Some of them are reserved for clocking system internal purposes,
//! like reference clock for Dfll or Dpll.
//!
//! Every [`Pclk`] can be powered by any instantiated and enabled
//! [`Gclk`][`super::gclk::Gclk`].
//!
//! Abstractions representing peripherals that depend on a configured
//! corresponding [`Pclk`] instance should consume it and release it upon
//! destruction. Thus, it is possible to freeze adequate part of the clocking
//! tree that running peripheral depends on.

use core::marker::PhantomData;

use paste::paste;
use seq_macro::seq;

use crate::pac;

pub use crate::pac::gclk::pchctrl::GEN_A as PclkSourceEnum;

use crate::clock::types::{Counter, Decrement, Enabled, Increment};
use crate::clock::v2::{Source, SourceMarker};
use crate::sercom::v2::*;
use crate::time::Hertz;
use crate::typelevel::Sealed;

use super::dpll::{Pll0, Pll1};
use super::gclk::*;

//==============================================================================
// PclkToken
//==============================================================================

/// Token type required to construct a [`Pclk`] type instance.
///
/// From a [`atsamd_hal`][`crate`] external user perspective, it does not
/// contain any methods and serves only a token purpose.
///
/// Within a [`atsamd_hal`][`crate`], [`PclkToken`] struct is a low-level access
/// abstraction for HW register calls.
pub struct PclkToken<P: PclkType> {
    pclk: PhantomData<P>,
}

impl<P: PclkType> PclkToken<P> {
    /// Create a new instance of [`PclkToken`]
    ///
    /// # Safety
    ///
    /// Users must never create two simultaneous instances of this `struct` with
    /// the same [`PclkType`]
    #[inline]
    unsafe fn new() -> Self {
        PclkToken { pclk: PhantomData }
    }

    #[inline]
    fn gclk(&self) -> &pac::gclk::RegisterBlock {
        unsafe { &*pac::GCLK::ptr() }
    }

    /// Provide access to pchctrl, primary control interface for Pclk
    #[inline]
    fn pchctrl(&self) -> &pac::gclk::PCHCTRL {
        &self.gclk().pchctrl[P::ID as usize]
    }

    /// Set a clock as the [`Pclk`] source
    #[inline]
    fn set_source(&mut self, variant: PclkSourceEnum) {
        self.pchctrl().modify(|_, w| w.gen().variant(variant));
    }

    /// Enable the [`Pclk`]
    #[inline]
    fn enable(&mut self) {
        self.pchctrl().modify(|_, w| w.chen().set_bit());
    }

    /// Disable the [`Pclk`]
    #[inline]
    fn disable(&mut self) {
        self.pchctrl().modify(|_, w| w.chen().clear_bit());
    }
}

//==============================================================================
// PclkType
//==============================================================================

/// Type-level `enum` for the 48 peripheral clock variants
pub trait PclkType: Sealed {
    /// Numeric Pclk ID
    const ID: PclkId;
}

// If a suitable type already exists in the HAL, reuse it to implement
// `PclkType` trait ([`Pll0`] or `Sercom` type are a good example). Otherwise,
// define new enum type.
macro_rules! pclk_type {
    // A type already exists; reuse it
    ( true, $Type:ident, $Id:ident ) => {
        impl PclkType for $Type {
            const ID: PclkId = PclkId::$Id;
        }
    };
    // A type does not exist yet; create one
    ( false, $Type:ident, $Id:ident ) => {
        /// [`PclkType`] variant
        pub enum $Type {}
        impl Sealed for $Type {}
        impl PclkType for $Type {
            const ID: PclkId = PclkId::$Id;
        }
    };
}

//==============================================================================
// PclkSource
//==============================================================================

/// Source marker trait for [`Pclk`] sources
///
/// All [`Gclk`]s can act as a source for [`Pclk`]s
pub trait PclkSourceMarker: GenNum + SourceMarker {
    /// Associated constant provides a variant of a low level enum type from PAC
    /// that is used during a HW register write
    const PCLK_SRC: PclkSourceEnum;
}

seq!(N in 0..=11 {
    impl PclkSourceMarker for Gen#N {
        const PCLK_SRC: PclkSourceEnum = PclkSourceEnum::GCLK#N;
    }
});

/// This trait represents a [`Pclk`] source provider.
pub trait PclkSource: Source {
    /// Associated type used in order to mark [`Pclk<_, T: PclkSourceMarker>`]
    /// type with a proper `T`, according to what `gclk` was passed into the
    /// [`Pclk::enable`] and to only allow calls into [`Pclk::disable`] with a
    /// matching `gclk`
    type Type: PclkSourceMarker;
}

impl<G, T, N> PclkSource for Enabled<Gclk<G, T>, N>
where
    G: PclkSourceMarker,
    T: GclkSourceMarker,
    N: Counter,
{
    type Type = G;
}

//==============================================================================
// Pclk - Peripheral Channel Clock
//==============================================================================

/// Struct representing a [`Pclk`] abstraction
///
/// It is generic over:
/// - a peripheral it is bound to via concept of [`PclkType`]
/// - a clock source ([`PclkSourceMarker`]; variants are provided through
///   [`Gen0`], [`Gen1`], `GenX` types)
pub struct Pclk<P, T>
where
    P: PclkType,
    T: PclkSourceMarker,
{
    token: PclkToken<P>,
    src: PhantomData<T>,
    freq: Hertz,
}

impl<P, T> Pclk<P, T>
where
    P: PclkType,
    T: PclkSourceMarker,
{
    /// Enable a peripheral channel clock
    ///
    /// Increase the clock source user counter
    #[inline]
    pub fn enable<S>(mut token: PclkToken<P>, gclk: S) -> (Self, S::Inc)
    where
        S: PclkSource<Type = T> + Increment,
    {
        let freq = gclk.freq();
        token.set_source(T::PCLK_SRC);
        token.enable();
        let pclk = Pclk {
            token,
            src: PhantomData,
            freq,
        };
        (pclk, gclk.inc())
    }

    /// Disable the peripheral channel clock
    ///
    /// Decrease the clock source user counter
    #[inline]
    pub fn disable<S>(mut self, gclk: S) -> (PclkToken<P>, S::Dec)
    where
        S: PclkSource<Type = T> + Decrement,
    {
        self.token.disable();
        (self.token, gclk.dec())
    }

    /// Return the [`Pclk`] frequency
    #[inline]
    pub fn freq(&self) -> Hertz {
        self.freq
    }
}

impl<P, T> Sealed for Pclk<P, T>
where
    P: PclkType,
    T: PclkSourceMarker,
{
}

//==============================================================================
// Pclks generation
//==============================================================================

macro_rules! pclks {
    (
        $(
            $( #[$cfg:meta] )?
            ($exists:literal, $Type:ident, $Id:ident)
        ),+
    ) =>
    {
        paste! {
            /// Internal `enum` used to index the correct peripheral channel
            /// register
            pub enum PclkId {
                $(
                    $( #[$cfg] )?
                    /// PclkId
                    [<$Id:camel>],
                )+
            }

            $(
                $( #[$cfg] )?
                pclk_type!( $exists, [<$Type:camel>], [<$Id:camel>] );
            )+

            /// Struct containing all possible peripheral clock tokens
            pub struct Tokens {
                $(
                    $( #[$cfg] )?
                    /// Exposed [`PclkToken`]
                    pub [<$Id:lower>]: PclkToken<[<$Type:camel>]>,
                )+
            }

            impl Tokens {
                pub(super) fn new() -> Self {
                    Tokens {
                        $(
                            // Create new [`PclkToken`]
                            $( #[$cfg] )?
                            [<$Id:lower>]: unsafe { PclkToken::new() },
                        )+
                    }
                }
            }
        }
    };
}

// Try to use existing types as tokens, if possible. Otherwise, create new ones.
pclks!(
    (false, dfll48, dfll48),
    (true, Pll0, dpll0),
    (true, Pll1, dpll1),
    (false, slow_32k, slow_32k),
    (false, eic, eic),
    (false, freqm_msr, freqm_msr),
    (false, freqm_ref, freqm_ref),
    (true, sercom0, sercom0),
    (true, sercom1, sercom1),
    (false, tc0_tc1, tc0_tc1),
    (false, usb, usb),
    (false, evsys0, evsys0),
    (false, evsys1, evsys1),
    (false, evsys2, evsys2),
    (false, evsys3, evsys3),
    (false, evsys4, evsys4),
    (false, evsys5, evsys5),
    (false, evsys6, evsys6),
    (false, evsys7, evsys7),
    (false, evsys8, evsys8),
    (false, evsys9, evsys9),
    (false, evsys10, evsys10),
    (false, evsys11, evsys11),
    (true, sercom2, sercom2),
    (true, sercom3, sercom3),
    (false, tcc0_tcc1, tcc0_tcc1),
    (false, tc2_tc3, tc2_tc3),
    (false, can0, can0),
    (false, can1, can1),
    (false, tcc2_tcc3, tcc2_tcc3),
    (false, tc4_tc5, tc4_tc5),
    (false, pdec, pdec),
    (false, ac, ac),
    (false, ccl, ccl),
    (true, sercom4, sercom4),
    (true, sercom5, sercom5),
    #[cfg(feature = "min-samd51n")]
    (true, sercom6, sercom6),
    #[cfg(feature = "min-samd51n")]
    (true, sercom7, sercom7),
    (false, tcc4, tcc4),
    (false, tc6_tc7, tc6_tc7),
    (false, adc0, adc0),
    (false, adc1, adc1),
    (false, dac, dac),
    (false, i2s0, i2s0),
    (false, i2s1, i2s1),
    (false, sdhc0, sdhc0),
    (false, sdhc1, sdhc1),
    (false, cm4_trace, cm4_trace)
);
