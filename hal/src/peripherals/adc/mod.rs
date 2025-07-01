//! Analog-to-Digital Converter
//!
//! This module provides an interface to the Analog-to-Digital Converter (ADC)
//! peripheral(s).  Support is provided for single-ended, software triggered
//! conversions.  Async functionality can be enabled via the `async` feature.
//!
//! ```
//! # use atsamd_hal::adc::{AdcResolution, Reference};
//! let apb_adc0 = buses.apb.enable(tokens.apbs.adc0);
//! let (pclk_adc0, _gclk0) = Pclk::enable(tokens.pclks.adc0, clocks.gclk0);
//!
//! let mut adc = AdcBuilder::new(Accumulation::single(AdcResolution::_12))
//!     .with_clock_cycles_per_sample(5)
//!     .with_clock_divider(Prescaler::Div32)
//!     .with_vref(Reference::Arefa)
//!     .enable(peripherals.adc0, apb_adc0, &pclk_adc0)
//!     .unwrap();
//!
//! let mut adc_pin = pins.a0.into_alternate();
//!
//! let mut _buffer = [0; 16];
//! adc.read_buffer(&mut adc_pin, &mut _buffer).unwrap();
//! ```

use crate::{gpio::AnyPin, pac, typelevel::Sealed};
use atsamd_hal_macros::{hal_cfg, hal_module};
use core::ops::Deref;
use pac::Peripherals;

#[cfg(feature = "dma")]
use pac::dmac;

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

#[cfg(feature = "dma")]
#[hal_cfg("adc-d5x")]
use dmac::channel::chctrla::Trigsrcselect as TriggerSelect;

#[cfg(feature = "dma")]
#[hal_cfg(any("adc-d11", "adc-d21"))]
use dmac::chctrlb::Trigsrcselect as TriggerSelect;

pub use adc0::refctrl::Refselselect as Reference;

/// ADC Settings when reading Internal sensors (Like VREF and Temperatures)
/// These settings are based on the minimums suggested in the datasheet
const ADC_SETTINGS_INTERNAL_READ: AdcSettings = AdcSettings {
    clk_divider: Prescaler::Div64,
    sample_clock_cycles: 32,
    accumulation: Accumulation::average(SampleCount::_4),
    vref: Reference::Intvcc1,
};

/// Based on Temperature log row information (NVM)x
#[hal_cfg(any("adc-d21", "adc-d11"))]
const ADC_SETTINGS_INTERNAL_READ_D21_TEMP: AdcSettings = AdcSettings {
    clk_divider: Prescaler::Div64,
    sample_clock_cycles: 32,
    accumulation: Accumulation::average(SampleCount::_4),
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
    /// DMA Error
    #[cfg(feature = "dma")]
    DmaError(crate::dmac::Error),
}

#[cfg(feature = "dma")]
impl From<crate::dmac::Error> for Error {
    fn from(value: crate::dmac::Error) -> Self {
        Self::DmaError(value)
    }
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
    /// Bandgap reference voltage - 1.1V
    Bandgap = 0x19,
    /// Core voltage - 1.2V
    Core = 0x1A,
    /// IO voltage - 1.62V to 3.63V
    Io = 0x1B,
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

    #[cfg(feature = "dma")]
    const DMA_TRIGGER: TriggerSelect;

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
    discard: bool,
}

/// ADC Instance
#[hal_cfg("adc-d5x")]
pub struct Adc<I: AdcInstance> {
    adc: I::Instance,
    _apbclk: crate::clock::v2::apb::ApbClk<I::ClockId>,
    cfg: AdcSettings,
    discard: bool,
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
            discard: true,
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
        let mut new_adc = Self {
            adc,
            cfg: settings,
            discard: true,
        };
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
        let max = match self.cfg.accumulation.output_resolution() {
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
        self.check_read_discard();
        self.start_conversion();
        while !self.read_flags().contains(Flags::RESRDY) {
            core::hint::spin_loop();
        }
        self.conversion_result()
    }

    // If the ADC has to discard the next value, then we try to read it
    // and then discard it
    #[inline]
    pub fn check_read_discard(&mut self) {
        if self.discard {
            self.start_conversion();
            while !self.read_flags().contains(Flags::RESRDY) {
                core::hint::spin_loop();
            }
            self.discard = false;
        }
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
        if self.inner.discard {
            // Read and discard if something was changed
            self.inner.start_conversion();
            let _ = self.wait_flags(Flags::RESRDY).await;
            self.inner.discard = false;
            let _ = self.inner.conversion_result();
        }
        self.inner.start_conversion();
        // Here we explicitly ignore the result, because we know that
        // overrun errors are impossible since the ADC is configured in one-shot mode.
        let _ = self.wait_flags(Flags::RESRDY).await;
        let res = self.inner.conversion_result();
        //self.inner.power_down();
        self.inner.sync();
        res
    }
}

