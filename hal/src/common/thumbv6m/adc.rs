use crate::clock::GenericClockController;
use crate::gpio::*;
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
        self.adc.avgctrl.modify(|_, w| {
            w.samplenum().variant(samples);
            // I don't see any reason to divide ourselves. peripheral will automatically
            // shift as needed
            unsafe { w.adjres().bits(0) }
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
        let result = self.adc.result.read().result().bits();

        result
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
    ($($pin:ident: $chan:expr),+) => {
        $(

impl Channel<ADC> for $pin<PfB> {
   type ID = u8;
   fn channel() -> u8 { $chan }
}
        )+
    }
}

#[cfg(feature = "samd11")]
adc_pins! {
    Pa2: 0,
    Pa3: 1,
    Pa4: 2,
    Pa5: 3,
    Pa6: 4,
    Pa7: 5,
    Pa10: 8,
    Pa11: 9,
    Pa14: 6,
    Pa15: 7
}

#[cfg(feature = "samd21")]
adc_pins! {
    Pa2: 0,
    Pa3: 1,
    Pa4: 4,
    Pa5: 5,
    Pa6: 6,
    Pa7: 7,
    Pa8: 16,
    Pa9: 17,
    Pa10: 18,
    Pa11: 19
}

#[cfg(any(feature = "samd21g18a", feature = "samd21j18a"))]
adc_pins! {
    Pb0: 8,
    Pb1: 9,
    Pb2: 10,
    Pb3: 11,
    Pb4: 12,
    Pb5: 13,
    Pb6: 14,
    Pb7: 15,
    Pb8: 2,
    Pb9: 3
}
