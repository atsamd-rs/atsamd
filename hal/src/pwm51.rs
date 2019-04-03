//TC0 PA04 WO[0]
//TC0 PA05 WO[1]
//TC0 PA08 WO[0]
//TC0 PA09 WO[1]
//TC0 PB30 WO[0]
//TC0 PB31 WO[1]

//TC1 PA06 WO[0]
//TC1 PA07 WO[1]
//TC1 PA10 WO[0]
//TC1 PA11 WO[1]

//TC2 PA00 WO[0]
//TC2 PA01 WO[1]
//TC2 PA12 WO[0]
//TC2 PA13 WO[1]
//TC2 PA16 WO[0]
//TC2 PA17 WO[1]

//TC3 PA14 WO[0]
//TC3 PA15 WO[1]
//TC3 PA18 WO[0]
//TC3 PA19 WO[1]

//TC4 PA22 WO[0]
//TC4 PA23 WO[1]
//TC4 PB08 WO[0]
//TC4 PB09 WO[1]
//TC4 PB12 WO[0]
//TC4 PB13 WO[1]

//TC5 PA24 WO[0]
//TC5 PA25 WO[1]
//TC5 PB10 WO[0]
//TC5 PB11 WO[1]
//TC5 PB14 WO[0]
//TC5 PB15 WO[1]

//TC6 PA30 WO[0]
//TC6 PA31 WO[1]
//TC6 PB02 WO[0]
//TC6 PB03 WO[1]
//TC6 PB16 WO[0]
//TC6 PB17 WO[1]

//TC7 PA20 WO[0]
//TC7 PA21 WO[1]
//TC7 PB00 WO[0]
//TC7 PB01 WO[1]
//TC7 PB22 WO[0]
//TC7 PB23 WO[1]

use gpio::{Pa1, Pa5, Pa7, Pa9, Pa11, Pa13, Pa15, Pa17, Pa19, Pa23, Pa25, Pb9,
    Pb11, Pb13, Pb15, Pb31, PfE};
use clock;
use timer::TimerParams;
use time::Hertz;
use hal::Pwm;

use target_device::{TC0, TC1, TC2, TC3, MCLK};

#[cfg(all(not(feature = "samd51g19a"), not(feature = "samd51g18a")))]
use target_device::{TC4, TC5};

pub enum TC0Pinout {
    Pa5(Pa5<PfE>),
    Pa9(Pa9<PfE>),
    Pb31(Pb31<PfE>),
}

pub enum TC1Pinout {
    Pa7(Pa7<PfE>),
    Pa11(Pa11<PfE>),
}

pub enum TC2Pinout {
    Pa1(Pa1<PfE>),
    Pa13(Pa13<PfE>),
    Pa17(Pa17<PfE>),
}

pub enum TC3Pinout {
    Pa15(Pa15<PfE>),
    Pa19(Pa19<PfE>),
}

#[cfg(all(not(feature = "samd51g19a"), not(feature = "samd51g18a")))]
pub enum TC4Pinout {
    Pa23(Pa23<PfE>),
    Pb9(Pb9<PfE>),
    Pb13(Pb13<PfE>),
}

#[cfg(all(not(feature = "samd51g19a"), not(feature = "samd51g18a")))]
pub enum TC5Pinout {
    Pa25(Pa25<PfE>),
    Pb11(Pb11<PfE>),
    Pb15(Pb15<PfE>),
}

pub enum Channels { 
    C0
}

