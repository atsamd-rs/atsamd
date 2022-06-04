//! Analogue-to-Digital Conversion
use crate::clock::GenericClockController;
#[rustfmt::skip]
use crate::gpio::*;
use crate::ehal::adc::{Channel, OneShot};
use crate::pac::gclk::genctrl::SRC_A::DFLL;
use crate::pac::gclk::pchctrl::GEN_A;
use crate::pac::{adc0, ADC0, ADC1, MCLK};

use crate::calibration;

/// Samples per reading
pub use adc0::avgctrl::SAMPLENUM_A as SampleRate;
/// Clock frequency relative to the system clock
pub use adc0::ctrla::PRESCALER_A as Prescaler;
/// Reading resolution in bits
pub use adc0::ctrlb::RESSEL_A as Resolution;
/// Reference voltage (or its source)
pub use adc0::refctrl::REFSEL_A as Reference;

/// An ADC where results are accessible via interrupt servicing.
pub struct InterruptAdc<ADC, C>
where
    C: ConversionMode<ADC>,
{
    adc: Adc<ADC>,
    m: core::marker::PhantomData<C>,
}

/// `Adc` encapsulates the device ADC
pub struct Adc<ADC> {
    adc: ADC,
}

/// Describes how an interrupt-driven ADC should finalize the peripheral
/// upon the completion of a conversion.
pub trait ConversionMode<ADC> {
    fn on_start(adc: &mut Adc<ADC>);
    fn on_complete(adc: &mut Adc<ADC>);
    fn on_stop(adc: &mut Adc<ADC>);
}

pub struct SingleConversion;
pub struct FreeRunning;

