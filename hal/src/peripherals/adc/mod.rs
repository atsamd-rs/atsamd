//! Analog-to-Digital Converter

use core::ops::Deref;

use atsamd_hal_macros::{hal_cfg, hal_module};
use pac::Peripherals;

use crate::{gpio::AnyPin, pac, typelevel::Sealed};

#[hal_module(
    any("adc-d11", "adc-d21") => "d11/mod.rs",
    "adc-d5x" => "d5x/mod.rs",
)]
mod impls {}

pub use impls::*;

#[cfg(feature = "async")]
mod async_api;
#[cfg(feature = "async")]
pub use async_api::*;

mod builder;
pub use builder::*;

#[hal_cfg(any("adc-d11", "adc-d21"))]
use crate::pac::adc as adc0;
#[hal_cfg("adc-d5x")]
use crate::pac::adc0;

pub use adc0::refctrl::Refselselect as Reference;

/// ADC Settings when reading Internal sensors (Like VREF and Temperatures)
/// These settings are based on the minimums suggested in the datasheet
const ADC_SETTINGS_INTERNAL_READ: AdcSettings = AdcSettings {
    clk_divider: Prescaler::Div64,
    sample_clock_cycles: 6,
    accumulation: Accumulation::Single(AdcResolution::_12),
    vref: Reference::Intvcc1,
};

/// Based on Temperature log row information (NVM)x
#[hal_cfg(any("adc-d21", "adc-d11"))]
const ADC_SETTINGS_INTERNAL_READ_D21_TEMP: AdcSettings = AdcSettings {
    clk_divider: Prescaler::Div256,
    sample_clock_cycles: 64,
    accumulation: Accumulation::Single(AdcResolution::_12),
    vref: Reference::Int1v,
};

/// Errors that may occur when operating the ADC
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Clock too fast.
    ///
    /// The ADC requires that it's fed a GCLK that does not exceed a certain
    /// frequency. These maximums are:
    ///
    /// * **SAMD/E5x** - 100Mhz
    /// * **SAMC/D21** - 48Mhz
    /// * **SAMD11** - 48Mhz
    ///
    /// SAMx51 specific: If you are running the CPU at temperatures past 100C,
    /// then the maximum GCLK clock speed should be 90Mhz
    ClockTooFast,
    /// Buffer overflowed
    BufferOverrun,
    /// Temperature sensor not enabled
    ///
    /// This is returned when attempting to read the CPU temperature, and
    /// the SUPC peripheral has not been configured correctly to expose
    /// the temperature sensors.
    TemperatureSensorNotEnabled,
}

/// Voltage source to use when using the ADC to measure the CPU voltage
#[hal_cfg("adc-d5x")]
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum CpuVoltageSource {
    /// Core voltage
    Core = 0x18,
    /// VBAT supply voltage
    Vbat = 0x19,
    /// IO supply voltage
    Io = 0x1A,
}

/// Voltage source to use when using the ADC to measure the CPU voltage
#[hal_cfg(any("adc-d21", "adc-d11"))]
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum CpuVoltageSource {
    /// Core voltage
    Core = 0x1A,
    /// VBAT supply voltage
    Vbat = 0x1B,
}

bitflags::bitflags! {
    /// ADC interrupt flags
    #[derive(Clone, Copy)]
    pub struct Flags: u8 {
        /// Window monitor interrupt
        const WINMON = 0x04;
        /// Buffer overrun interrupt
        const OVERRUN = 0x02;
        /// Result ready interrupt
        const RESRDY = 0x01;
    }
}

/// Marker for which ADC has access to the CPUs internal sensors
pub trait PrimaryAdc {}

/// Trait representing an ADC instance
pub trait AdcInstance {
    #[cfg(feature = "async")]
    type Interrupt: crate::async_hal::interrupts::InterruptSource;

    // The Adc0 and Adc1 PAC types implement Deref
    type Instance: Deref<Target = adc0::RegisterBlock>;