#[cfg(feature = "dma")]
mod dma {
    use super::*;
    #[cfg(feature = "async")]
    use crate::dmac::ReadyFuture;
    use crate::dmac::{AnyChannel, Buffer, Ready};

    use atsamd_hal_macros::hal_macro_helper;
    #[hal_cfg("adc-d5x")]
    use pac::dmac::channel::chctrla::Trigactselect as TriggerAction;

    #[hal_cfg(any("adc-d11", "adc-d21"))]
    use pac::dmac::chctrlb::Trigactselect as TriggerAction;

    pub struct AdcDmaPtr(pub *mut u16);

    unsafe impl Buffer for AdcDmaPtr {
        type Beat = u16;

        fn dma_ptr(&mut self) -> *mut Self::Beat {
            self.0
        }

        fn incrementing(&self) -> bool {
            false
        }

        fn buffer_len(&self) -> usize {
            1
        }
    }

    impl<I: AdcInstance> Adc<I> {
        /// Read into a buffer from the provided ADC pin using DMA
        /// in a blocking fashion
        #[inline]
        pub fn read_buffer<CH, P: AdcPin<I>>(
            &mut self,
            _pin: &mut P,
            dst: &mut [u16],
            channel: &mut CH,
        ) -> Result<(), Error>
        where
            CH: AnyChannel<Status = Ready>,
        {
            self.read_buffer_channel(P::CHANNEL, dst, channel)
        }

        /// Read into a buffer from the provided channel, in a blocking fashion
        #[inline]
        #[hal_macro_helper]
        fn read_buffer_channel<CH>(
            &mut self,
            ch: u8,
            mut dst: &mut [u16],
            channel: &mut CH,
        ) -> Result<(), Error>
        where
            CH: AnyChannel<Status = Ready>,
        {
            // Clear overrun errors that might've occured before we try to read anything
            self.clear_all_flags();
            self.disable_interrupts(Flags::all());
            self.mux(ch);
            self.enable_freerunning();
            self.start_conversion();
            if self.discard {
                // Discard first result
                while !self.read_flags().contains(Flags::RESRDY) {
                    core::hint::spin_loop();
                }
                self.clear_all_flags();
                self.discard = false;
            }
            // Now read via DMA
            let mut src = AdcDmaPtr(self.adc.result().as_ptr());

            #[hal_cfg(any("adc-d5x"))]
            let action = TriggerAction::Burst;
            #[hal_cfg(any("adc-d11", "adc-d21"))]
            let action = TriggerAction::Beat;
            // SAFETY: We must make sure that any DMA transfer is complete or stopped before
            // returning.
            unsafe {
                channel
                    .as_mut()
                    .transfer(&mut src, &mut dst, I::DMA_TRIGGER, action, None)?;
            }
            while !channel.as_mut().xfer_complete() {
                core::hint::spin_loop();
            }
            channel.as_mut().stop();
            channel.as_mut().xfer_success()?;
            self.disable_freerunning();
            Ok(())
        }
    }

    #[cfg(feature = "async")]
    impl<I: AdcInstance, F> FutureAdc<I, F>
    where
        F: crate::async_hal::interrupts::Binding<I::Interrupt, async_api::InterruptHandler<I>>,
    {
        /// Read into a buffer from the provided ADC pin using DMA
        #[inline]
        pub async fn read_buffer<CH, P: AdcPin<I>>(
            &mut self,
            _pin: &mut P,
            dst: &mut [u16],
            channel: &mut CH,
        ) -> Result<(), Error>
        where
            CH: AnyChannel<Status = ReadyFuture>,
        {
            self.read_buffer_channel(P::CHANNEL, dst, channel).await
        }

        #[inline]
        #[hal_macro_helper]
        async fn read_buffer_channel<CH>(
            &mut self,
            ch: u8,
            dst: &mut [u16],
            channel: &mut CH,
        ) -> Result<(), Error>
        where
            CH: AnyChannel<Status = ReadyFuture>,
        {
            // Clear overrun errors that might've occured before we try to read anything
            self.inner.clear_all_flags();
            self.inner.mux(ch);
            self.inner.enable_freerunning();

            if self.inner.discard {
                // Discard first result
                let _ = self.wait_flags(Flags::RESRDY).await;
                let _ = self.inner.conversion_result();
                self.inner.discard = false;
                self.inner.clear_all_flags();
            }
            let src = AdcDmaPtr(self.inner.adc.result().as_ptr());
            #[hal_cfg(any("adc-d5x"))]
            let action = TriggerAction::Burst;
            #[hal_cfg(any("adc-d11", "adc-d21"))]
            let action = TriggerAction::Beat;
            channel
                .as_mut()
                .transfer_future(src, dst, I::DMA_TRIGGER, action)
                .await?;

            self.inner.disable_freerunning();
            Ok(())
        }
    }
}
