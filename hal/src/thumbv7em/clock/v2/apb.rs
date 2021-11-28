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

use crate::sercom::v2::*;
use crate::typelevel::Sealed;

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

/// Enum of all APB clocks in bridge `A`
///
/// Note that this type is `repr(u8)` and each variant maps to the corresponding
/// bit within the APBAMASK register.
#[repr(u8)]
#[allow(missing_docs)]
pub enum DynApbAId {
    Pac = 0,
    Pm = 1,
    Mclk = 2,
    RstC = 3,
    OscCtrl = 4,
    Osc32kCtrl = 5,
    SupC = 6,
    Gclk = 7,
    Wdt = 8,
    Rtc = 9,
    Eic = 10,
    FreqM = 11,
    Sercom0 = 12,
    Sercom1 = 13,
    Tc0 = 14,
    Tc1 = 15,
}

/// Enum of all APB clocks in bridge `B`
///
/// Note that this type is `repr(u8)` and each variant maps to the corresponding
/// bit within the APBBMASK register.
#[repr(u8)]
#[allow(missing_docs)]
pub enum DynApbBId {
    Usb = 0,
    Dsu = 1,
    NvmCtrl = 2,
    Port = 4,
    EvSys = 7,
    Sercom2 = 9,
    Sercom3 = 10,
    Tcc0 = 11,
    Tcc1 = 12,
    Tc2 = 13,
    Tc3 = 14,
    RamEcc = 16,
}

/// Enum of all APB clocks in bridge `C`
///
/// Note that this type is `repr(u8)` and each variant maps to the corresponding
/// bit within the APBCMASK register.
#[repr(u8)]
#[allow(missing_docs)]
pub enum DynApbCId {
    #[cfg(any(feature = "same53", feature = "same54"))]
    Gmac = 2,
    Tcc2 = 3,
    #[cfg(feature = "min-samd51j")]
    Tcc3 = 4,
    #[cfg(feature = "min-samd51j")]
    Tc4 = 5,
    #[cfg(feature = "min-samd51j")]
    Tc5 = 6,
    Pdec = 7,
    Ac = 8,
    Aes = 9,
    Trng = 10,
    Icm = 11,
    Qspi = 13,
    Ccl = 14,
}

/// Enum of all APB clocks in bridge `D`
///
/// Note that this type is `repr(u8)` and each variant maps to the corresponding
/// bit within the APBDMASK register.
#[repr(u8)]
#[allow(missing_docs)]
pub enum DynApbDId {
    Sercom4 = 0,
    Sercom5 = 1,
    #[cfg(feature = "min-samd51n")]
    Sercom6 = 2,
    #[cfg(feature = "min-samd51n")]
    Sercom7 = 3,
    #[cfg(feature = "min-samd51j")]
    Tcc4 = 4,
    #[cfg(feature = "min-samd51n")]
    Tc6 = 5,
    #[cfg(feature = "min-samd51n")]
    Tc7 = 6,
    Adc0 = 7,
    Adc1 = 8,
    Dac = 9,
    #[cfg(feature = "min-samd51j")]
    I2S = 10,
    Pcc = 11,
}

/// Enum of all APB clocks
///
/// Note that each variant of `ApbId` is `repr(u8)` and each maps its variants
/// to the bit within the corresponding APB mask register.
///
/// This is the value-level equivalent of [`ApbId`].
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

/// Macro implementing an [`ApbType`] for a marker type and defining one if
/// necessary
macro_rules! apb_id {
    (false, $Reg:ident, $Type:ident) => {
        /// Marker type implementing [`ApbType`] for a specific synchronous peripheral
        /// clock
        pub enum $Type {}
        impl Sealed for $Type {}
        apb_id!(true, $Reg, $Type);
    };
    (true, $Reg:ident, $Type:ident) => {
        paste! {
            impl ApbId for $Type {
                const DYN: DynApbId = DynApbId::$Reg([<DynApb $Reg Id>]::$Type);
            }
        }
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
                    ($exists:literal, $Type:ident, $init:ident),
                )+
            ],
        )+
    ) => {
        paste! {
            $(
                $(
                    $( #[$cfg] )?
                    apb_id!($exists, $Reg, $Type);
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
        (false, Pac, enabled),
        (false, Pm, enabled),
        (false, Mclk, enabled),
        (false, RstC, enabled),
        (false, OscCtrl, enabled),
        (false, Osc32kCtrl, enabled),
        (false, SupC, enabled),
        (false, Gclk, enabled),
        (false, Wdt, enabled),
        (false, Rtc, enabled),
        (false, Eic, enabled),
        (false, FreqM, disabled),
        (true, Sercom0, disabled),
        (true, Sercom1, disabled),
        (false, Tc0, disabled),
        (false, Tc1, disabled),
    ],
    B:
    [
        (false, Usb, disabled),
        (false, Dsu, enabled),
        (false, NvmCtrl, enabled),
        (false, Port, enabled),
        (false, EvSys, disabled),
        (true, Sercom2, disabled),
        (true, Sercom3, disabled),
        (false, Tcc0, disabled),
        (false, Tcc1, disabled),
        (false, Tc2, disabled),
        (false, Tc3, disabled),
        (false, RamEcc, enabled),
    ],
    C:
    [
        #[cfg(any(feature = "same53", feature = "same54"))]
        (false, Gmac, enabled),
        (false, Tcc2, disabled),
        #[cfg(feature = "min-samd51j")]
        (false, Tcc3, disabled),
        #[cfg(feature = "min-samd51j")]
        (false, Tc4, enabled),
        #[cfg(feature = "min-samd51j")]
        (false, Tc5, disabled),
        (false, Pdec, disabled),
        (false, Ac, disabled),
        (false, Aes, disabled),
        (false, Trng, disabled),
        (false, Icm, disabled),
        (false, Qspi, enabled),
        (false, Ccl, disabled),
    ],
    D:
    [
        (true, Sercom4, disabled),
        (true, Sercom5, disabled),
        #[cfg(feature = "min-samd51n")]
        (true, Sercom6, disabled),
        #[cfg(feature = "min-samd51n")]
        (true, Sercom7, disabled),
        #[cfg(feature = "min-samd51j")]
        (false, Tcc4, disabled),
        #[cfg(feature = "min-samd51n")]
        (false, Tc6, disabled),
        #[cfg(feature = "min-samd51n")]
        (false, Tc7, disabled),
        (false, Adc0, disabled),
        (false, Adc1, disabled),
        (false, Dac, disabled),
        #[cfg(feature = "min-samd51j")]
        (false, I2S, disabled),
        (false, Pcc, disabled),
    ],
);
