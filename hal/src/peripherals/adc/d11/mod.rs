use super::{
    Accumulation, Adc, AdcInstance, AdcSettings, CpuVoltageSource, Error, Flags, PrimaryAdc,
    SampleCount, ADC_SETTINGS_INTERNAL_READ, ADC_SETTINGS_INTERNAL_READ_D21_TEMP,
};

#[cfg(feature = "async")]
use super::{async_api, FutureAdc};

use crate::{calibration, pac};
use pac::adc::inputctrl::Gainselect;
use pac::Peripherals;
use pac::Sysctrl;
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

impl<I: AdcInstance> Adc<I> {
    #[inline]
    /// Configures the ADC.
    pub(crate) fn configure(&mut self, cfg: AdcSettings) {
        self.power_down();
        self.sync();
        I::calibrate(&self.adc);
        self.sync();
        self.adc
            .ctrlb()
            .modify(|_, w| w.prescaler().variant(cfg.clk_divider));
        self.sync();
        self.adc
            .ctrlb()
            .modify(|_, w| w.ressel().variant(cfg.accumulation.resolution()));
        self.sync();

        self.adc
            .sampctrl()
            .modify(|_, w| unsafe { w.samplen().bits(cfg.sample_clock_cycles) }); // sample length
        self.sync();
        self.adc.inputctrl().modify(|_, w| {
            w.muxneg().gnd();
            w.gain().variant(Gainselect::Div2)
        }); // No negative input (internal gnd)
        self.sync();
        let (sample_cnt, adjres) = match self.cfg.accumulation {
            // 1 sample to be used as is
            Accumulation::Single(_) => (SampleCount::_1, 0),
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
        self.set_reference(cfg.vref);
        self.sync();
        self.adc.ctrla().modify(|_, w| w.enable().set_bit());
        self.sync();
        self.cfg = cfg;
    }

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

    /// Clear the specified interrupt flags
    #[cfg(feature = "async")]
    #[inline]
    pub(super) fn clear_flags(&mut self, flags: &Flags) {
        unsafe {
            self.adc.intflag().write(|w| w.bits(flags.bits()));
        }
    }

    /// Clear all interrupt flags
    #[inline]
    pub(super) fn clear_all_flags(&mut self) {
        unsafe {
            // nb SAMD1x and SAMD2x have a SYNCRDY flag, SAMx5x doesn't
            self.adc.intflag().write(|w| w.bits(0b1111));
        }
    }

    /// Check whether the provided flags contain an `OVERRUN` error
    #[inline]
    pub(super) fn check_overrun(&mut self, flags: &Flags) -> Result<(), Error> {
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

impl<I: AdcInstance + PrimaryAdc> Adc<I> {
    #[inline]
    /// Returns the CPU temperature in degrees C
    ///
    /// This requires that the [pac::Sysctrl] peripheral is configured with
    /// the tsen bit enabled, otherwise this function will return an error
    /// [Error::TemperatureSensorNotEnabled]
    pub fn read_cpu_temperature(&mut self, sysctrl: &Sysctrl) -> Result<f32, Error> {
        let vref = sysctrl.vref().read();
        if vref.tsen().bit_is_clear() {
            return Err(Error::TemperatureSensorNotEnabled);
        }
        let room_temp = calibration::room_temp();
        let room_reading = calibration::room_temp_adc_val() as f32;

        let hot_temp = calibration::hot_temp();
        let hot_reading = calibration::hot_temp_adc_val() as f32;

        let room_int1v_ref = 1.0 - (calibration::room_int1v_val() as f32 / 1000.0);
        let hot_int1v_ref = 1.0 - (calibration::hot_int1v_val() as f32 / 1000.0);

        let room_voltage_compensated = room_reading * room_int1v_ref / 4095.0;
        let hot_voltage_compensated = hot_reading * hot_int1v_ref / 4095.0;

        let adc_val = self.with_specific_settings(ADC_SETTINGS_INTERNAL_READ_D21_TEMP, |adc| {
            // IMPORTANT - We also have to modify gain, but this is not exposed in ADC Settings
            adc.adc.inputctrl().modify(|_, w| w.gain()._1x());
            let res = adc.read_channel(0x18);
            // Set gain back to normal
            adc.adc.inputctrl().modify(|_, w| w.gain().div2());
            res
        });

        // Source:
        // https://github.com/ElectronicCats/ElectronicCats_InternalTemperatureZero/blob/master/src/TemperatureZero.cpp
        let tsen_val = adc_val as f32 / 4095.0;

        let coarse_temp = room_temp
            + (((hot_temp - room_temp) / (hot_voltage_compensated - room_voltage_compensated))
                * (tsen_val - room_voltage_compensated));
        let ref_1v = room_int1v_ref
            + (((hot_int1v_ref - room_int1v_ref) * (coarse_temp - room_temp))
                / (hot_temp - room_temp));
        let measured_voltage_compensated = adc_val as f32 * ref_1v / 4095.0;

        let temp = room_temp
            + (((hot_temp - room_temp) / (hot_voltage_compensated - room_voltage_compensated))
                * (measured_voltage_compensated - room_voltage_compensated));

        Ok(temp)
    }

    #[inline]
    pub fn read_cpu_voltage(&mut self, src: CpuVoltageSource) -> u16 {
        let voltage = self.with_specific_settings(ADC_SETTINGS_INTERNAL_READ, |adc| {
            let res = adc.read_channel(src as u8);
            adc.reading_to_f32(res) * 3.3 * 4.0
        });
        (voltage * 1000.0) as u16
    }
}

#[cfg(feature = "async")]
impl<I: AdcInstance + PrimaryAdc, F> FutureAdc<I, F>
where
    F: crate::async_hal::interrupts::Binding<I::Interrupt, async_api::InterruptHandler<I>>,
{
    /// Reads the CPU temperature. Value returned is in Celcius
    pub fn read_cpu_temperature(&mut self, sysctrl: &Sysctrl) -> Result<f32, super::Error> {
        self.inner.read_cpu_temperature(sysctrl)
    }

    /// Reads one of the CPU voltage source. Value returnned is in millivolts (mV)
    pub fn read_cpu_voltage(&mut self, src: CpuVoltageSource) -> u16 {
        self.inner.read_cpu_voltage(src)
    }
}
