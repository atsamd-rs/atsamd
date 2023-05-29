#![allow(non_snake_case)]

use paste::paste;
use seq_macro::seq;

use crate::clock;
use crate::ehal::{Pwm, PwmPin};
use crate::gpio::*;
use crate::gpio::{AlternateE, AnyPin, Pin};
use crate::pac::MCLK;
use crate::time::Hertz;
use crate::timer_params::TimerParams;

seq!(N in 0..=7 {
    paste! {
        #[cfg(feature = "has-" tc~N)]
        use crate::pac::TC~N;
    }
});

seq!(N in 0..=4 {
    paste! {
        #[cfg(feature = "has-" tcc~N)]
        use crate::pac::TCC~N;
    }
});

// Timer/Counter (TCx)

/// This is a major syntax hack.
///
/// The previous Pinout types were enums that took specific v1::Pin types. As a
/// result, there was no way to make that implementation simultaneously
/// compatible with both v1::Pin and Pin.
///
/// BUT, the enum variant syntax is the same as the namespaced function syntax.
/// I converted the enums to structs, and I created constructor methods with the
/// same names as the previous enum variants. By constructing Pinout types with
/// functions rather than enum variants, you can make it generic over v1::Pin
/// and Pin types.
///
/// This is (mostly) backwards compatible with the current syntax, and all the
/// existing calls compile. The only incompatible change is the requirement of
/// type parameters on the Pwm types. Most of the type, the type parameters can
/// be inferred, so this is mostly backwards compatible as well. But there were
/// one or two instances where I had to add explicit type parameters to existing
/// BSP code.
macro_rules! impl_tc_pinout {
    (
        $Type:ident: [ $(
            $( #[$attr:meta] )?
            ($func: ident, $Id: ident)
        ),+ ]
    ) => {
        pub struct $Type<I: PinId> {
            _pin: Pin<I, AlternateE>,
        }

        $(
            $( #[$attr] )?
            impl $Type<$Id> {
                #[inline]
                pub fn $func(pin: impl AnyPin<Id = $Id>) -> Self {
                    let _pin = pin.into().into_alternate();
                    Self { _pin }
                }
            }
        )+
    };
}

#[cfg(feature = "has-tc0")]
impl_tc_pinout!(TC0Pinout: [
    (Pa5, PA05),
    (Pa9, PA09),
    #[cfg(feature = "pins-64")]
    (Pb31, PB31)
]);

#[cfg(feature = "has-tc1")]
impl_tc_pinout!(TC1Pinout: [
    (Pa7, PA07),
    (Pa11, PA11)
]);

#[cfg(feature = "has-tc2")]
impl_tc_pinout!(TC2Pinout: [
    #[cfg(feature = "has-pa01")]
    (Pa1, PA01),
    (Pa13, PA13),
    (Pa17, PA17)
]);

#[cfg(feature = "has-tc3")]
impl_tc_pinout!(TC3Pinout: [
    (Pa15, PA15),
    (Pa19, PA19)
]);

#[cfg(feature = "has-tc4")]
impl_tc_pinout!(TC4Pinout: [
    (Pa23, PA23),
    #[cfg(feature = "has-pb00")]
    (Pb0, PB09),
    #[cfg(feature = "pins-64")]
    (Pb13, PB13)
]);

#[cfg(feature = "has-tc5")]
impl_tc_pinout!(TC5Pinout: [
    (Pa25, PA25),
    (Pb11, PB11),
    #[cfg(feature = "pins-64")]
    (Pb15, PB15)
]);

#[cfg(feature = "has-tc6")]
impl_tc_pinout!(TC6Pinout: [
    #[cfg(feature = "has-pb03")]
    (Pb3, PB03),
    #[cfg(feature = "pins-64")]
    (Pb17, PB17),
    #[cfg(feature = "pins-64")]
    (Pa31, PA31)
]);

#[cfg(feature = "has-tc7")]
impl_tc_pinout!(TC7Pinout: [
    (Pa21, PA21),
    #[cfg(feature = "has-pb23")]
    (Pb23, PB23),
    #[cfg(feature = "has-pb01")]
    (Pb1, PB01)
]);

macro_rules! pwm {
    ($($TYPE:ident: ($TC:ident, $pinout:ident, $clock:ident, $apmask:ident, $apbits:ident, $wrapper:ident)),+) => {
        $(

pub struct $TYPE<I: PinId> {
    /// The frequency of the attached clock, not the period of the pwm.
    /// Used to calculate the period of the pwm.
    clock_freq: Hertz,
    tc: $TC,
    #[allow(dead_code)]
    pinout: $pinout<I>,
}

impl<I: PinId> $TYPE<I> {
    pub fn new(
        clock: &clock::$clock,
        freq: Hertz,
        tc: $TC,
        pinout: $pinout<I>,
        mclk: &mut MCLK,
    ) -> Self {
        let count = tc.count16();
        let params = TimerParams::new(freq.convert(), clock.freq());
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
        self.clock_freq / divisor as u32 / (top + 1) as u32
    }

    pub fn set_period(&mut self, period: Hertz)
    {
        let period = period.into();
        let params = TimerParams::new(period, self.clock_freq);
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

impl<I: PinId> PwmPin for $TYPE<I> {
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

#[cfg(feature = "has-tc0")]
pwm! { Pwm0: (TC0, TC0Pinout, Tc0Tc1Clock, apbamask, tc0_, Pwm0Wrapper) }
#[cfg(feature = "has-tc1")]
pwm! { Pwm1: (TC1, TC1Pinout, Tc0Tc1Clock, apbamask, tc1_, Pwm1Wrapper) }
#[cfg(feature = "has-tc2")]
pwm! { Pwm2: (TC2, TC2Pinout, Tc2Tc3Clock, apbbmask, tc2_, Pwm2Wrapper) }
#[cfg(feature = "has-tc3")]
pwm! { Pwm3: (TC3, TC3Pinout, Tc2Tc3Clock, apbbmask, tc3_, Pwm3Wrapper) }
#[cfg(feature = "has-tc4")]
pwm! { Pwm4: (TC4, TC4Pinout, Tc4Tc5Clock, apbcmask, tc4_, Pwm4Wrapper) }
#[cfg(feature = "has-tc5")]
pwm! { Pwm5: (TC5, TC5Pinout, Tc4Tc5Clock, apbcmask, tc5_, Pwm5Wrapper) }
#[cfg(feature = "has-tc6")]
pwm! { Pwm6: (TC6, TC6Pinout, Tc6Tc7Clock, apbdmask, tc6_, Pwm6Wrapper) }
#[cfg(feature = "has-tc7")]
pwm! { Pwm7: (TC7, TC7Pinout, Tc6Tc7Clock, apbdmask, tc7_, Pwm7Wrapper) }

// Timer/Counter for Control Applications (TCCx)

#[derive(Copy, Clone)]
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

/// This is a major syntax hack.
///
/// The previous Pinout types were enums that took specific v1::Pin types. As a
/// result, there was no way to make that implementation simultaneously
/// compatible with both v1::Pin and Pin.
///
/// BUT, the enum variant syntax is the same as the namespaced function syntax.
/// I converted the enums to structs, and I created constructor methods with the
/// same names as the previous enum variants. By constructing Pinout types with
/// functions rather than enum variants, you can make it generic over v1::Pin
/// and Pin types.
///
/// This is (mostly) backwards compatible with the current syntax, and all the
/// existing calls compile. The only incompatible change is the requirement of
/// type parameters on the Pwm types. Most of the type, the type parameters can
/// be inferred, so this is mostly backwards compatible as well. But there were
/// one or two instances where I had to add explicit type parameters to existing
/// BSP code.
macro_rules! impl_tcc_pinout {
    (
        $Type:ident: [ $(
            $( #[$attr:meta] )?
            ($func: ident, $Id: ident, $Mode:ident)
        ),+ ]
    ) => {
        pub struct $Type<I: PinId, M: PinMode> {
            _pin: Pin<I, M>,
        }

        $(
            $( #[$attr] )?
            impl $Type<$Id, $Mode> {
                #[inline]
                pub fn $func(pin: impl AnyPin<Id = $Id>) -> Self {
                    let _pin = pin.into().into_alternate();
                    Self { _pin }
                }
            }
        )+
    };
}

#[cfg(feature = "has-tcc0")]
impl_tcc_pinout!(TCC0Pinout: [
    (Pa8, PA08, AlternateF),
    (Pa9, PA09, AlternateF),
    (Pa10, PA10, AlternateF),
    (Pa11, PA11, AlternateF),
    (Pa12, PA12, AlternateF),
    (Pa13, PA13, AlternateF),
    (Pa16, PA16, AlternateG),
    (Pa17, PA17, AlternateG),
    (Pa18, PA18, AlternateG),
    (Pa19, PA19, AlternateG),
    (Pa20, PA20, AlternateG),
    (Pa21, PA21, AlternateG),
    (Pa22, PA22, AlternateG),
    (Pa23, PA23, AlternateG),
    (Pb10, PB10, AlternateF),
    (Pb11, PB11, AlternateF),
    #[cfg(feature = "pins-64")]
    (Pb12, PB12, AlternateG),
    #[cfg(feature = "pins-64")]
    (Pb13, PB13, AlternateG),
    #[cfg(feature = "pins-64")]
    (Pb14, PB14, AlternateG),
    #[cfg(feature = "pins-64")]
    (Pb15, PB15, AlternateG),
    #[cfg(feature = "pins-64")]
    (Pb16, PB16, AlternateG),
    #[cfg(feature = "pins-64")]
    (Pb17, PB17, AlternateG),
    #[cfg(feature = "pins-64")]
    (Pb30, PB30, AlternateG),
    #[cfg(feature = "pins-64")]
    (Pb31, PB31, AlternateG),
    #[cfg(feature = "pins-100")]
    (Pc10, PC10, AlternateF),
    #[cfg(feature = "pins-100")]
    (Pc11, PC11, AlternateF),
    #[cfg(feature = "pins-100")]
    (Pc12, PC12, AlternateF),
    #[cfg(feature = "pins-100")]
    (Pc13, PC13, AlternateF),
    #[cfg(feature = "pins-100")]
    (Pc14, PC14, AlternateF),
    #[cfg(feature = "pins-100")]
    (Pc15, PC15, AlternateF),
    #[cfg(feature = "pins-100")]
    (Pc16, PC16, AlternateF),
    #[cfg(feature = "pins-100")]
    (Pc17, PC17, AlternateF),
    #[cfg(feature = "pins-100")]
    (Pc18, PC18, AlternateF),
    #[cfg(feature = "pins-100")]
    (Pc19, PC19, AlternateF),
    #[cfg(feature = "pins-100")]
    (Pc20, PC20, AlternateF),
    #[cfg(feature = "pins-100")]
    (Pc21, PC21, AlternateF),
    #[cfg(feature = "pins-128")]
    (Pc4, PC04, AlternateF),
    #[cfg(feature = "pins-128")]
    (Pc22, PC22, AlternateF),
    #[cfg(feature = "pins-128")]
    (Pc23, PC23, AlternateF),
    #[cfg(feature = "pins-128")]
    (Pd8, PD08, AlternateF),
    #[cfg(feature = "pins-128")]
    (Pd9, PD09, AlternateF),
    #[cfg(feature = "pins-128")]
    (Pd10, PD10, AlternateF),
    #[cfg(feature = "pins-128")]
    (Pd11, PD11, AlternateF),
    #[cfg(feature = "pins-128")]
    (Pd12, PD12, AlternateF)
]);

#[cfg(feature = "has-tcc1")]
impl_tcc_pinout!(TCC1Pinout: [
    (Pa8, PA08, AlternateG),
    (Pa9, PA09, AlternateG),
    (Pa10, PA10, AlternateG),
    (Pa11, PA11, AlternateG),
    (Pa12, PA12, AlternateG),
    (Pa13, PA13, AlternateG),
    (Pa14, PA14, AlternateG),
    (Pa15, PA15, AlternateG),
    (Pa16, PA16, AlternateF),
    (Pa17, PA17, AlternateF),
    (Pa18, PA18, AlternateF),
    (Pa19, PA19, AlternateF),
    (Pa20, PA20, AlternateF),
    (Pa21, PA21, AlternateF),
    (Pa22, PA22, AlternateF),
    (Pa23, PA23, AlternateF),
    (Pb10, PB10, AlternateG),
    (Pb11, PB11, AlternateG),
    #[cfg(feature = "pins-100")]
    (Pb18, PB18, AlternateF),
    #[cfg(feature = "pins-100")]
    (Pb19, PB19, AlternateF),
    #[cfg(feature = "pins-100")]
    (Pb20, PB20, AlternateF),
    #[cfg(feature = "pins-100")]
    (Pb21, PB21, AlternateF),
    #[cfg(feature = "pins-100")]
    (Pc10, PC10, AlternateG),
    #[cfg(feature = "pins-100")]
    (Pc11, PC11, AlternateG),
    #[cfg(feature = "pins-100")]
    (Pc12, PC12, AlternateG),
    #[cfg(feature = "pins-100")]
    (Pc13, PC13, AlternateG),
    #[cfg(feature = "pins-100")]
    (Pc14, PC14, AlternateG),
    #[cfg(feature = "pins-100")]
    (Pc15, PC15, AlternateG),
    #[cfg(feature = "pins-128")]
    (Pb26, PB26, AlternateF),
    #[cfg(feature = "pins-128")]
    (Pb27, PB27, AlternateF),
    #[cfg(feature = "pins-128")]
    (Pb28, PB28, AlternateF),
    #[cfg(feature = "pins-128")]
    (Pb29, PB29, AlternateF),
    #[cfg(feature = "pins-128")]
    (Pd20, PD20, AlternateF),
    #[cfg(feature = "pins-128")]
    (Pd21, PD21, AlternateF)
]);

#[cfg(feature = "has-tcc2")]
impl_tcc_pinout!(TCC2Pinout: [
    (Pa14, PA14, AlternateF),
    (Pa15, PA15, AlternateF),
    (Pa24, PA24, AlternateF),
    (Pa30, PA30, AlternateF),
    (Pa31, PA31, AlternateF),
    #[cfg(feature = "has-pb02")]
    (Pb2,  PB02, AlternateF)
]);

#[cfg(feature = "has-tcc3")]
impl_tcc_pinout!(TCC3Pinout: [
    #[cfg(feature = "pins-64")]
    (Pb12, PB12, AlternateF),
    #[cfg(feature = "pins-64")]
    (Pb13, PB13, AlternateF),
    #[cfg(feature = "pins-64")]
    (Pb16, PB16, AlternateF),
    #[cfg(feature = "pins-64")]
    (Pb17, PB17, AlternateF)
]);

#[cfg(feature = "has-tcc4")]
impl_tcc_pinout!(TCC4Pinout: [
    #[cfg(feature = "pins-64")]
    (Pb14, PB14, AlternateF),
    #[cfg(feature = "pins-64")]
    (Pb15, PB15, AlternateF),
    #[cfg(feature = "pins-64")]
    (Pb30, PB30, AlternateF),
    #[cfg(feature = "pins-64")]
    (Pb31, PB31, AlternateF)
]);

macro_rules! pwm_tcc {
    ($($TYPE:ident: ($TCC:ident, $pinout:ident, $clock:ident, $apmask:ident, $apbits:ident, $wrapper:ident)),+) => {
        $(

pub struct $TYPE<I: PinId, M: PinMode> {
    /// The frequency of the attached clock, not the period of the pwm.
    /// Used to calculate the period of the pwm.
    clock_freq: Hertz,
    tcc: $TCC,
    #[allow(dead_code)]
    pinout: $pinout<I, M>,
}

impl<I: PinId, M: PinMode> $TYPE<I, M> {
    pub fn new(
        clock: &clock::$clock,
        freq: Hertz,
        tcc: $TCC,
        pinout: $pinout<I, M>,
        mclk: &mut MCLK,
    ) -> Self {
        let params = TimerParams::new(freq.convert(), clock.freq());
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

        Self {
            clock_freq: clock.freq(),
            tcc,
            pinout,
        }
    }
}

impl<I: PinId, M: PinMode> Pwm for $TYPE<I, M> {
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
        self.clock_freq / divisor as u32 / (top + 1) as u32
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
        let params = TimerParams::new(period.into().convert(), self.clock_freq);
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

#[cfg(feature = "has-tcc0")]
pwm_tcc! { Tcc0Pwm: (TCC0, TCC0Pinout, Tcc0Tcc1Clock, apbbmask, tcc0_, TccPwm0Wrapper) }
#[cfg(feature = "has-tcc1")]
pwm_tcc! { Tcc1Pwm: (TCC1, TCC1Pinout, Tcc0Tcc1Clock, apbbmask, tcc1_, TccPwm1Wrapper) }
#[cfg(feature = "has-tcc2")]
pwm_tcc! { Tcc2Pwm: (TCC2, TCC2Pinout, Tcc2Tcc3Clock, apbcmask, tcc2_, TccPwm2Wrapper) }
#[cfg(feature = "has-tcc3")]
pwm_tcc! { Tcc3Pwm: (TCC3, TCC3Pinout, Tcc2Tcc3Clock, apbcmask, tcc3_, TccPwm3Wrapper) }
#[cfg(feature = "has-tcc4")]
pwm_tcc! { Tcc4Pwm: (TCC4, TCC4Pinout, Tcc4Clock,     apbdmask, tcc4_, TccPwm4Wrapper) }
