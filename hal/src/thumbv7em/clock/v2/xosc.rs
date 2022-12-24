//! # Xosc - External Oscillator
//! TODO
//!
//! Used with external clock generators or crystal oscillators

use core::marker::PhantomData;

use typenum::U0;

use crate::pac::oscctrl::xoscctrl::{CFDPRESC_A, STARTUP_A};
use crate::pac::oscctrl::{RegisterBlock, XOSCCTRL};

use crate::clock::types::{Counter, Enabled};
use crate::clock::v2::{Source, SourceMarker};
use crate::gpio::v2::{AnyPin, FloatingDisabled, Pin, PinId, PA14, PA15, PB22, PB23};
use crate::time::{Hertz, U32Ext};
use crate::typelevel::Sealed;

use super::dpll::{DpllSource, DpllSourceMarker, DpllSrc};
use super::gclk::{GclkSource, GclkSourceEnum, GclkSourceMarker, GenNum};
use super::gclkio::NotGclkInput;

//==============================================================================
// XoscNum
//==============================================================================

pub trait XoscNum: Sealed {
    const NUM: usize;
    const DPLL_SRC: DpllSrc;
    type XIn: PinId;
    type XOut: PinId;
}

/// Oscillator Source 0
pub enum Osc0 {}

impl Sealed for Osc0 {}

impl XoscNum for Osc0 {
    const NUM: usize = 0;
    const DPLL_SRC: DpllSrc = DpllSrc::XOSC0;
    type XIn = PA14;
    type XOut = PA15;
}

/// Oscillator Source 1
pub enum Osc1 {}

impl Sealed for Osc1 {}

impl XoscNum for Osc1 {
    const NUM: usize = 1;
    const DPLL_SRC: DpllSrc = DpllSrc::XOSC1;
    type XIn = PB22;
    type XOut = PB23;
}

#[derive(Clone, PartialEq)]
/// Current mutliplier/reference pair
pub enum CrystalCurrent {
    /// 8MHz
    BaseFreq8m,
    /// 8 to 16MHz
    LowFreq8mTo16m,
    /// 16 to 24MHz
    MedFreq16mTo24m,
    /// 24 to 48MHz
    HighFreq24mTo48m,
}

impl CrystalCurrent {
    /// Get the current multiplier
    pub fn imult(&self) -> u8 {
        match &self {
            Self::BaseFreq8m => 3,
            Self::LowFreq8mTo16m => 4,
            Self::MedFreq16mTo24m => 5,
            Self::HighFreq24mTo48m => 6,
        }
    }

    /// Get the current reference
    pub fn iptat(&self) -> u8 {
        match &self {
            Self::BaseFreq8m => 2,
            Self::LowFreq8mTo16m => 3,
            Self::MedFreq16mTo24m => 3,
            Self::HighFreq24mTo48m => 3,
        }
    }
}

//==============================================================================
// XoscToken
//==============================================================================

pub struct XoscToken<X: XoscNum> {
    osc: PhantomData<X>,
}

impl<X: XoscNum> XoscToken<X> {
    /// Create a new instance of [`XoscToken`]
    #[inline]
    pub(super) unsafe fn new() -> Self {
        Self { osc: PhantomData }
    }

    #[inline]
    fn oscctrl(&self) -> &RegisterBlock {
        unsafe { &*crate::pac::OSCCTRL::ptr() }
    }

    #[inline]
    fn xoscctrl(&self) -> &XOSCCTRL {
        &self.oscctrl().xoscctrl[X::NUM]
    }

    #[inline]
    fn set_start_up(&mut self, start_up: StartUp) {
        self.xoscctrl().modify(|_, w| w.startup().variant(start_up));
    }

    #[inline]
    fn set_on_demand(&mut self, on_demand: bool) {
        self.xoscctrl().modify(|_, w| w.ondemand().bit(on_demand));
    }

    #[inline]
    fn set_run_standby(&mut self, run_standby: bool) {
        self.xoscctrl().modify(|_, w| w.runstdby().bit(run_standby));
    }

    #[inline]
    fn from_clock(&mut self) {
        self.xoscctrl().modify(|_, w| w.xtalen().bit(false));
    }

