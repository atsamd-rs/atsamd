//! TODO

use core::marker::PhantomData;

use crate::pac::osc32kctrl::rtcctrl::RTCSEL_A;
use crate::pac::osc32kctrl::{RegisterBlock, RTCCTRL};
use crate::pac::OSC32KCTRL;

use crate::time::Hertz;
use crate::typelevel::{Decrement, Increment};

use super::osculp32k::{OscUlp1kId, OscUlp32kId};
use super::xosc32k::{Xosc1kId, Xosc32kId};
use super::Source;

//==============================================================================
// Registers
//==============================================================================

struct Registers;

impl Registers {
    const OSC32KCTRL: *const RegisterBlock = OSC32KCTRL::ptr();

    #[inline]
    fn rtcctrl(&self) -> &RTCCTRL {
        unsafe { &(*Self::OSC32KCTRL).rtcctrl }
    }

    #[inline]
    fn set_source(&mut self, source: DynRtcSourceId) {
        self.rtcctrl().write(|w| w.rtcsel().variant(source.into()));
    }

    fn reset_source(&mut self) {
        self.rtcctrl().reset();
    }
}

//==============================================================================
// RtcOscToken
//==============================================================================

/// Token used to retrieve the RTC oscillator
pub struct RtcOscToken {
    regs: Registers,
}

impl RtcOscToken {
    /// Create a new instance of the token
    ///
    /// Safety: There must never be more than one instance of this token at any
    /// given time.
    pub(super) unsafe fn new() -> Self {
        Self { regs: Registers }
    }
}

//==============================================================================
// RtcSourceId
//==============================================================================

/// Value-level version of [`RtcSourceId`]
///
/// Represents the source of the RTC oscillator
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

/// Type-level `enum` for the RTC oscillator clock source
///
/// The RTC can be sourced from any of [`OscUlp1k`](super::osculp32k::OscUlp1k),
/// [`OscUlp32k`](super::osculp32k::OscUlp32k),
/// [`Xosc1k`](super::xosc32k::Xosc1k) or [`Xosc32k`](super::xosc32k::Xosc32k).
///
/// See the [type-level enum] documentation for more details on the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub trait RtcSourceId {
    /// Corresponding [`DynSourceId`]
    const DYN: DynRtcSourceId;
    /// Clock source frequency, either 1 kHz or 32 kHz
    const FREQ: Hertz;
}

impl RtcSourceId for OscUlp1kId {
    const DYN: DynRtcSourceId = DynRtcSourceId::OscUlp1k;
    const FREQ: Hertz = Hertz(1024);
}
impl RtcSourceId for OscUlp32kId {
    const DYN: DynRtcSourceId = DynRtcSourceId::OscUlp32k;
    const FREQ: Hertz = Hertz(32_768);
}
impl RtcSourceId for Xosc1kId {
    const DYN: DynRtcSourceId = DynRtcSourceId::Xosc1k;
    const FREQ: Hertz = Hertz(1024);
}
impl RtcSourceId for Xosc32kId {
    const DYN: DynRtcSourceId = DynRtcSourceId::Xosc32k;
    const FREQ: Hertz = Hertz(32_768);
}

//==============================================================================
// RtcOsc
//==============================================================================

/// Oscillator for the [`Rtc`](crate::rtc::Rtc)
pub struct RtcOsc<I: RtcSourceId> {
    regs: Registers,
    source: PhantomData<I>,
}

impl<I: RtcSourceId> RtcOsc<I> {
    /// Consume the [`RtcOscToken`] and return the [`RtcOsc`]
    ///
    /// Enabling the `RtcOsc` will [`Increment`] the
    /// [`Counter`](crate::typelevel::Counter) of the
    /// [`Enabled`](super::Enabled) clock [`Source`].
    pub fn enable<S>(mut token: RtcOscToken, source: S) -> (Self, S::Inc)
    where
        S: Source<Id = I> + Increment,
    {
        token.regs.set_source(I::DYN);
        let rtc_osc = Self {
            regs: token.regs,
            source: PhantomData,
        };
        (rtc_osc, source.inc())
    }

    /// Consume the [`RtcOsc`] and return the [`RtcOscToken`]
    ///
    /// Disabling the `RtcOsc` will [`Decrement`] the
    /// [`Counter`](crate::typelevel::Counter) of its
    /// [`Enabled`](super::Enabled) clock [`Source`].
    pub fn disable<S>(mut self, source: S) -> (RtcOscToken, S::Dec)
    where
        S: Source<Id = I> + Decrement,
    {
        self.regs.reset_source();
        let token = RtcOscToken { regs: self.regs };
        (token, source.dec())
    }
}