macro_rules! adc_hal {
    ($($ADC:ident: ($init:ident, $mclk:ident, $apmask:ident, $compcal:ident, $refcal:ident, $r2rcal:ident),)+) => {
        $(
impl Adc<$ADC> {
    pub fn $init(adc: $ADC, mclk: &mut MCLK, clocks: &mut GenericClockController, gclk:GEN_A) -> Self {
        mclk.$mclk.modify(|_, w| w.$apmask().set_bit());
        // set to 1/(1/(48000000/32) * 6) = 250000 SPS
        let adc_clock = clocks.configure_gclk_divider_and_source(gclk, 1, DFLL, false)
            .expect("adc clock setup failed");
        clocks.$init(&adc_clock).expect("adc clock setup failed");
        adc.ctrla.modify(|_, w| w.prescaler().div32());
        adc.ctrlb.modify(|_, w| w.ressel()._12bit());
        while adc.syncbusy.read().ctrlb().bit_is_set() {}
        adc.sampctrl.modify(|_, w| unsafe {w.samplen().bits(5)}); // sample length
        while adc.syncbusy.read().sampctrl().bit_is_set() {}
        adc.inputctrl.modify(|_, w| w.muxneg().gnd()); // No negative input (internal gnd)
        while adc.syncbusy.read().inputctrl().bit_is_set() {}

        adc.calib.write(|w| unsafe {
            w.biascomp().bits(calibration::$compcal());
            w.biasrefbuf().bits(calibration::$refcal());
            w.biasr2r().bits(calibration::$r2rcal())
        });

        let mut newadc = Self { adc };
        newadc.samples(adc0::avgctrl::SAMPLENUM_A::_1);
        newadc.reference(adc0::refctrl::REFSEL_A::INTVCC1);

        newadc
    }

    /// Set the sample rate
    pub fn samples(&mut self, samples: SampleRate) {
        use adc0::avgctrl::SAMPLENUM_A;
        self.adc.avgctrl.modify(|_, w| {
            w.samplenum().variant(samples);
            unsafe {
                // Table 45-3 (45.6.2.10) specifies the adjres
                // values necessary for each SAMPLENUM value.
                w.adjres().bits(match samples {
                    SAMPLENUM_A::_1 => 0,
                    SAMPLENUM_A::_2 => 1,
                    SAMPLENUM_A::_4 => 2,
                    SAMPLENUM_A::_8 => 3,
                    _ => 4,
                })
            }
        });
        while self.adc.syncbusy.read().avgctrl().bit_is_set() {}
    }

    /// Set the voltage reference
    pub fn reference(&mut self, reference: Reference) {
        self.adc
            .refctrl
            .modify(|_, w| w.refsel().variant(reference));
        while self.adc.syncbusy.read().refctrl().bit_is_set() {}
    }

    /// Set the prescaler for adjusting the clock relative to the system clock
    pub fn prescaler(&mut self, prescaler: Prescaler) {
        self.adc
            .ctrla
            .modify(|_, w| w.prescaler().variant(prescaler));
        // Note there is no syncbusy for ctrla
    }

    /// Set the input resolution
    pub fn resolution(&mut self, resolution: Resolution) {
        self.adc
            .ctrlb
            .modify(|_, w| w.ressel().variant(resolution));
        while self.adc.syncbusy.read().ctrlb().bit_is_set() {}
    }

    fn power_up(&mut self) {
        while self.adc.syncbusy.read().enable().bit_is_set() {}
        self.adc.ctrla.modify(|_, w| w.enable().set_bit());
        while self.adc.syncbusy.read().enable().bit_is_set() {}
    }

    fn power_down(&mut self) {
        while self.adc.syncbusy.read().enable().bit_is_set() {}
        self.adc.ctrla.modify(|_, w| w.enable().clear_bit());
        while self.adc.syncbusy.read().enable().bit_is_set() {}
    }

    #[inline(always)]
    fn start_conversion(&mut self) {
        // start conversion
        self.adc.swtrig.modify(|_, w| w.start().set_bit());
        // do it again because the datasheet tells us to
        self.adc.swtrig.modify(|_, w| w.start().set_bit());
    }

    fn enable_freerunning(&mut self) {
        self.adc.ctrlb.modify(|_, w| w.freerun().set_bit());
        while self.adc.syncbusy.read().ctrlb().bit_is_set() {}
    }

    fn disable_freerunning(&mut self) {
        self.adc.ctrlb.modify(|_, w| w.freerun().set_bit());
        while self.adc.syncbusy.read().ctrlb().bit_is_set() {}
    }

    fn synchronous_convert(&mut self) -> u16 {
        self.start_conversion();
        while self.adc.intflag.read().resrdy().bit_is_clear() {}

        self.adc.result.read().result().bits()
    }

    /// Enables an interrupt when conversion is ready.
    fn enable_interrupts(&mut self) {
        self.adc.intflag.write(|w| w.resrdy().set_bit());
        self.adc.intenset.write(|w| w.resrdy().set_bit());
    }

    /// Disables the interrupt for when conversion is ready.
    fn disable_interrupts(&mut self) {
        self.adc.intenclr.write(|w| w.resrdy().set_bit());
    }

    fn service_interrupt_ready(&mut self) -> Option<u16> {
        if self.adc.intflag.read().resrdy().bit_is_set() {
            self.adc.intflag.write(|w| w.resrdy().set_bit());

            Some(self.adc.result.read().result().bits())
        } else {
            None
        }
    }

    /// Sets the mux to a particular pin. The pin mux is enabled-protected,
    /// so must be called while the peripheral is disabled.
    fn mux<PIN: Channel<$ADC, ID=u8>>(&mut self, _pin: &mut PIN) {
        let chan = PIN::channel();
        while self.adc.syncbusy.read().inputctrl().bit_is_set() {}
        self.adc.inputctrl.modify(|_, w| w.muxpos().bits(chan));
    }
}

impl ConversionMode<$ADC> for SingleConversion  {
    fn on_start(_adc: &mut Adc<$ADC>) {
    }
    fn on_complete(adc: &mut Adc<$ADC>) {
        adc.disable_interrupts();
        adc.power_down();
    }
    fn on_stop(_adc: &mut Adc<$ADC>) {
    }
}

impl ConversionMode<$ADC> for FreeRunning {
    fn on_start(adc: &mut Adc<$ADC>) {
        adc.enable_freerunning();
    }
    fn on_complete(_adc: &mut Adc<$ADC>) {
    }
    fn on_stop(adc: &mut Adc<$ADC>) {
        adc.disable_interrupts();
        adc.power_down();
        adc.disable_freerunning();
    }
}

impl<C> InterruptAdc<$ADC, C>
    where C: ConversionMode<$ADC>
{
    pub fn service_interrupt_ready(&mut self) -> Option<u16> {
        if let Some(res) = self.adc.service_interrupt_ready() {
            C::on_complete(&mut self.adc);
            Some(res)
        } else {
            None
        }
    }

    /// Starts a conversion sampling the specified pin.
    pub fn start_conversion<PIN: Channel<$ADC, ID=u8>>(&mut self, pin: &mut PIN) {
        self.adc.mux(pin);
        self.adc.power_up();
        C::on_start(&mut self.adc);
        self.adc.enable_interrupts();
        self.adc.start_conversion();
    }

    pub fn stop_conversion(&mut self) {
        C::on_stop(&mut self.adc);
    }
}

impl<C> From<Adc<$ADC>> for InterruptAdc<$ADC, C>
    where C: ConversionMode<$ADC>
{
    fn from(adc: Adc<$ADC>) -> Self {
        Self {
            adc,
            m: core::marker::PhantomData{},
        }
    }
}

impl<WORD, PIN> OneShot<$ADC, WORD, PIN> for Adc<$ADC>
where
   WORD: From<u16>,
   PIN: Channel<$ADC, ID=u8>,
{
   type Error = ();

   fn read(&mut self, pin: &mut PIN) -> nb::Result<WORD, Self::Error> {
        self.mux(pin);
        self.power_up();
        let result = self.synchronous_convert();
        self.power_down();
        Ok(result.into())
   }
}
        )+
    }
}