    #[inline]
    fn from_crystal(&mut self) {
        self.xoscctrl().modify(|_, w| w.xtalen().bit(true));
    }

    #[inline]
    fn enable(&mut self) {
        self.xoscctrl().modify(|_, w| w.enable().bit(true));
    }

    #[inline]
    fn disable(&mut self) {
        self.xoscctrl().modify(|_, w| w.enable().bit(false));
    }

    #[inline]
    fn wait_ready(&self) {
        let mask = 1 << X::NUM;
        while self.oscctrl().status.read().bits() & mask == 0 {}
    }

    #[inline]
    #[allow(dead_code)]
    fn set_clock_failure_detection(&mut self, cfden: bool) {
        self.xoscctrl().modify(|_, w| w.cfden().bit(cfden));
    }

    #[inline]
    #[allow(dead_code)]
    fn set_clock_failure_detection_prescaler(&mut self, prescale: CFDPRESC_A) {
        self.xoscctrl()
            .modify(|_, w| w.cfdpresc().variant(prescale));
    }
    #[inline]
    fn set_current(&mut self, cc: &CrystalCurrent) {
        self.xoscctrl().modify(|_, w| unsafe {
            w.imult().bits(cc.imult());
            w.iptat().bits(cc.iptat())
        });
    }

    #[inline]
    fn set_amplitude_loop_control(&mut self, enalc: bool) {
        self.xoscctrl().modify(|_, w| w.enalc().bit(enalc));
    }

    #[inline]
    fn set_clock_switch(&mut self, swben: bool) {
        self.xoscctrl().modify(|_, w| w.swben().bit(swben));
    }

    #[inline]
    fn set_low_buf_gain(&mut self, lowbufgain: bool) {
        self.xoscctrl()
            .modify(|_, w| w.lowbufgain().bit(lowbufgain));
    }
}

//==============================================================================
// Aliases
//==============================================================================

/// When Clock Failure Detection (CFD) is active, sets how many clock cycles
/// before monitoring of the clock begins when starting the oscillator.
pub type StartUp = STARTUP_A;

/// Type alias for Xosc Input pin
pub type XIn<X> = Pin<<X as XoscNum>::XIn, FloatingDisabled>;

/// Type alias for Xosc Output pin
pub type XOut<X> = Pin<<X as XoscNum>::XOut, FloatingDisabled>;

//==============================================================================
// Mode structure for Xosc
//==============================================================================

pub trait Mode: Sealed {}

pub struct ClockMode {}
impl Mode for ClockMode {}
impl Sealed for ClockMode {}

pub struct XoscMode<X: XoscNum> {
    xout: XOut<X>,
    current: CrystalCurrent,
    amplitude_loop_control: bool,
    low_buf_gain: bool,
}
impl<X: XoscNum> Mode for XoscMode<X> {}
impl<X: XoscNum> Sealed for XoscMode<X> {}

//==============================================================================
// Xosc
//==============================================================================

pub struct Xosc<X, M>
where
    X: XoscNum,
    M: Mode,
{
    token: XoscToken<X>,
    mode: M,
    xin: XIn<X>,
    src_freq: Hertz,
    start_up_cycles: StartUp,
    on_demand: bool,
    run_standby: bool,
    clock_switch: bool,
}

/// Type alias for [Xosc0]
pub type Xosc0<M> = Xosc<Osc0, M>;

/// Type alias for [Xosc1]
pub type Xosc1<M> = Xosc<Osc1, M>;

