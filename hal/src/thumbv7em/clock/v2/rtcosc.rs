// TODO

#![allow(missing_docs)]

use core::marker::PhantomData;

use crate::pac::osc32kctrl::rtcctrl::RTCSEL_A;
use crate::pac::osc32kctrl::{RegisterBlock, RTCCTRL};

use crate::time::Hertz;
use crate::typelevel::{Decrement, Increment};

use super::osculp32k::{OscUlp1kId, OscUlp32kId};
use super::xosc32k::{Xosc1kId, Xosc32kId};
use super::Driver;

//==============================================================================
// Registers
//==============================================================================

struct Registers;

impl Registers {
    const OSC32KCTRL: *const RegisterBlock = crate::pac::OSC32KCTRL::ptr();

    #[inline]
    fn rtcctrl(&self) -> &RTCCTRL {
        unsafe { &(*Self::OSC32KCTRL).rtcctrl }
    }

    #[inline]
    fn set_source_freq(&mut self, source: DynRtcSourceId) {
        use DynRtcSourceId::*;
        use RTCSEL_A::*;
        let variant = match source {
            OscUlp1k => ULP1K,
            OscUlp32k => ULP32K,
            Xosc1k => XOSC1K,
            Xosc32k => XOSC32K,
        };
        self.rtcctrl().write(|w| w.rtcsel().variant(variant));
    }

    fn reset_source_freq(&mut self) {
        self.rtcctrl().reset();
    }
}

//==============================================================================
// RtcOscToken
//==============================================================================

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

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DynRtcSourceId {
    OscUlp1k,
    OscUlp32k,
    Xosc1k,
    Xosc32k,
}

pub trait RtcSourceId {
    const DYN: DynRtcSourceId;
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

pub struct RtcOsc<S: RtcSourceId> {
    regs: Registers,
    source: PhantomData<S>,
}

impl<S: RtcSourceId> RtcOsc<S> {
    pub fn enable<D>(mut token: RtcOscToken, driver: D) -> (Self, D::Inc)
    where
        D: Driver<Source = S> + Increment,
    {
        token.regs.set_source_freq(S::DYN);
        let rtc_osc = Self {
            regs: token.regs,
            source: PhantomData,
        };
        (rtc_osc, driver.inc())
    }

    pub fn disable<D>(mut self, driver: D) -> (RtcOscToken, D::Dec)
    where
        D: Driver<Source = S> + Decrement,
    {
        self.regs.reset_source_freq();
        let token = RtcOscToken { regs: self.regs };
        (token, driver.dec())
    }
}
