use gpio::{Pa1, Pa5, Pa7, Pa9, Pa11, Pa13, Pa15, Pa17, Pa19, Pb31, PfE};

#[cfg(all(not(feature = "samd51g19a"), not(feature = "samd51g18a")))]
use gpio::{Pa23, Pa25, Pb9, Pb11, Pb13, Pb15};

use clock;
use timer::TimerParams;
use time::Hertz;
use hal::{Pwm, PwmPin};
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

pub enum Channel { 
    C0
}

macro_rules! pwm {
    ($($TYPE:ident: ($TC:ident, $pinout:ident, $clock:ident, $apmask:ident, $apbits:ident, $wrapper:ident),)+) => {
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
    type Channel = Channel;
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

pub struct $wrapper {
    pub pwm: $TYPE,
} 

impl PwmPin for $wrapper {
    type Duty = u16;
    fn disable(&mut self) {
        self.pwm.disable(Channel::C0);
    }
    fn enable(&mut self) {
        self.pwm.enable(Channel::C0);
    }
    fn get_duty(&self) -> Self::Duty {
        self.pwm.get_duty(Channel::C0)
    }
    fn get_max_duty(&self) -> Self::Duty {
        self.pwm.get_max_duty()
    }
    fn set_duty(&mut self, duty: Self::Duty) {
        self.pwm.set_duty(Channel::C0, duty);
    }
}

)+}}

pwm! {
    Pwm0: (TC0, TC0Pinout, Tc0Tc1Clock, apbamask, tc0_, Pwm0Wrapper),
    Pwm1: (TC1, TC1Pinout, Tc0Tc1Clock, apbamask, tc1_, Pwm1Wrapper),
    Pwm2: (TC2, TC2Pinout, Tc2Tc3Clock, apbbmask, tc2_, Pwm2Wrapper),
    Pwm3: (TC3, TC3Pinout, Tc2Tc3Clock, apbbmask, tc3_, Pwm3Wrapper),
}

#[cfg(all(not(feature = "samd51g19a"), not(feature = "samd51g18a")))]
pwm! {
    Pwm4: (TC4, TC4Pinout, Tc4Tc5Clock, apbcmask, tc4_, Pwm4Wrapper),
    Pwm5: (TC5, TC5Pinout, Tc4Tc5Clock, apbcmask, tc5_, Pwm5Wrapper),
}
