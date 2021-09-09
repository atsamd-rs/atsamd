#![deny(missing_docs)]
//! # Osculp32k - Ultra Low power 32 kHz oscillator
//!
//! Always-on internal oscillator capable of producing
//! 32 kHz and 1 kHz output.
//!
//! * 1 kHz signal is only accessible by specific peripherals (datasheet c.
//!   13.1)
//! * 32 kHz signal is a general purpose source that can be used by any
//! [`Gclk`](crate::clock::v2::gclk::Gclk).
//!
//! Independently controllable default-on outputs for 32 kHz and 1 kHz
//!
//! See:
//! * [`Enabled<Osculp32k>::activate_32k`]
//! * [`Enabled<Osculp32k>::activate_1k`]

use core::marker::PhantomData;
use typenum::U0;

use crate::pac::gclk::genctrl::SRC_A;
use crate::pac::osc32kctrl::rtcctrl::RTCSEL_A;
use crate::pac::osc32kctrl::OSCULP32K;

use crate::clock::types::{Counter, Enabled, PrivateIncrement};
use crate::clock::v2::{Source, SourceMarker};
use crate::time::{Hertz, U32Ext};
use crate::typelevel::Sealed;

use super::gclk::{GclkSource, GclkSourceMarker, GenNum};
use super::gclkio::NotGclkInput;
use super::rtc::*;

//==============================================================================
// OscUlp32kToken
//==============================================================================

/// Token struct that is essential in order to construct an instance of a
/// [`OscUlp32k`].
///
/// Irrelevant to the user as
/// * [`Enabled`]`<`[`OscUlp32k`]`>` is undisableable and undeconstructable
/// * [`Enabled`]`<`[`OscUlp32k`]`>` is already provided within a tuple as
///   return value from [`retrieve_clocks`](crate::clock::v2::retrieve_clocks)
///   and one does not have to create it
pub struct OscUlp32kToken {
    __: (),
}

impl OscUlp32kToken {
    /// Create a new instance of [`Xosc32kToken`]
    #[inline]
    pub(crate) unsafe fn new() -> Self {
        Self { __: () }
    }

    #[inline]
    fn osculp32k(&self) -> &OSCULP32K {
        unsafe { &(*crate::pac::OSC32KCTRL::ptr()).osculp32k }
    }

    #[inline]
    fn set_calib(&mut self, calib: u8) {
        self.osculp32k()
            .modify(|_, w| unsafe { w.calib().bits(calib & 0x3F) });
    }

    #[inline]
    fn activate_1k(&mut self, enabled: bool) {
        self.osculp32k().modify(|_, w| w.en1k().bit(enabled));
    }

    #[inline]
    fn activate_32k(&mut self, enabled: bool) {
        self.osculp32k().modify(|_, w| w.en32k().bit(enabled));
    }

    #[inline]
    fn wrtlock(&mut self) {
        self.osculp32k().modify(|_, w| w.wrtlock().bit(true));
    }
}

//==============================================================================
// OscUlp32k
//==============================================================================

/// Struct representing a disabled ultra low power oscillator
///
/// In reality, this oscillator is always enabled and
/// [`Enabled`]`<`[`OscUlp32k`]`>` does not have a `disable` method that would
/// allow to retreive underlying [`OscUlp32k`] struct.
///
/// User can only activate/deactivate outgoing signals (as in making them
/// visible outside of an oscillator).
/// Therefore [`Enabled`]`<`[`OscUlp32k`]`<`[`Inactive32k`]`, `[`Inactive1k`]`>,
/// _>` represents an **always** enabled oscillator with disabled outgoing
/// signals.
///
/// It is generic over:
/// - An output state of a 32 kHz signal (active/inactive)
/// - An output state of a 1 kHz signal (active/inactive)
pub struct OscUlp32k<X, Y>
where
    X: Output32k,
    Y: Output1k,
{
    token: OscUlp32kToken,
    output32k: PhantomData<X>,
    output1k: PhantomData<Y>,
}

impl<X, Y> OscUlp32k<X, Y>
where
    X: Output32k,
    Y: Output1k,
{
    /// Create a new instance of [`OscUlp32k`]
    #[inline]
    pub(crate) fn new(token: OscUlp32kToken) -> Self {
        Self {
            token,
            output32k: PhantomData,
            output1k: PhantomData,
        }
    }
}

