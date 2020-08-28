use crate::clock;
use crate::gpio::*;
use crate::hal::{Pwm, PwmPin};
use crate::time::Hertz;
use crate::timer::TimerParams;

use crate::target_device::{MCLK, TC0, TC1, TC2, TC3, TCC0, TCC1, TCC2};
#[cfg(all(not(feature = "samd51g19a"), not(feature = "samd51g18a")))]
use crate::target_device::{TC4, TC5, TCC3, TCC4};
#[cfg(all(feature = "samd51p19a", feature = "same54"))]
use crate::target_device::{TC6, TC7};

// Timer/Counter (TCx)

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

#[cfg(any(feature = "samd51p19a", feature = "same54"))]
pub enum TC6Pinout {
    Pb3(Pb3<PfE>),
    Pb17(Pb17<PfE>),
    Pa31(Pa31<PfE>),
}

#[cfg(any(feature = "samd51p19a", feature = "same54"))]
pub enum TC7Pinout {
    Pa21(Pa21<PfE>),
    Pb23(Pb23<PfE>),
    Pb1(Pb1<PfE>),
}

macro_rules! pwm {
    ($($TYPE:ident: ($TC:ident, $pinout:ident, $clock:ident, $apmask:ident, $apbits:ident, $wrapper:ident),)+) => {
        $(

pub struct $TYPE {
    /// The frequency of the attached clock, not the period of the pwm.
    /// Used to calculate the period of the pwm.
    clock_freq: Hertz,
    tc: $TC,
    #[allow(dead_code)]
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

    pub fn get_period(&self) -> Hertz {
        let count = self.tc.count16();
        let divisor = count.ctrla.read().prescaler().bits();
        let top = count.cc[0].read().cc().bits();
        Hertz(self.clock_freq.0 / divisor as u32 / (top + 1) as u32)
    }

    pub fn set_period<P>(&mut self, period: P)
    where
        P: Into<Hertz>
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

impl PwmPin for $TYPE {
    type Duty = u16;

    fn disable(&mut self) {
        let count = self.tc.count16();
        count.ctrla.modify(|_, w| w.enable().clear_bit());
    }

    fn enable(&mut self) {
        let count = self.tc.count16();
        count.ctrla.modify(|_, w| w.enable().set_bit());
    }


    fn get_duty(&self) -> Self::Duty {
        let count = self.tc.count16();
        let duty: u16 = count.ccbuf[1].read().ccbuf().bits();
        duty
    }

    fn get_max_duty(&self) -> Self::Duty {
        let count = self.tc.count16();
        let top = count.cc[0].read().cc().bits();
        top
    }

    fn set_duty(&mut self, duty: Self::Duty) {
        let count = self.tc.count16();
        count.ccbuf[1].write(|w| unsafe {w.ccbuf().bits(duty)});
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

#[cfg(all(feature = "samd51p19a", feature = "same54"))]
pwm! {
    Pwm6: (TC6, TC6Pinout, Tc6Tc7Clock, apbdmask, tc6_, Pwm6Wrapper),
    Pwm7: (TC7, TC7Pinout, Tc6Tc7Clock, apbdmask, tc7_, Pwm7Wrapper),
}

// Timer/Counter for Control Applications (TCCx)

pub enum Channel {
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
}

macro_rules! pwm_tcc {
    ($($TYPE:ident: ($TCC:ident, $clock:ident, $apmask:ident, $apbits:ident, $wrapper:ident),)+) => {
        $(

pub struct $TYPE {
    /// The frequency of the attached clock, not the period of the pwm.
    /// Used to calculate the period of the pwm.
    clock_freq: Hertz,
    tcc: $TCC,
}

impl $TYPE {
    pub fn new<F: Into<Hertz>> (
        clock: &clock::$clock,
        freq: F,
        tcc: $TCC,
        mclk: &mut MCLK,
    ) -> Self {
        let freq = freq.into();
        {
            let params = TimerParams::new(freq, clock.freq().0);
            mclk.$apmask.modify(|_, w| w.$apbits().set_bit());
            tcc.ctrla.write(|w| w.swrst().set_bit());
            while tcc.syncbusy.read().swrst().bit_is_set() {}
            tcc.ctrlbclr.write(|w| w.dir().set_bit() );
            while tcc.syncbusy.read().ctrlb().bit_is_set() {}
            tcc.ctrla.modify(|_, w| w.enable().clear_bit());
            tcc.ctrla.modify(|_, w| {
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
            tcc.wave.write(|w| w.wavegen().npwm());
            while tcc.syncbusy.read().wave().bit_is_set() {}
            tcc.per().write(|w| unsafe { w.bits(params.cycles as u32) });
            while tcc.syncbusy.read().per().bit_is_set() {}
            tcc.ctrla.modify(|_, w| w.enable().set_bit());
        }

        Self {
            clock_freq: clock.freq(),
            tcc,
        }
    }
}

impl Pwm for $TYPE {
    type Channel = Channel;
    type Time = Hertz;
    type Duty = u32;

    fn disable(&mut self, _channel: Self::Channel) {
        self.tcc.ctrla.modify(|_, w| w.enable().clear_bit());
        while self.tcc.syncbusy.read().enable().bit_is_set() {}
    }

    fn enable(&mut self, _channel: Self::Channel) {
        self.tcc.ctrla.modify(|_, w| w.enable().set_bit());
        while self.tcc.syncbusy.read().enable().bit_is_set() {}
    }

    fn get_period(&self) -> Self::Time {
        let divisor = self.tcc.ctrla.read().prescaler().bits();
        let top = self.tcc.per().read().bits();
        Hertz(self.clock_freq.0 / divisor as u32 / (top + 1) as u32)
    }

    fn get_duty(&self, channel: Self::Channel) -> Self::Duty {
        let cc = self.tcc.cc();
        let duty = cc[channel as usize].read().cc().bits();
        duty
    }

    fn get_max_duty(&self) -> Self::Duty {
        let top = self.tcc.per().read().bits();
        top
    }

    fn set_duty(&mut self, channel: Self::Channel, duty: Self::Duty) {
        let cc = self.tcc.cc();
        cc[channel as usize].write(|w| unsafe { w.cc().bits(duty) });
    }

    fn set_period<P>(&mut self, period: P)
    where
        P: Into<Self::Time>,
    {
        let period = period.into();
        let params = TimerParams::new(period, self.clock_freq.0);
        self.tcc.ctrla.modify(|_, w| w.enable().clear_bit());
        while self.tcc.syncbusy.read().enable().bit_is_set() {}
        self.tcc.ctrla.modify(|_, w| {
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
        self.tcc.ctrla.modify(|_, w| w.enable().set_bit());
        while self.tcc.syncbusy.read().enable().bit_is_set() {}
        self.tcc.per().write(|w| unsafe { w.bits(params.cycles as u32) });
        while self.tcc.syncbusy.read().per().bit() {}
    }
}

        )+
    };
}

pwm_tcc! {
    Tcc0Pwm: (TCC0, Tcc0Tcc1Clock, apbbmask, tcc0_, TccPwm0Wrapper),
    Tcc1Pwm: (TCC1, Tcc0Tcc1Clock, apbbmask, tcc1_, TccPwm1Wrapper),
    Tcc2Pwm: (TCC2, Tcc2Tcc3Clock, apbcmask, tcc2_, TccPwm2Wrapper),
}

#[cfg(all(not(feature = "samd51g19a"), not(feature = "samd51g18a")))]
pwm_tcc! {
    Tcc3Pwm: (TCC3, Tcc2Tcc3Clock, apbcmask, tcc3_, TccPwm3Wrapper),
    Tcc4Pwm: (TCC4, Tcc4Clock,     apbdmask, tcc4_, TccPwm4Wrapper),
}
