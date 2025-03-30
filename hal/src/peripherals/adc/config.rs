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

/// Result accumulation strategy for the ADC
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Accumulation {
    /// The ADC will read once and then the result is ready.
    ///
    /// The result will be in the users chosen bitwidth
    Single,
    /// The ADC will read [SampleCount] samples, average them out
    /// into a 16 bit wide value, and then the result is ready.
    ///
    /// The result will be in the range of 0-65535 (16bit)
    Average(SampleCount),
    /// The ADC will read [SampleCount] samples, sum them
    /// into a 16 bit wide value, and then the result is ready.
    ///
    /// The result will be in the range of 0-65535 (16bit)
    Summed(SampleCount),
}

/// # ADC configuration builder
///
/// Multiple factors can affect the ADCs overall sampling rate, and this
/// structure allows for the configuring of the majority of factors that affect
/// the sample rate of the ADC
///
/// To begin with, the ADC Clock is driven by the peripheral clock divided with
/// a divider (see [Config::clock_divider]).
///
/// Each sample is read by the ADC over
/// [Config::sample_clock_cycles] clock cycles, and then transmitted
/// to the ADC register over [Config::bit_width] clock cycles (1
/// clock cycle per bit)
///
/// The ADC can also be configured to combine multiple simultaneous readings in
/// either an average or summed mode (See [Accumulation]), this also affects
/// the overall sample rate of the ADC as the ADC has to do multiple
/// samples before a result is ready.
///
/// Therefore, the overall formula for calculating Sample rate (SPS) can be
/// calculated like so:
///
/// ## For single sample
/// ```
/// SPS = (GCLK_ADC / clk_divider) / (sample_clock_cycles + bit_width)
/// ```
/// ## For multiple samples (Averaging or Summed)
/// ```
/// SPS = (GCLK_ADC / clk_divider) / (n * (sample_clock_cycles + bit_width))
/// ```
#[derive(Copy, Clone)]
pub struct Config {
    pub clk_divider: Prescaler,
    pub sample_clock_cycles: u8,
    pub bit_width: Resolution,
    pub accumulation: Accumulation,
    pub vref: Reference,
}

impl Config {
    /// Configure the ADC to sample at 250_000 SPS (Assuming the ADC Gclk source is
    /// 48MHz) using the following settings:
    /// * clock divider factor of 32
    /// * 6 clock cycles per sample
    /// * 12bit ADC result resolution
    /// * ADC will not perform any averaging or summation of multiple readings
    /// * Use Intvcc1 (Analog reference voltage) as reference voltage for a full 0.0-3.3V reading
    pub fn new() -> Self {
        Self {
            clk_divider: Prescaler::Div32,
            sample_clock_cycles: 6,
            bit_width: Resolution::_12bit,
            accumulation: Accumulation::Single,
            vref: Reference::Intvcc1,
        }
    }

    /// This setting adjusts the ADC clock frequency by dividing the input clock
    /// for the ADC.
    ///
    /// ## Example:
    /// * Input clock 48MHz, div 32 => ADC Clock is 1.5MHz
    pub fn clock_divider(mut self, div: Prescaler) -> Self {
        self.clk_divider = div;
        self
    }

    /// Sets the ADC output resolution
    ///
    /// ## Application Notes
    /// [Resolution::_16bit] can only be used if the ADC is in summation or averaging mode. See [Config::accumulation_method] for
    /// more detail
    pub fn sample_resolution(mut self, bit_width: Resolution) -> Self {
        self.bit_width = bit_width;
        self
    }

    /// Sets the ADC reference voltage source
    pub fn with_vref(mut self, reference: Reference) -> Self {
        self.vref = reference;
        self
    }

    /// Sets how the ADC will accumulate values before actually returning a
    /// value.
    ///
    /// The default is single (ADC will return a sample as soon as it is
    /// measured)
    ///
    /// Setting [Accumulation::Summed] will make the ADC take 'n' samples,
    /// and sum the total before returning it
    ///
    /// Setting [Accumulation::Average] will make the ADC take 'n' samples,
    /// and average the total before returning it
    ///
    /// ## Application notes
    /// 
    /// * Selecting [Accumulation::Summed] or [Accumulation::Average]
    /// will reduce the overall ADC sample rate by a factor of 1/n, and 
    /// the ADC resolution will be set to [Resolution::_16bit] which is required
    /// in these modes.
    pub fn accumulation_method(mut self, method: Accumulation) -> Self {
        self.accumulation = method;
        if Accumulation::Single != self.accumulation {
            // Auto set 16bit
            self.bit_width = Resolution::_16bit;
        }
        self
    }

    /// Sets the number of ADC clock cycles taken to sample a single
    /// sample. The higher this number, the longer it will take the ADC to
    /// sample each sample. Smaller values will make the ADC perform more samples per second,
    /// but there may be more noise in each sample leading to irratic values.
    ///
    /// ## Safety
    /// * This function will clamp input value between 1 and 63, to conform to the ADC registers
    /// min and max values.
    pub fn clock_cycles_per_sample(mut self, num: u8) -> Self {
        self.sample_clock_cycles = num.clamp(1, 63); // Clamp in range
        self
    }

    /// Returns a calculated sample rate based on the settings used
    pub fn calculate_sps(&self, clock_freq: u32) -> u32 {
        let div = self.clk_divider as u32;
        let adc_clk_freq = clock_freq / div;

        let mut clocks_per_sample = self.sample_clock_cycles as u32 + (self.bit_width as u32);

        let multi = match self.accumulation {
            Accumulation::Single => 1,
            Accumulation::Average(adc_sample_count) => adc_sample_count as u32,
            Accumulation::Summed(adc_sample_count) => adc_sample_count as u32,
        };
        clocks_per_sample *= multi;
        adc_clk_freq / clocks_per_sample
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
