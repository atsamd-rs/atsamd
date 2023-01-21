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
//!     pac.OSCCTRL,
//!     pac.OSC32KCTRL,
//!     pac.GCLK,
//!     pac.MCLK,
//!     &mut pac.NVMCTRL,
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
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
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
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
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
//!     pac.OSCCTRL,
//!     pac.OSC32KCTRL,
//!     pac.GCLK,
//!     pac.MCLK,
//!     &mut pac.NVMCTRL,
//! );
//! let apb_port = clocks.apbs.port;
//! let apb_sercom0 = buses.apb.enable(tokens.apbs.sercom0);
//! ```
//!
//! [`clock` module documentation]: super
//! [`clock_system_at_reset`]: super::clock_system_at_reset
//! [`Clocks`]: super::Clocks
//! [`Buses`]: super::Buses

use core::marker::PhantomData;

use bitflags;
use paste::paste;

#[cfg(feature = "has-mclk-oscctrl")]
mod imports {
    pub use crate::pac::mclk::{RegisterBlock as BLOCK, APBAMASK, APBBMASK, APBCMASK, APBDMASK};
    pub use crate::pac::MCLK as PERIPHERAL;
}

#[cfg(feature = "has-sysctrl")]
mod imports {
    pub use crate::pac::pm::{RegisterBlock as BLOCK, APBAMASK, APBBMASK, APBCMASK};
    pub use crate::pac::PM as PERIPHERAL;
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
        unsafe { &*PERIPHERAL::PTR }
    }

    #[inline]
    fn apbamask(&mut self) -> &APBAMASK {
        &self.mclk().apbamask
    }

    #[inline]
    fn apbbmask(&mut self) -> &APBBMASK {
        &self.mclk().apbbmask
    }

    #[inline]
    fn apbcmask(&mut self) -> &APBCMASK {
        &self.mclk().apbcmask
    }

    #[inline]
    #[cfg(feature = "has-apbd")]
    fn apbdmask(&mut self) -> &APBDMASK {
        &self.mclk().apbdmask
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
                #[cfg(feature = "has-apbd")]
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
                #[cfg(feature = "has-apbd")]
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
    #[cfg(feature = "has-apbd")]
    D(ApbDMask),
}

/// Define several APB-related types
///
/// Define the [`DynApbId`], `ApbXMask`, [`ApbTokens`] and [`ApbClks`] types.
///
/// This macro uses a slight hack to simplify its implementation. It uses
/// `#[cfg(all())]` and `#[cfg(any())]` to represent `#[cfg(true)]` and
/// `#[cfg(false)]`, respectively. We can use this to selectively place each
/// APB type into the [`ApbTokens`] struct or the [`ApbClks`] struct, depending
/// on whether or not the corresponding bit is enabled at power-on reset.
macro_rules! define_apb_types {
    (
        $(
            $Reg:ident {
                $(
                    $( #[$( $cfg:tt )+] )?
                    $Type:ident = ($BIT:literal, $token:ident, $clk:ident)
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
                        #[cfg($token())]
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
                                #[cfg($token())]
                                [<$Type:snake>]: ApbToken::new(),
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
                        #[cfg($clk())]
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
                                #[cfg($clk())]
                                [<$Type:snake>]: ApbClk::new(ApbToken::new()),
                            )+
                        )+
                    }
                }
            }
        }
    };
}