adc_hal! {
    ADC0: (adc0, apbdmask, adc0_, adc0_biascomp_scale_cal, adc0_biasref_scale_cal, adc0_biasr2r_scale_cal),
    ADC1: (adc1, apbdmask, adc1_, adc1_biascomp_scale_cal, adc1_biasref_scale_cal, adc1_biasr2r_scale_cal),
}

macro_rules! adc_pins {
    (
        $(
            $PinId:ident: ($ADC:ident, $CHAN:literal),
        )+
    ) => {
        $(
            impl Channel<$ADC> for Pin<$PinId, AlternateB> {
               type ID = u8;
               fn channel() -> u8 { $CHAN }
            }
        )+
    }
}

adc_pins! {
    PA02: (ADC0, 0),
    PA03: (ADC0, 1),
    PB08: (ADC0, 2),
    PB09: (ADC0, 3),
    PA04: (ADC0, 4),
    PA05: (ADC0, 5),
    PA06: (ADC0, 6),
    PA07: (ADC0, 7),
    PA08: (ADC0, 8),
    PA09: (ADC0, 9),
    PA10: (ADC0, 10),
    PA11: (ADC0, 11),
    PB02: (ADC0, 14),
    PB03: (ADC0, 15),

    PB08: (ADC1, 0),
    PB09: (ADC1, 1),
    PA08: (ADC1, 2),
    PA09: (ADC1, 3),
}

#[cfg(feature = "min-samd51j")]
adc_pins! {
    PB00: (ADC0, 12),
    PB01: (ADC0, 13),
    PB04: (ADC1, 6),
    PB05: (ADC1, 7),
    PB06: (ADC1, 8),
    PB07: (ADC1, 9),
}

#[cfg(feature = "min-samd51n")]
adc_pins! {
    PC02: (ADC1, 4),
    PC03: (ADC1, 5),
    PC00: (ADC1, 10),
    PC01: (ADC1, 11),
}

#[cfg(feature = "min-samd51p")]
adc_pins! {
    PC30: (ADC1, 12),
    PC31: (ADC1, 13),
    PD00: (ADC1, 14),
    PD01: (ADC1, 15),
}
