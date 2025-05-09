use crate::typelevel::NoneT;

use super::{Accumulation, Adc, AdcInstance, Error, Flags, PrimaryAdc, SampleCount};

use crate::{calibration, pac};
use pac::adc::inputctrl::Gainselect;
use pac::Peripherals;
pub mod pin;

/// Wrapper around the ADC instance
pub struct Adc0 {
    _adc: pac::Adc,
}

impl PrimaryAdc for Adc0 {}

impl AdcInstance for Adc0 {
    type Instance = pac::Adc;

    #[cfg(feature = "async")]
    type Interrupt = crate::async_hal::interrupts::ADC;

    #[inline]
    fn peripheral_reg_block(p: &mut Peripherals) -> &pac::adc::RegisterBlock {
        &p.adc
    }

    #[inline]
    fn enable_pm(pm: &mut pac::Pm) {
        pm.apbcmask().modify(|_, w| w.adc_().set_bit());
    }

    #[inline]
    fn calibrate(instance: &Self::Instance) {
        instance.calib().write(|w| unsafe {
            w.bias_cal().bits(calibration::adc_bias_cal());
            w.linearity_cal().bits(calibration::adc_linearity_cal())
        });
    }

    #[cfg(feature = "async")]
    #[inline]
    fn waker() -> &'static embassy_sync::waitqueue::AtomicWaker {
        use super::async_api;
        &async_api::ADC_WAKERS[0]
    }
}

impl<I: AdcInstance> Adc<I, NoneT> {
    #[inline]
    pub fn configure(&mut self) -> Result<(), super::Error> {
        // Reset ADC here as we cannot guarantee its state
        // This also disables the ADC
        self.software_reset();
        I::calibrate(&self.adc);

        self.adc.ctrlb().modify(|_, w| {
            w.prescaler().variant(self.cfg.clk_divider);
            w.ressel().variant(self.cfg.accumulation.bits())
        });
        self.sync();

        self.adc
            .sampctrl()
            .modify(|_, w| unsafe { w.samplen().bits(self.cfg.sample_clock_cycles) }); // sample length

        self.adc.inputctrl().modify(|_, w| {
            // No negative input (internal gnd)
            w.muxneg().gnd();
            w.gain().variant(Gainselect::Div2)
        });
        self.sync();

        let (sample_count, adjres) = match self.cfg.accumulation {
            // 1 sample to be used as is
            Accumulation::Single(_) => (SampleCount::_1, 0),
            // A total of `adc_sample_count` elements will be averaged by the ADC
            // before it returns the result
            // SAMD21 datasheet table 32-3 / SAMD11 datasheet table 31-3
            Accumulation::Average(cnt) => (cnt, core::cmp::min(cnt as u8, 0x04)),
            // A total of `adc_sample_count` elements will be summed by the ADC
            // before it returns the result
            Accumulation::Summed(cnt) => (cnt, 0),
        };

        self.adc.avgctrl().modify(|_, w| {
            w.samplenum().variant(sample_count);
            unsafe { w.adjres().bits(adjres) }
        });
        self.sync();

        self.set_reference(self.cfg.vref);
        self.sync();

        self.disable_freerunning();

        self.power_up();

        Ok(())
    }
}

impl<I: AdcInstance + PrimaryAdc, F> Adc<I, F> {}

impl<I: AdcInstance, T> Adc<I, T> {
    #[inline]
    pub(super) fn sync(&self) {
        while self.adc.status().read().syncbusy().bit_is_set() {
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
