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
use crate::pac::gclk::pchctrl::GEN_A;

use crate::sercom::v2::*;
use crate::time::Hertz;
use crate::typelevel::{Decrement, Increment, Sealed};

use super::dpll::{DpllId0, DpllId1};
use super::gclk::*;
use super::Driver;

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
pub struct PclkToken<P: PclkId> {
    pclk: PhantomData<P>,
}

impl<P: PclkId> PclkToken<P> {
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
        &self.gclk().pchctrl[P::DYN as usize]
    }

    /// Set a clock as the [`Pclk`] source
    #[inline]
    fn set_source(&mut self, source: DynPclkSourceId) {
        self.pchctrl().modify(|_, w| w.gen().variant(source.into()));
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

/// Type-level `enum` for the 48 peripheral channel clock variants
pub trait PclkId: Sealed {
    /// Corresponding variant of [`DynPclkId`]
    const DYN: DynPclkId;
}

/// Create a type-level variant of [`PclkId`]
macro_rules! pclk_id {
    // If the type doesn't yet exist, define it
    ( false, $Type:ident, $Id:ident ) => {
        /// Type-level variant of [`PclkId`] for the corresponding peripheral
        /// channel clock
        pub enum $Type {}
        impl Sealed for $Type {}
        pclk_id!(true, $Type, $Id);
    };
    // If the type does exist, implement `PclkId`
    ( true, $Type:ident, $Id:ident ) => {
        impl PclkId for $Type {
            const DYN: DynPclkId = DynPclkId::$Id;
        }
    };
}

//==============================================================================
// PclkSourceId
//==============================================================================

/// Value-level version of [`PclkSourceId`]
///
/// Peripheral channel clocks must be sourced from a GCLK, so `DynPclkSourceId`
/// is just a type alias for [`DynGclkId`]
pub type DynPclkSourceId = DynGclkId;

/// Convert from [`DynPclkSourceId`] to the equivalent [PAC](crate::pac) type
impl From<DynPclkSourceId> for GEN_A {
    fn from(source: DynPclkSourceId) -> Self {
        seq!(N in 0..=11 {
            match source {
                #(
                    DynPclkSourceId::Gclk #N => GEN_A::GCLK #N,
                )*
            }
        })
    }
}

/// Type-level `enum` for PCLK sources
///
/// [`Pclk`]s can only be driven by [`Gclk`]s, so the only valid variants are
/// [`GclkId`]s. See the documentation on [type-level enums] for more details
/// on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub trait PclkSourceId: GclkId {}

impl<G: GclkId> PclkSourceId for G {}

//==============================================================================
// Pclk - Peripheral Channel Clock
//==============================================================================

/// Struct representing a [`Pclk`] abstraction
///
/// It is generic over:
/// - a peripheral it is bound to via concept of [`PclkType`]
/// - a clock source ([`PclkSourceMarker`]; variants are provided through
///   [`marker::Gclk0`], [`marker::Gclk1`], `marker::GclkX` types)
pub struct Pclk<P, T>
where
    P: PclkId,
    T: PclkSourceId,
{
    token: PclkToken<P>,
    src: PhantomData<T>,
    freq: Hertz,
}

impl<P, T> Pclk<P, T>
where
    P: PclkId,
    T: PclkSourceId,
{
    /// Enable a peripheral channel clock
    ///
    /// Increase the clock source user counter
    #[inline]
    pub fn enable<S>(mut token: PclkToken<P>, gclk: S) -> (Self, S::Inc)
    where
        S: Driver<Source = T> + Increment,
    {
        let freq = gclk.freq();
        token.set_source(T::DYN);
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
        S: Driver<Source = T> + Decrement,
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
    P: PclkId,
    T: PclkSourceId,
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
            pub enum DynPclkId {
                $(
                    $( #[$cfg] )?
                    /// PclkId
                    [<$Id:camel>],
                )+
            }

            $(
                $( #[$cfg] )?
                pclk_id!( $exists, [<$Type:camel>], [<$Id:camel>] );
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
                #[inline]
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
    (true, DpllId0, dpll0),
    (true, DpllId1, dpll1),
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
