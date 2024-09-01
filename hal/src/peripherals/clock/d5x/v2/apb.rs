//! # Advanced peripheral bus clocks
//!
//! ## Overview
//!
//! APB clocks facilitate communication between the processor core and
//! peripherals on the APB bus. To communicate with a peripheral, the
//! corresponding APB clock must be enabled, which is done by setting a bit in
//! one of the four `APBXMASK` registers.
//!
//! In this module, *enabled* APB clocks are represented by the [`ApbClk<A>`]
//! struct, where the type parameter `A` is a type that implements [`ApbId`] and
//! corresponds to one of the bits in an `APBXMASK` register.
//!
//! While most other clocks in the `clock` module are configured through
//! mutually exclusive registers, the [`ApbClk`]s share the four `APBXMASK`
//! registers. This presents a challenge for memory safety. Specifically, if we
//! allowed unrestricted access to the corresponding `APBXMASK` register through
//! each `ApbClk`, we could create data races.
//!
//! To solve this problem, we restrict access to the `APBXMASK` registers using
//! the [`Apb`] type. `Apb` was created to act as a gateway to the `APBXMASK`
//! registers, allowing us to use `&mut Apb` as compile-time proof of exclusive
//! access to them.
//!
//! ## Example
//!
//! Enabling and disabling the [`ApbClk`]s proceeds according to the principles
//! outlined in the [`clock` module documentation]. It is best shown with an
//! example.
//!
//! Let's start by using [`clock_system_at_reset`] to access the HAL clocking
//! structs.
//!
//! ```no_run
//! use atsamd_hal::{
//!     clock::v2::{
//!         clock_system_at_reset,
//!     },
//!     pac::Peripherals,
//! };
//! let mut pac = Peripherals::take().unwrap();
//! let (mut buses, clocks, tokens) = clock_system_at_reset(
//!     pac.oscctrl,
//!     pac.osc32kctrl,
//!     pac.gclk,
//!     pac.mclk,
//!     &mut pac.nvmctrl,
//! );
//! ```
//!
//! Some APB clocks are enabled at power-on reset. We can find these in the
//! [`Clocks`] struct.
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #     },
//! #     pac::Peripherals,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (mut buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.oscctrl,
//! #     pac.osc32kctrl,
//! #     pac.gclk,
//! #     pac.mclk,
//! #     &mut pac.nvmctrl,
//! # );
//! let apb_port = clocks.apbs.port;
//! ```
//!
//! Other APB clocks are disabled at power-on reset. To enable these, we must
//! have access to the [`Apb`] bus type, which is found in the [`Buses`] struct.
//! As described above, [`Apb`] mediates access to the shared `APBXMASK`
//! registers. We call [`Apb::enable`] to convert an [`ApbToken`] into the
//! corresponding [`ApbClk`]. The existence of each `ApbClk` type represents
//! proof that the corresponding APB clock has been enabled.
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #     },
//! #     pac::Peripherals,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (mut buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.oscctrl,
//! #     pac.osc32kctrl,
//! #     pac.gclk,
//! #     pac.mclk,
//! #     &mut pac.nvmctrl,
//! # );
//! # let apb_port = clocks.apbs.port;
//! let apb_sercom0 = buses.apb.enable(tokens.apbs.sercom0);
//! ```
//!
//! The complete example is shown below.
//!
//! ```no_run
//! use atsamd_hal::{
//!     clock::v2::{
//!         clock_system_at_reset,
//!     },
//!     pac::Peripherals,
//! };
//! let mut pac = Peripherals::take().unwrap();
//! let (mut buses, clocks, tokens) = clock_system_at_reset(
//!     pac.oscctrl,
//!     pac.osc32kctrl,
//!     pac.gclk,
//!     pac.mclk,
//!     &mut pac.nvmctrl,
//! );
//! let apb_port = clocks.apbs.port;
//! let apb_sercom0 = buses.apb.enable(tokens.apbs.sercom0);
//! ```
//!
//! [`clock` module documentation]: super
//! [`clock_system_at_reset`]: super::clock_system_at_reset
//! [`Clocks`]: super::Clocks
//! [`Buses`]: super::Buses

