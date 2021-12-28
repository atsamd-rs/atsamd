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

use bitflags::bitflags;
use paste::paste;

use crate::pac::{mclk, MCLK};

use super::types::*;

//==============================================================================
// Ahb
//==============================================================================

/// AHB mask controller
///
/// This struct mediates access to the `AHBMASK` register. Each bit in the
/// `AHBMASK` register is represented as a type-level variant of [`AhbId`]. And
/// each AHB clock is represented as either an `AhbToken<A>` or an `AhbClk<A>`,
/// where `A: AhbId`. `AhbClk` represents an enabled AHB clock, while `AhbToken`
/// represents a disabled AHB clock.
///
/// **NOTE:** All AHB clocks are enabled by default at power-on reset.
///
/// Use the [`enable`](self::enable) and [`disable`](self::disable) methods to
/// convert tokens into clocks and vice versa.
///
/// ```
/// // We do not need USB, so disable the USB AHB clock
/// let usb = ahb.disable(ahb_clks.usb)
/// ```
pub struct Ahb(());

impl Ahb {
    #[inline]
    unsafe fn new() -> Self {
        Self(())
    }

    #[inline]
    fn mclk(&self) -> &mclk::RegisterBlock {
        unsafe { &*MCLK::ptr() }
    }

    #[inline]
    fn ahbmask(&mut self) -> &mclk::AHBMASK {
        &self.mclk().ahbmask
    }

    #[inline]
    fn enable_mask(&mut self, mask: DynAhbMask) {
        self.ahbmask()
            .modify(|r, w| unsafe { w.bits(r.bits() | mask.bits()) });
    }

    #[inline]
    fn disable_mask(&mut self, mask: DynAhbMask) {
        self.ahbmask()
            .modify(|r, w| unsafe { w.bits(r.bits() & !mask.bits()) });
    }

    /// Enable the corresponding AHB clock
    ///
    /// Consume an [`AhbToken`], enable the corresponding AHB clock and return
    /// an [`AhbClk`]. The `AhbClk` represents proof that the corresponding AHB
    /// clock has been enabled.
    #[inline]
    pub fn enable<A: AhbId>(&mut self, token: AhbToken<A>) -> AhbClk<A> {
        self.enable_mask(A::DYN.into());
        AhbClk::new(token)
    }

    /// Disable the corresponding AHB clock
    ///
    /// Consume the [`AhbClk`], disable the corresponding AHB clock and return
    /// the [`AhbToken`].
    #[inline]
    pub fn disable<A: AhbId>(&mut self, clock: AhbClk<A>) -> AhbToken<A> {
        self.disable_mask(A::DYN.into());
        clock.free()
    }
}

//==============================================================================
// AhbId
//==============================================================================

/// Type-level `enum` for AHB clock types
///
/// See the documentation on [type-level enums] for more details on the pattern.
/// The value-level equivalent is [`DynAhbId`].
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub trait AhbId: crate::typelevel::Sealed {
    /// Corresponding [`DynAhbId`]
    const DYN: DynAhbId;
}

//==============================================================================
// AhbToken
//==============================================================================

/// A singleton type representing a disabled AHB clock
///
/// An `AhbToken` can be exchanged for an enabled [`AhbClk`] using the
/// [`Ahb::enable`] function.
pub struct AhbToken<A: AhbId> {
    id: PhantomData<A>,
}

impl<A: AhbId> AhbToken<A> {
    /// Create an `AhbToken`
    ///
    /// SAFETY: There should be *exactly one* instance of `AhbToken` for each
    /// [`AhbId`] type.
    #[inline]
    unsafe fn new() -> Self {
        AhbToken { id: PhantomData }
    }
}

//==============================================================================
// AhbClk
//==============================================================================

/// A singleton type representing an enabled AHB clock
///
/// An `AhbClk` can be disabled, returning an [`AhbToken`], using the
/// [`Ahb::disable`] function.
pub struct AhbClk<A: AhbId> {
    token: AhbToken<A>,
}

impl<A: AhbId> AhbClk<A> {
    #[inline]
    fn new(token: AhbToken<A>) -> Self {
        AhbClk { token }
    }

    #[inline]
    fn free(self) -> AhbToken<A> {
        self.token
    }
}

//==============================================================================
// DynAhbId & AhbClks
//==============================================================================

/// Root level macro generating all type definitions and trait implementations.
/// Among other things, it defines and populates an [`AhbClks`] struct with
/// appropriate fields.
macro_rules! ahb_clks {
    (
        $(
            $( #[$( $cfg:tt )+] )?
            $Type:ident = $BIT:literal,
        )+
    ) => {
        paste! {
            bitflags! {
                /// AHB clock register mask
                ///
                /// This is a [`bitflags`] struct with a binary representation
                /// that exactly matches the `AHBMASK` register.
                pub struct DynAhbMask: u32 {
                    $(
                        $( #[$( $cfg )+] )?
                        #[allow(missing_docs)]
                        const [<$Type:upper>] = 1 << $BIT;
                    )+
                }
            }

            /// Value-level representation of AHB clocks
            ///
            /// This is the value-level version of the [type-level enum]
            /// [`AhbId`].
            ///
            /// It can be cast to an integer type, like `u8`, to recover each
            /// clock's bit number within the `AHBMASK` register.
            ///
            /// [type-level enum]: crate::typelevel#type-level-enum
            #[repr(u8)]
            pub enum DynAhbId {
                $(
                    $( #[$( $cfg )+] )?
                    #[allow(missing_docs)]
                    $Type = $BIT,
                )+
            }

            impl From<DynAhbId> for DynAhbMask {
                #[inline]
                fn from(id: DynAhbId) -> DynAhbMask {
                    match id {
                        $(
                            $( #[$( $cfg )+] )?
                            DynAhbId::$Type => DynAhbMask::[<$Type:upper>],
                        )+
                    }
                }
            }

            $(
                $( #[$( $cfg )+] )?
                impl AhbId for $Type {
                    const DYN: DynAhbId = DynAhbId::$Type;
                }
            )+

            /// Struct aggregating [`AhbClk`] type instances representing
            /// default states of synchronous peripheral clocks
            #[allow(missing_docs)]
            pub struct AhbClks {
                $(
                    $( #[$( $cfg )+] )?
                    pub [<$Type:lower>]: AhbClk<$Type>,
                )+
            }
            impl AhbClks {
                /// Create instances of all [`AhbClk`]s
                ///
                /// SAFETY: Each `AhbClk` must be a singleton, so this function
                /// must not be called more than once if any prior `AhbClk`s
                /// still exist.
                #[inline]
                pub(super) unsafe fn new() -> Self {
                    AhbClks {
                        $(
                            $( #[$( $cfg )+] )?
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
