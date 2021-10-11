//! RTC - Real Time Clock
//!
//! This module contains a thumbv7em-specific wrapper type for
//! [`crate::rtc::Rtc`] that allows to handle a signal source for [`Rtc`] safely
//! in a context of Clocking API v2.
//!
//! Unsafe counterpart is also available as a set of standalone functions.
//!
//! Note for maintainers:
//! Preferably, [`Rtc`] implementation should be moved here since it is so
//! integrated with OSC32KCTRL.

use core::marker::PhantomData;
use core::ops::Deref;
use core::ops::DerefMut;

use crate::clock::v2::types::{Decrement, Increment};
use crate::pac::osc32kctrl::rtcctrl::RTCSEL_A;
use crate::pac::OSC32KCTRL;
use crate::pac::{MCLK, RTC};
use crate::rtc::{ClockMode, Count32Mode, Rtc as InnerRtc, RtcMode};
use crate::time::U32Ext;
use crate::typelevel::Sealed;

/// Wrapper type for [`crate::rtc::Rtc`]
///
/// Its construction functions allow to handle Rtc signal sources safely and
/// soundly in a context of Clocking API v2
///
/// It is generic over:
/// - a mode (already managed by underlying [`crate::rtc::Rtc`])
/// - a signal source frequency (either [`32 kHz`](Freq32k) or [`1
///   kHz`](Freq1k))
/// - a signal source (implementors of [`RtcSourceMarker`])
pub struct Rtc<M: RtcMode, F: FreqMode, T: RtcSourceMarker> {
    rtc: InnerRtc<M>,
    f: PhantomData<F>,
    t: PhantomData<T>,
}

/// Trait generalizing over source signal frequencies that are allowed by an
/// [`Rtc`]
pub trait FreqMode: Sealed {}

/// Type-state representing an [`Rtc`] running on a 32 kHz signal source
pub enum Freq32k {}

impl Sealed for Freq32k {}
impl FreqMode for Freq32k {}

/// Type-state representing an [`Rtc`] running on a 1 kHz signal source
pub enum Freq1k {}

impl Sealed for Freq1k {}
impl FreqMode for Freq1k {}

impl<T: RtcSourceMarker> Rtc<ClockMode, Freq32k, T> {
    /// Type-safe variant of [`crate::rtc::Rtc::clock_mode`] function setting
    /// [`Rtc`] up to run on 32 kHz signal source
    #[inline]
    pub fn clock_mode_from_32k_source<S: RtcSource32k<Type = T> + Increment>(
        rtc: RTC,
        rtc_signal_source: S,
        mclk: &mut MCLK,
    ) -> (Self, S::Inc) {
        (
            Self {
                rtc: InnerRtc::clock_mode(rtc, 32.khz().into(), mclk),
                f: PhantomData,
                t: PhantomData,
            },
            unsafe { set_rtc_clock_32k(rtc_signal_source) },
        )
    }
}

impl<T: RtcSourceMarker> Rtc<ClockMode, Freq1k, T> {
    /// Type-safe variant of [`crate::rtc::Rtc::clock_mode`] function setting
    /// [`Rtc`] up to run on 1 kHz signal source
    #[inline]
    pub fn clock_mode_from_1k_source<S: RtcSource1k<Type = T> + Increment>(
        rtc: RTC,
        rtc_signal_source: S,
        mclk: &mut MCLK,
    ) -> (Self, S::Inc) {
        (
            Self {
                rtc: InnerRtc::clock_mode(rtc, 1.khz().into(), mclk),
                f: PhantomData,
                t: PhantomData,
            },
            unsafe { set_rtc_clock_1k(rtc_signal_source) },
        )
    }
}

impl<T: RtcSourceMarker> Rtc<Count32Mode, Freq32k, T> {
    /// Type-safe variant of [`crate::rtc::Rtc::count32_mode`] function setting
    /// [`Rtc`] up to run on 32 kHz signal source
    #[inline]
    pub fn count32_mode_from_32k_source<S: RtcSource32k<Type = T> + Increment>(
        rtc: RTC,
        rtc_signal_source: S,
        mclk: &mut MCLK,
    ) -> (Self, S::Inc) {
        (
            Self {
                rtc: InnerRtc::count32_mode(rtc, 32.khz().into(), mclk),
                f: PhantomData,
                t: PhantomData,
            },
            unsafe { set_rtc_clock_32k(rtc_signal_source) },
        )
    }
}

impl<T: RtcSourceMarker> Rtc<Count32Mode, Freq1k, T> {
    /// Type-safe variant of [`crate::rtc::Rtc::count32_mode`] function setting
    /// [`Rtc`] up to run on 1 kHz signal source
    #[inline]
    pub fn count32_mode_from_1k_source<S: RtcSource1k<Type = T> + Increment>(
        rtc: RTC,
        rtc_signal_source: S,
        mclk: &mut MCLK,
    ) -> (Self, S::Inc) {
        (
            Self {
                rtc: InnerRtc::count32_mode(rtc, 1.khz().into(), mclk),
                f: PhantomData,
                t: PhantomData,
            },
            unsafe { set_rtc_clock_1k(rtc_signal_source) },
        )
    }
}

