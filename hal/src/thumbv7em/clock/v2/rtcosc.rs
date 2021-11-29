// TODO
#![allow(missing_docs)]

use core::marker::PhantomData;

use crate::pac::osc32kctrl::rtcctrl::RTCSEL_A;
use crate::pac::osc32kctrl::{RegisterBlock, RTCCTRL};

use crate::typelevel::{Counter, Decrement, Increment, Sealed};

use super::osculp32k::{OscUlp32k, OscUlp32kId};
use super::xosc32k::{self, Xosc32k, Xosc32kId};
use super::Enabled;

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
    fn set_source_freq(&mut self, source: DynRtcSourceId, freq: DynRtcFreq) {
        use DynRtcFreq::*;
        use DynRtcSourceId::*;
        use RTCSEL_A::*;

        let variant = match (source, freq) {
            (OscUlp32k, Freq1k) => ULP1K,
            (OscUlp32k, Freq32k) => ULP32K,
            (Xosc32k, Freq1k) => XOSC32K,
            (Xosc32k, Freq32k) => XOSC1K,
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
    pub(super) unsafe fn new() -> Self {
        Self { regs: Registers }
    }
}

//==============================================================================
// RtcSourceId
//==============================================================================

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DynRtcSourceId {
    OscUlp32k,
    Xosc32k,
}

pub trait RtcSourceId {
    const DYN: DynRtcSourceId;
}

impl RtcSourceId for OscUlp32kId {
    const DYN: DynRtcSourceId = DynRtcSourceId::OscUlp32k;
}

impl RtcSourceId for Xosc32kId {
    const DYN: DynRtcSourceId = DynRtcSourceId::Xosc32k;
}

//==============================================================================
// RtcFreq
//==============================================================================

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DynRtcFreq {
    Freq1k,
    Freq32k,
}

pub trait RtcFreq: Sealed {
    const DYN: DynRtcFreq;
}

pub enum Freq32k {}
impl Sealed for Freq32k {}
impl RtcFreq for Freq32k {
    const DYN: DynRtcFreq = DynRtcFreq::Freq32k;
}

pub enum Freq1k {}
impl Sealed for Freq1k {}
impl RtcFreq for Freq1k {
    const DYN: DynRtcFreq = DynRtcFreq::Freq1k;
}

//==============================================================================
// RtcDriver
//==============================================================================

mod private {
    use super::super::Driver;
    use super::{DynRtcFreq, RtcSourceId};

    pub trait RtcDriver: Driver
    where
        Self::Source: RtcSourceId,
    {
        fn enable_rtc_osc(&mut self, freq: DynRtcFreq);
        fn disable_rtc_osc(&mut self, freq: DynRtcFreq);
    }
}

use private::RtcDriver;

impl<N> RtcDriver for Enabled<OscUlp32k, N>
where
    N: Counter,
{
    fn enable_rtc_osc(&mut self, freq: DynRtcFreq) {
        use DynRtcFreq::*;
        match freq {
            Freq1k => self.0.token.activate_1k(true),
            Freq32k => self.0.token.activate_32k(true),
        }
    }
    fn disable_rtc_osc(&mut self, freq: DynRtcFreq) {
        use DynRtcFreq::*;
        match freq {
            Freq1k => self.0.token.activate_1k(false),
            Freq32k => self.0.token.activate_32k(false),
        }
    }
}

impl<M, N> RtcDriver for Enabled<Xosc32k<M>, N>
where
    M: xosc32k::Mode,
    N: Counter,
{
    fn enable_rtc_osc(&mut self, freq: DynRtcFreq) {
        use DynRtcFreq::*;
        match freq {
            Freq1k => self.0.token.activate_1k(true),
            Freq32k => self.0.token.activate_32k(true),
        }
    }
    fn disable_rtc_osc(&mut self, freq: DynRtcFreq) {
        use DynRtcFreq::*;
        match freq {
            Freq1k => self.0.token.activate_1k(false),
            Freq32k => self.0.token.activate_32k(false),
        }
    }
}

//==============================================================================
// RtcOsc
//==============================================================================

pub struct RtcOsc<S: RtcSourceId, F: RtcFreq> {
    regs: Registers,
    source: PhantomData<S>,
    freq: PhantomData<F>,
}

impl<S: RtcSourceId, F: RtcFreq> RtcOsc<S, F> {
    pub fn enable<D>(token: RtcOscToken, mut driver: D) -> (Self, D::Inc)
    where
        D: RtcDriver<Source = S> + Increment,
    {
        let mut regs = token.regs;
        driver.enable_rtc_osc(F::DYN);
        regs.set_source_freq(S::DYN, F::DYN);
        let rtc_osc = Self {
            regs,
            source: PhantomData,
            freq: PhantomData,
        };
        (rtc_osc, driver.inc())
    }

    pub fn disable<D>(self, mut driver: D) -> (RtcOscToken, D::Dec)
    where
        D: RtcDriver<Source = S> + Decrement,
    {
        let mut regs = self.regs;
        regs.reset_source_freq();
        driver.disable_rtc_osc(F::DYN);
        let token = RtcOscToken { regs };
        (token, driver.dec())
    }
}
