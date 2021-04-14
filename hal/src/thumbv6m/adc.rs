use crate::clock::GenericClockController;
use crate::gpio::v1;
use crate::gpio::v2::*;
use crate::hal::adc::{Channel, OneShot};
use crate::target_device::{adc, ADC, PM};

pub struct Adc<ADC> {
    adc: ADC,
}

impl Adc<ADC> {
    pub fn adc(adc: ADC, pm: &mut PM, clocks: &mut GenericClockController) -> Self {
        pm.apbcmask.modify(|_, w| w.adc_().set_bit());

        // set to 1 / (1 / (48000000 / 32) * 6) = 250000 SPS
        let gclk0 = clocks.gclk0();
        clocks.adc(&gclk0).expect("adc clock setup failed");
        while adc.status.read().syncbusy().bit_is_set() {}

        adc.ctrla.modify(|_, w| w.swrst().set_bit());
        while adc.status.read().syncbusy().bit_is_set() {}

        adc.ctrlb.modify(|_, w| {
            w.prescaler().div32();
            w.ressel()._12bit()
        });
        while adc.status.read().syncbusy().bit_is_set() {}

        adc.sampctrl.modify(|_, w| unsafe { w.samplen().bits(5) }); //sample length
        while adc.status.read().syncbusy().bit_is_set() {}

        adc.inputctrl.modify(|_, w| w.muxneg().gnd()); // No negative input (internal gnd)
        while adc.status.read().syncbusy().bit_is_set() {}

        let mut newadc = Self { adc };
        newadc.samples(adc::avgctrl::SAMPLENUM_A::_1);
        newadc.gain(adc::inputctrl::GAIN_A::DIV2);
        newadc.reference(adc::refctrl::REFSEL_A::INTVCC1);

        newadc
    }

    pub fn samples(&mut self, samples: adc::avgctrl::SAMPLENUM_A) {
        use adc::avgctrl::SAMPLENUM_A;
        self.adc.avgctrl.modify(|_, w| {
            w.samplenum().variant(samples);
            unsafe {
                // Table 32-3 (32.6.7) specifies the adjres
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
        while self.adc.status.read().syncbusy().bit_is_set() {}
    }

    pub fn gain(&mut self, gain: adc::inputctrl::GAIN_A) {
        self.adc.inputctrl.modify(|_, w| w.gain().variant(gain));
        while self.adc.status.read().syncbusy().bit_is_set() {}
    }

    pub fn reference(&mut self, reference: adc::refctrl::REFSEL_A) {
        self.adc
            .refctrl
            .modify(|_, w| w.refsel().variant(reference));
        while self.adc.status.read().syncbusy().bit_is_set() {}
    }

    fn power_up(&mut self) {
        while self.adc.status.read().syncbusy().bit_is_set() {}
        self.adc.ctrla.modify(|_, w| w.enable().set_bit());
        while self.adc.status.read().syncbusy().bit_is_set() {}
    }

    fn power_down(&mut self) {
        while self.adc.status.read().syncbusy().bit_is_set() {}
        self.adc.ctrla.modify(|_, w| w.enable().clear_bit());
        while self.adc.status.read().syncbusy().bit_is_set() {}
    }

    fn convert(&mut self) -> u16 {
        self.adc.swtrig.modify(|_, w| w.start().set_bit());
        while self.adc.intflag.read().resrdy().bit_is_clear() {}
        while self.adc.status.read().syncbusy().bit_is_set() {}

        // Clear the interrupt flag
        self.adc.intflag.modify(|_, w| w.resrdy().set_bit());

        // Start conversion again, since The first conversion after the reference is
        // changed must not be used.
        self.adc.swtrig.modify(|_, w| w.start().set_bit());
        while self.adc.intflag.read().resrdy().bit_is_clear() {}
        while self.adc.status.read().syncbusy().bit_is_set() {}

        self.adc.result.read().result().bits()
    }
}

impl<WORD, PIN> OneShot<ADC, WORD, PIN> for Adc<ADC>
where
    WORD: From<u16>,
    PIN: Channel<ADC, ID = u8>,
{
    type Error = ();

    fn read(&mut self, _pin: &mut PIN) -> nb::Result<WORD, Self::Error> {
        let chan = PIN::channel();
        while self.adc.status.read().syncbusy().bit_is_set() {}

        self.adc
            .inputctrl
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
            $PinId:ident: $CHAN:literal
        ),+
    ) => {
        $(
            impl Channel<ADC> for Pin<$PinId, AlternateB> {
               type ID = u8;
               fn channel() -> u8 { $CHAN }
            }
        )+
    }
}

/// Implement [`Channel`] for [`v1::Pin`]s based on the implementations for
/// `v2` [`Pin`]s
impl<I> Channel<ADC> for v1::Pin<I, v1::PfB>
where
    I: PinId,
    Pin<I, AlternateB>: Channel<ADC, ID = u8>,
{
    type ID = u8;
    fn channel() -> u8 {
        Pin::<I, AlternateB>::channel()
    }
}

#[cfg(feature = "samd11")]
adc_pins! {
    PA02: 0,
    PA04: 2,
    PA05: 3,
    PA14: 6,
    PA15: 7
}

#[cfg(feature = "samd21")]
adc_pins! {
    PA02: 0,
    PA03: 1,
    PA04: 4,
    PA05: 5,
    PA06: 6,
    PA07: 7,
    PA08: 16,
    PA09: 17,
    PA10: 18,
    PA11: 19
}

#[cfg(feature = "min-samd21g")]
adc_pins! {
    PB02: 10,
    PB03: 11,
    PB08: 2,
    PB09: 3
}

#[cfg(feature = "min-samd21j")]
adc_pins! {
    PB00: 8,
    PB01: 9,
    PB04: 12,
    PB05: 13,
    PB06: 14,
    PB07: 15
}