    #[hal_cfg("adc-d5x")]
    type ClockId: crate::clock::v2::apb::ApbId + crate::clock::v2::pclk::PclkId;

    fn peripheral_reg_block(p: &mut Peripherals) -> &adc0::RegisterBlock;

    #[hal_cfg(any("adc-d11", "adc-d21"))]
    fn enable_pm(pm: &mut pac::Pm);

    fn calibrate(instance: &Self::Instance);

    #[cfg(feature = "async")]
    fn waker() -> &'static embassy_sync::waitqueue::AtomicWaker;
}

/// Trait representing a GPIO pin which can be used as an input for an ADC
pub trait AdcPin<I>: AnyPin<Mode = crate::gpio::AlternateB> + Sealed
where
    I: AdcInstance,
{
    const CHANNEL: u8;
}

/// ADC Instance
#[hal_cfg(any("adc-d11", "adc-d21"))]
pub struct Adc<I: AdcInstance> {
    adc: I::Instance,
    cfg: AdcSettings,
}

/// ADC Instance
#[hal_cfg("adc-d5x")]
pub struct Adc<I: AdcInstance> {
    adc: I::Instance,
    _apbclk: crate::clock::v2::apb::ApbClk<I::ClockId>,
    cfg: AdcSettings,
}

#[cfg(feature = "async")]
pub struct FutureAdc<I: AdcInstance, F> {
    inner: Adc<I>,
    irqs: F,
}

impl<I: AdcInstance> Adc<I> {
    /// Construct a new ADC instance
    ///
    /// ## Important
    ///
    /// This function will return `Err` if the clock source provided
    /// is faster than 100 MHz, since this is the maximum frequency for
    /// GCLK_ADCx as per the datasheet.
    ///
    /// The [`new`](Self::new) function currently takes an `&` reference to a
    /// [`Pclk`](crate::clock::v2::pclk::Pclk). In the future this will likely
    /// change to taking full ownership of it; in the meantime, you must ensure
    /// that the PCLK is enabled for the `Adc` struct's lifetime.
    ///
    /// NOTE: If you plan to run the chip above 100°C, then the maximum GCLK
    /// frequency for the ADC is restricted to 90Mhz for stable performance.
    #[hal_cfg("adc-d5x")]
    #[inline]
    pub(crate) fn new<PS: crate::clock::v2::pclk::PclkSourceId>(
        adc: I::Instance,
        settings: AdcSettings,
        clk: crate::clock::v2::apb::ApbClk<I::ClockId>,
        pclk: &crate::clock::v2::pclk::Pclk<I::ClockId, PS>,
    ) -> Result<Self, Error> {
        // TODO: Ideally, the ADC struct would take ownership of the Pclk type here.
        // However, since clock::v2 is not implemented for all chips yet, the
        // generics for the Adc type would be different between chip families,
        // leading to massive and unnecessary code duplication. In the meantime,
        // we use a "lite" variation of the typelevel guarantees laid out by the
        // clock::v2 module, meaning that we can guarantee that the clocks are enabled
        // at the time of creation of the Adc struct; however we can't guarantee
        // that the clock will stay enabled for the duration of its lifetime.

        if pclk.freq() > fugit::HertzU32::from_raw(100_000_000) {
            // Clock source is too fast
            return Err(Error::ClockTooFast);
        }

        let mut new_adc = Self {
            adc,
            _apbclk: clk,
            cfg: settings,
        };
        new_adc.configure(settings);
        Ok(new_adc)
    }

    /// Construct a new ADC instance
    ///
    /// ## Important
    ///
    /// This function will return [Error::ClockTooFast] if the clock source
    /// provided is faster than 48 MHz, since this is the maximum frequency
    /// for the ADC as per the datasheet.
    #[hal_cfg(any("adc-d11", "adc-d21"))]
    #[inline]
    pub(crate) fn new(
        adc: I::Instance,
        settings: AdcSettings,
        pm: &mut pac::Pm,
        clock: &crate::clock::AdcClock,
    ) -> Result<Self, Error> {
        if (clock.freq() as crate::time::Hertz).to_Hz() > 48_000_000 {
            // Clock source is too fast
            return Err(Error::ClockTooFast);
        }

        I::enable_pm(pm);
        let mut new_adc = Self { adc, cfg: settings };
        new_adc.configure(settings);
        Ok(new_adc)
    }

