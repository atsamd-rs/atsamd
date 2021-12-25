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

use crate::time::Hertz;
use crate::typelevel::{Decrement, Increment, Sealed};

use super::gclk::{DynGclkId, GclkId};
use super::Source;

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
    pub(super) unsafe fn new() -> Self {
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
// DynPclkId
//==============================================================================

/// Internal `enum` used to index the correct peripheral channel register
#[allow(missing_docs)]
pub enum DynPclkId {
    Dfll,
    Dpll0,
    Dpll1,
    Slow,
    Eic,
    FreqmMeasure,
    FreqmReference,
    Sercom0,
    Sercom1,
    Tc0Tc1,
    Usb,
    EvSys0,
    EvSys1,
    EvSys2,
    EvSys3,
    EvSys4,
    EvSys5,
    EvSys6,
    EvSys7,
    EvSys8,
    EvSys9,
    EvSys10,
    EvSys11,
    Sercom2,
    Sercom3,
    Tcc0Tcc1,
    Tc2Tc3,
    Can0,
    Can1,
    Tcc2Tcc3,
    Tc4Tc5,
    Pdec,
    Ac,
    Ccl,
    Sercom4,
    Sercom5,
    #[cfg(feature = "min-samd51n")]
    Sercom6,
    #[cfg(feature = "min-samd51n")]
    Sercom7,
    Tcc4,
    Tc6Tc7,
    Adc0,
    Adc1,
    Dac,
    I2s0,
    I2s1,
    Sdhc0,
    Sdhc1,
    Cm4Trace,
}

//==============================================================================
// PclkId types
//==============================================================================

/// Append the list of all [`PclkId`] types and `snake_case` id names to the
/// arguments of a macro call
///
/// This macro will perform the embedded macro call with a list of tuples
/// appended to the arguments. Each tuple contains a type implementing
/// [`PclkId`] and the `snake_case` name of the corresponding token in the
/// [`pclk::Tokens`](Tokens) struct.
///
/// **Note:** The entries within [`DynPclkId`] do not match the type names.
/// Rather, they match the `snake_case` names converted to `CamelCase`.
///
/// An optional attribute is added just before each tuple. These are mainly used
/// to declare the conditions under which the corresponding peripheral exists.
/// For example, `Sercom6` and `Sercom7` are tagged with
/// `#[cfg(feature = "min-samd51n")]`.
///
/// The example below shows the pattern that should be used to match against the
/// appended tokens.
///
/// ```
/// macro_rules! some_macro {
///     (
///         $first_arg:tt,
///         $second_arg:tt
///         $(
///             $( #[$cfg:meta] )?
///             ($Type:ident, $Id:ident)
///         )+
///     ) =>
///     {
///         // implementation here ...
///     }
/// }
///
/// with_pclk_types_ids!(some_macro!(first, second));
/// ```
macro_rules! with_pclk_types_ids {
    ( $some_macro:ident ! ( $( $args:tt )* ) ) => {
        crate::thumbv7em::clock::v2::pclk::__dispatch!(
            without_existence,
            $some_macro ! ( $( $args )* )
        );
    };
}

/// Append information about the [`PclkId`] types to the arguments of a macro
/// call
///
/// This macro is nearly identical to [`with_pclk_types_ids`]. The only
/// difference is that a third element is added to each tuple. This element is
/// either `already_exists`, if the type is already defined elsewhere in the
/// HAL, or it is `does_not_exist`, if the corresponding peripheral has no HAL
/// implementation. The types marked `does_not_exist` will be created here in
/// the [`pclk::ids`](ids) module.
///
/// It is unlikely this macro will find use outside of this module.
macro_rules! with_pclk_types_ids_existence {
    ( $some_macro:ident ! ( $( $args:tt )* ) ) => {
        crate::thumbv7em::clock::v2::pclk::__dispatch!(
            with_existence,
            $some_macro ! ( $( $args )* )
        );
    };
}

/// Internal implementation detail of the `with_pclk_type_ids*` macros
///
/// Append the [`PclkId`] type information and dispatch to the correct branch of
/// [`__call_macro`].
macro_rules! __dispatch {
    ( $existence:ident, $some_macro:ident ! ( $( $args:tt )* ) ) => {
        crate::thumbv7em::clock::v2::pclk::__call_macro!(
            $existence,
            $some_macro ! ( $( $args )* ),
            (DfllId, dfll, already_exists)
            (Dpll0Id, dpll0, already_exists)
            (Dpll1Id, dpll1, already_exists)
            (Slow, slow, does_not_exist)
            (Eic, eic, does_not_exist)
            (FreqmMeasure, freqm_measure, does_not_exist)
            (FreqmReference, freqm_reference, does_not_exist)
            (Sercom0, sercom0, already_exists)
            (Sercom1, sercom1, already_exists)
            (Tc0Tc1, tc0_tc1, does_not_exist)
            (Usb, usb, does_not_exist)
            (EvSys0, ev_sys0, does_not_exist)
            (EvSys1, ev_sys1, does_not_exist)
            (EvSys2, ev_sys2, does_not_exist)
            (EvSys3, ev_sys3, does_not_exist)
            (EvSys4, ev_sys4, does_not_exist)
            (EvSys5, ev_sys5, does_not_exist)
            (EvSys6, ev_sys6, does_not_exist)
            (EvSys7, ev_sys7, does_not_exist)
            (EvSys8, ev_sys8, does_not_exist)
            (EvSys9, ev_sys9, does_not_exist)
            (EvSys10, ev_sys10, does_not_exist)
            (EvSys11, ev_sys11, does_not_exist)
            (Sercom2, sercom2, already_exists)
            (Sercom3, sercom3, already_exists)
            (Tcc0Tcc1, tcc0_tcc1, does_not_exist)
            (Tc2Tc3, tc2_tc3, does_not_exist)
            (Can0, can0, does_not_exist)
            (Can1, can1, does_not_exist)
            (Tcc2Tcc3, tcc2_tcc3, does_not_exist)
            (Tc4Tc5, tc4_tc5, does_not_exist)
            (PDec, pdec, does_not_exist)
            (AC, ac, does_not_exist)
            (CCL, ccl, does_not_exist)
            (Sercom4, sercom4, already_exists)
            (Sercom5, sercom5, already_exists)
            #[cfg(feature = "min-samd51n")]
            (Sercom6, sercom6, already_exists)
            #[cfg(feature = "min-samd51n")]
            (Sercom7, sercom7, already_exists)
            (Tcc4, tcc4, does_not_exist)
            (Tc6Tc7, tc6_tc7, does_not_exist)
            (Adc0, adc0, does_not_exist)
            (Adc1, adc1, does_not_exist)
            (Dac, dac, does_not_exist)
            (I2S0, i2s0, does_not_exist)
            (I2S1, i2s1, does_not_exist)
            (Sdhc0, sdhc0, does_not_exist)
            (Sdhc1, sdhc1, does_not_exist)
            (CM4Trace, cm4_trace, does_not_exist)
        );
    };
}

/// Internal implementation detail of the `with_pclk_type_ids*` macros
///
/// Call the supplied macro and append the [`PclkId`] type information. Filter
/// out the "existence" tokens when they are not requested.
macro_rules! __call_macro {
    (
        without_existence,
        $some_macro:ident ! ( $( $args:tt )* ),
        $(
            $( #[$cfg:meta] )?
            ($Type:ident, $id:ident, $existence:ident)
        )+
    ) => {
        $some_macro!(
            $( $args )*
            $(
                $( #[$cfg] )?
                ($Type, $id)
            )+
        );
    };
    (
        with_existence,
        $some_macro:ident ! ( $( $args:tt )* ),
        $(
            $( #[$cfg:meta] )?
            ($Type:ident, $id:ident, $existence:ident)
        )+
    ) => {
        $some_macro!(
            $( $args )*
            $(
                $( #[$cfg] )?
                ($Type, $id, $existence)
            )+
        );
    };
}

pub(super) use __call_macro;
pub(super) use __dispatch;
pub(super) use with_pclk_types_ids;
pub(super) use with_pclk_types_ids_existence;

/// Module containing only the types implementing [`PclkId`]
///
/// Because there are so many types that implement `PclkId`, it is helpful to
/// have them defined in a separate module, so that you can import all of them
/// with a wildcard (`*`) without importing anything else, i.e.
///
/// ```
/// use pclk::ids::*;
/// ```
pub(crate) mod ids {

    use crate::typelevel::Sealed;

    #[cfg(feature = "min-samd51g")]
    pub use crate::sercom::v2::{Sercom0, Sercom1, Sercom2, Sercom3, Sercom4, Sercom5};
    #[cfg(feature = "min-samd51n")]
    pub use crate::sercom::v2::{Sercom6, Sercom7};

    pub use super::super::dfll::DfllId;
    pub use super::super::dpll::{Dpll0Id, Dpll1Id};
    pub use super::super::gclk::{
        Gclk0Id, Gclk10Id, Gclk11Id, Gclk1Id, Gclk2Id, Gclk3Id, Gclk4Id, Gclk5Id, Gclk6Id, Gclk7Id,
        Gclk8Id, Gclk9Id,
    };

    /// Define any [`PclkId`] types that are not already defined by some
    /// existing HAL peripheral
    macro_rules! define_pclk_types {

        // Ignore types that already exist
        ($Type:ident, $id:ident, already_exists) => {};

        // Define types that don't exist
        ($Type:ident, $id:ident, does_not_exist) => {
            pub enum $Type {}
            impl Sealed for $Type {}
        };

        // Convert a single call with a list of tokens to a list of calls
        (
            $(
                $( #[$cfg:meta] )?
                ( $Type:ident, $id:ident, $existence:ident )
            )+
        ) => {
            $(
                $( #[$cfg] )?
                define_pclk_types!($Type, $id, $existence);
            )+
        };
    }

    super::with_pclk_types_ids_existence!(define_pclk_types!());
}

use ids::*;

//==============================================================================
// PclkId
//==============================================================================

/// Type-level `enum` for the 48 peripheral channel clock variants
pub trait PclkId: Sealed {
    /// Corresponding variant of [`DynPclkId`]
    const DYN: DynPclkId;
}

macro_rules! impl_pclk_id {
    (
        $(
            $( #[$cfg:meta] )?
            ($Type:ident, $Id:ident)
        )+
    ) => {
        $(
            paste! {
                impl PclkId for $Type {
                    const DYN: DynPclkId = DynPclkId::[<$Id:camel>];
                }
            }
        )+
    };
}

with_pclk_types_ids!(impl_pclk_id!());

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
                    DynPclkSourceId::Gclk~N => GEN_A::GCLK~N,
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
        S: Source<Id = T> + Increment,
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
        S: Source<Id = T> + Decrement,
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
// Tokens
//==============================================================================

macro_rules! define_pclk_tokens_struct {
    (
        $( #[$docs:meta] )?
        $Tokens:ident
        $(
            $( #[$cfg:meta] )?
            ($Type:ident, $Id:ident)
        )+
    ) =>
    {
        $( #[$docs] )?
        #[allow(missing_docs)]
        pub struct $Tokens {
            $(
                $( #[$cfg] )?
                pub $Id: PclkToken<$Type>,
            )+
        }

        impl $Tokens {
            #[inline]
            pub(super) fn new() -> Self {
                unsafe {
                    $Tokens {
                        $(
                            $( #[$cfg] )?
                            $Id: PclkToken::new(),
                        )+
                    }
                }
            }
        }
    };
}

pub(super) use define_pclk_tokens_struct;

with_pclk_types_ids!(define_pclk_tokens_struct!(
    /// Struct containing all possible peripheral clock tokens
    Tokens
));
