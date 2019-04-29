use crate::hal::adc::{Channel, OneShot};
use crate::target_device::{ADC0, ADC1, MCLK};
use crate::target_device::gclk::genctrl::SRCR::DFLL;
use crate::target_device::gclk::pchctrl::GENR::GCLK11;
use crate::clock::GenericClockController;
use crate::gpio::{Pa2, PfB};

pub struct Adc<ADC> {
    adc: ADC
}

impl Adc<ADC0> {
    pub fn new(adc: ADC0, mclk: &mut MCLK, clocks: &mut GenericClockController) -> Self {
        mclk.apbdmask.modify(|_, w| w.adc0_().set_bit());
        // set to 1/(1/(48000000/32) * 6) = 250000 SPS
        let gclk11 = clocks.configure_gclk_divider_and_source(GCLK11, 1, DFLL, false)
            .expect("adc clock setup failed");
        clocks.adc0(&gclk11).expect("adc clock setup failed");
        adc.ctrla.modify(|_, w| w.prescaler().div32());
        adc.ctrlb.modify(|_, w| w.ressel()._12bit());
        while adc.syncbusy.read().ctrlb().bit_is_set() {}
        adc.sampctrl.modify(|_, w| unsafe {w.samplen().bits(5)}); // sample length
        while adc.syncbusy.read().sampctrl().bit_is_set() {}
        adc.inputctrl.modify(|_, w| w.muxneg().gnd()); // No negative input (internal gnd)
        while adc.syncbusy.read().inputctrl().bit_is_set() {}
        adc.avgctrl.modify(|_, w| {
            w.samplenum()._1();  // No averaging
            unsafe { w.adjres().bits(0) } // adjust result by 0
        });
        while adc.syncbusy.read().avgctrl().bit_is_set() {}
        // in arduino-land there's something about an inputctrl gain reg here
        // https://github.com/adafruit/ArduinoCore-samd/blob/ff2cb608a9d2506b3279dd287628c9962dbba249/cores/arduino/wiring_analog.c#L253
        adc.refctrl.modify(|_, w| w.refsel().intvcc1()); // 1.65V voltage reference
        Self { adc }
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

    fn convert(&mut self, pin: u8) -> u16 {
        while self.adc.syncbusy.read().inputctrl().bit_is_set() {}
        // select pin
        self.adc.inputctrl.modify(|_, w| unsafe {w.muxpos().bits(pin)});
        // start conversion
        self.adc.swtrig.modify(|_, w| w.start().set_bit());
        // clear the data ready flag
        self.adc.intflag.modify(|_, w| w.resrdy().clear_bit()); 
        // do it again because the datasheet tells us to 
        self.adc.swtrig.modify(|_, w| w.start().set_bit());
        while self.adc.intflag.read().resrdy().bit_is_set() {}
        let result = self.adc.result.read().result().bits();
        result 
    }
}

impl<WORD, PIN> OneShot<ADC0, WORD, PIN> for Adc<ADC0>
where
   WORD: From<u16>,
   PIN: Channel<ADC0, ID=u8>,
{
   type Error = ();

   fn read(&mut self, _pin: &mut PIN) -> nb::Result<WORD, Self::Error> {
       let chan = 1 << PIN::channel();
       self.power_up();
       let result = self.convert(chan);
       self.power_down();
       Ok(result.into())
   }
}

impl Channel<ADC0> for Pa2<PfB> {
   type ID = u8;
   fn channel() -> u8 { 0x00 }
}