    /// Switch the ['Adc'] to ['FutureAdc'], allowing for the use of async
    /// reading methods. You are required to provide the struct created by
    /// the [`bind_interrupts`](crate::bind_interrupts) macro to prove
    /// that the interrupt sources have been correctly configured. This function
    /// will automatically enable the relevant NVIC interrupt sources. However,
    /// you are required to configure the desired interrupt priorities prior to
    /// calling this method. Consult [`crate::async_hal::interrupts`]
    /// module-level documentation for more information.
    /// [`bind_interrupts`](crate::bind_interrupts).
    #[cfg(feature = "async")]
    #[atsamd_hal_macros::hal_macro_helper]
    #[inline]
    pub fn into_future<F>(self, irqs: F) -> FutureAdc<I, F>
    where
        F: crate::async_hal::interrupts::Binding<I::Interrupt, async_api::InterruptHandler<I>>,
    {
        use crate::async_hal::interrupts::InterruptSource;
        unsafe {
            I::Interrupt::unpend();
            I::Interrupt::enable();
        }
        FutureAdc { inner: self, irqs }
    }
}

impl<I: AdcInstance> Adc<I> {
    /// Converts our ADC Reading (0-n) to the range 0.0-1.0, where
    /// 1.0 = 2^(reading_bitwidth)
    fn reading_to_f32(&self, raw: u16) -> f32 {
        let max = match self.cfg.accumulation.resolution() {
            Resolution::_16bit => 65535,
            Resolution::_12bit => 4095,
            Resolution::_10bit => 1023,
            Resolution::_8bit => 255,
        };
        raw as f32 / max as f32
    }

    /// Runs something using the ADC with overridden settings
    ///
    /// This is used mainly for internal voltage readings, where the ADC
    /// must be configured with specific settings for optimal and accurate
    /// reading
    pub(crate) fn with_specific_settings<F: FnOnce(&mut Adc<I>) -> T, T>(
        &mut self,
        settings: AdcSettings,
        f: F,
    ) -> T {
        let old_cfg = self.cfg;
        self.configure(settings);
        let ret = f(self);
        self.configure(old_cfg);
        ret
    }

    #[inline]
    fn set_reference(&mut self, reference: Reference) {
        self.adc
            .refctrl()
            .modify(|_, w| w.refsel().variant(reference));
        self.sync();
    }

    /// Read a single value from the provided ADC pin.
    #[inline]
    pub fn read<P: AdcPin<I>>(&mut self, _pin: &mut P) -> u16 {
        self.read_channel(P::CHANNEL)
    }

    /// Read a single value from the provided channel, in a blocking fashion
    #[inline]
    fn read_channel(&mut self, ch: u8) -> u16 {
        // Clear overrun errors that might've occured before we try to read anything
        self.clear_all_flags();
        self.disable_interrupts(Flags::all());
        self.disable_freerunning();
        self.sync();
        self.mux(ch);
        self.power_up();
        self.start_conversion();
        while !self.read_flags().contains(Flags::RESRDY) {
            core::hint::spin_loop();
        }
        let res = self.conversion_result();
        self.power_down();
        res
    }

    /// Read into a buffer from the provided ADC pin, in a blocking fashion
    #[inline]
    pub fn read_buffer<P: AdcPin<I>>(
        &mut self,
        _pin: &mut P,
        dst: &mut [u16],
    ) -> Result<(), Error> {
        self.read_buffer_channel(P::CHANNEL, dst)
    }