#[cfg(feature = "samd5xe5x")]
define_apb_types!(
    A {
        Pac0 = (0, all, any)
        Pm = (1, all, any)
        Mclk = (2, all, any)
        RstC = (3, all, any)
        OscCtrl = (4, all, any)
        Osc32kCtrl = (5, all, any)
        SupC = (6, all, any)
        Gclk = (7, all, any)
        Wdt = (8, all, any)
        Rtc = (9, all, any)
        Eic = (10, all, any)
        FreqM = (11, any, all)
        Sercom0 = (12, any, all)
        Sercom1 = (13, any, all)
        Tc0 = (14, any, all)
        Tc1 = (15, any, all)
    }
    B {
        Usb = (0, any, all)
        Dsu = (1, all, any)
        NvmCtrl = (2, all, any)
        Port = (4, all, any)
        EvSys = (7, any, all)
        Sercom2 = (9, any, all)
        Sercom3 = (10, any, all)
        Tcc0 = (11, any, all)
        Tcc1 = (12, any, all)
        Tc2 = (13, any, all)
        Tc3 = (14, any, all)
        RamEcc = (16, all, any)
    }
    C {
        #[cfg(feature = "has-gmac")]
        Gmac = (2, all, any)
        Tcc2 = (3, any, all)
        #[cfg(feature = "has-tcc3")]
        Tcc3 = (4, any, all)
        #[cfg(feature = "has-tc4")]
        Tc4 = (5, all, any)
        #[cfg(feature = "has-tc5")]
        Tc5 = (6, any, all)
        PDec = (7, any, all)
        Ac = (8, any, all)
        Aes = (9, any, all)
        Trng = (10, any, all)
        Icm = (11, any, all)
        Qspi = (13, all, any)
        Ccl = (14, any, all)
    }
    D {
        Sercom4 = (0, any, all)
        Sercom5 = (1, any, all)
        #[cfg(feature = "has-sercom6")]
        Sercom6 = (2, any, all)
        #[cfg(feature = "has-sercom7")]
        Sercom7 = (3, any, all)
        #[cfg(feature = "has-tcc4")]
        Tcc4 = (4, any, all)
        #[cfg(feature = "has-tc6")]
        Tc6 = (5, any, all)
        #[cfg(feature = "has-tc7")]
        Tc7 = (6, any, all)
        Adc0 = (7, any, all)
        Adc1 = (8, any, all)
        Dac = (9, any, all)
        #[cfg(feature = "has-i2s")]
        I2S = (10, any, all)
        Pcc = (11, any, all)
    }
);

#[cfg(feature = "samd21")]
define_apb_types!(
    A {
        Pac0 = (0, all, any)
        Pm = (1, all, any)
        SysCtrl = (2, all, any)
        Gclk = (3, all, any)
        Wdt = (4, all, any)
        Rtc = (5, all, any)
        Eic = (6, all, any)
    }
    B {
        Pac1 = (0, all, any)
        Dsu = (1, all, any)
        NvmCtrl = (2, all, any)
        Port = (3, all, any)
        Dmac = (4, all, any)
        Usb = (5, all, any)
    }
    C {
        Pac2 = (0, any, all)
        EvSys = (1, any, all)
        Sercom0 = (2, any, all)
        Sercom1 = (3, any, all)
        Sercom2 = (4, any, all)
        Sercom3 = (5, any, all)
        #[cfg(feature = "has-sercom4")]
        Sercom4 = (6, any, all)
        #[cfg(feature = "has-sercom5")]
        Sercom5 = (7, any, all)
        Tcc0 = (8, any, all)
        Tcc1 = (9, any, all)
        Tcc2 = (10, any, all)
        Tc3 = (11, any, all)
        Tc4 = (12, any, all)
        Tc5 = (13, any, all)
        #[cfg(feature = "has-tc6")]
        Tc6 = (14, any, all)
        #[cfg(feature = "has-tc7")]
        Tc7 = (15, any, all)
        Adc0 = (16, all, any)
        Ac = (17, any, all)
        Dac = (18, any, all)
        Ptc = (19, any, all)
        I2S = (20, any, all)
        Ac1 = (21, any, all)
    }
);

#[cfg(feature = "samd11")]
define_apb_types!(
    A {
        Pac0 = (0, all, any)
        Pm = (1, all, any)
        SysCtrl = (2, all, any)
        Gclk = (3, all, any)
        Wdt = (4, all, any)
        Rtc = (5, all, any)
        Eic = (6, all, any)
    }
    B {
        Pac1 = (0, all, any)
        Dsu = (1, all, any)
        NvmCtrl = (2, all, any)
        Port = (3, all, any)
        Dmac = (4, all, any)
        Usb = (5, all, any)
    }
    C {
        Pac2 = (0, any, all)
        EvSys = (1, any, all)
        Sercom0 = (2, any, all)
        Sercom1 = (3, any, all)
        #[cfg(feature = "has-sercom2")]
        Sercom2 = (4, any, all)
        Tcc0 = (5, any, all)
        Tc1 = (6, any, all)
        Tc2 = (7, any, all)
        Adc0 = (8, all, any)
        Ac = (9, any, all)
        Dac = (10, any, all)
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
