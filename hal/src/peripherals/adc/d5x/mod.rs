pub mod pin;

use super::{
    Accumulation, Adc, AdcInstance, Config, Error, Flags, PrimaryAdc, Resolution, SampleCount,
};
use crate::typelevel::NoneT;
use crate::{calibration, pac};

/// ADC instance 0
pub struct Adc0 {
    _adc: pac::Adc0,
}

impl PrimaryAdc for Adc0 {}

impl AdcInstance for Adc0 {
    type Instance = pac::Adc0;

    type ClockId = crate::clock::v2::pclk::ids::Adc0;

    #[cfg(feature = "async")]
    type Interrupt = crate::async_hal::interrupts::ADC0;

    #[inline]
    fn peripheral_reg_block(p: &mut pac::Peripherals) -> &pac::adc0::RegisterBlock {
        &p.adc0
    }

    #[inline]
    fn calibrate(instance: &Self::Instance) {
        instance.calib().write(|w| unsafe {
            w.biascomp().bits(calibration::adc0_biascomp_scale_cal());
            w.biasrefbuf().bits(calibration::adc0_biasref_scale_cal());
            w.biasr2r().bits(calibration::adc0_biasr2r_scale_cal())
        });
    }

    #[cfg(feature = "async")]
    #[inline]
    fn waker() -> &'static embassy_sync::waitqueue::AtomicWaker {
        use super::async_api;
        &async_api::ADC_WAKERS[0]
    }
}

/// ADC instance 0
pub struct Adc1 {
    _adc: pac::Adc1,
}

impl AdcInstance for Adc1 {
    type Instance = pac::Adc1;

    type ClockId = crate::clock::v2::pclk::ids::Adc1;

    #[cfg(feature = "async")]
    type Interrupt = crate::async_hal::interrupts::ADC1;

    #[inline]
    fn peripheral_reg_block(p: &mut pac::Peripherals) -> &pac::adc0::RegisterBlock {
        &p.adc1
    }

    #[inline]
    fn calibrate(instance: &Self::Instance) {
        instance.calib().write(|w| unsafe {
            w.biascomp().bits(calibration::adc1_biascomp_scale_cal());
            w.biasrefbuf().bits(calibration::adc1_biasref_scale_cal());
            w.biasr2r().bits(calibration::adc1_biasr2r_scale_cal())
        });
    }

    #[cfg(feature = "async")]
    #[inline]
    fn waker() -> &'static embassy_sync::waitqueue::AtomicWaker {
        use super::async_api;
        &async_api::ADC_WAKERS[1]
    }
}

impl<I: AdcInstance> Adc<I, NoneT> {
    #[inline]
    pub fn configure(&mut self, config: Config) -> Result<(), super::Error> {
        // Reset ADC here as we cannot guarantee its state
        // This also disables the ADC
        self.software_reset();
        self.sync();
        I::calibrate(&self.adc);
        self.sync();
        self.adc
            .ctrla()
            .modify(|_, w| w.prescaler().variant(config.clk_divider));
        self.sync();
        self.adc
            .ctrlb()
            .modify(|_, w| w.ressel().variant(config.bit_width));
        self.sync();

        self.adc
            .sampctrl()
            .modify(|_, w| unsafe { w.samplen().bits(config.sample_clock_cycles) }); // sample length
        self.sync();
        self.adc.inputctrl().modify(|_, w| {
            w.muxneg().gnd();
            w.diffmode().clear_bit()
        }); // No negative input (internal gnd)
        self.sync();

        // Check bit width selected
        if config.accumulation != Accumulation::Single && config.bit_width != Resolution::_16bit {
            return Err(super::Error::InvalidSampleBitWidth);
        }

        let (sample_cnt, adjres) = match config.accumulation {
            // 1 sample to be used as is
            Accumulation::Single => (SampleCount::_1, 0),
            // A total of `adc_sample_count` elements will be averaged by the ADC
            // before it returns the result
            // Table 45-3 SAMx5x datasheet
            Accumulation::Average(cnt) => (cnt, core::cmp::min(cnt as u8, 0x04)),
            // A total of `adc_sample_count` elements will be summed by the ADC
            // before it returns the result
            Accumulation::Summed(cnt) => (cnt, 0),
        };
        self.adc.avgctrl().modify(|_, w| {
            w.samplenum().variant(sample_cnt);
            unsafe { w.adjres().bits(adjres) }
        });
        self.sync();

        self.set_reference(config.vref);
        self.sync();

        self.disable_freerunning();

        self.power_up();

        Ok(())
    }
}

