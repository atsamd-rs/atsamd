//! Analogue-to-Digital Conversion
use atsamd_hal_macros::hal_cfg;

use crate::clock::GenericClockController;
use crate::ehal_02::adc::{Channel, OneShot};
use crate::gpio::*;
use crate::pac::{self, adc, Pm};

/// Samples per reading
pub use adc::avgctrl::Samplenumselect as SampleRate;
/// Clock frequency relative to the system clock
pub use adc::ctrlb::Prescalerselect as Prescaler;
/// Reading resolution in bits
///
/// For the resolution of Arduino boards,
/// see the [analogueRead](https://www.arduino.cc/reference/en/language/functions/analog-io/analogread/) docs.
pub use adc::ctrlb::Resselselect as Resolution;
/// The gain level
pub use adc::inputctrl::Gainselect as Gain;
/// Reference voltage (or its source)
pub use adc::refctrl::Refselselect as Reference;

/// `Adc` encapsulates the device ADC
pub struct Adc<ADC> {
    adc: ADC,
}

impl Adc<pac::Adc> {
    /// Create a new `Adc` instance. The default configuration is:
    /// * 1/32 prescaler
    /// * 12 bit resolution
    /// * 1 sample
    /// * 1/2 gain
    /// * 1/2 VDDANA reference voltage
    #[allow(clippy::self_named_constructors)]
    pub fn adc(adc: pac::Adc, pm: &mut Pm, clocks: &mut GenericClockController) -> Self {
        pm.apbcmask().modify(|_, w| w.adc_().set_bit());

        // set to 1 / (1 / (48000000 / 32) * 6) = 250000 SPS
        let gclk0 = clocks.gclk0();
        clocks.adc(&gclk0).expect("adc clock setup failed");
        while adc.status().read().syncbusy().bit_is_set() {}

        adc.ctrla().modify(|_, w| w.swrst().set_bit());
        while adc.status().read().syncbusy().bit_is_set() {}

        adc.ctrlb().modify(|_, w| {
            w.prescaler().div32();
            w.ressel()._12bit()
        });
        while adc.status().read().syncbusy().bit_is_set() {}

        adc.sampctrl().modify(|_, w| unsafe { w.samplen().bits(5) }); //sample length
        while adc.status().read().syncbusy().bit_is_set() {}

        adc.inputctrl().modify(|_, w| w.muxneg().gnd()); // No negative input (internal gnd)
        while adc.status().read().syncbusy().bit_is_set() {}

        let mut newadc = Self { adc };
        newadc.samples(adc::avgctrl::Samplenumselect::_1);
        newadc.gain(adc::inputctrl::Gainselect::Div2);
        newadc.reference(adc::refctrl::Refselselect::Intvcc1);

        newadc
    }

    /// Set the sample rate
    pub fn samples(&mut self, samples: SampleRate) {
        use adc::avgctrl::Samplenumselect;
        self.adc.avgctrl().modify(|_, w| {
            w.samplenum().variant(samples);
            unsafe {
                // Table 32-3 (32.6.7) specifies the adjres
                // values necessary for each SAMPLENUM value.
                w.adjres().bits(match samples {
                    Samplenumselect::_1 => 0,
                    Samplenumselect::_2 => 1,
                    Samplenumselect::_4 => 2,
                    Samplenumselect::_8 => 3,
                    _ => 4,
                })
            }
        });
        while self.adc.status().read().syncbusy().bit_is_set() {}
    }

    /// Set the gain factor
    pub fn gain(&mut self, gain: Gain) {
        self.adc.inputctrl().modify(|_, w| w.gain().variant(gain));
        while self.adc.status().read().syncbusy().bit_is_set() {}
    }

    /// Set the voltage reference
    pub fn reference(&mut self, reference: Reference) {
        self.adc
            .refctrl()
            .modify(|_, w| w.refsel().variant(reference));
        while self.adc.status().read().syncbusy().bit_is_set() {}
    }

    /// Set the prescaler for adjusting the clock relative to the system clock
    pub fn prescaler(&mut self, prescaler: Prescaler) {
        self.adc
            .ctrlb()
            .modify(|_, w| w.prescaler().variant(prescaler));
        while self.adc.status().read().syncbusy().bit_is_set() {}
    }

    /// Set the input resolution.
    pub fn resolution(&mut self, resolution: Resolution) {
        self.adc
            .ctrlb()
            .modify(|_, w| w.ressel().variant(resolution));
        while self.adc.status().read().syncbusy().bit_is_set() {}
    }