use atsamd_hal_macros::hal_macro_helper;
use core::marker::PhantomData;

use bitflags;
use paste::paste;

use crate::pac::{self, mclk};

use crate::typelevel::Sealed;

use super::types::*;

//==============================================================================
// Registers
//==============================================================================

/// APB clock controller
///
/// As described in the [module-level documentation](self), this struct mediates
/// access to the shared `APBXMASK` registers. Users can convert a disabled
/// [`ApbToken<A>`] into an enabled [`ApbClk<A>`] using [`Apb::enable`], and
/// vice versa with [`Apb::disable`].
pub struct Apb(());

impl Apb {
    /// Create a new instance of [`Apb`]
    ///
    /// # Safety
    ///
    /// Because the `Apb` mediates access to the `APBMASK` registers, it must be
    /// a singleton. There must never be two simulatenous instances of it at a
    /// time. See the notes on `Token` types and memory safety in the root of
    /// the `clock` module for more details.
    #[inline]
    pub(super) unsafe fn new() -> Self {
        Self(())
    }

    #[inline]
    fn mclk(&self) -> &mclk::RegisterBlock {
        // Safety: The `Apb` type has exclusive access to the `APBXMASK`
        // registers, and it uses a shared reference to the register block. See
        // the notes on `Token` types and memory safety in the root of the
        // `clock` module for more details.
        unsafe { &*pac::Mclk::PTR }
    }

    #[inline]
    fn apbamask(&mut self) -> &mclk::Apbamask {
        self.mclk().apbamask()
    }

    #[inline]
    fn apbbmask(&mut self) -> &mclk::Apbbmask {
        self.mclk().apbbmask()
    }

    #[inline]
    fn apbcmask(&mut self) -> &mclk::Apbcmask {
        self.mclk().apbcmask()
    }

    #[inline]
    fn apbdmask(&mut self) -> &mclk::Apbdmask {
        self.mclk().apbdmask()
    }

    #[inline]
    fn enable_mask(&mut self, mask: ApbMask) {
        // Safety: The mask bits are derived from a `bitflags` struct, so they
        // are guaranteed to be valid.
        unsafe {
            match mask {
                ApbMask::A(mask) => {
                    self.apbamask()
                        .modify(|r, w| w.bits(r.bits() | mask.bits()));
                }
                ApbMask::B(mask) => {
                    self.apbbmask()
                        .modify(|r, w| w.bits(r.bits() | mask.bits()));
                }
                ApbMask::C(mask) => {
                    self.apbcmask()
                        .modify(|r, w| w.bits(r.bits() | mask.bits()));
                }
                ApbMask::D(mask) => {
                    self.apbdmask()
                        .modify(|r, w| w.bits(r.bits() | mask.bits()));
                }
            }
        }
    }

