use atsamd_hal_macros::hal_cfg;

#[hal_cfg("adc-d5x")]
use crate::pac::adc0;

#[hal_cfg(any("adc-d21", "adc-d11"))]
use crate::pac::adc as adc0;

#[hal_cfg(any("adc-d21", "adc-d11"))]
pub use adc0::ctrlb::Prescalerselect as Prescaler;

#[hal_cfg("adc-d5x")]
pub use adc0::ctrla::Prescalerselect as Prescaler;

pub use adc0::avgctrl::Samplenumselect as SampleCount;

pub use adc0::ctrlb::Resselselect as Resolution;

pub use adc0::refctrl::Refselselect as Reference;

use super::{Adc, AdcInstance};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AdcResolution {
    _8,
    _10,
    _12,
}

impl From<AdcResolution> for Resolution {
    fn from(val: AdcResolution) -> Self {
        match val {
            AdcResolution::_8 => Resolution::_8bit,
            AdcResolution::_10 => Resolution::_10bit,
            AdcResolution::_12 => Resolution::_12bit,
        }
    }
}

/// Result accumulation strategy for the ADC
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Accumulation {
    /// The ADC will read once and then the result is ready.
    ///
    /// The result will be in the users chosen bitwidth
    Single(AdcResolution),
    /// The ADC will read [SampleCount] samples, average them out
    /// and then the result is ready.
    ///
    /// The result will be in the range of 0-4095 (12bit)
    Average(SampleCount),
    /// The ADC will read [SampleCount] samples, sum them
    /// into a 16 bit wide value, and then the result is ready.
    ///
    /// The result will be in the range of 0-65535 (16bit),
    /// but will consist of the sum of multiple 12bit reads
    Summed(SampleCount),
}

impl Accumulation {
    /// Read the ADC once
    pub const fn single(res: AdcResolution) -> Self {
        Self::Single(res)
    }

    /// Accumulate multiple samples and average together
    pub const fn average(count: SampleCount) -> Self {
        Self::Average(count)
    }

    /// Accumulate multiple samples and add them together
    pub const fn summed(count: SampleCount) -> Self {
        Self::Summed(count)
    }

    pub(crate) fn resolution(&self) -> Resolution {
        if let Self::Single(res) = self {
            (*res).into()
        } else {
            Resolution::_16bit
        }
    }

    pub(crate) fn output_resolution(&self) -> Resolution {
        if let Self::Single(res) = self {
            (*res).into()
        } else if let Self::Average(_) = self {
            Resolution::_12bit
        } else {
            Resolution::_16bit
        }
    }