macro_rules! pwm {
    ($($TYPE:ident: ($TC:ident, $pinout:ident, $clock:ident, $apmask:ident, $apbits:ident),)+) => {
        $(

pub struct $TYPE {
    /// The frequency of the attached clock, not the period of the pwm.
    /// Used to calculate the period of the pwm.
    clock_freq: Hertz,
    tc: $TC,
    pinout: $pinout,
}

impl $TYPE {
    pub fn new<F: Into<Hertz>> (
        clock: &clock::$clock,
        freq: F,
        tc: $TC,
        pinout: $pinout,
        mclk: &mut MCLK,
    ) -> Self {
        let freq = freq.into();
        {
            let count = tc.count16();
            let params = TimerParams::new(freq, clock.freq().0);
            mclk.$apmask.modify(|_, w| w.$apbits().set_bit());
            count.ctrla.write(|w| w.swrst().set_bit());
            while count.ctrla.read().bits() & 1 != 0 {}
            count.ctrla.modify(|_, w| w.enable().clear_bit());
            count.ctrla.modify(|_, w| {
                match params.divider {
                    1 => w.prescaler().div1(),
                    2 => w.prescaler().div2(),
                    4 => w.prescaler().div4(),
                    8 => w.prescaler().div8(),
                    16 => w.prescaler().div16(),
                    64 => w.prescaler().div64(),
                    256 => w.prescaler().div256(),
                    1024 => w.prescaler().div1024(),
                    _ => unreachable!(),
                }
            });
            count.wave.write(|w| w.wavegen().mpwm());
            count.cc[0].write(|w| unsafe { w.cc().bits(params.cycles as u16) });
            while count.syncbusy.read().cc0().bit_is_set() {}
            count.cc[1].write(|w| unsafe { w.cc().bits(0) });
            while count.syncbusy.read().cc1().bit_is_set() {}
            count.ctrla.modify(|_, w| w.enable().set_bit());
        }

        Self {
            clock_freq: clock.freq(),
            tc,
            pinout,
        }
    }
}

impl Pwm for $TYPE {
    type Channel = Channels;
    type Time = Hertz;
    type Duty = u16;

    fn disable(&mut self, _channel: Self::Channel) {
        let count = self.tc.count16();
        count.ctrla.modify(|_, w| w.enable().clear_bit());
    }

    fn enable(&mut self, _channel: Self::Channel) {
        let count = self.tc.count16();
        count.ctrla.modify(|_, w| w.enable().set_bit());
    }

    fn get_period(&self) -> Self::Time {
        let count = self.tc.count16();
        let divisor = count.ctrla.read().prescaler().bits(); 
        let top = count.cc[0].read().cc().bits();
        Hertz(self.clock_freq.0 / divisor as u32 / (top + 1) as u32)
    }

    fn get_duty(&self, _channel: Self::Channel) -> Self::Duty {
        let count = self.tc.count16();
        let duty: u16 = count.ccbuf[1].read().ccbuf().bits();
        duty 
    }

    fn get_max_duty(&self) -> Self::Duty {
        let count = self.tc.count16();
        let top = count.cc[0].read().cc().bits();
        top
    }

    fn set_duty(&mut self, _channel: Self::Channel, duty: Self::Duty) {
        let count = self.tc.count16();
        count.ccbuf[1].write(|w| unsafe {w.ccbuf().bits(duty)});
    }

    fn set_period<P>(&mut self, period: P)
    where
        P: Into<Self::Time>
    {
        let period = period.into();
        let params = TimerParams::new(period, self.clock_freq.0);
        let count = self.tc.count16();
        count.ctrla.modify(|_, w| w.enable().clear_bit());
        count.ctrla.modify(|_, w| {
                match params.divider {
                    1 => w.prescaler().div1(),
                    2 => w.prescaler().div2(),
                    4 => w.prescaler().div4(),
                    8 => w.prescaler().div8(),
                    16 => w.prescaler().div16(),
                    64 => w.prescaler().div64(),
                    256 => w.prescaler().div256(),
                    1024 => w.prescaler().div1024(),
                    _ => unreachable!(),
                }
            });
        count.ctrla.modify(|_, w| w.enable().set_bit());
        count.cc[0].write(|w| unsafe { w.cc().bits(params.cycles as u16) });
        while count.syncbusy.read().cc0().bit_is_set() {}
    }
}
)+}}

pwm! {
    Pwm0: (TC0, TC0Pinout, Tc0Tc1Clock, apbamask, tc0_),
    Pwm1: (TC1, TC1Pinout, Tc0Tc1Clock, apbamask, tc1_),
    Pwm2: (TC2, TC2Pinout, Tc2Tc3Clock, apbbmask, tc2_),
    Pwm3: (TC3, TC3Pinout, Tc2Tc3Clock, apbbmask, tc3_),
}

#[cfg(all(not(feature = "samd51g19a"), not(feature = "samd51g18a")))]
pwm! {
    Pwm4: (TC4, TC4Pinout, Tc4Tc5Clock, apbcmask, tc4_),
    Pwm5: (TC5, TC5Pinout, Tc4Tc5Clock, apbcmask, tc5_),
}
