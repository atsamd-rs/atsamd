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

use gpio::{
    Pa0, Pa1, Pa4, Pa5, Pa6, Pa7, Pa8, Pa9, Pa10, Pa11, Pa12, Pa13, Pa14, Pa15,
    Pa16, Pa17, Pa18, Pa19, Pb30, Pb31, PfE
};
use clock;
use timer::TimerParams;
use time::Hertz;
use hal::Pwm;

use target_device::{TC0, TC1, TC2, TC3, MCLK};

pub enum TC0Pinout {
    Pa4Pa5(Pa4<PfE>, Pa5<PfE>),
    Pa8Pa9(Pa8<PfE>, Pa9<PfE>),
    Pb30Pb31(Pb30<PfE>, Pb31<PfE>),
}

pub enum TC1Pinout {
    Pa6Pa7(Pa6<PfE>, Pa7<PfE>),
    Pa10Pa11(Pa10<PfE>, Pa11<PfE>),
}

pub enum TC3Pinout {
    Pa14Pa15(Pa14<PfE>, Pa15<PfE>),
    Pa18Pa19(Pa18<PfE>, Pa19<PfE>),
}

pub enum TC2Pinout {
    //Pa0Pa1(Pa0<PfE>, Pa1<PfE>),
    //Pa12Pa13(Pa12<PfE>, Pa13<PfE>),
    //Pa16Pa17(Pa16<PfE>, Pa17<PfE>),
    Pa17(Pa17<PfE>)
}

pub struct Pwm2 {
    /// The frequency of the attached clock, not the period of the pwm.
    /// Used to calculate the period of the pwm.
    clock_freq: Hertz,
    tc: TC2,
    pinout: TC2Pinout,
}

impl Pwm2 {
    pub fn new<F: Into<Hertz>> (
        clock: &clock::Tc2Tc3Clock,
        freq: F,
        tc: TC2,
        pinout: TC2Pinout,
        mclk: &mut MCLK,
    ) -> Pwm2 {
        let freq = freq.into();
        {
            let count = tc.count16();
            let prescaler: [u16;8] = [ 1, 2, 4, 8, 16, 64, 256, 1024 ];
            let mut divider: u8 = 0;
            let mut top: u32 = 0;
            while divider < 8 {
                top = (clock.freq().0 / prescaler[divider as usize] as u32 / freq.0) - 1;
                if top < (1 << 16) {
                    break;
                }
                divider += 1;
            }
            mclk.apbbmask.modify(|_, w| w.tc2_().set_bit());
            count.ctrla.write(|w| w.swrst().set_bit());
            while count.ctrla.read().bits() & 1 != 0 {}
            count.ctrla.modify(|_, w| w.enable().clear_bit());
            count.ctrla.modify(|_, w| {
                match prescaler[divider as usize] {
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
            count.cc[0].write(|w| unsafe { w.cc().bits(top as u16) });
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

pub enum Channels { 
    C0
}

impl Pwm for Pwm2 {
    type Channel = Channels;
    type Time = Hertz;
    type Duty = u16;

    fn disable(&mut self, channel: Self::Channel) {
        // We don't have the ability to do this on a channel by channel basis,
        // and we only have 1 channel anyways
    }

    fn enable(&mut self, channel: Self::Channel) {
        // We don't have the ability to do this on a channel by channel basis,
        // and we only have 1 channel anyways
    }

    fn get_period(&self) -> Self::Time {
        let count = self.tc.count16();
        let divisor = count.ctrla.read().prescaler().bits(); 
        let top = count.cc[0].read().cc().bits();
        Hertz(self.clock_freq.0 / divisor as u32 / (top + 1) as u32)
    }

    fn get_duty(&self, channel: Self::Channel) -> Self::Duty {
        let count = self.tc.count16();
        let duty: u16 = count.cc[1].read().cc().bits();
        duty 
    }

    fn get_max_duty(&self) -> Self::Duty {
        let count = self.tc.count16();
        let top = count.cc[0].read().cc().bits();
        top
    }

    fn set_duty(&mut self, channel: Self::Channel, duty: Self::Duty) {
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
        let top: u16 = (self.clock_freq.0 / params.divider as u32 / params.cycles) as u16 - 1;
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
        count.cc[0].write(|w| unsafe { w.cc().bits(top) });
        while count.syncbusy.read().cc0().bit_is_set() {}
    }
}