    fn power_up(&mut self) {
        while self.adc.status().read().syncbusy().bit_is_set() {}
        self.adc.ctrla().modify(|_, w| w.enable().set_bit());
        while self.adc.status().read().syncbusy().bit_is_set() {}
    }

    fn power_down(&mut self) {
        while self.adc.status().read().syncbusy().bit_is_set() {}
        self.adc.ctrla().modify(|_, w| w.enable().clear_bit());
        while self.adc.status().read().syncbusy().bit_is_set() {}
    }

    fn convert(&mut self) -> u16 {
        self.adc.swtrig().modify(|_, w| w.start().set_bit());
        while self.adc.intflag().read().resrdy().bit_is_clear() {}
        while self.adc.status().read().syncbusy().bit_is_set() {}

        // Clear the interrupt flag
        self.adc.intflag().modify(|_, w| w.resrdy().set_bit());

        // Start conversion again, since The first conversion after the reference is
        // changed must not be used.
        self.adc.swtrig().modify(|_, w| w.start().set_bit());
        while self.adc.intflag().read().resrdy().bit_is_clear() {}
        while self.adc.status().read().syncbusy().bit_is_set() {}

        self.adc.result().read().result().bits()
    }
}

impl<WORD, PIN> OneShot<pac::Adc, WORD, PIN> for Adc<pac::Adc>
where
    WORD: From<u16>,
    PIN: Channel<pac::Adc, ID = u8>,
{
    type Error = ();

    fn read(&mut self, _pin: &mut PIN) -> nb::Result<WORD, Self::Error> {
        let chan = PIN::channel();
        while self.adc.status().read().syncbusy().bit_is_set() {}

        self.adc
            .inputctrl()
            .modify(|_, w| unsafe { w.muxpos().bits(chan) });
        self.power_up();
        let result = self.convert();
        self.power_down();

        Ok(result.into())
    }
}

macro_rules! adc_pins {
    (
        $(
            $( #[$cfg:meta] )?
            $PinId:ident: $CHAN:literal
        ),+
        $(,)?
    ) => {
        $(
            $( #[$cfg] )?
            impl Channel<$crate::pac::Adc> for Pin<$PinId, AlternateB> {
               type ID = u8;
               fn channel() -> u8 { $CHAN }
            }
        )+
    }
}

#[hal_cfg("adc-d11")]
adc_pins! {
    #[hal_cfg("pa02")]
    PA02: 0,
    #[hal_cfg("pa03")]
    PA03: 1,
    #[hal_cfg("pa04")]
    PA04: 2,
    #[hal_cfg("pa05")]
    PA05: 3,
    #[hal_cfg("pa06")]
    PA06: 4,
    #[hal_cfg("pa07")]
    PA07: 5,
    #[hal_cfg("pa14")]
    PA14: 6,
    #[hal_cfg("pa15")]
    PA15: 7,
    #[hal_cfg("pa10")]
    PA10: 8,
    #[hal_cfg("pa11")]
    PA11: 9,
}

#[hal_cfg("adc-d21")]
adc_pins! {
    #[hal_cfg("pa02")]
    PA02: 0,
    #[hal_cfg("pa03")]
    PA03: 1,
    #[hal_cfg("pb08")]
    PB08: 2,
    #[hal_cfg("pb09")]
    PB09: 3,
    #[hal_cfg("pa04")]
    PA04: 4,
    #[hal_cfg("pa05")]
    PA05: 5,
    #[hal_cfg("pa06")]
    PA06: 6,
    #[hal_cfg("pa07")]
    PA07: 7,
    #[hal_cfg("pb00")]
    PB00: 8,
    #[hal_cfg("pb01")]
    PB01: 9,
    #[hal_cfg("pb02")]
    PB02: 10,
    #[hal_cfg("pb03")]
    PB03: 11,
    #[hal_cfg("pb04")]
    PB04: 12,
    #[hal_cfg("pb05")]
    PB05: 13,
    #[hal_cfg("pb06")]
    PB06: 14,
    #[hal_cfg("pb07")]
    PB07: 15,
    #[hal_cfg("pa08")]
    PA08: 16,
    #[hal_cfg("pa09")]
    PA09: 17,
    #[hal_cfg("pa10")]
    PA10: 18,
    #[hal_cfg("pa11")]
    PA11: 19,
}