impl<M: RtcMode, F: FreqMode, T: RtcSourceMarker> Rtc<M, F, T> {
    /// Deconstruct the [`Rtc`] and reset the signal source configuration to
    /// default values
    #[inline]
    pub fn free<S: RtcSource<Type = T> + Decrement>(self, rtc_signal_source: S) -> (RTC, S::Dec) {
        (self.rtc.free(), unsafe {
            unset_rtc_clock(rtc_signal_source)
        })
    }
}

impl<M: RtcMode, F: FreqMode, T: RtcSourceMarker> Deref for Rtc<M, F, T> {
    type Target = InnerRtc<M>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.rtc
    }
}

impl<M: RtcMode, F: FreqMode, T: RtcSourceMarker> DerefMut for Rtc<M, F, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rtc
    }
}

/// Configure the [`RTC`] to use the 32 kHz signal that is being generated by
/// the source passed in as an argument
#[inline]
pub unsafe fn set_rtc_clock_32k<S>(source: S) -> S::Inc
where
    S: RtcSource32k + Increment,
{
    use crate::pac::osc32kctrl::RegisterBlock;
    let osc32kctrl = OSC32KCTRL::ptr() as *mut RegisterBlock;
    (*osc32kctrl)
        .rtcctrl
        .write(|w| w.rtcsel().variant(S::RTC_SRC_32K));
    source.inc()
}

/// Configure the [`RTC`] to use the 1 kHz signal that is being generated by
/// the source passed in as an argument
#[inline]
pub unsafe fn set_rtc_clock_1k<S>(source: S) -> S::Inc
where
    S: RtcSource1k + Increment,
{
    use crate::pac::osc32kctrl::RegisterBlock;
    let osc32kctrl = OSC32KCTRL::ptr() as *mut RegisterBlock;
    (*osc32kctrl)
        .rtcctrl
        .write(|w| w.rtcsel().variant(S::RTC_SRC_1K));
    source.inc()
}

/// Reset the [`RTC`] signal source to the default value
#[inline]
pub unsafe fn unset_rtc_clock<S>(source: S) -> S::Dec
where
    S: RtcSource + Decrement,
{
    use crate::pac::osc32kctrl::RegisterBlock;
    let osc32kctrl = OSC32KCTRL::ptr() as *mut RegisterBlock;
    (*osc32kctrl).rtcctrl.reset();
    source.dec()
}

//==============================================================================
// RtcSource
//==============================================================================

/// Trait implemented by all clocking abstractions that can feed an [`Rtc`] with
/// a clock signal
pub trait RtcSource: Sealed {
    /// A marker type corresponding to the source type implementing this trait
    type Type: RtcSourceMarker;
}

/// Trait implemented by markers of all clocking abstractions that can feed an
/// [`Rtc`] with a clock signal
pub trait RtcSourceMarker {}

/// Trait implemented by all clocking abstractions that can feed an [`Rtc`] with
/// a 32 kHz clock signal
pub trait RtcSource32k: RtcSource {
    /// Associated constant containing a low-level enum variant used during a HW
    /// write which sets up an [`Rtc's`](Rtc) signal source
    const RTC_SRC_32K: RTCSEL_A;
}

/// Trait implemented by all clocking abstractions that can feed an [`Rtc`] with
/// a 1 kHz clock signal
pub trait RtcSource1k: RtcSource {
    /// Associated constant containing a low-level enum variant used during a HW
    /// write which sets up an [`Rtc's`](Rtc) signal source
    const RTC_SRC_1K: RTCSEL_A;
}

/// Trait generalizing over the output state for the 32 kHz clock signal
pub trait Output32k: Sealed {}

/// Trait generalizing over the output state for the 1 kHz clock signal
pub trait Output1k: Sealed {}

/// Type-state representing the active output state for the 32 kHz clock signal
pub enum Active32k {}

impl Sealed for Active32k {}
impl Output32k for Active32k {}

/// Type-state representing the active output state for the 1 kHz clock signal
pub enum Active1k {}

impl Sealed for Active1k {}
impl Output1k for Active1k {}

/// Type-state representing the inactive output state for the 32 kHz clock
/// signal
pub enum Inactive32k {}

impl Sealed for Inactive32k {}
impl Output32k for Inactive32k {}

/// Type-state representing the inactive output state for the 1 kHz clock signal
pub enum Inactive1k {}

impl Sealed for Inactive1k {}
impl Output1k for Inactive1k {}