    #[inline]
    fn disable_mask(&mut self, mask: ApbMask) {
        // Safety: The mask bits are derived from a `bitflags` struct, so they
        // are guaranteed to be valid.
        unsafe {
            match mask {
                ApbMask::A(mask) => {
                    self.apbamask()
                        .modify(|r, w| w.bits(r.bits() & !mask.bits()));
                }
                ApbMask::B(mask) => {
                    self.apbbmask()
                        .modify(|r, w| w.bits(r.bits() & !mask.bits()));
                }
                ApbMask::C(mask) => {
                    self.apbcmask()
                        .modify(|r, w| w.bits(r.bits() & !mask.bits()));
                }
                ApbMask::D(mask) => {
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
// DynApbId & ApbMask
//==============================================================================

/// A mask corresponding to one of the APB bridge registers
///
/// Each variant is a [`bitflags`] struct with a binary representation exactly
/// matching the corresponding APB `MASK` register.
enum ApbMask {
    A(ApbAMask),
    B(ApbBMask),
    C(ApbCMask),
    D(ApbDMask),
}

macro_rules! define_apb_types {
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
        /// Value-level enum identifying a single APB clock
        ///
        /// Each variant of this enum corresponds to a specific bit in one of
        /// the four `APBXMASK` registers and identifies one of many possible
        /// APB clocks, which can vary by chip.
        ///
        /// `DynApbId` is the value-level equivalent of [`ApbId`].
        #[repr(u8)]
        pub enum DynApbId {
            $(
                $(
                    $( #[$( $cfg )+] )?
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
                bitflags::bitflags! {
                    #[
                        doc =
                            "APB bridge `" $Reg "` register mask\n"
                            "\n"
                            "This is a [`bitflags`] struct with a binary representation "
                            "exactly matching the `APB" $Reg "MASK` register."
                    ]
                    struct [<Apb $Reg Mask>]: u32 {
                        $(
                            $( #[$( $cfg )+] )?
                            const [<$Type:upper>] = 1 << $BIT;
                        )+
                    }
                }

            )+

            impl From<DynApbId> for ApbMask {
                #[inline]
                fn from(id: DynApbId) -> Self {
                    use DynApbId::*;
                    match id {
                        $(
                            $(
                                $( #[$( $cfg )+] )?
                                $Type => ApbMask::$Reg([<Apb $Reg Mask>]::[<$Type:upper>]),
                            )+
                        )+
                    }
                }
            }
        }
    };
}

#[hal_macro_helper]
define_apb_types!(
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
        #[hal_cfg("gmac")]
        Gmac = 2,
        Tcc2 = 3,
        #[hal_cfg("tcc3")]
        Tcc3 = 4,
        #[hal_cfg("tc4")]
        Tc4 = 5,
        #[hal_cfg("tc5")]
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
        #[hal_cfg("sercom6")]
        Sercom6 = 2,
        #[hal_cfg("sercom7")]
        Sercom7 = 3,
        #[hal_cfg("tcc4")]
        Tcc4 = 4,
        #[hal_cfg("tc6")]
        Tc6 = 5,
        #[hal_cfg("tc7")]
        Tc7 = 6,
        Adc0 = 7,
        Adc1 = 8,
        Dac = 9,
        #[hal_cfg("i2s")]
        I2S = 10,
        Pcc = 11,
    }
);

//==============================================================================
// ApbId
//==============================================================================

/// Type-level enum identifying one of the possible APB clocks
///
/// The types implementing this trait are type-level variants of `ApbId`, and
/// they identify one of the many possible APB clocks, which can vary by chip.
/// Each type corresponds to a specific bit in one of the four `APBXMASK`
/// registers.
///
/// `ApbId` is the type-level equivalent of [`DynApbId`]. See the documentation
/// on [type-level programming] and specifically [type-level enums] for more
/// details.
///
/// [type-level programming]: crate::typelevel
/// [type-level enums]: crate::typelevel#type-level-enums
pub trait ApbId: Sealed {
    /// Corresponding variant of [`DynApbId`]
    const DYN: DynApbId;
}

//==============================================================================
// ApbToken
//==============================================================================

/// Singleton token that can be exchanged for an [`ApbClk`]
///
/// As explained in the [`clock` module documentation](super), instances of
/// various `Token` types can be exchanged for actual clock types. They
/// represent clocks that are disabled.
///
/// The type parameter `A` is an [`ApbId`] indicating which APB clock is
/// represented by this token. To enable the corresponding APB clock, use the
/// [`Apb::enable`] method.
pub struct ApbToken<A: ApbId> {
    id: PhantomData<A>,
}

impl<A: ApbId> ApbToken<A> {
    /// Create a new instance of [`ApbToken`]
    ///
    /// # Safety
    ///
    /// Each `ApbToken` is a singleton. There must never be two simulatenous
    /// instances with the same [`ApbId`]. See the notes on `Token` types and
    /// memory safety in the root of the `clock` module for more details.
    #[inline]
    unsafe fn new() -> Self {
        ApbToken { id: PhantomData }
    }
}

//==============================================================================
// ApbClk
//==============================================================================

/// An enabled APB clock
///
/// An [`ApbClk`] represents an enabled APB clock. The type parameter `A` is an
/// [`ApbId`], which corresponds to a particular bit in the `APBXMASK`
/// registers. An `ApbClk` can be disabled with the [`Apb::disable`] method.
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

/// Set of [`ApbToken`]s for APB clocks that are disabled at power-on reset
#[hal_macro_helper]
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
    #[hal_cfg("tc4")]
    pub tc4: ApbToken<Tc4>,
    pub tcc2: ApbToken<Tcc2>,
    #[hal_cfg("tcc3")]
    pub tcc3: ApbToken<Tcc3>,
    #[hal_cfg("tc5")]
    pub tc5: ApbToken<Tc5>,
    pub p_dec: ApbToken<PDec>,
    pub ac: ApbToken<Ac>,
    pub aes: ApbToken<Aes>,
    pub trng: ApbToken<Trng>,
    pub icm: ApbToken<Icm>,
    pub ccl: ApbToken<Ccl>,
    pub sercom4: ApbToken<Sercom4>,
    pub sercom5: ApbToken<Sercom5>,
    #[hal_cfg("sercom6")]
    pub sercom6: ApbToken<Sercom6>,
    #[hal_cfg("sercom7")]
    pub sercom7: ApbToken<Sercom7>,
    #[hal_cfg("tcc4")]
    pub tcc4: ApbToken<Tcc4>,
    #[hal_cfg("tc6")]
    pub tc6: ApbToken<Tc6>,
    #[hal_cfg("tc7")]
    pub tc7: ApbToken<Tc7>,
    pub adc0: ApbToken<Adc0>,
    pub adc1: ApbToken<Adc1>,
    pub dac: ApbToken<Dac>,
    #[hal_cfg("i2s")]
    pub i2s: ApbToken<I2S>,
    pub pcc: ApbToken<Pcc>,
}

impl ApbTokens {
    /// Create the set of [`ApbToken`]s
    ///
    /// # Safety
    ///
    /// All invariants required by `ApbToken::new` must be upheld here as well.
    #[inline]
    #[hal_macro_helper]
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
            #[hal_cfg("tc4")]
            tc4: ApbToken::new(),
            tcc2: ApbToken::new(),
            #[hal_cfg("tcc3")]
            tcc3: ApbToken::new(),
            #[hal_cfg("tc5")]
            tc5: ApbToken::new(),
            p_dec: ApbToken::new(),
            ac: ApbToken::new(),
            aes: ApbToken::new(),
            trng: ApbToken::new(),
            icm: ApbToken::new(),
            ccl: ApbToken::new(),
            sercom4: ApbToken::new(),
            sercom5: ApbToken::new(),
            #[hal_cfg("sercom6")]
            sercom6: ApbToken::new(),
            #[hal_cfg("sercom7")]
            sercom7: ApbToken::new(),
            #[hal_cfg("tcc4")]
            tcc4: ApbToken::new(),
            #[hal_cfg("tc6")]
            tc6: ApbToken::new(),
            #[hal_cfg("tc7")]
            tc7: ApbToken::new(),
            adc0: ApbToken::new(),
            adc1: ApbToken::new(),
            dac: ApbToken::new(),
            #[hal_cfg("i2s")]
            i2s: ApbToken::new(),
            pcc: ApbToken::new(),
        }
    }
}

//==============================================================================
// ApbClks
//==============================================================================

/// Set of [`ApbClk`]s for APB clocks that are enabled at power-on reset
#[hal_macro_helper]
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
    #[hal_cfg("gmac")]
    pub gmac: ApbClk<Gmac>,
    pub qspi: ApbClk<Qspi>,
}

impl ApbClks {
    /// Create the set of [`ApbClk`]s
    ///
    /// # Safety
    ///
    /// All invariants required by `ApbToken::new` must be upheld here as well.
    #[inline]
    #[hal_macro_helper]
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
            #[hal_cfg("gmac")]
            gmac: ApbClk::new(ApbToken::new()),
            qspi: ApbClk::new(ApbToken::new()),
        }
    }
}
