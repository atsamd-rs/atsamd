
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AdcSampleCount {
    Count1 = 1,
    Count2 = 2,
    Count4 = 4,
    Count8 = 8,
    Count16 = 16,
    Count32 = 32,
    Count64 = 64,
    Count128 = 128,
    Count256 = 256,
    Count512 = 512,
    Count1024 = 1024
}

#[derive(Copy, Clone)]
pub enum AdcBitWidth {
    Eight = 8,
    Ten = 10,
    Twelve = 12,
}

/// Result accumulation strategy for the ADC
pub enum AdcAccumulation {
    /// The ADC will read once and then the result is ready
    Single,
    /// The ADC will read [AdcSampleCount] samples, average them out
    /// into a 16 bit wide value, and then the result is ready
    Average(AdcSampleCount),
    /// The ADC will read [AdcSampleCount] samples, sum them
    /// into a 16 bit wide value, and then the result is ready
    Summed(AdcSampleCount)
}

#[derive(Copy, Clone)]
pub enum AdcDivider {
    Div2 = 2,
    Div4 = 4,
    Div8 = 8,
    Div16 = 16,
    Div32 = 32,
    Div64 = 64,
    Div128 = 128,
    Div256 = 256
}

/// # ADC sampling rate settings
/// 
/// Multiple factors can affect the ADCs overall sampling rate, and this structure
/// allows for the configuring of the majority of factors that affect the sample rate of the ADC
/// 
/// To begin with, the ADC Clock is driven by the peripheral clock divided with a divider ([AdcDivider]).
/// 
/// Each sample is read by the ADC over [AdcSettingsBuilder::sample_clock_cycles] clock cycles, and then
/// transmitted to the ADC register over [AdcSettingsBuilder::bit_width] clock cycles (1 clock cycle per bit)
/// 
/// The ADC can also be configured to combine multiple simultaneous readings in either an average or summed mode
/// (See [AdcAccumulation]), this also affects the overall sample rate of the ADC as the ADC has to do multiple
/// samples before a result is ready.
/// 
/// Therefore, the overall formula for calculating Sample rate (SPS) can be calculated like so:
/// 
/// ## For single sample
/// ```
/// SPS = (GCLK_ADC / clk_divider) / (sample_clock_cycles + bit_width)
/// ```
/// ## For multiple samples (Averaging or Summed)
/// ```
/// SPS = (GCLK_ADC / clk_divider) / (n * (sample_clock_cycles + bit_width))
/// ```
pub struct AdcSettingsBuilder {
    pub clk_divider: AdcDivider,
    pub sample_clock_cycles: u8,
    pub bit_width: AdcBitWidth,
    pub accumulation: AdcAccumulation
}

impl AdcSettingsBuilder {
    /// 
    /// Configure the ADC to sample at 250_000 SPS (Assuming the clock source is 48_000_000) using the following settings:
    /// * clock divider factor of 32
    /// * 5 clock cycles per sample
    /// * 12bit sampling
    /// * Single accumulation (No averaging or summing)
    /// 
    pub fn new() -> Self {
        Self {
            clk_divider: AdcDivider::Div32,
            sample_clock_cycles: 5,
            bit_width: AdcBitWidth::Twelve,
            accumulation: AdcAccumulation::Single
        }
    }

    /// 
    /// This setting adjusts the ADC clock frequency by dividing the input clock for the ADC.
    /// 
    /// ## Example:
    /// * Input clock 48MHz, div 32 => ADC Clock is 1.5MHz
    /// 
    pub fn clock_divider(mut self, div: AdcDivider) -> Self {
        self.clk_divider = div;
        self
    }

    /// This setting adjusts the bit width of each ADC sample
    pub fn sample_bit_width(mut self, bit_width: AdcBitWidth) -> Self {
        self.bit_width = bit_width;
        self
    }

    /// Sets how the ADC will accumulate values before actually returning a value.
    /// 
    /// The default is single (ADC will return a sample as soon as it is measured)
    /// 
    /// Setting [AdcAccumulation::Summed] will make the ADC take 'n' samples, and sum the 
    /// total before returning it
    /// 
    /// Setting [AdcAccumulation::Average] will make the ADC take 'n' samples, and average the 
    /// total before returning it
    /// 
    /// NOTE: Selecting [AdcAccumulation::Summed] or [AdcAccumulation::Average] will reduce the overall
    /// ADC sample rate by a factor of 1/n, and the returned value will be 16bits long no matter
    /// what the sample Bit width was selected as
    pub fn accumulation_method(mut self, method: AdcAccumulation) -> Self {
        self.accumulation = method;
        self
    }

    /// This adjusts the number of ADC clock cycles taken to sample a single sample.
    /// The higher this number, the longer it will take the ADC to sample each sample.
    /// 
    /// ## Safety
    /// Internally, this function will clamp the minimum input value to 1 to avoid 0
    /// 
    pub fn clock_cycles_per_sample(mut self, num: u8) -> Self {
        self.sample_clock_cycles = 1.max(num); // Prevent 0
        self
    }

    /// 
    /// Returns a calculated sample rate of the ADC with these settings
    /// 
    pub fn calculate_sps(&self, clock_freq: u32) -> u32 {
        let div = self.clk_divider as u32;
        let adc_clk_freq = clock_freq / div;

        let mut clocks_per_sample = self.sample_clock_cycles as u32 + (self.bit_width as u32);

        let multi = match self.accumulation {
            AdcAccumulation::Single => 1,
            AdcAccumulation::Average(adc_sample_count) => adc_sample_count as u32,
            AdcAccumulation::Summed(adc_sample_count) => adc_sample_count as u32,
        };
        clocks_per_sample *= multi as u32;
        adc_clk_freq / clocks_per_sample
    }
}