//! # Advanced high performance bus clocks
//!
//! ## Overview
//!
//! AHB clocks facilitate communication between the processor core and
//! peripherals on the AHB bus. To communicate with a peripheral, the
//! corresponding AHB clock must be enabled, which is done by setting a bit in
//! the `AHBMASK` register.
//!
//! In this module, *enabled* AHB clocks are represented by the [`AhbClk<A>`]
//! struct, where the type parameter `A` is a type that implements [`AhbId`] and
//! corresponds to one of the bits in the `AHBMASK` register.
//!
//! While most other clocks in the `clock` module are configured through
//! mutually exclusive registers, the [`AhbClk`]s share a single `AHBMASK`
//! register. This presents a challenge for memory safety. Specifically, if we
//! allowed unrestricted access to the `AHBMASK` register through each `AhbClk`,
//! we could create data races.
//!
//! To solve this problem, we restrict access to the `AHBMASK` register using
//! the [`Ahb`] type. `Ahb` was created to act as a gateway to the `AHBMASK`
//! register, allowing us to use `&mut Ahb` as compile-time proof of exclusive
//! access to it.
//!
//! ## Example
//!
//! Enabling and disabling the [`AhbClk`]s proceeds according to the principles
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
//! All AHB clocks are enabled at power-on reset. We can find them in the
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
//! let ahb_qspi = clocks.ahbs.qspi;
//! ```
//!
//! To disable an `AhbClk`, we must have access to the [`Ahb`] bus type, which
//! is found in the [`Buses`] struct. As described above, [`Ahb`] mediates
//! access to the shared `AHBMASK` register. We call [`Ahb::disable`] to convert
//! an [`AhbClk`] into the corresponding [`AhbToken`].
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
//! # let ahb_qspi = clocks.ahbs.qspi;
//! let ahb_qspi = buses.ahb.disable(ahb_qspi);
//! ```
//!
//! To reenable an `AhbClk`, users must save the `AhbToken` and use it when
//! calling [`Ahb::enable`].
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
//! let ahb_qspi = clocks.ahbs.qspi;
//! let ahb_qspi = buses.ahb.disable(ahb_qspi);
//! ```
//!
//! [`clock` module documentation]: super
//! [`clock_system_at_reset`]: super::clock_system_at_reset
//! [`Clocks`]: super::Clocks
//! [`Buses`]: super::Buses

use core::marker::PhantomData;

use bitflags;
use paste::paste;

use crate::pac::{mclk, MCLK};

use super::types::*;

//==============================================================================
// Ahb
//==============================================================================

/// AHB clock controller
///
/// As described in the [module-level documentation](self), this struct mediates
/// access to the shared `AHBMASK` register. Users can convert a disabled
/// [`AhbToken<A>`] into an enabled [`AhbClk<A>`] using [`Ahb::enable`], and
/// vice versa with [`Ahb::disable`].
pub struct Ahb(());

impl Ahb {
    /// Create a new instance of [`Ahb`]
    ///
    /// # Safety
    ///
    /// Because the `Ahb` mediates access to the `AHBMASK` register, it must be
    /// a singleton. There must never be two simulatenous instances of it at a
    /// time. See the notes on `Token` types and memory safety in the root of
    /// the `clock` module for more details.
    #[inline]
    pub(super) unsafe fn new() -> Self {
        Self(())
    }

    #[inline]
    fn ahbmask(&mut self) -> &mclk::AHBMASK {
        // Safety: The `Ahb` type has exclusive access to the `AHBMASK`
        // register. See the notes on `Token` types and memory safety in the
        // root of the `clock` module for more details.
        unsafe { &(*MCLK::PTR).ahbmask }
    }

    #[inline]
    fn enable_mask(&mut self, mask: AhbMask) {
        // Safety: The mask bits are derived from a `bitflags` struct, so they
        // are guaranteed to be valid.
        self.ahbmask()
            .modify(|r, w| unsafe { w.bits(r.bits() | mask.bits()) });
    }