impl<X, M> Xosc<X, M>
where
    X: XoscNum,
    M: Mode,
{
    /// Returns the frequency of the oscillator
    #[inline]
    pub fn freq(&self) -> Hertz {
        self.src_freq
    }

    /// Sets the number of cycles allowed to pass before Clock Failure Detection
    /// (CFD) starts monitoring the external oscillator.
    #[inline]
    pub fn set_start_up(mut self, start_up: StartUp) -> Self {
        self.start_up_cycles = start_up;
        self
    }
    /// Controls the on demand functionality of the clock source
    ///
    /// Only starts the clock source when a peripheral uses it
    ///
    /// If cleared the clock will be always active
    /// See Datasheet c. 13.5 for details
    #[inline]
    pub fn set_on_demand(mut self, on_demand: bool) -> Self {
        self.on_demand = on_demand;
        self
    }

    /// Controls the clock source behaviour during standby
    ///
    /// See Datasheet c. 28.6.2
    #[inline]
    pub fn set_run_standby(mut self, run_standby: bool) -> Self {
        self.run_standby = run_standby;
        self
    }

    /// Xosc lock Switch Enable
    ///
    /// Controls if XOSCn switches back to external clock or crystal in case of
    /// clock recovery
    #[inline]
    pub fn set_clock_switch(mut self, swben: bool) -> Self {
        self.clock_switch = swben;
        self
    }

    /// Busy-wait until ready
    #[inline]
    pub fn wait_ready(&self) {
        self.token.wait_ready();
    }
}

impl<X: XoscNum> Xosc<X, ClockMode> {
    /// Create a Xosc sourced from a clock
    #[inline]
    pub fn from_clock(
        token: XoscToken<X>,
        xin: impl AnyPin<Id = X::XIn>,
        src_freq: impl Into<Hertz>,
    ) -> Self {
        let xin = xin.into().into_floating_disabled();
        let start_up_cycles = StartUp::CYCLE1;
        // Mimic default reset state
        let on_demand = true;
        let run_standby = false;
        let clock_switch = false;
        Self {
            token,
            mode: ClockMode {},
            xin,
            src_freq: src_freq.into(),
            start_up_cycles,
            on_demand,
            run_standby,
            clock_switch,
        }
    }

    /// Modify hardware to realise the desired state
    /// stored within the [`Xosc`]
    ///
    /// Returns the Enabled Xosc
    #[inline]
    pub fn enable(mut self) -> Enabled<Xosc<X, ClockMode>, U0> {
        self.token.from_clock();
        self.token.set_start_up(self.start_up_cycles);
        self.token.set_on_demand(self.on_demand);
        self.token.set_run_standby(self.run_standby);
        self.token.set_clock_switch(self.clock_switch);

        self.token.enable();
        Enabled::new(self)
    }

    /// Deconstruct the Xosc and return the inner XoscToken
    #[inline]
    pub fn free_xosc(self) -> (XoscToken<X>, XIn<X>) {
        (self.token, self.xin)
    }
}

impl<X: XoscNum> Xosc<X, XoscMode<X>> {
    /// Creates an Xosc input fed from a crystal oscillator.
    ///
    /// The crystal oscillator frequency must be supported,
    /// for valid frequencies see [CrystalCurrent].
    ///
    /// By default `Amplitude Loop Control` is set,
    /// see [Xosc::set_amplitude_loop_control()]
    #[inline]
    pub fn from_crystal(
        token: XoscToken<X>,
        xin: impl AnyPin<Id = X::XIn>,
        xout: impl AnyPin<Id = X::XOut>,
        src_freq: impl Into<Hertz>,
    ) -> Self {
        let xin = xin.into().into_floating_disabled();
        let xout = xout.into().into_floating_disabled();

        // Calculate the CrystalCurrent from the supplied
        // crystal frequency
        let (current, frequency) = match src_freq.into().0 {
            freq @ 8_000_000 => (CrystalCurrent::BaseFreq8m, freq),
            freq @ 8_000_001..=16_000_000 => (CrystalCurrent::LowFreq8mTo16m, freq),
            freq @ 16_000_001..=24_000_000 => (CrystalCurrent::MedFreq16mTo24m, freq),
            freq @ 24_000_001..=48_000_000 => (CrystalCurrent::HighFreq24mTo48m, freq),
            _ => panic!("Xosc fed with invalid frequency"),
        };

        // Lowers power usage and protects the crystal
        let amplitude_loop_control = true;

        let low_buf_gain = false;

        let start_up_cycles = StartUp::CYCLE1;
        let on_demand = true;
        let run_standby = false;
        let clock_switch = false;
        Self {
            token,
            mode: XoscMode {
                xout,
                current,
                amplitude_loop_control,
                low_buf_gain,
            },
            xin,
            src_freq: frequency.hz(),
            start_up_cycles,
            on_demand,
            run_standby,
            clock_switch,
        }
    }

