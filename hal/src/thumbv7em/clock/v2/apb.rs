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

use bitflags::bitflags;
use paste::paste;

use crate::pac::{mclk, MCLK};

use crate::typelevel::Sealed;

use super::types::*;

//==============================================================================
// Registers
//==============================================================================

/// APB mask controller
///
/// This struct mediates access to the APB `MASK` registers. Each bit in the
/// APB `MASK` registers is represented as a type-level variant of [`ApbId`].
/// And each APB clock is represented as either an `ApbToken<A>` or an
/// `ApbClk<A>`, where `A: ApbId`. `ApbClk` represents an enabled APB clock,
/// while `ApbToken` represents a disabled APB clock.
///
/// Use the [`enable`](self::enable) and [`disable`](self::disable) methods to
/// convert tokens into clocks and vice versa.
pub struct Apb(());

impl Apb {
    #[inline]
    unsafe fn new() -> Self {
        Self(())
    }

    #[inline]
    fn mclk(&self) -> &mclk::RegisterBlock {
        unsafe { &*MCLK::ptr() }
    }

    #[inline]
    fn apbamask(&mut self) -> &mclk::APBAMASK {
        &self.mclk().apbamask
    }

    #[inline]
    fn apbbmask(&mut self) -> &mclk::APBBMASK {
        &self.mclk().apbbmask
    }

    #[inline]
    fn apbcmask(&mut self) -> &mclk::APBCMASK {
        &self.mclk().apbcmask
    }

    #[inline]
    fn apbdmask(&mut self) -> &mclk::APBDMASK {
        &self.mclk().apbdmask
    }

    #[inline]
    fn enable_mask(&mut self, mask: DynApbMask) {
        unsafe {
            match mask {
                DynApbMask::A(mask) => {
                    self.apbamask()
                        .modify(|r, w| w.bits(r.bits() | mask.bits()));
                }
                DynApbMask::B(mask) => {
                    self.apbbmask()
                        .modify(|r, w| w.bits(r.bits() | mask.bits()));
                }
                DynApbMask::C(mask) => {
                    self.apbcmask()
                        .modify(|r, w| w.bits(r.bits() | mask.bits()));
                }
                DynApbMask::D(mask) => {
                    self.apbdmask()
                        .modify(|r, w| w.bits(r.bits() | mask.bits()));
                }
            }
        }
    }

    #[inline]
    fn disable_mask(&mut self, mask: DynApbMask) {
        unsafe {
            match mask {
                DynApbMask::A(mask) => {
                    self.apbamask()
                        .modify(|r, w| w.bits(r.bits() & !mask.bits()));
                }
                DynApbMask::B(mask) => {
                    self.apbbmask()
                        .modify(|r, w| w.bits(r.bits() & !mask.bits()));
                }
                DynApbMask::C(mask) => {
                    self.apbcmask()
                        .modify(|r, w| w.bits(r.bits() & !mask.bits()));
                }
                DynApbMask::D(mask) => {
                    self.apbdmask()
                        .modify(|r, w| w.bits(r.bits() & !mask.bits()));
                }
            }
        }
    }

    /// Enable the corresponding APB clock
    ///
    /// Consume an [`ApbToken`], enable the corresponding APB clock and return
    /// an [`ApbClk`]. The `ApbClk` represents proof that the corresponding APB
    /// clock has been enabled.
    #[inline]
    pub fn enable<A: ApbId>(&mut self, token: ApbToken<A>) -> ApbClk<A> {
        self.enable_mask(A::DYN.into());
        ApbClk::new(token)
    }

    /// Disable the corresponding APB clock
    ///
    /// Consume the [`ApbClk`], disable the corresponding APB clock and return
    /// the [`ApbToken`].
    #[inline]
    pub fn disable<A: ApbId>(&mut self, clock: ApbClk<A>) -> ApbToken<A> {
        self.disable_mask(A::DYN.into());
        clock.free()
    }
}

//==============================================================================
// DynApbId & DynApbMask
//==============================================================================

/// Selection of APB register masks
///
/// The mask within each variant is a [`bitflags`] struct with a binary
/// representation matching the corresponding APB `MASK` register.
#[allow(missing_docs)]
pub enum DynApbMask {
    A(DynApbAMask),
    B(DynApbBMask),
    C(DynApbCMask),
    D(DynApbDMask),
}

