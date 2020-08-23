use crate::clock::GenericClockController;
#[rustfmt::skip]
use crate::gpio::{
    Pa2, Pa3, Pa4, Pa5, Pa6, Pa7, Pa8, Pa9, Pa10, Pa11,
    Pb0, Pb1, Pb2, Pb3, Pb4, Pb5, Pb6, Pb7, Pb8, Pb9,
    PfB,
};
#[cfg(feature = "same54")]
use crate::gpio::{Pc0, Pc1, Pc2, Pc3, Pc30, Pc31, Pd0, Pd1};
use crate::hal::adc::{Channel, OneShot};
use crate::target_device::gclk::genctrl::SRC_A::DFLL;
use crate::target_device::gclk::pchctrl::GEN_A;
use crate::target_device::{adc0, ADC0, ADC1, MCLK};

pub struct Adc<ADC> {
    adc: ADC,
}

macro_rules! adc_hal {
    ($($ADC:ident: ($init:ident, $mclk:ident, $apmask:ident),)+) => {
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

        let mut newadc = Self { adc };
        newadc.samples(adc0::avgctrl::SAMPLENUM_A::_1);
        newadc.reference(adc0::refctrl::REFSEL_A::INTVCC1);

        newadc
    }

    pub fn samples(&mut self, samples: adc0::avgctrl::SAMPLENUM_A) {
        self.adc.avgctrl.modify(|_, w| {
            w.samplenum().variant(samples);
            // I don't see any reason to divide ourselves. peripheral will automatically
            // shift as needed
            unsafe { w.adjres().bits(0) }
        });
        while self.adc.syncbusy.read().avgctrl().bit_is_set() {}
    }

    pub fn reference(&mut self, reference: adc0::refctrl::REFSEL_A) {
        self.adc
            .refctrl
            .modify(|_, w| w.refsel().variant(reference));
        while self.adc.syncbusy.read().refctrl().bit_is_set() {}
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

    fn convert(&mut self) -> u16 {
        // start conversion
        self.adc.swtrig.modify(|_, w| w.start().set_bit());
        // do it again because the datasheet tells us to
        self.adc.swtrig.modify(|_, w| w.start().set_bit());
        while self.adc.intflag.read().resrdy().bit_is_clear() {}
        let result = self.adc.result.read().result().bits();
        result
    }
}

impl<WORD, PIN> OneShot<$ADC, WORD, PIN> for Adc<$ADC>
where
   WORD: From<u16>,
   PIN: Channel<$ADC, ID=u8>,
{
   type Error = ();

   fn read(&mut self, _pin: &mut PIN) -> nb::Result<WORD, Self::Error> {
        let chan = PIN::channel();
        while self.adc.syncbusy.read().inputctrl().bit_is_set() {}
        // pin must be selected before adc is enabled
        self.adc.inputctrl.modify(|_, w| w.muxpos().bits(chan));
        self.power_up();
        let result = self.convert();
        self.power_down();
        Ok(result.into())
   }
}
        )+
    }
}

macro_rules! adc_pins {
    ($($pin:ident: ($ADC:ident, $chan:expr),)+) => {
        $(

impl Channel<$ADC> for $pin<PfB> {
   type ID = u8;
   fn channel() -> u8 { $chan }
}
        )+
    }
}

adc_hal! {
    ADC0: (adc0, apbdmask, adc0_),
    ADC1: (adc1, apbdmask, adc1_),
}

adc_pins! {
    Pa2:  (ADC0, 0),
    Pa3:  (ADC0, 1),
    Pb8:  (ADC0, 2),
    Pb9:  (ADC0, 3),
    Pa4:  (ADC0, 4),
    Pa5:  (ADC0, 5),
    Pa6:  (ADC0, 6),
    Pa7:  (ADC0, 7),
    Pa8:  (ADC0, 8),
    Pa9:  (ADC0, 9),
    Pa10: (ADC0, 10),
    Pa11: (ADC0, 11),
    Pb0:  (ADC0, 12),
    Pb1:  (ADC0, 13),
    Pb2:  (ADC0, 14),
    Pb3:  (ADC0, 15),

    Pb8:  (ADC1, 0),
    Pb9:  (ADC1, 1),
    Pa8:  (ADC1, 2),
    Pa9:  (ADC1, 3),
    Pb4:  (ADC1, 6),
    Pb5:  (ADC1, 7),
    Pb6:  (ADC1, 8),
    Pb7:  (ADC1, 9),
}

#[cfg(feature = "same54")]
adc_pins! {
    Pc2:  (ADC1, 4),
    Pc3:  (ADC1, 5),
    Pc0:  (ADC1, 10),
    Pc1:  (ADC1, 11),
    Pc30: (ADC1, 12),
    Pc31: (ADC1, 13),
    Pd0:  (ADC1, 14),
    Pd1:  (ADC1, 15),
}