impl<X, Y> Enabled<OscUlp32k<X, Y>, U0>
where
    X: Output32k,
    Y: Output1k,
{
    /// Set calibration parameters
    ///
    /// This data is used to compensate for process variations
    ///
    /// This method performs a HW register write. At startup, this register is
    /// populated with a default parametrization from Flash Calibration at
    /// startup
    #[inline]
    pub fn set_calibration(mut self, calib: u8) -> Self {
        self.0.token.set_calib(calib);
        self
    }
}

impl<Y> Enabled<OscUlp32k<Inactive32k, Y>, U0>
where
    Y: Output1k,
{
    /// Activate the 32 kHz signal output
    #[inline]
    pub fn activate_32k(mut self) -> Enabled<OscUlp32k<Active32k, Y>, U0> {
        self.0.token.activate_32k(true);
        let osculp32k = OscUlp32k {
            token: self.0.token,
            output32k: PhantomData,
            output1k: self.0.output1k,
        };
        Enabled::new(osculp32k)
    }
}

impl<Y> Enabled<OscUlp32k<Active32k, Y>, U0>
where
    Y: Output1k,
{
    /// Deactivate the 32 kHz signal output
    #[inline]
    pub fn deactivate_32k(mut self) -> Enabled<OscUlp32k<Inactive32k, Y>, U0> {
        self.0.token.activate_32k(false);
        let osculp32k = OscUlp32k {
            token: self.0.token,
            output32k: PhantomData,
            output1k: self.0.output1k,
        };
        Enabled::new(osculp32k)
    }
}

impl<X> Enabled<OscUlp32k<X, Inactive1k>, U0>
where
    X: Output32k,
{
    /// Activate the 1 kHz signal output
    #[inline]
    pub fn activate_1k(mut self) -> Enabled<OscUlp32k<X, Active1k>, U0> {
        self.0.token.activate_1k(true);
        let osculp32k = OscUlp32k {
            token: self.0.token,
            output32k: self.0.output32k,
            output1k: PhantomData,
        };
        Enabled::new(osculp32k)
    }
}

impl<X> Enabled<OscUlp32k<X, Active1k>, U0>
where
    X: Output32k,
{
    /// Deactivate the 1 kHz signal output
    #[inline]
    pub fn deactivate_1k(mut self) -> Enabled<OscUlp32k<X, Inactive1k>, U0> {
        self.0.token.activate_1k(false);
        let osculp32k = OscUlp32k {
            token: self.0.token,
            output32k: self.0.output32k,
            output1k: PhantomData,
        };
        Enabled::new(osculp32k)
    }
}

impl<X, Y, N> Enabled<OscUlp32k<X, Y>, N>
where
    X: Output32k,
    Y: Output1k,
    N: Counter + PrivateIncrement,
{
    /// Write lock the OscUlp32k
    ///
    /// Locked until a Power-On Reset (POR) is detected.
    ///
    /// TODO, how should we model the hardware write lock?
    /// For now artificially raise the use counter by 1
    #[inline]
    pub fn write_lock(mut self) -> <Self as PrivateIncrement>::Inc {
        self.0.token.wrtlock();
        self.inc()
    }
}

//==============================================================================
// GclkSource
//==============================================================================

/// A marker type. More information at [`SourceMarker`] documentation entry
pub enum Ulp32k {}

impl Sealed for Ulp32k {}

impl SourceMarker for Ulp32k {}

impl GclkSourceMarker for Ulp32k {
    const GCLK_SRC: SRC_A = SRC_A::OSCULP32K;
}

impl NotGclkInput for Ulp32k {}

impl<G, Y, N> GclkSource<G> for Enabled<OscUlp32k<Active32k, Y>, N>
where
    G: GenNum,
    Y: Output1k,
    N: Counter,
{
    type Type = Ulp32k;
}

impl<Y, N> Source for Enabled<OscUlp32k<Active32k, Y>, N>
where
    Y: Output1k,
    N: Counter,
{
    #[inline]
    fn freq(&self) -> Hertz {
        32768.hz()
    }
}

//==============================================================================
// RtcClock
//==============================================================================

impl<Y, N> RtcSource32k for Enabled<OscUlp32k<Active32k, Y>, N>
where
    Y: Output1k,
    N: Counter,
{
    const RTC_SRC_32K: RTCSEL_A = RTCSEL_A::ULP32K;
}

impl<X, N> RtcSource1k for Enabled<OscUlp32k<X, Active1k>, N>
where
    X: Output32k,
    N: Counter,
{
    const RTC_SRC_1K: RTCSEL_A = RTCSEL_A::ULP1K;
}
