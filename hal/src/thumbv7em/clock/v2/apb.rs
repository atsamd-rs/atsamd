//! # APBx bus clocks
//!
//! This module provides abstractions allowing to deal with a synchronous
//! clocking domain, specifically modules clocked via APB bus. It provides type
//! representation for disabled and enabled synchronous clocks available through
//! APB bus and means of switching.
//!
//! - [`ApbToken<T>`] type represents a disabled clock for a peripheral of type
//!   `T`: [`ApbType`]
//! - [`ApbClk<T>`] type represents an enabled clock for a peripheral of type
//!   `T:` [`ApbType`]
//!
//! One can enable a peripheral `T` synchronous clock via
//! [`ApbToken<T>::enable`] `->` [`ApbClk<T>`] method.
//!
//! One can disable a peripheral `T` synchronous clock via
//! [`ApbClk<T>::disable`] `->` [`ApbToken<T>`] method.
//!
//! Clocks in a default state are provided
//! - in an instance of a struct [`ApbClks`]
//! - in a field [`crate::clock::v2::Tokens::apbs`]
//! - in a return value of [`crate::clock::v2::retrieve_clocks`]
use core::marker::PhantomData;

use paste::paste;

use crate::pac::{mclk, MCLK};

use crate::typelevel::Sealed;

use super::types::*;

//==============================================================================
// Registers
//==============================================================================

/// [`Registers`] struct is a low-level access abstraction for HW register
/// calls. It is generic over [`ApbType`] as it needs appropriate mask values
/// and register variants depending on a peripheral.
struct Registers<A: ApbId> {
    apb: PhantomData<A>,
}

impl<A: ApbId> Registers<A> {
    #[inline(always)]
    unsafe fn new() -> Self {
        Registers { apb: PhantomData }
    }

    #[inline(always)]
    fn mclk(&mut self) -> &mclk::RegisterBlock {
        unsafe { &*MCLK::ptr() }
    }

    #[inline(always)]
    fn apbamask(&mut self) -> &mclk::APBAMASK {
        &self.mclk().apbamask
    }

    #[inline(always)]
    fn apbbmask(&mut self) -> &mclk::APBBMASK {
        &self.mclk().apbbmask
    }

    #[inline(always)]
    fn apbcmask(&mut self) -> &mclk::APBCMASK {
        &self.mclk().apbcmask
    }

    #[inline(always)]
    fn apbdmask(&mut self) -> &mclk::APBDMASK {
        &self.mclk().apbdmask
    }

    #[inline(always)]
    fn enable(&mut self) {
        unsafe {
            match A::DYN {
                DynApbId::A(_) => {
                    self.apbamask().modify(|r, w| w.bits(r.bits() | A::MASK));
                }
                DynApbId::B(_) => {
                    self.apbbmask().modify(|r, w| w.bits(r.bits() | A::MASK));
                }
                DynApbId::C(_) => {
                    self.apbcmask().modify(|r, w| w.bits(r.bits() | A::MASK));
                }
                DynApbId::D(_) => {
                    self.apbdmask().modify(|r, w| w.bits(r.bits() | A::MASK));
                }
            }
        }
    }

    #[inline(always)]
    fn disable(&mut self) {
        unsafe {
            match A::DYN {
                DynApbId::A(_) => {
                    self.apbamask().modify(|r, w| w.bits(r.bits() & !A::MASK));
                }
                DynApbId::B(_) => {
                    self.apbbmask().modify(|r, w| w.bits(r.bits() & !A::MASK));
                }
                DynApbId::C(_) => {
                    self.apbcmask().modify(|r, w| w.bits(r.bits() & !A::MASK));
                }
                DynApbId::D(_) => {
                    self.apbdmask().modify(|r, w| w.bits(r.bits() & !A::MASK));
                }
            }
        }
    }
}

//==============================================================================
// DynApbId
//==============================================================================

/// Value-level `enum` of all APB clocks
///
/// This is the value-level equivalent of the [type-level enum] [`ApbId`]. The
/// contents of each variant can be cast to an integer type, like `u8`, to
/// recover the clock's bit number within the corresponding `APBMASK` register.
///
/// [type-level enum]: crate::typelevel#type-level-enum
#[allow(missing_docs)]
pub enum DynApbId {
    A(DynApbAId),
    B(DynApbBId),
    C(DynApbCId),
    D(DynApbDId),
}

//==============================================================================
// ApbId
//==============================================================================

/// Type-level `enum` for APB clocks
///
/// See the documentation on [type-level enums] for more details on the pattern.
/// The value-level equivalent is [`DynApbId`].
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub trait ApbId: Sealed {
    /// Corresponding variant of [`DynApbId`]
    const DYN: DynApbId;
    /// Mask for the bit of the corresponding APB mask register
    const MASK: u32 = 1
        << match Self::DYN {
            DynApbId::A(id) => id as u8,
            DynApbId::B(id) => id as u8,
            DynApbId::C(id) => id as u8,
            DynApbId::D(id) => id as u8,
        };
}

//==============================================================================
// ApbToken
//==============================================================================

/// A type representing a synchronous peripheral clock in a disabled state
pub struct ApbToken<A: ApbId> {
    regs: Registers<A>,
}

impl<A: ApbId> ApbToken<A> {
    /// Constructor
    ///
    /// Unsafe: There should always be only a single instance thereof. It is
    /// being provided by a framework in a [`ApbClks`] struct instance
    #[inline]
    unsafe fn new() -> Self {
        ApbToken {
            regs: Registers::new(),
        }
    }

    /// Enable a synchronous peripheral clock
    #[inline]
    pub fn enable(mut self) -> ApbClk<A> {
        self.regs.enable();
        ApbClk::new(self)
    }
}

