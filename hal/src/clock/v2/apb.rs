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

use atsamd_hal_macros::{hal_cfg, hal_macro_helper};
use core::marker::PhantomData;

use bitflags;
use paste::paste;

#[hal_cfg("clock-d5x")]
mod imports {
    pub use crate::pac::Mclk as Peripheral;
    pub use crate::pac::mclk::{Apbamask, Apbbmask, Apbcmask, Apbdmask, RegisterBlock as BLOCK};
}

#[hal_cfg(any("clock-d11", "clock-d21"))]
mod imports {
    pub use crate::pac::Pm as Peripheral;
    pub use crate::pac::pm::{Apbamask, Apbbmask, Apbcmask, RegisterBlock as BLOCK};
}

use imports::*;

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
    fn mclk(&self) -> &BLOCK {
        // Safety: The `Apb` type has exclusive access to the `APBXMASK`
        // registers, and it uses a shared reference to the register block. See
        // the notes on `Token` types and memory safety in the root of the
        // `clock` module for more details.
        unsafe { &*Peripheral::PTR }
    }

    #[inline]
    fn apbamask(&mut self) -> &Apbamask {
        self.mclk().apbamask()
    }

    #[inline]
    fn apbbmask(&mut self) -> &Apbbmask {
        self.mclk().apbbmask()
    }

    #[inline]
    fn apbcmask(&mut self) -> &Apbcmask {
        self.mclk().apbcmask()
    }

    #[inline]
    #[hal_cfg("clock-d5x")]
    fn apbdmask(&mut self) -> &Apbdmask {
        self.mclk().apbdmask()
    }

    #[inline]
    #[hal_macro_helper]
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
                #[hal_cfg("clock-d5x")]
                ApbMask::D(mask) => {
                    self.apbdmask()
                        .modify(|r, w| w.bits(r.bits() | mask.bits()));
                }
            }
        }
    }

    #[inline]
    #[hal_macro_helper]
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
                #[hal_cfg("clock-d5x")]
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
#[hal_macro_helper]
enum ApbMask {
    A(ApbAMask),
    B(ApbBMask),
    C(ApbCMask),
    #[hal_cfg("clock-d5x")]
    D(ApbDMask),
}

