//! # AHB bus clocks
//!
//! This module provides abstractions allowing to deal with a synchronous
//! clocking domain, specifically modules clocked via AHB bus. It provides type
//! representation for disabled and enabled synchronous clocks available through
//! AHB bus and means of switching.
//!
//! - [`AhbToken<T>`] type represents a disabled clock for a peripheral of type
//!   `T`: [`AhbType`]
//! - [`AhbClk<T>`] type represents an enabled clock for a peripheral of type
//!   `T:` [`AhbType`]
//!
//! One can enable a peripheral `T` synchronous clock via
//! [`AhbToken<T>::enable`] `->` [`AhbClk<T>`] method.
//!
//! One can disable a peripheral `T` synchronous clock via
//! [`AhbClk<T>::disable`] `->` [`AhbToken<T>`] method.
//!
//! Clocks in a default state are provided
//! - in an instance of a struct [`AhbClks`]
//! - in a field [`crate::clock::v2::Tokens::ahbs`]
//! - in a return value of [`crate::clock::v2::retrieve_clocks`]
use core::marker::PhantomData;

use paste::paste;

use crate::pac::{mclk, MCLK};

use super::types::*;

//==============================================================================
// Registers
//==============================================================================

/// [`Registers`] struct is a low-level access abstraction for HW register
/// calls. It is generic over [`AhbType`] as it needs appropriate mask values
/// depending on a peripheral.
struct Registers<A: AhbId> {
    ahb: PhantomData<A>,
}

impl<A: AhbId> Registers<A> {
    #[inline(always)]
    unsafe fn new() -> Self {
        Registers { ahb: PhantomData }
    }

    #[inline(always)]
    fn mclk(&mut self) -> &mclk::RegisterBlock {
        unsafe { &*MCLK::ptr() }
    }

    #[inline(always)]
    fn ahbmask(&mut self) -> &mclk::AHBMASK {
        &self.mclk().ahbmask
    }

    #[inline(always)]
    fn enable(&mut self) {
        self.ahbmask()
            .modify(|r, w| unsafe { w.bits(r.bits() | A::MASK) });
    }

    #[inline(always)]
    fn disable(&mut self) {
        self.ahbmask()
            .modify(|r, w| unsafe { w.bits(r.bits() & !A::MASK) });
    }
}

//==============================================================================
// AhbId
//==============================================================================

/// Type-level `enum` for AHB clocks
///
/// See the documentation on [type-level enums] for more details on the pattern.
/// The value-level equivalent is [`DynAhbId`].
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub trait AhbId: crate::typelevel::Sealed {
    /// Corresponding variant of [`DynAhbId`]
    const DYN: DynAhbId;
    /// Mask for the corresponding bit of the AHBMASK register
    const MASK: u32 = 1 << (Self::DYN as u8);
}

//==============================================================================
// AhbToken
//==============================================================================

/// A type representing a synchronous peripheral clock in a disabled state
pub struct AhbToken<A: AhbId> {
    regs: Registers<A>,
}

impl<A: AhbId> AhbToken<A> {
    /// Constructor
    ///
    /// Unsafe: There should always be only a single instance thereof. It is
    /// being provided by a framework in a [`AhbClks`] struct instance
    #[inline]
    unsafe fn new() -> Self {
        AhbToken {
            regs: Registers::new(),
        }
    }

    /// Enable a synchronous peripheral clock
    #[inline]
    pub fn enable(mut self) -> AhbClk<A> {
        self.regs.enable();
        AhbClk::new(self)
    }
}

//==============================================================================
// AhbClk
//==============================================================================

/// A type representing a synchronous peripheral clock in an enabled state
pub struct AhbClk<A: AhbId> {
    token: AhbToken<A>,
}

impl<A: AhbId> AhbClk<A> {
    /// Constructor
    #[inline]
    fn new(token: AhbToken<A>) -> Self {
        AhbClk { token }
    }

    /// Disable a synchronous peripheral clock
    #[inline]
    pub fn disable(mut self) -> AhbToken<A> {
        self.token.regs.disable();
        self.token
    }
}

//==============================================================================
// AhbClks
//==============================================================================

/// Root level macro generating all type definitions and trait implementations.
/// Among other things, it defines and populates an [`AhbClks`] struct with
/// appropriate fields.
macro_rules! ahb_clks {
    (
        $(
            $( #[$cfg:meta] )?
            $Type:ident = $N:literal,
        )+
    ) => {
        paste! {
            /// Value-level `enum` of all AHB clocks
            ///
            /// This is the value-level equivalent of the [type-level enum]
            /// [`AhbId`]. When cast to an integer type, like `u8`, each variant
            /// of this `enum` maps to the corresponding bit number within the
            /// AHBMASK register.
            ///
            /// [type-level enum]: crate::typelevel#type-level-enum
            #[allow(missing_docs)]
            pub enum DynAhbId {
                $(
                    $( #[$cfg] )?
                    $Type = $N,
                )+
            }

            $(
                $( #[$cfg] )?
                impl AhbId for $Type {
                    const DYN: DynAhbId = DynAhbId::$Type;
                }
            )+

            /// Struct aggregating [`AhbClk`] type instances representing
            /// default states of synchronous peripheral clocks
            #[allow(missing_docs)]
            pub struct AhbClks {
                $(
                    $( #[$cfg] )?
                    pub [<$Type:lower>]: AhbClk<$Type>,
                )+
            }
            impl AhbClks {
                #[inline]
                pub(super) unsafe fn new() -> Self {
                    AhbClks {
                        $(
                            $( #[$cfg] )?
                            [<$Type:lower>]: AhbClk::new(AhbToken::new()),
                        )+
                    }
                }
            }
        }
    };
}

ahb_clks!(
    Hpb0 = 0,
    Hpb1 = 1,
    Hpb2 = 2,
    Hpb3 = 3,
    Dsu = 4,
    NvmCtrl = 6,
    Cmcc = 8,
    Dmac = 9,
    Usb = 10,
    Pac = 12,
    Qspi = 13,
    #[cfg(any(feature = "same53", feature = "same54"))]
    Gmac = 14,
    Sdhc0 = 15,
    #[cfg(feature = "min-samd51n")]
    Sdhc1 = 16,
    #[cfg(any(feature = "same51", feature = "same53", feature = "same54"))]
    Can0 = 17,
    #[cfg(any(feature = "same51", feature = "same53", feature = "same54"))]
    Can1 = 18,
    Icm = 19,
    Pukcc = 20,
    Qspi2x = 21,
    NvmCtrlSmeeProm = 22,
    NvmCtrlCache = 23,
);