macro_rules! define_dyn_apb_id_masks {
    (
        $(
            $Reg:ident {
                $(
                    $( #[$( $cfg:tt )+] )?
                    $Type:ident = $BIT:literal,
                )+
            }
        )+
    ) => {
        /// Value-level `enum` of all APB clocks
        ///
        /// This is the value-level version of the [type-level enum] [`AhbId`].
        ///
        /// [type-level enum]: crate::typelevel#type-level-enum
        #[repr(u8)]
        pub enum DynApbId {
            $(
                $(
                    $( #[$( $cfg )+] )?
                    #[allow(missing_docs)]
                    $Type,
                )+
            )+
        }

        $(
            $(
                $( #[$( $cfg )+] )?
                impl ApbId for $Type {
                    const DYN: DynApbId = DynApbId::$Type;
                }
            )+
        )+

        paste! {
            $(
                bitflags! {
                    #[
                        doc =
                            "APB bridge `" $Reg "` register mask\n"
                            "\n"
                            "This is a [`bitflags`] struct with a binary representation "
                            "that exactly matches the `APB" $Reg "MASK` register."
                    ]
                    pub struct [<DynApb $Reg Mask>]: u32 {
                        $(
                            $( #[$( $cfg )+] )?
                            #[allow(missing_docs)]
                            const [<$Type:upper>] = 1 << $BIT;
                        )+
                    }
                }

            )+

            impl From<DynApbId> for DynApbMask {
                #[inline]
                fn from(id: DynApbId) -> Self {
                    use DynApbId::*;
                    match id {
                        $(
                            $(
                                $( #[$( $cfg )+] )?
                                $Type => DynApbMask::$Reg([<DynApb $Reg Mask>]::[<$Type:upper>]),
                            )+
                        )+
                    }
                }
            }
        }
    };
}

define_dyn_apb_id_masks!(
    A {
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
    B {
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
    C {
        #[cfg(any(feature = "same53", feature = "same54"))]
        Gmac = 2,
        Tcc2 = 3,
        #[cfg(feature = "min-samd51j")]
        Tcc3 = 4,
        #[cfg(feature = "min-samd51j")]
        Tc4 = 5,
        #[cfg(feature = "min-samd51j")]
        Tc5 = 6,
        PDec = 7,
        Ac = 8,
        Aes = 9,
        Trng = 10,
        Icm = 11,
        Qspi = 13,
        Ccl = 14,
    }
    D {
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
);

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
    /// Corresponding [`DynApbId`] bit mask
    const DYN: DynApbId;
}

//==============================================================================
// ApbToken
//==============================================================================

/// A type representing a synchronous peripheral clock in a disabled state
pub struct ApbToken<A: ApbId> {
    id: PhantomData<A>,
}

impl<A: ApbId> ApbToken<A> {
    /// Constructor
    ///
    /// Unsafe: There should always be only a single instance thereof. It is
    /// being provided by a framework in a [`ApbClks`] struct instance
    #[inline]
    unsafe fn new() -> Self {
        ApbToken { id: PhantomData }
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
    #[inline]
    fn new(token: ApbToken<A>) -> Self {
        ApbClk { token }
    }

    #[inline]
    fn free(self) -> ApbToken<A> {
        self.token
    }
}

//==============================================================================
// ApbTokens
//==============================================================================

#[allow(missing_docs)]
pub struct ApbTokens {
    pub freq_m: ApbToken<FreqM>,
    pub sercom0: ApbToken<Sercom0>,
    pub sercom1: ApbToken<Sercom1>,
    pub tc0: ApbToken<Tc0>,
    pub tc1: ApbToken<Tc1>,
    pub usb: ApbToken<Usb>,
    pub ev_sys: ApbToken<EvSys>,
    pub sercom2: ApbToken<Sercom2>,
    pub sercom3: ApbToken<Sercom3>,
    pub tcc0: ApbToken<Tcc0>,
    pub tcc1: ApbToken<Tcc1>,
    pub tc2: ApbToken<Tc2>,
    pub tc3: ApbToken<Tc3>,
    pub tcc2: ApbToken<Tcc2>,
    #[cfg(feature = "min-samd51j")]
    pub tcc3: ApbToken<Tcc3>,
    #[cfg(feature = "min-samd51j")]
    pub tc5: ApbToken<Tc5>,
    pub p_dec: ApbToken<PDec>,
    pub ac: ApbToken<Ac>,
    pub aes: ApbToken<Aes>,
    pub trng: ApbToken<Trng>,
    pub icm: ApbToken<Icm>,
    pub ccl: ApbToken<Ccl>,
    pub sercom4: ApbToken<Sercom4>,
    pub sercom5: ApbToken<Sercom5>,
    #[cfg(feature = "min-samd51n")]
    pub sercom6: ApbToken<Sercom6>,
    #[cfg(feature = "min-samd51n")]
    pub sercom7: ApbToken<Sercom7>,
    #[cfg(feature = "min-samd51j")]
    pub tcc4: ApbToken<Tcc4>,
    #[cfg(feature = "min-samd51n")]
    pub tc6: ApbToken<Tc6>,
    #[cfg(feature = "min-samd51n")]
    pub tc7: ApbToken<Tc7>,
    pub adc0: ApbToken<Adc0>,
    pub adc1: ApbToken<Adc1>,
    pub dac: ApbToken<Dac>,
    #[cfg(feature = "min-samd51j")]
    pub i2s: ApbToken<I2S>,
    pub pcc: ApbToken<Pcc>,
}

impl ApbTokens {
    pub(super) unsafe fn new() -> Self {
        Self {
            freq_m: ApbToken::new(),
            sercom0: ApbToken::new(),
            sercom1: ApbToken::new(),
            tc0: ApbToken::new(),
            tc1: ApbToken::new(),
            usb: ApbToken::new(),
            ev_sys: ApbToken::new(),
            sercom2: ApbToken::new(),
            sercom3: ApbToken::new(),
            tcc0: ApbToken::new(),
            tcc1: ApbToken::new(),
            tc2: ApbToken::new(),
            tc3: ApbToken::new(),
            tcc2: ApbToken::new(),
            #[cfg(feature = "min-samd51j")]
            tcc3: ApbToken::new(),
            #[cfg(feature = "min-samd51j")]
            tc5: ApbToken::new(),
            p_dec: ApbToken::new(),
            ac: ApbToken::new(),
            aes: ApbToken::new(),
            trng: ApbToken::new(),
            icm: ApbToken::new(),
            ccl: ApbToken::new(),
            sercom4: ApbToken::new(),
            sercom5: ApbToken::new(),
            #[cfg(feature = "min-samd51n")]
            sercom6: ApbToken::new(),
            #[cfg(feature = "min-samd51n")]
            sercom7: ApbToken::new(),
            #[cfg(feature = "min-samd51j")]
            tcc4: ApbToken::new(),
            #[cfg(feature = "min-samd51n")]
            tc6: ApbToken::new(),
            #[cfg(feature = "min-samd51n")]
            tc7: ApbToken::new(),
            adc0: ApbToken::new(),
            adc1: ApbToken::new(),
            dac: ApbToken::new(),
            #[cfg(feature = "min-samd51j")]
            i2s: ApbToken::new(),
            pcc: ApbToken::new(),
        }
    }
}

//==============================================================================
// ApbClks
//==============================================================================

#[allow(missing_docs)]
pub struct ApbClks {
    pub pac: ApbClk<Pac>,
    pub pm: ApbClk<Pm>,
    pub mclk: ApbClk<Mclk>,
    pub rst_c: ApbClk<RstC>,
    pub osc_ctrl: ApbClk<OscCtrl>,
    pub osc32k_ctrl: ApbClk<Osc32kCtrl>,
    pub sup_c: ApbClk<SupC>,
    pub gclk: ApbClk<Gclk>,
    pub wdt: ApbClk<Wdt>,
    pub rtc: ApbClk<Rtc>,
    pub eic: ApbClk<Eic>,
    pub dsu: ApbClk<Dsu>,
    pub nvm_ctrl: ApbClk<NvmCtrl>,
    pub port: ApbClk<Port>,
    pub ram_ecc: ApbClk<RamEcc>,
    #[cfg(any(feature = "same53", feature = "same54"))]
    pub gmac: ApbClk<Gmac>,
    #[cfg(feature = "min-samd51j")]
    pub tc4: ApbClk<Tc4>,
    pub qspi: ApbClk<Qspi>,
}

impl ApbClks {
    #[inline]
    pub(super) unsafe fn new() -> Self {
        ApbClks {
            pac: ApbClk::new(ApbToken::new()),
            pm: ApbClk::new(ApbToken::new()),
            mclk: ApbClk::new(ApbToken::new()),
            rst_c: ApbClk::new(ApbToken::new()),
            osc_ctrl: ApbClk::new(ApbToken::new()),
            osc32k_ctrl: ApbClk::new(ApbToken::new()),
            sup_c: ApbClk::new(ApbToken::new()),
            gclk: ApbClk::new(ApbToken::new()),
            wdt: ApbClk::new(ApbToken::new()),
            rtc: ApbClk::new(ApbToken::new()),
            eic: ApbClk::new(ApbToken::new()),
            dsu: ApbClk::new(ApbToken::new()),
            nvm_ctrl: ApbClk::new(ApbToken::new()),
            port: ApbClk::new(ApbToken::new()),
            ram_ecc: ApbClk::new(ApbToken::new()),
            #[cfg(any(feature = "same53", feature = "same54"))]
            gmac: ApbClk::new(ApbToken::new()),
            #[cfg(feature = "min-samd51j")]
            tc4: ApbClk::new(ApbToken::new()),
            qspi: ApbClk::new(ApbToken::new()),
        }
    }
}
