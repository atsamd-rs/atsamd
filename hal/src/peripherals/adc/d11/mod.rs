use crate::typelevel::NoneT;

use super::{Adc, AdcAccumulation, AdcInstance, Config, Error, Flags, PrimaryAdc};

use crate::pac;
use pac::adc::avgctrl::Samplenumselect;
use pac::adc::ctrlb::Resselselect;
use pac::adc::inputctrl::Gainselect;
use pac::Peripherals;
pub mod pin;

pub struct Adc0 {
    _adc: pac::Adc,
}

impl PrimaryAdc for Adc0 {}

impl AdcInstance for Adc0 {
    type Instance = pac::Adc;
    type Clock = crate::clock::AdcClock;

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
    fn calibrate(_instance: &Self::Instance) {}

    #[cfg(feature = "async")]
    #[inline]
    fn waker() -> &'static embassy_sync::waitqueue::AtomicWaker {
        use super::async_api;
        &async_api::ADC_WAKERS[0]
    }
}

impl<I: AdcInstance> Adc<I, NoneT> {
    #[inline]
    pub fn configure(&mut self, config: Config) -> Result<(), super::Error> {
        // Reset ADC here as we cannot guarantee its state
        // This also disables the ADC
        self.software_reset();
        I::calibrate(&self.adc);
        self.sync();
        self.adc
            .ctrlb()
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
            w.gain().variant(Gainselect::Div2)
        }); // No negative input (internal gnd)
        self.sync();

        // Check bit width selected
        if config.accumulation != AdcAccumulation::Single
            && config.bit_width != Resselselect::_16bit
        {
            return Err(super::Error::InvalidSampleBitWidth);
        }
        match config.accumulation {
            AdcAccumulation::Single => {
                // 1 sample to be used as is
                self.adc.avgctrl().modify(|_, w| {
                    w.samplenum().variant(Samplenumselect::_1);
                    unsafe { w.adjres().bits(0) }
                });
            }
            AdcAccumulation::Average(adc_sample_count) => {
                // A total of `adc_sample_count` elements will be averaged by the ADC
                // before it returns the result
                self.adc.avgctrl().modify(|_, w| {
                    w.samplenum().variant(adc_sample_count);
                    unsafe {
                        // Table 45-3 SAME51 datasheet
                        w.adjres()
                            .bits(core::cmp::min(adc_sample_count as u8, 0x04))
                    }
                });
            }
            AdcAccumulation::Summed(adc_sample_count) => {
                // A total of `adc_sample_count` elements will be summed by the ADC
                // before it returns the result
                self.adc.avgctrl().modify(|_, w| {
                    w.samplenum().variant(adc_sample_count);
                    unsafe { w.adjres().bits(0) }
                });
            }
        }

        self.sync();
        self.set_reference(config.vref);
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
        self.adc.swtrig().modify(|_, w| w.start().set_bit());
    }

    #[inline]
    pub(super) fn enable_freerunning(&mut self) {
        self.adc.ctrlb().modify(|_, w| w.freerun().set_bit());
        self.sync();
    }

    #[inline]
    pub(super) fn disable_freerunning(&mut self) {
        self.adc.ctrlb().modify(|_, w| w.freerun().set_bit());
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
