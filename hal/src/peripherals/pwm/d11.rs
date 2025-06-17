use atsamd_hal_macros::hal_cfg;

use crate::clock;
use crate::pac::Pm;
use crate::time::Hertz;
use crate::timer_params::TimerParams;

// Timer/Counter (TCx)

macro_rules! pwm {
    ($($TYPE:ident: ($TC:ident, $clock:ident, $apmask:ident, $apbits:ident, $wrapper:ident)),+) => {
        $(

pub struct $TYPE {
    /// The frequency of the attached clock, not the period of the pwm.
    /// Used to calculate the period of the pwm.
    clock_freq: Hertz,
    tc: crate::pac::$TC,
}

impl $TYPE {
    pub fn new(
        clock: &clock::$clock,
        freq: Hertz,
        tc: crate::pac::$TC,
        pm: &mut Pm,
    ) -> Self {
        let count = tc.count16();
        let params = TimerParams::new(freq.convert(), clock.freq());
        pm.$apmask().modify(|_, w| w.$apbits().set_bit());
        count.ctrla().write(|w| w.swrst().set_bit());
        while count.ctrla().read().bits() & 1 != 0 {}
        count.ctrla().modify(|_, w| w.enable().clear_bit());
        while count.status().read().syncbusy().bit_is_set() {}
        count.ctrla().modify(|_, w| {
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
        count.ctrla().write(|w| w.wavegen().mpwm());
        count.cc(0).write(|w| unsafe { w.cc().bits(params.cycles as u16) });
        count.cc(1).write(|w| unsafe { w.cc().bits(0) });
        count.ctrla().modify(|_, w| w.enable().set_bit());
        while count.status().read().syncbusy().bit_is_set() {}

        Self {
            clock_freq: clock.freq(),
            tc,
        }
    }

    pub fn set_period(&mut self, period: Hertz)
    {
        let params = TimerParams::new(period, self.clock_freq);
        let count = self.tc.count16();
        count.ctrla().modify(|_, w| w.enable().clear_bit());
        while count.status().read().syncbusy().bit_is_set() {}
        count.ctrla().modify(|_, w| {
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
        count.ctrla().modify(|_, w| w.enable().set_bit());
        while count.status().read().syncbusy().bit_is_set() {}
        count.cc(0).write(|w| unsafe { w.cc().bits(params.cycles as u16) });
    }

    pub fn get_period(&self) -> Hertz {
        let count = self.tc.count16();
        let divisor = count.ctrla().read().prescaler().bits();
        let top = count.cc(0).read().cc().bits();
        self.clock_freq / divisor as u32 / (top + 1) as u32
    }
}

impl $crate::ehal::pwm::ErrorType for$TYPE {
    type Error = ::core::convert::Infallible;
}

impl $crate::ehal::pwm::SetDutyCycle for $TYPE {
    fn max_duty_cycle(&self) -> u16 {
        let count = self.tc.count16();
        let top = count.cc(0).read().cc().bits();
        top
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        let count = self.tc.count16();
        unsafe { count.cc(1).write(|w| w.cc().bits(duty)); }
        Ok(())
    }
}

impl $crate::ehal_02::PwmPin for $TYPE {
    type Duty = u16;

    fn disable(&mut self) {
        let count = self.tc.count16();
        count.ctrla().modify(|_, w| w.enable().clear_bit());
        while count.status().read().syncbusy().bit_is_set() {}
    }

    fn enable(&mut self) {
        let count = self.tc.count16();
        count.ctrla().modify(|_, w| w.enable().set_bit());
        while count.status().read().syncbusy().bit_is_set() {}
    }

    fn get_duty(&self) -> Self::Duty {
        let count = self.tc.count16();
        let duty: u16 = count.cc(1).read().cc().bits();
        duty
    }

    fn get_max_duty(&self) -> Self::Duty {
        use $crate::ehal::pwm::SetDutyCycle;
        self.max_duty_cycle()
    }

    fn set_duty(&mut self, duty: Self::Duty) {
        use $crate::ehal::pwm::SetDutyCycle;
        let _ignore_infaillible = self.set_duty_cycle(duty);
    }
}

)+}}

#[hal_cfg("tc1")]
pwm! { Pwm1: (Tc1, Tc1Tc2Clock, apbcmask, tc1_, Pwm1Wrapper) }
#[hal_cfg("tc2")]
pwm! { Pwm2: (Tc2, Tc1Tc2Clock, apbcmask, tc2_, Pwm2Wrapper) }
#[hal_cfg("tc3")]
pwm! { Pwm3: (Tc3, Tcc2Tc3Clock, apbcmask, tc3_, Pwm3Wrapper) }
#[hal_cfg("tc4")]
pwm! { Pwm4: (Tc4, Tc4Tc5Clock, apbcmask, tc4_, Pwm4Wrapper) }
#[hal_cfg("tc5")]
pwm! { Pwm5: (Tc5, Tc4Tc5Clock, apbcmask, tc5_, Pwm5Wrapper) }

#[hal_cfg("tc6")]
pwm! { Pwm6: (Tc6, Tc6Tc7Clock, apbcmask, tc6_, Pwm6Wrapper) }
#[hal_cfg("tc7")]
pwm! { Pwm7: (Tc7, Tc6Tc7Clock, apbcmask, tc7_, Pwm7Wrapper) }

// Timer/Counter for Control Applications (TCCx)

#[derive(Copy, Clone)]
pub enum Channel {
    _0,
    _1,
    _2,
    _3,
}

macro_rules! pwm_tcc {
    ($($TYPE:ident: ($TCC:ident, $clock:ident, $apmask:ident, $apbits:ident, $wrapper:ident)),+) => {
        $(

pub struct $TYPE {
    /// The frequency of the attached clock, not the period of the pwm.
    /// Used to calculate the period of the pwm.
    clock_freq: Hertz,
    tcc: crate::pac::$TCC,
}

impl $TYPE {
    pub fn new<F: Into<Hertz>> (
        clock: &clock::$clock,
        freq: F,
        tcc: crate::pac::$TCC,
        pm: &mut Pm,
    ) -> Self {
        let freq = freq.into();
        {
            let params = TimerParams::new(freq, clock.freq());
            pm.$apmask().modify(|_, w| w.$apbits().set_bit());
            tcc.ctrla().write(|w| w.swrst().set_bit());
            while tcc.syncbusy().read().swrst().bit_is_set() {}
            tcc.ctrlbclr().write(|w| w.dir().set_bit() );
            while tcc.syncbusy().read().ctrlb().bit_is_set() {}
            tcc.ctrla().modify(|_, w| w.enable().clear_bit());
            while tcc.syncbusy().read().enable().bit_is_set() {}
            tcc.ctrla().modify(|_, w| {
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
            tcc.wave().write(|w| w.wavegen().npwm());
            while tcc.syncbusy().read().wave().bit_is_set() {}
            tcc.per().write(|w| unsafe { w.bits(params.cycles as u32) });
            while tcc.syncbusy().read().per().bit_is_set() {}
            tcc.ctrla().modify(|_, w| w.enable().set_bit());
            while tcc.syncbusy().read().enable().bit_is_set() {}
        }

        Self {
            clock_freq: clock.freq(),
            tcc,
        }
    }
}

impl $crate::ehal_02::Pwm for $TYPE {
    type Channel = Channel;
    type Time = Hertz;
    type Duty = u32;

    fn disable(&mut self, _channel: Self::Channel) {
        self.tcc.ctrla().modify(|_, w| w.enable().clear_bit());
        while self.tcc.syncbusy().read().enable().bit_is_set() {}
    }

    fn enable(&mut self, _channel: Self::Channel) {
        self.tcc.ctrla().modify(|_, w| w.enable().set_bit());
        while self.tcc.syncbusy().read().enable().bit_is_set() {}
    }

    fn get_period(&self) -> Self::Time {
        let divisor = self.tcc.ctrla().read().prescaler().bits();
        let top = self.tcc.per().read().bits();
        self.clock_freq / divisor as u32 / (top + 1) as u32
    }

    fn get_duty(&self, channel: Self::Channel) -> Self::Duty {
        let cc = self.tcc.cc(channel as usize);
        let duty: u32 = cc.read().cc().bits();
        duty
    }

    fn get_max_duty(&self) -> Self::Duty {
        let top = self.tcc.per().read().bits();
        top
    }

    fn set_duty(&mut self, channel: Self::Channel, duty: Self::Duty) {
        let cc = self.tcc.cc(channel as usize);
        cc.write(|w| unsafe { w.cc().bits(duty) });
    }

    fn set_period<P>(&mut self, period: P)
    where
        P: Into<Self::Time>,
    {
        let period = period.into();
        let params = TimerParams::new(period, self.clock_freq);
        self.tcc.ctrla().modify(|_, w| w.enable().clear_bit());
        while self.tcc.syncbusy().read().enable().bit_is_set() {}
        self.tcc.ctrla().modify(|_, w| {
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
        self.tcc.ctrla().modify(|_, w| w.enable().set_bit());
        while self.tcc.syncbusy().read().enable().bit_is_set() {}
        self.tcc.per().write(|w| unsafe { w.bits(params.cycles as u32) });
        while self.tcc.syncbusy().read().per().bit() {}
    }
}

)+}}

#[hal_cfg("tcc0-d11")]
pwm_tcc! { Pwm0: (Tcc0, Tcc0Clock, apbcmask, tcc0_, Pwm0Wrapper) }
#[hal_cfg("tcc0-d21")]
pwm_tcc! { Pwm0: (Tcc0, Tcc0Tcc1Clock, apbcmask, tcc0_, Pwm0Wrapper) }
#[hal_cfg("tcc1")]
pwm_tcc! { Pwm1: (Tcc1, Tcc0Tcc1Clock, apbcmask, tcc1_, Pwm1Wrapper) }
#[hal_cfg("tcc1")]
pwm_tcc! { Pwm2: (Tcc2, Tcc2Tc3Clock, apbcmask, tcc2_, Pwm2Wrapper) }