/// Define several APB-related types
///
/// Define the [`DynApbId`], `ApbXMask`, [`ApbTokens`] and [`ApbClks`] types.
///
/// This macro takes a parameter `enabled_at_reset` which is a boolean.
/// If the clock is enabled at reset, then the clock is placed into the
/// [`ApbClks`] struct, otherwise, the clock APB type is placed as a token
/// into [`ApbTokens`]
macro_rules! define_apb_types {
    (
        $(
            $Reg:ident {
                $(
                    $( #[$( $cfg:tt )+] )?
                    $Type:ident = ($BIT:literal, $enabled_at_reset:literal)
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

            /// Set of [`ApbToken`]s for APB clocks that are disabled at power-on reset
            pub struct ApbTokens {
                $(
                    $(
                        $( #[$( $cfg )+] )?
                        #[cfg(not($enabled_at_reset))]
                        pub [<$Type:snake>]: ApbToken<$Type>,
                    )+
                )+
            }

            impl ApbTokens {
                /// Create the set of [`ApbToken`]s
                ///
                /// # Safety
                ///
                /// All invariants required by `ApbToken::new` must be upheld here as well.
                #[inline]
                pub(super) unsafe fn new() -> Self {
                    Self {
                        $(
                            $(
                                $( #[$( $cfg )+] )?
                                #[cfg(not($enabled_at_reset))]
                                [<$Type:snake>]: unsafe { ApbToken::new() },
                            )+
                        )+
                    }
                }
            }

            /// Set of [`ApbClk`]s for APB clocks that are enabled at power-on reset
            pub struct ApbClks {
                $(
                    $(
                        $( #[$( $cfg )+] )?
                        #[cfg($enabled_at_reset)]
                        pub [<$Type:snake>]: ApbClk<$Type>,
                    )+
                )+
            }

            impl ApbClks {
                /// Create the set of [`ApbClk`]s
                ///
                /// # Safety
                ///
                /// All invariants required by `ApbToken::new` must be upheld here as well.
                #[inline]
                pub(super) unsafe fn new() -> Self {
                    Self {
                        $(
                            $(
                                $( #[$( $cfg )+] )?
                                #[cfg($enabled_at_reset)]
                                [<$Type:snake>]: ApbClk::new( unsafe { ApbToken::new() } ),
                            )+
                        )+
                    }
                }
            }
        }
    };
}

// (N, all, any) => include in clocks not tokens = enabled at power-on
#[hal_macro_helper]
#[hal_cfg("clock-d5x")]
define_apb_types!(
    A {
        Pac0 = (0, true)
        Pm = (1, true)
        Mclk = (2, true)
        RstC = (3, true)
        OscCtrl = (4, true)
        Osc32kCtrl = (5, true)
        SupC = (6, true)
        Gclk = (7, true)
        Wdt = (8, true)
        Rtc = (9, true)
        Eic = (10, true)
        FreqM = (11, false)
        Sercom0 = (12, false)
        Sercom1 = (13, false)
        Tc0 = (14, false)
        Tc1 = (15, false)
    }
    B {
        Usb = (0, false)
        Dsu = (1, true)
        NvmCtrl = (2, true)
        Port = (4, true)
        EvSys = (7, false)
        Sercom2 = (9, false)
        Sercom3 = (10, false)
        Tcc0 = (11, false)
        Tcc1 = (12, false)
        Tc2 = (13, false)
        Tc3 = (14, false)
        RamEcc = (16, true)
    }
    C {
        #[hal_cfg("gmac")]
        Gmac = (2, false)
        Tcc2 = (3, false)
        #[hal_cfg("tcc3")]
        Tcc3 = (4, false)
        #[hal_cfg("tc4")]
        Tc4 = (5, false) // TODO double check this is correct
        #[hal_cfg("tc5")]
        Tc5 = (6, false)
        PDec = (7, false)
        Ac = (8, false)
        Aes = (9, false)
        Trng = (10, false)
        Icm = (11, false)
        Qspi = (13, true)
        Ccl = (14, false)
    }
    D {
        Sercom4 = (0, false)
        Sercom5 = (1, false)
        #[hal_cfg("sercom6")]
        Sercom6 = (2, false)
        #[hal_cfg("sercom7")]
        Sercom7 = (3, false)
        #[hal_cfg("tcc4")]
        Tcc4 = (4, false)
        #[hal_cfg("tc6")]
        Tc6 = (5, false)
        #[hal_cfg("tc7")]
        Tc7 = (6, false)
        Adc0 = (7, false)
        Adc1 = (8, false)
        Dac = (9, false)
        #[hal_cfg("i2s")]
        I2S = (10, false)
        Pcc = (11, false)
    }
);

// SAMD21/DA1 datasheet DS40001882H, Table 12-1. Peripherals Configuration
// Summary
#[hal_macro_helper]
#[hal_cfg("clock-d21")]
define_apb_types!(
    A {
        Pac0 = (0, true)
        Pm = (1, true)
        SysCtrl = (2, true)
        Gclk = (3, true)
        Wdt = (4, true)
        Rtc = (5, true)
        Eic = (6, true)
    }
    B {
        Pac1 = (0, true)
        Dsu = (1, true)
        NvmCtrl = (2, true)
        Port = (3, true)
        Dmac = (4, true)
        #[hal_cfg("usb")]
        Usb = (5, true)
    }
    C {
        Pac2 = (0, false)
        EvSys = (1, true)
        Sercom0 = (2, false)
        Sercom1 = (3, false)
        Sercom2 = (4, false)
        Sercom3 = (5, false)
        #[hal_cfg("sercom4")]
        Sercom4 = (6, false)
        #[hal_cfg("sercom5")]
        Sercom5 = (7, false)
        Tcc0 = (8, false)
        Tcc1 = (9, false)
        Tcc2 = (10, false)
        Tc3 = (11, false)
        Tc4 = (12, false)
        Tc5 = (13, false)
        Adc0 = (16, true)
        Ac = (17, false)
        Dac = (18, false)
        Ptc = (19, false)
        #[hal_cfg("i2s")]
        I2S = (20, false)
        Ac1 = (21, false)
        #[hal_cfg("tcc3")]
        Tcc3 = (22, false)
    }
);

// Atmel-42363H-SAM-D11-Datasheet_09/2016, Table 11-1. Peripherals Configuration
// Summary
#[hal_macro_helper]
#[hal_cfg("clock-d11")]
define_apb_types!(
    A {
        Pac0 = (0, true)
        Pm = (1, true)
        SysCtrl = (2, true)
        Gclk = (3, true)
        Wdt = (4, true)
        Rtc = (5, true)
        Eic = (6, true)
    }
    B {
        Pac1 = (0, true)
        Dsu = (1, true)
        NvmCtrl = (2, true)
        Port = (3, true)
        Dmac = (4, true)
        #[hal_cfg("usb")]
        Usb = (5, true)
    }
    C {
        Pac2 = (0, false)
        EvSys = (1, true)
        Sercom0 = (2, false)
        Sercom1 = (3, false)
        #[hal_cfg("sercom2")]
        Sercom2 = (4, false)
        Tcc0 = (5, false)
        Tc1 = (6, false)
        Tc2 = (7, false)
        Adc0 = (8, true)
        Ac = (9, false)
        Dac = (10, false)
        Ptc = (11, false)
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