    /// Read into a buffer from the provided channel, in a blocking fashion
    #[inline]
    fn read_buffer_channel(&mut self, ch: u8, dst: &mut [u16]) -> Result<(), Error> {
        // Clear overrun errors that might've occured before we try to read anything
        self.clear_all_flags();

        self.disable_interrupts(Flags::all());
        self.mux(ch);

        self.enable_freerunning();
        self.power_up();
        self.start_conversion();

        for result in dst.iter_mut() {
            while !self.read_flags().contains(Flags::RESRDY) {
                core::hint::spin_loop();
            }

            let flags = self.read_flags();
            self.clear_all_flags();
            if let Err(e) = self.check_overrun(&flags) {
                self.power_down();
                self.disable_freerunning();

                return Err(e);
            }

            *result = self.conversion_result();
        }
        self.power_down();
        self.disable_freerunning();

        Ok(())
    }

    /// Return the underlying ADC PAC object.
    #[hal_cfg(any("adc-d11", "adc-d21"))]
    #[inline]
    pub fn free(mut self) -> I::Instance {
        self.software_reset();
        self.adc
    }

    /// Return the underlying ADC PAC object and the enabled APB ADC clock.
    #[hal_cfg("adc-d5x")]
    #[inline]
    pub fn free(mut self) -> (I::Instance, crate::clock::v2::apb::ApbClk<I::ClockId>) {
        self.software_reset();
        (self.adc, self._apbclk)
    }

    /// Reset the peripheral.
    ///
    /// This also disables the ADC.
    #[inline]
    fn software_reset(&mut self) {
        self.adc.ctrla().modify(|_, w| w.swrst().set_bit());
        self.sync();
    }
}

#[cfg(feature = "async")]
/// Implementation for async mode only methods
impl<I: AdcInstance, F> FutureAdc<I, F>
where
    F: crate::async_hal::interrupts::Binding<I::Interrupt, async_api::InterruptHandler<I>>,
{
    /// Convert the Async ADC back into a Blocking ADC, and return
    /// the IRQs
    pub fn into_blocking(self) -> (Adc<I>, F) {
        (self.inner, self.irqs)
    }

    /// Read a single value from the provided ADC pin.
    #[inline]
    pub async fn read<P: AdcPin<I>>(&mut self, _pin: &mut P) -> u16 {
        self.read_channel(P::CHANNEL).await
    }

    /// Read a single value from the provided channel ID
    #[inline]
    async fn read_channel(&mut self, ch: u8) -> u16 {
        // Clear overrun errors that might've occured before we try to read anything
        self.inner.clear_all_flags();
        self.inner.disable_freerunning();
        self.inner.mux(ch);
        self.inner.power_up();
        self.inner.start_conversion();
        // Here we explicitly ignore the result, because we know that
        // overrun errors are impossible since the ADC is configured in one-shot mode.
        let _ = self.wait_flags(Flags::RESRDY).await;
        let res = self.inner.conversion_result();
        self.inner.power_down();
        self.inner.sync();
        res
    }

    /// Read into a buffer from the provided ADC pin
    #[inline]
    pub async fn read_buffer<P: AdcPin<I>>(
        &mut self,
        _pin: &mut P,
        dst: &mut [u16],
    ) -> Result<(), Error> {
        self.read_buffer_channel(P::CHANNEL, dst).await
    }

    /// Read into a buffer from the provided channel ID
    #[inline]
    async fn read_buffer_channel(&mut self, ch: u8, dst: &mut [u16]) -> Result<(), Error> {
        // Clear overrun errors that might've occured before we try to read anything
        self.inner.clear_all_flags();

        self.inner.mux(ch);

        self.inner.enable_freerunning();
        self.inner.power_up();
        self.inner.start_conversion();

        for result in dst.iter_mut() {
            if let Err(e) = self.wait_flags(Flags::RESRDY).await {
                self.inner.power_down();
                self.inner.disable_freerunning();

                return Err(e);
            }
            *result = self.inner.conversion_result();
        }

        self.inner.power_down();
        self.inner.disable_freerunning();

        Ok(())
    }
}