    /// Modify hardware to realise the desired state
    /// stored within the [`Xosc`]
    ///
    /// Returns the `Enabled<Xosc...>`
    #[inline]
    pub fn enable(mut self) -> Enabled<Xosc<X, XoscMode<X>>, U0> {
        self.token.from_crystal();
        self.token.set_start_up(self.start_up_cycles);
        self.token.set_on_demand(self.on_demand);
        self.token.set_run_standby(self.run_standby);
        self.token.set_clock_switch(self.clock_switch);

        // Crystal specific
        self.token.set_current(&self.mode.current);
        self.token
            .set_amplitude_loop_control(self.mode.amplitude_loop_control);
        self.token.set_low_buf_gain(self.mode.low_buf_gain);
        self.token.enable();
        Enabled::new(self)
    }

    /// Sets the current drive strength for the crystal
    ///
    /// See [CrystalCurrent] for possible values
    #[inline]
    pub fn set_current(mut self, current: CrystalCurrent) -> Self {
        self.mode.current = current;
        self
    }

    /// Controls the automatic amplitude loop control
    ///
    /// Recommended option, ensures the crystal is not overdriven,
    /// and lowers power consumption. See datasheet c. 54.13 p. 1811
    #[inline]
    pub fn set_amplitude_loop_control(mut self, enalc: bool) -> Self {
        self.mode.amplitude_loop_control = enalc;
        self
    }

    /// If `LOWBUFGAIN`is set when `ENALC` is enabled,
    /// the oscillators amplitude is increased by approximately a factor 2.
    ///
    /// Default value (0) should be used together with low amplitude
    /// oscillators. Can be used to solve stability issues.
    #[inline]
    pub fn set_low_buf_gain(mut self, lowbufgain: bool) -> Self {
        self.mode.low_buf_gain = lowbufgain;
        self
    }

    /// Deconstruct the Xosc and return the inner XoscToken
    #[inline]
    pub fn free_crystal(self) -> (XoscToken<X>, XIn<X>, XOut<X>) {
        (self.token, self.xin, self.mode.xout)
    }
}

impl<X, M> Enabled<Xosc<X, M>, U0>
where
    X: XoscNum,
    M: Mode,
{
    /// Disable the [`Xosc`]
    ///
    /// Only possible when nothing uses the `Xosc`
    #[inline]
    pub fn disable(mut self) -> Xosc<X, M> {
        self.0.token.disable();
        self.0
    }
}

//==============================================================================
// GclkSource
//==============================================================================

impl GclkSourceMarker for Osc0 {
    const GCLK_SRC: GclkSourceEnum = GclkSourceEnum::XOSC0;
}

impl NotGclkInput for Osc0 {}

impl GclkSourceMarker for Osc1 {
    const GCLK_SRC: GclkSourceEnum = GclkSourceEnum::XOSC1;
}

impl NotGclkInput for Osc1 {}

impl<G, X, M, N> GclkSource<G> for Enabled<Xosc<X, M>, N>
where
    G: GenNum,
    X: XoscNum + GclkSourceMarker,
    M: Mode,
    N: Counter,
{
    type Type = X;
}

//==============================================================================
// DpllSource
//==============================================================================

impl SourceMarker for Osc0 {}

impl DpllSourceMarker for Osc0 {
    const DPLL_SRC: DpllSrc = DpllSrc::XOSC0;
}

impl SourceMarker for Osc1 {}

impl DpllSourceMarker for Osc1 {
    const DPLL_SRC: DpllSrc = DpllSrc::XOSC1;
}

impl<X, M, N> DpllSource for Enabled<Xosc<X, M>, N>
where
    X: XoscNum + DpllSourceMarker,
    M: Mode,
    N: Counter,
{
    type Type = X;
}

//==============================================================================
// Source
//==============================================================================

impl<X, M, N> Source for Enabled<Xosc<X, M>, N>
where
    X: XoscNum,
    M: Mode,
    N: Counter,
{
    #[inline]
    fn freq(&self) -> Hertz {
        self.0.freq()
    }
}
