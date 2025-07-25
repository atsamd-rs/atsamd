//! Open-loop 8MHz oscillator

use crate::pac::sysctrl::osc8m::{Frangeselect, Prescselect};
use crate::pac::sysctrl::Osc8m;
use crate::pac::Sysctrl;

use fugit::RateExtU32;
use crate::time::Hertz;
use crate::typelevel::Sealed;

use super::{Enabled, Source};

pub struct OscToken(());

impl OscToken {
    pub(super) unsafe fn new() -> Self {
        Self(())
    }

    fn osc8m(&self) -> &Osc8m {
        unsafe { &(*Sysctrl::PTR).osc8m() }
    }

    fn enable(&mut self, settings: Settings) {
        self.osc8m().modify(|_, w| {
            if let Some(freq_range) = settings.freq_range {
                w.frange().variant(freq_range.into());
            }
            if let Some(calibration) = settings.calibration {
                // Safety: The PAC will truncate the value to 12 bits,
                // and all 12-bit values are valid
                unsafe { w.calib().bits(calibration) };
            }
            w.presc().variant(settings.prescaler.into());
            w.ondemand().bit(settings.on_demand);
            w.runstdby().bit(settings.run_standby);
            w.enable().set_bit()
        });
    }

    fn disable(&mut self) {
        self.osc8m().modify(|_, w| w.enable().clear_bit())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Settings {
    freq_range: Option<FreqRange>,
    calibration: Option<u16>,
    prescaler: Prescaler,
    on_demand: bool,
    run_standby: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
/// Frequency ranges in Megahertz
pub enum FreqRange {
    FourToSix,
    SixToEight,
    EightToEleven,
    ElevenToFifteen,
}

impl From<FreqRange> for Frangeselect {
    fn from(freq_range: FreqRange) -> Self {
        match freq_range {
            FreqRange::FourToSix => Self::_0,
            FreqRange::SixToEight => Self::_1,
            FreqRange::EightToEleven => Self::_2,
            FreqRange::ElevenToFifteen => Self::_3,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Prescaler {
    One,
    Two,
    Four,
    Eight,
}

impl From<Prescaler> for Prescselect {
    fn from(prescaler: Prescaler) -> Self {
        match prescaler {
            Prescaler::One => Self::_0,
            Prescaler::Two => Self::_1,
            Prescaler::Four => Self::_2,
            Prescaler::Eight => Self::_3,
        }
    }
}

pub enum OscId {}

impl Sealed for OscId {}

pub struct Osc {
    token: OscToken,
    settings: Settings,
}

impl Osc {
    pub fn new(token: OscToken) -> Self {
        let settings = Settings {
            freq_range: None,
            calibration: None,
            prescaler: Prescaler::Eight,
            on_demand: true,
            run_standby: false,
        };
        Self { token, settings }
    }

    pub fn freq_range(mut self, freq_range: FreqRange) -> Self {
        self.settings.freq_range = Some(freq_range);
        self
    }

    pub fn calibration(mut self, calibration: u16) -> Self {
        self.settings.calibration = Some(calibration);
        self
    }

    pub fn prescaler(mut self, prescaler: Prescaler) -> Self {
        self.settings.prescaler = prescaler;
        self
    }

    pub fn on_demand(mut self, on_demand: bool) -> Self {
        self.settings.on_demand = on_demand;
        self
    }

    pub fn run_standby(mut self, run_standby: bool) -> Self {
        self.settings.run_standby = run_standby;
        self
    }

    pub fn freq(&self) -> Hertz {
        let div = match self.settings.prescaler {
            Prescaler::One => 1,
            Prescaler::Two => 2,
            Prescaler::Four => 4,
            Prescaler::Eight => 8,
        };
        (8_000_000u32 / div).Hz()
    }

    pub fn enable(mut self) -> Enabled<Self> {
        self.token.enable(self.settings);
        Enabled::new(self)
    }
}

pub type EnabledOsc<N> = Enabled<Osc, N>;

impl<N> EnabledOsc<N> {
    pub fn disable(mut self) -> Osc {
        self.0.token.disable();
        self.0
    }
}

impl<N> Source for EnabledOsc<N> {
    type Id = OscId;

    fn freq(&self) -> Hertz {
        self.0.freq()
    }
}