//==============================================================================
// ApbClk
//==============================================================================

/// A type representing a synchronous peripheral clock in an enabled state
pub struct ApbClk<A: ApbId> {
    token: ApbToken<A>,
}

impl<A: ApbId> ApbClk<A> {
    /// Constructor
    #[inline]
    fn new(token: ApbToken<A>) -> Self {
        ApbClk { token }
    }

    /// Disable a synchronous peripheral clock
    #[inline]
    pub fn disable(mut self) -> ApbToken<A> {
        self.token.regs.disable();
        self.token
    }
}

//==============================================================================
// ApbClks
//==============================================================================

/// Helper macro providing a full type definition depending on a default state
/// of a synchronous peripheral clock
macro_rules! init_type {
    (enabled, $Type:ident) => { ApbClk<$Type> };
    (disabled, $Type:ident) => { ApbToken<$Type> };
}

/// Helper macro generating an appropriate initialization code depending on a
/// default state of a synchronous peripheral clock
macro_rules! init_expr {
    (enabled) => {
        ApbClk::new(ApbToken::new())
    };
    (disabled) => {
        ApbToken::new()
    };
}

/// Root level macro generating all type definitions and trait implementations.
/// Among other things, it defines and populates an [`ApbClks`] struct with
/// appropriate fields.
macro_rules! apb_clks {
    (
        $(
            $Reg:ident:
            [
                $(
                    $( #[$cfg:meta] )?
                    ($Type:ident = $N:literal, $init:ident),
                )+
            ],
        )+
    ) => {
        paste! {
            $(
                #[
                    doc = "Value-level `enum` of all APB clocks in bridge `" $Reg "`\n\n"
                    "When cast to an integer type, like `u8`, each variant maps "
                    "to the corresponding bit within the `APB" $Reg "MASK` register."
                ]
                #[allow(missing_docs)]
                pub enum [<DynApb $Reg Id>] {
                    $(
                        $( #[$cfg] )?
                        $Type = $N,
                    )+
                }

                $(
                    $( #[$cfg] )?
                    impl ApbId for $Type {
                        const DYN: DynApbId = DynApbId::$Reg([<DynApb $Reg Id>]::$Type);
                    }
                )+
            )+
            /// Struct aggregating [`ApbClk`] and [`ApbToken`] type instances
            /// representing default states of synchronous peripheral clocks
            #[allow(missing_docs)]
            pub struct ApbClks {
                $(
                    $(
                        $( #[$cfg] )?
                        pub [<$Type:lower>]: init_type!($init, $Type),
                    )+
                )+
            }
            impl ApbClks {
                #[inline]
                pub(super) unsafe fn new() -> Self {
                    ApbClks {
                        $(
                            $(
                                $( #[$cfg] )?
                                [<$Type:lower>]: init_expr!($init),
                            )+
                        )+
                    }
                }
            }
        }
    };
}

apb_clks!(
    A:
    [
        (Pac = 0, enabled),
        (Pm = 1, enabled),
        (Mclk = 2, enabled),
        (RstC = 3, enabled),
        (OscCtrl = 4, enabled),
        (Osc32kCtrl = 5, enabled),
        (SupC = 6, enabled),
        (Gclk = 7, enabled),
        (Wdt = 8, enabled),
        (Rtc = 9, enabled),
        (Eic = 10, enabled),
        (FreqM = 11, disabled),
        (Sercom0 = 12, disabled),
        (Sercom1 = 13, disabled),
        (Tc0 = 14, disabled),
        (Tc1 = 15, disabled),
    ],
    B:
    [
        (Usb = 0, disabled),
        (Dsu = 1, enabled),
        (NvmCtrl = 2, enabled),
        (Port = 4, enabled),
        (EvSys = 7, disabled),
        (Sercom2 = 9, disabled),
        (Sercom3 = 10, disabled),
        (Tcc0 = 11, disabled),
        (Tcc1 = 12, disabled),
        (Tc2 = 13, disabled),
        (Tc3 = 14, disabled),
        (RamEcc = 16, enabled),
    ],
    C:
    [
        #[cfg(any(feature = "same53", feature = "same54"))]
        (Gmac = 2, enabled),
        (Tcc2 = 3, disabled),
        #[cfg(feature = "min-samd51j")]
        (Tcc3 = 4, disabled),
        #[cfg(feature = "min-samd51j")]
        (Tc4 = 5, enabled),
        #[cfg(feature = "min-samd51j")]
        (Tc5 = 6, disabled),
        (PDec = 7, disabled),
        (Ac = 8, disabled),
        (Aes = 9, disabled),
        (Trng = 10, disabled),
        (Icm = 11, disabled),
        (Qspi = 13, enabled),
        (Ccl = 14, disabled),
    ],
    D:
    [
        (Sercom4 = 0, disabled),
        (Sercom5 = 1, disabled),
        #[cfg(feature = "min-samd51n")]
        (Sercom6 = 2, disabled),
        #[cfg(feature = "min-samd51n")]
        (Sercom7 = 3, disabled),
        #[cfg(feature = "min-samd51j")]
        (Tcc4 = 4, disabled),
        #[cfg(feature = "min-samd51n")]
        (Tc6 = 5, disabled),
        #[cfg(feature = "min-samd51n")]
        (Tc7 = 6, disabled),
        (Adc0 = 7, disabled),
        (Adc1 = 8, disabled),
        (Dac = 9, disabled),
        #[cfg(feature = "min-samd51j")]
        (I2S = 10, disabled),
        (Pcc = 11, disabled),
    ],
);
