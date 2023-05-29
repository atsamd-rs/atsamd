//! RTC oscillator
//!
//! This module provides a representation of the RTC oscillator, which
//! can be selected from one of four possible options. The [`RtcOsc`] type
//! represents proof that the RTC oscillator has been properly configured and
//! that its clock source has been locked and cannot be modified.
//!
//! For the moment, the [`RtcOsc`] is not used anywhere else in the HAL. A
//! future change to the [`rtc`](crate::rtc) module should either take ownership
//! of the [`RtcOsc`] at creation of the [`Rtc`] or replace the [`RtcOsc`]
//! entirely with an integrated [`Rtc`] API.
//!
//! # Example
//!
//! To create the [`RtcOsc`] let's start by using [`clock_system_at_reset`] to
//! access the HAL clocking structs.
//!
//! ```no_run
//! use atsamd_hal::{
//!     pac::Peripherals,
//!     thumbv7em::clock::v2::{clock_system_at_reset, osculp32k::OscUlp32k, rtcosc::RtcOsc},
//! };
//! let mut pac = Peripherals::take().unwrap();
//! let (buses, clocks, tokens) = clock_system_at_reset(
//!     pac.OSCCTRL,
//!     pac.OSC32KCTRL,
//!     pac.GCLK,
//!     pac.MCLK,
//!     &mut pac.NVMCTRL,
//! );
//! ```
//!
//! Next, let's enable the 32 kHz output of the internal, ultra-low power
//! oscillator.
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     pac::Peripherals,
//! #     thumbv7em::clock::v2::{clock_system_at_reset, osculp32k::OscUlp32k, rtcosc::RtcOsc},
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! let (osculp32k, base) = OscUlp32k::enable(tokens.osculp32k.osculp32k, clocks.osculp32k_base);
//! ```
//!
//! Finally, we can use the [`EnabledOscUlp32k`] to enabled the [`RtcOsc`].
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     pac::Peripherals,
//! #     thumbv7em::clock::v2::{clock_system_at_reset, osculp32k::OscUlp32k, rtcosc::RtcOsc},
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! # let (osculp32k, base) = OscUlp32k::enable(tokens.osculp32k.osculp32k, clocks.osculp32k_base);
//! let (rtc_osc, osculp32k) = RtcOsc::enable(tokens.rtcosc, osculp32k);
//! ```
//!
//! The entire example is provided below.
//!
//! ```no_run
//! use atsamd_hal::{
//!     pac::Peripherals,
//!     thumbv7em::clock::v2::{clock_system_at_reset, osculp32k::OscUlp32k, rtcosc::RtcOsc},
//! };
//! let mut pac = Peripherals::take().unwrap();
//! let (buses, clocks, tokens) = clock_system_at_reset(
//!     pac.OSCCTRL,
//!     pac.OSC32KCTRL,
//!     pac.GCLK,
//!     pac.MCLK,
//!     &mut pac.NVMCTRL,
//! );
//! let (osculp32k, base) = OscUlp32k::enable(tokens.osculp32k.osculp32k, clocks.osculp32k_base);
//! let (rtc_osc, osculp32k) = RtcOsc::enable(tokens.rtcosc, osculp32k);
//! ```
//!
//! [`clock_system_at_reset`]: super::clock_system_at_reset
//! [`Rtc`]: crate::rtc::Rtc
//! [`EnabledOscUlp32k`]: super::osculp32k::EnabledOscUlp32k

use core::marker::PhantomData;

use crate::pac::osc32kctrl::rtcctrl::RTCSEL_A;
use crate::pac::osc32kctrl::RTCCTRL;
use crate::pac::OSC32KCTRL;

use crate::time::Hertz;
use crate::typelevel::{Decrement, Increment};

use super::osculp32k::{OscUlp1kId, OscUlp32kId};
use super::xosc32k::{Xosc1kId, Xosc32kId};
use super::Source;

//==============================================================================
// RtcOscToken
//==============================================================================

/// Token used to retrieve the RTC oscillator
/// Singleton token that can be exchanged for the [`RtcOsc`]
///
/// As explained in the [`clock` module documentation](super), instances of
/// various `Token` types can be exchanged for actual clock types. They
/// typically represent clocks that are disabled at power-on reset.
///
/// To use the [`RtcOsc`], you must first exchange this token with the
/// [`RtcOsc::enable`] function.
pub struct RtcOscToken(());

impl RtcOscToken {
    /// Create a new instance of [`RtcOscToken`]
    ///
    /// # Safety
    ///
    /// The `RtcOscToken` is a singleton. There must never be two simulatenous
    /// instances of it. See the notes on `Token` types and memory safety in the
    /// root of the `clock` module for more details.
    #[inline]
    pub(super) unsafe fn new() -> Self {
        Self(())
    }

    #[inline]
    fn rtcctrl(&self) -> &RTCCTRL {
        // Safety: The `RtcOsc` only has exclusive access to the RTCCTRL
        // register. See the notes on `Token` types and memory safety in the
        // root of the `clock` module for more details.
        unsafe { &(*OSC32KCTRL::PTR).rtcctrl }
    }

    #[inline]
    fn set_source(&mut self, source: DynRtcSourceId) {
        self.rtcctrl().write(|w| w.rtcsel().variant(source.into()));
    }
}