    #[inline]
    fn disable_mask(&mut self, mask: AhbMask) {
        // Safety: The mask bits are derived from a `bitflags` struct, so they
        // are guaranteed to be valid.
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

/// Type-level enum identifying one of the possible AHB clocks
///
/// The types implementing this trait are type-level variants of `AhbId`, and
/// they identify one of the possible AHB clocks, which can vary by chip. Each
/// type corresponds to a specific bit in the `AHBMASK` register.
///
/// `AhbId` is the type-level equivalent of [`DynAhbId`]. See the documentation
/// on [type-level programming] and specifically [type-level enums] for more
/// details.
///
/// [type-level programming]: crate::typelevel
/// [type-level enums]: crate::typelevel#type-level-enums
pub trait AhbId: crate::typelevel::Sealed {
    /// Corresponding [`DynAhbId`]
    const DYN: DynAhbId;
}

//==============================================================================
// AhbToken
//==============================================================================

/// Singleton token that can be exchanged for an [`AhbClk`]
///
/// As explained in the [`clock` module documentation](super), instances of
/// various `Token` types can be exchanged for actual clock types. They
/// represent clocks that are disabled.
///
/// The type parameter `A` is an [`AhbId`] indicating which AHB clock is
/// represented by this token. To enable the corresponding AHB clock, use the
/// [`Ahb::enable`] method.
pub struct AhbToken<A: AhbId> {
    id: PhantomData<A>,
}

impl<A: AhbId> AhbToken<A> {
    /// Create a new instance of [`AhbToken`]
    ///
    /// # Safety
    ///
    /// Each `AhbToken` is a singleton. There must never be two simulatenous
    /// instances with the same [`AhbId`]. See the notes on `Token` types and
    /// memory safety in the root of the `clock` module for more details.
    #[inline]
    unsafe fn new() -> Self {
        AhbToken { id: PhantomData }
    }
}

//==============================================================================
// AhbClk
//==============================================================================

/// An enabled AHB clock
///
/// An [`AhbClk`] represents an enabled AHB clock. The type parameter `A` is an
/// [`AhbId`], which corresponds to a particular bit in the `AHBMASK`
/// register. An `AhbClk` can be disabled with the [`Ahb::disable`] method.
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

macro_rules! define_ahb_types {
    (
        $(
            $( #[$( $cfg:tt )+] )?
            $Type:ident = $BIT:literal,
        )+
    ) => {
        paste! {
            bitflags::bitflags! {
                /// AHB clock register mask
                ///
                /// This is a [`bitflags`] struct with a binary representation
                /// exactly matching the `AHBMASK` register.
                struct AhbMask: u32 {
                    $(
                        $( #[$( $cfg )+] )?
                        const [<$Type:upper>] = 1 << $BIT;
                    )+
                }
            }

            /// Value-level enum identifying a single AHB clock
            ///
            /// Each variant of this enum corresponds to a specific bit in the
            /// `AHBMASK` register and identifies one of the possible AHB
            /// clocks, which can vary by chip.
            ///
            /// `DynAhbId` is the value-level equivalent of [`AhbId`].
            #[repr(u8)]
            pub enum DynAhbId {
                $(
                    $( #[$( $cfg )+] )?
                    $Type = $BIT,
                )+
            }

            impl From<DynAhbId> for AhbMask {
                #[inline]
                fn from(id: DynAhbId) -> AhbMask {
                    match id {
                        $(
                            $( #[$( $cfg )+] )?
                            DynAhbId::$Type => AhbMask::[<$Type:upper>],
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

            /// Set of all [`AhbClk`]s
            ///
            /// All [`AhbClk`]s are enabled at power-on reset.
            pub struct AhbClks {
                $(
                    $( #[$( $cfg )+] )?
                    pub [<$Type:snake>]: AhbClk<$Type>,
                )+
            }
            impl AhbClks {
                /// Create the set of [`AhbClk`]s
                ///
                /// # Safety
                ///
                /// All invariants of `AhbToken::new` must be upheld here.
                #[inline]
                pub(super) unsafe fn new() -> Self {
                    AhbClks {
                        $(
                            $( #[$( $cfg )+] )?
                            [<$Type:snake>]: AhbClk::new(AhbToken::new()),
                        )+
                    }
                }
            }
        }
    };
}

define_ahb_types!(
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
    #[cfg(feature = "has-gmac")]
    Gmac = 14,
    Sdhc0 = 15,
    #[cfg(feature = "has-sdhc1")]
    Sdhc1 = 16,
    #[cfg(feature = "has-can0")]
    Can0 = 17,
    #[cfg(feature = "has-can1")]
    Can1 = 18,
    Icm = 19,
    Pukcc = 20,
    Qspi2x = 21,
    NvmCtrlSmeeProm = 22,
    NvmCtrlCache = 23,
);
