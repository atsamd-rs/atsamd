use super::{
    ADC_SETTINGS_INTERNAL_READ, ADC_SETTINGS_INTERNAL_READ_D21_TEMP, Accumulation, Adc,
    AdcInstance, AdcSettings, CpuVoltageSource, Error, Flags, PrimaryAdc, SampleCount,
};

#[cfg(feature = "async")]
use super::{FutureAdc, async_api};

#[cfg(feature = "dma")]
use pac::dmac::chctrlb::Trigsrcselect as TriggerSelect;

use crate::{calibration, pac};
use pac::Peripherals;
use pac::Sysctrl;
use pac::adc::inputctrl::Gainselect;
pub mod pin;

/// Wrapper around the ADC instance
pub struct Adc0 {
    _adc: pac::Adc,
}

impl PrimaryAdc for Adc0 {}

impl AdcInstance for Adc0 {
    type Instance = pac::Adc;

    const DMA_TRIGGER: TriggerSelect = TriggerSelect::AdcResrdy;

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
        if cfg != self.cfg {
            // Set discard flag for next read
            self.discard = true;
        }
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
            .modify(|_, w| unsafe { w.samplen().bits(cfg.sample_clock_cycles.saturating_sub(1)) }); // sample length
        self.sync();
        self.adc.inputctrl().modify(|_, w| {
            w.muxneg().gnd();
            w.gain().variant(Gainselect::Div2)
        }); // No negative input (internal gnd)
        self.sync();
        let (sample_cnt, adjres) = match cfg.accumulation {
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
        // Write so we completely overwrite the last setting
        self.adc.avgctrl().write(|w| {
            w.samplenum().variant(sample_cnt);
            unsafe { w.adjres().bits(adjres) }
        });
        self.sync();
        self.set_reference(cfg.vref);
        self.sync();
        self.adc.ctrla().modify(|_, w| w.enable().set_bit());
        self.sync();
        self.cfg = cfg;
        self.power_up();
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
        self.adc.swtrig().modify(|_, w| w.start().set_bit());
        self.sync();
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

    #[cfg(feature = "async")]
    /// Clear the specified interrupt flags
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
        self.adc.inputctrl().modify(|r, w| {
            if r.muxpos().bits() != ch {
                self.discard = true;
            }
            unsafe { w.muxpos().bits(ch) }
        });
        self.sync()
    }

    #[inline]
    fn cpu_raw_to_temp(&self, reading: u16) -> f32 {
        // Source:
        // https://github.com/ElectronicCats/ElectronicCats_InternalTemperatureZero/blob/master/src/TemperatureZero.cpp
        let room_temp = calibration::room_temp();
        let adc_room = calibration::room_temp_adc_val() as f32;

        let hot_temp = calibration::hot_temp();
        let adc_hot = calibration::hot_temp_adc_val() as f32;

        let room_int1v = 1.0 - (calibration::room_int1v_val() as f32 / 1000.0);
        let hot_int1v = 1.0 - (calibration::hot_int1v_val() as f32 / 1000.0);

        let v_adc_room = (adc_room * room_int1v) / 4095.0;
        let v_adc_hot = (adc_hot * hot_int1v) / 4095.0;

        let v_adc = reading as f32 / 4095.0; // 1.0 is ommitted here (adc_val * 1.0) - 1.0 is the INT_V1 voltage (Always 1.0V)

        let coarse_temp = room_temp
            + (((hot_temp - room_temp) / (v_adc_hot - v_adc_room)) * (v_adc - v_adc_room));

        let int1v_real = room_int1v
            + (((hot_int1v - room_int1v) * (coarse_temp - room_temp)) / (hot_temp - room_temp));

        let v_adc_real = (reading as f32 * int1v_real) / 4095.0;

        room_temp
            + (((hot_temp - room_temp) / (v_adc_hot - v_adc_room)) * (v_adc_real - v_adc_room))
    }
}

impl<I: AdcInstance + PrimaryAdc> Adc<I> {
    #[inline]
    /// Returns the CPU temperature in degrees C
    ///
    /// NOTE: The temperature sensor is known to be out by up to ±10C, it
    /// therefore should not be relied on for critical temperature readings
    pub fn read_cpu_temperature(&mut self, sysctrl: &mut Sysctrl) -> f32 {
        let old_state = sysctrl.vref().read().tsen().bit();
        sysctrl.vref().modify(|_, w| w.tsen().set_bit());

        let adc_val = self.with_specific_settings(ADC_SETTINGS_INTERNAL_READ_D21_TEMP, |adc| {
            // IMPORTANT - We also have to modify gain, but this is not exposed in ADC
            // Settings
            adc.adc.inputctrl().modify(|_, w| w.gain()._1x());
            adc.discard = true;
            let res = adc.read_channel(0x18);
            // Set gain back to normal
            adc.adc.inputctrl().modify(|_, w| w.gain().div2());
            adc.discard = true;
            res
        });

        // Set sysctrl back
        sysctrl.vref().modify(|_, w| w.tsen().variant(old_state));
        self.cpu_raw_to_temp(adc_val)
    }

    #[inline]
    pub fn read_cpu_voltage(&mut self, src: CpuVoltageSource) -> u16 {
        let voltage = self.with_specific_settings(ADC_SETTINGS_INTERNAL_READ, |adc| {
            let res = adc.read_channel(src as u8);
            let mut res_f = adc.reading_to_f32(res) * 3.3;
            if CpuVoltageSource::Bandgap != src {
                res_f *= 4.0;
            }
            res_f
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
    pub async fn read_cpu_temperature(&mut self, sysctrl: &mut Sysctrl) -> f32 {
        let old_state = sysctrl.vref().read().tsen().bit();
        sysctrl.vref().modify(|_, w| w.tsen().set_bit());

        let old_adc_settings = self.inner.cfg;
        self.inner.configure(ADC_SETTINGS_INTERNAL_READ);
        self.inner.adc.inputctrl().modify(|_, w| w.gain()._1x());
        self.inner.discard = true;

        let res = self.read_channel(0x18).await;

        self.inner.adc.inputctrl().modify(|_, w| w.gain().div2());
        self.inner.configure(old_adc_settings);
        self.inner.discard = true;

        // Set sysctrl back
        sysctrl.vref().modify(|_, w| w.tsen().variant(old_state));
        self.inner.cpu_raw_to_temp(res)
    }

    /// Reads a CPU voltage source. Value returned is in millivolts (mV)
    pub async fn read_cpu_voltage(&mut self, src: CpuVoltageSource) -> u16 {
        let old_adc_settings = self.inner.cfg;
        self.inner.configure(ADC_SETTINGS_INTERNAL_READ);

        let res = self.read_channel(src as u8).await;
        let mut voltage = self.inner.reading_to_f32(res) * 3.3;
        if CpuVoltageSource::Bandgap != src {
            voltage *= 4.0;
        }

        self.inner.configure(old_adc_settings);
        (voltage * 1000.0) as u16
    }
}