    pub(crate) fn samples(&self) -> u16 {
        match self {
            Accumulation::Single(_) => 1,
            // Samplenumselect is 2^n to get number of samples
            Accumulation::Average(samplenumselect) => 2u16.pow(*samplenumselect as u32),
            Accumulation::Summed(samplenumselect) => 2u16.pow(*samplenumselect as u32),
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BuilderError {
    /// Clock divider missing
    MissingClockDiv,
    /// Samples per clock missing
    MissingSampleClocks,
    /// Vref missing
    MissingVref,
    AdcError(super::Error),
}

impl From<super::Error> for BuilderError {
    fn from(value: super::Error) -> Self {
        Self::AdcError(value)
    }
}

/// # ADC Configuration Builder
///
/// This structure provides configuration of multiple factors which affect the
/// ADC's sampling characteristics.
///
/// The ADC clock is driven by the peripheral clock divided with a divider
/// selected via [AdcBuilder::with_clock_divider()].
///
/// A sample is taken over a number of ADC clock cycles configured by
/// [AdcBuilder::with_clock_cycles_per_sample()], and then transmitted to the
/// ADC register 1 clock cycle per bit of resolution - resolution is determined
/// by the accumulation mode selected by [AdcBuilder::new()].
///
/// The ADC can be configured to combine multiple readings in either an average
/// or summed mode (See [Accumulation]).
///
/// The formula for calculating Sample rate (SPS) is shown below, and
/// implemented in a helper method [AdcBuilder::calculate_sps()]:
///
/// ## For single sample
/// ```
/// SPS = (GCLK_ADC / clk_divider) / (sample_clock_cycles + bit_width)
/// ```
/// ## For multiple samples 'n' (Averaging or Summed)
/// ```
/// SPS = (GCLK_ADC / clk_divider) / (n * (sample_clock_cycles + 12))
/// ```
#[derive(Copy, Clone)]
pub struct AdcBuilder {
    pub clk_divider: Option<Prescaler>,
    pub sample_clock_cycles: Option<u8>,
    pub accumulation: Accumulation,
    pub vref: Option<Reference>,
}

/// Version of [AdcBuilder] without any optional settings.
/// [AdcBuilder] is converted to this when passed to the ADC
#[derive(Copy, Clone, PartialEq)]
pub(crate) struct AdcSettings {
    pub clk_divider: Prescaler,
    pub sample_clock_cycles: u8,
    pub accumulation: Accumulation,
    pub vref: Reference,
}

impl AdcBuilder {
    /// Create a new settings builder
    pub fn new(accumulation_method: Accumulation) -> Self {
        Self {
            clk_divider: None,
            sample_clock_cycles: None,
            accumulation: accumulation_method,
            vref: None,
        }
    }

    pub(crate) fn check_params(&self) -> Result<(), BuilderError> {
        self.clk_divider.ok_or(BuilderError::MissingClockDiv)?;
        self.sample_clock_cycles
            .ok_or(BuilderError::MissingSampleClocks)?;
        self.vref.ok_or(BuilderError::MissingVref)?;
        Ok(())
    }

    pub(crate) fn to_settings(self) -> Result<AdcSettings, BuilderError> {
        self.check_params()?;
        Ok(AdcSettings {
            clk_divider: self.clk_divider.unwrap(),
            sample_clock_cycles: self.sample_clock_cycles.unwrap(),
            accumulation: self.accumulation,
            vref: self.vref.unwrap(),
        })
    }

    /// This setting adjusts the ADC clock frequency by dividing the input clock
    /// for the ADC.
    ///
    /// ## Example:
    /// * Input clock 48MHz, div 32 => ADC Clock is 1.5MHz
    pub fn with_clock_divider(mut self, div: Prescaler) -> Self {
        self.clk_divider = Some(div);
        self
    }

    /// Sets the ADC reference voltage source
    pub fn with_vref(mut self, reference: Reference) -> Self {
        self.vref = Some(reference);
        self
    }

    /// Sets the number of ADC clock cycles taken to sample a single sample. The
    /// higher this number, the longer it will take the ADC to sample each
    /// sample. Smaller values will make the ADC perform more samples per
    /// second, but there may be more noise in each sample leading to erratic
    /// values.
    ///
    /// ## Safety
    /// * This function clamps input value between 1 and 63, to conform to the
    ///   ADC registers min and max values.
    pub fn with_clock_cycles_per_sample(mut self, num: u8) -> Self {
        self.sample_clock_cycles = Some(num.clamp(1, 63)); // Clamp in range
        self
    }

    /// Returns a calculated sample rate based on the settings used
    pub fn calculate_sps(&self, clock_freq: u32) -> Result<u32, BuilderError> {
        self.check_params()?;

        let div = self.clk_divider.unwrap() as u32;
        let adc_clk_freq = clock_freq / div;

        let bit_width = match self.accumulation.resolution() {
            Resolution::_16bit => 16,
            Resolution::_12bit => 12,
            Resolution::_10bit => 10,
            Resolution::_8bit => 8,
        };

        let mut clocks_per_sample = self.sample_clock_cycles.unwrap() as u32 + bit_width;

        let samples = self.accumulation.samples();
        clocks_per_sample *= samples as u32;
        Ok(adc_clk_freq / clocks_per_sample)
    }

    /// Turn the builder into an ADC
    #[hal_cfg("adc-d5x")]
    #[inline]
    pub fn enable<I: AdcInstance, PS: crate::clock::v2::pclk::PclkSourceId>(
        self,
        adc: I::Instance,
        clk: crate::clock::v2::apb::ApbClk<I::ClockId>,
        pclk: &crate::clock::v2::pclk::Pclk<I::ClockId, PS>,
    ) -> Result<Adc<I>, BuilderError> {
        let settings = self.to_settings()?;
        Adc::new(adc, settings, clk, pclk).map_err(|e| e.into())
    }

    #[hal_cfg(any("adc-d11", "adc-d21"))]
    #[inline]
    pub fn enable<I: AdcInstance>(
        self,
        adc: I::Instance,
        pm: &mut crate::pac::Pm,
        clock: &crate::clock::AdcClock,
    ) -> Result<Adc<I>, BuilderError> {
        let settings = self.to_settings()?;
        Adc::new(adc, settings, pm, clock).map_err(|e| e.into())
    }
}