//==============================================================================
// DynRtcSourceId
//==============================================================================

/// Value-level enum of possible clock sources for the [`RtcOsc`]
///
/// The variants of this enum identify one of four possible clock sources for
/// the [`RtcOsc`].
///
/// `DynRtcSourceId` is the value-level equivalent of [`RtcSourceId`].
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DynRtcSourceId {
    /// The RTC is sourced from the [`OscUlp1k`](super::osculp32k::OscUlp1k)
    OscUlp1k,
    /// The RTC is sourced from the [`OscUlp32k`](super::osculp32k::OscUlp32k)
    OscUlp32k,
    /// The RTC is sourced from the [`Xosc1k`](super::xosc32k::Xosc1k)
    Xosc1k,
    /// The RTC is sourced from the [`Xosc32k`](super::xosc32k::Xosc32k)
    Xosc32k,
}

impl From<DynRtcSourceId> for RTCSEL_A {
    #[inline]
    fn from(source: DynRtcSourceId) -> Self {
        use DynRtcSourceId::*;
        use RTCSEL_A::*;
        match source {
            OscUlp1k => ULP1K,
            OscUlp32k => ULP32K,
            Xosc1k => XOSC1K,
            Xosc32k => XOSC32K,
        }
    }
}

//==============================================================================
// RtcSourceId
//==============================================================================

/// Type-level `enum` for the RTC oscillator clock source
///
/// The RTC can be sourced from any of [`OscUlp1k`](super::osculp32k::OscUlp1k),
/// [`OscUlp32k`](super::osculp32k::OscUlp32k),
/// [`Xosc1k`](super::xosc32k::Xosc1k) or [`Xosc32k`](super::xosc32k::Xosc32k).
///
/// See the [type-level enum] documentation for more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
/// Type-level enum of possible clock [`Source`]s for the [`RtcOsc`]
///
/// The types implementing this trait are type-level variants of `RtcSourceId`,
/// and they identify one of four possible clock [`Source`]s for the [`RtcOsc`].
/// All implementers of this trait are [`Id` types](super#id-types), which are
/// described in more detail in the [`clock` module documentation](super).
///
/// `RtcSourceId` is the type-level equivalent of [`DynRtcSourceId`]. See the
/// documentation on [type-level programming] and specifically
/// [type-level enums] for more details.
///
/// [type-level programming]: crate::typelevel
/// [type-level enums]: crate::typelevel#type-level-enums
pub trait RtcSourceId {
    /// Corresponding [`DynRtcSourceId`]
    const DYN: DynRtcSourceId;
    /// Clock source frequency, either 1 kHz or 32 kHz
    const FREQ: Hertz;
}

impl RtcSourceId for OscUlp1kId {
    const DYN: DynRtcSourceId = DynRtcSourceId::OscUlp1k;
    const FREQ: Hertz = Hertz::Hz(1024);
}
impl RtcSourceId for OscUlp32kId {
    const DYN: DynRtcSourceId = DynRtcSourceId::OscUlp32k;
    const FREQ: Hertz = Hertz::Hz(32_768);
}
impl RtcSourceId for Xosc1kId {
    const DYN: DynRtcSourceId = DynRtcSourceId::Xosc1k;
    const FREQ: Hertz = Hertz::Hz(1024);
}
impl RtcSourceId for Xosc32kId {
    const DYN: DynRtcSourceId = DynRtcSourceId::Xosc32k;
    const FREQ: Hertz = Hertz::Hz(32_768);
}

//==============================================================================
// RtcOsc
//==============================================================================

/// Oscillator for the [`Rtc`]
///
/// The `RtcOsc` represents proof that a clock source for the [`Rtc`] has been
/// selected and configured. It also guarantees that the clock source for the
/// RTC will not be modified or disabled while it is in use.
///
/// See the [module-level documentation](self) for an example of creating the
/// `RtcOsc`.
///
/// [`Rtc`]: crate::rtc::Rtc
pub struct RtcOsc<I: RtcSourceId> {
    token: RtcOscToken,
    source: PhantomData<I>,
}

impl<I: RtcSourceId> RtcOsc<I> {
    /// Consume the [`RtcOscToken`] and return the [`RtcOsc`]
    ///
    /// Enabling the [`RtcOsc`] will [`Increment`] the consumer count of its
    /// [`Enabled`](super::Enabled) clock [`Source`].
    #[inline]
    pub fn enable<S>(mut token: RtcOscToken, source: S) -> (Self, S::Inc)
    where
        S: Source<Id = I> + Increment,
    {
        token.set_source(I::DYN);
        let rtc_osc = Self {
            token,
            source: PhantomData,
        };
        (rtc_osc, source.inc())
    }

    /// Consume the [`RtcOsc`] and return the [`RtcOscToken`]
    ///
    /// Disabling the `RtcOsc` will [`Decrement`] the consumer count of its
    /// [`Enabled`](super::Enabled) clock [`Source`].
    #[inline]
    pub fn disable<S>(self, source: S) -> (RtcOscToken, S::Dec)
    where
        S: Source<Id = I> + Decrement,
    {
        (self.token, source.dec())
    }

    /// Return the [`RtcOsc`] frequency, which can either be 1 kHz or 32 kHz
    #[inline]
    pub fn freq(&self) -> Hertz {
        I::FREQ
    }
}