impl<I: AdcInstance + PrimaryAdc, F> Adc<I, F> {
    #[inline]
    fn tp_tc_to_temp(&self, tp: f32, tc: f32) -> f32 {
        let tl = calibration::tl();
        let th = calibration::th();
        let vpl = calibration::vpl() as f32;
        let vph = calibration::vph() as f32;
        let vcl = calibration::vcl() as f32;
        let vch = calibration::vch() as f32;

        (tl * vph * tc - vpl * th * tc - tl * vch * tp + th * vcl * tp)
            / (vcl * tp - vch * tp - vpl * tc + vph * tc)
    }

    #[inline]
    /// Returns the CPU temperature in degrees C
    ///
    /// This requires that the [pac::Supc] peripheral is configured with
    /// tsen and ondemand bits enabled, otherwise this function will return
    /// [Error::TemperatureSensorNotEnabled]
    pub fn read_cpu_temperature_blocking(&mut self, supc: &pac::Supc) -> Result<f32, Error> {
        let vref = supc.vref().read();
        if vref.tsen().bit_is_clear() || vref.ondemand().bit_is_clear() {
            return Err(Error::TemperatureSensorNotEnabled);
        }
        let mut tp = self.read_blocking_channel(0x1C) as f32;
        let mut tc = self.read_blocking_channel(0x1D) as f32;

        if let Accumulation::Summed(sum) = self.cfg.accumulation {
            // to prevent incorrect readings, divide by number of samples if the
            // ADC was already configured in summation mode
            let div: f32 = (2u16.pow(sum as u32)) as f32;
            tp /= div;
            tc /= div;
        }
        Ok(self.tp_tc_to_temp(tp, tc))
    }
}

impl<I: AdcInstance, T> Adc<I, T> {
    #[inline]
    pub(super) fn sync(&self) {
        // Slightly more performant than checking the individual bits
        // since we avoid an extra instruction to bit shift
        while self.adc.syncbusy().read().bits() != 0 {
            core::hint::spin_loop();
        }
    }

    #[inline]
    pub(super) fn power_up(&mut self) {
        self.adc.ctrla().modify(|_, w| w.enable().set_bit());
        self.sync();
    }

    #[inline]
    #[allow(dead_code)]
    pub(super) fn power_down(&mut self) {
        self.adc.ctrla().modify(|_, w| w.enable().clear_bit());
        self.sync();
    }

    #[inline]
    pub(super) fn start_conversion(&mut self) {
        // The double trigger here is in case the VREF value changed between
        // reads, this discards the conversion made just after the VREF changed,
        // which the data sheet tells us to do in order to not get a faulty reading
        // right after changing VREF value
        self.adc.swtrig().modify(|_, w| w.start().set_bit());
        self.sync();
        self.adc.intflag().write(|w| w.resrdy().set_bit()); // Clear RESRDY
        self.adc.swtrig().modify(|_, w| w.start().set_bit());
    }

    #[inline]
    pub(super) fn enable_freerunning(&mut self) {
        self.adc.ctrlb().modify(|_, w| w.freerun().set_bit());
        self.sync();
    }

    #[inline]
    pub(super) fn disable_freerunning(&mut self) {
        self.adc.ctrlb().modify(|_, w| w.freerun().clear_bit());
        self.sync();
    }

    #[inline]
    pub(super) fn read_flags(&self) -> Flags {
        let bits = self.adc.intflag().read().bits();
        Flags::from_bits_truncate(bits)
    }

    #[inline]
    pub(super) fn clear_flags(&mut self, flags: Flags) {
        unsafe {
            self.adc.intflag().write(|w| w.bits(flags.bits()));
        }
    }

    /// Check the interrupt flags, clears them and returns `Err` if an overflow
    /// occured
    #[inline]
    pub(super) fn check_and_clear_flags(&mut self, flags: Flags) -> Result<(), Error> {
        // Keep a copy around so we can check for errors later
        let flags_to_clear = flags;
        self.clear_flags(flags_to_clear);

        if flags.contains(Flags::OVERRUN) {
            Err(Error::BufferOverrun)
        } else {
            Ok(())
        }
    }

    /// Enables an interrupt when conversion is ready.
    #[inline]
    #[allow(dead_code)]
    pub(super) fn enable_interrupts(&mut self, flags: Flags) {
        unsafe { self.adc.intenset().write(|w| w.bits(flags.bits())) };
    }

    /// Disables the interrupt for when conversion is ready.
    #[inline]
    pub(super) fn disable_interrupts(&mut self, flags: Flags) {
        unsafe { self.adc.intenclr().write(|w| w.bits(flags.bits())) };
    }

    #[inline]
    pub(super) fn conversion_result(&self) -> u16 {
        self.adc.result().read().result().bits()
    }

    #[inline]
    pub(super) fn mux(&mut self, ch: u8) {
        self.adc
            .inputctrl()
            .modify(|_, w| unsafe { w.muxpos().bits(ch) });
        self.sync()
    }
}
