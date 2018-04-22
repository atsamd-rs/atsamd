//! Working with timer counter hardware
use atsamd21g18a::tc3::COUNT16;
#[allow(unused)]
use atsamd21g18a::{TC3, TC4, TC5, GCLK, PM};
use hal::timer::{CountDown, Periodic};

use clock::{Clocks, GenericClockGenerator};
use nb;
use time::Hertz;

// Note:
// TC3 + TC4 can be paired to make a 32-bit counter
// TC5 + TC6 can be paired to make a 32-bit counter

pub struct Width16;
pub struct Width32;

pub struct TimerCounter<TC> {
    clocks: Clocks,
    timeout: Hertz,
    tc: TC,
}

macro_rules! tc {
    ($($TYPE:ident: ($TC:ident, $pm:ident, $ctrl:ident),)+) => {
        $(
pub type $TYPE = TimerCounter<$TC>;

impl Periodic for TimerCounter<$TC> {}

impl CountDown for TimerCounter<$TC> {
    type Time = Hertz;

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Hertz>,
    {
        let timeout = timeout.into();
        let params = self.clocks.clock_params(timeout);
        let divider = params.divider;
        self.timeout = timeout;
        let mode = self.mode();

        // Disable the timer while we reconfigure it
        mode.ctrla.modify(|_, w| w.enable().clear_bit());
        while mode.status.read().syncbusy().bit_is_set() {}

        // Select an appropriate clock source based on the chosen
        // frequency.
        Self::clock_enable(params.generator);

        // Now that we have a clock routed to the peripheral, we
        // can ask it to perform a reset.
        mode.ctrla.write(|w| w.swrst().set_bit());
        while mode.status.read().syncbusy().bit_is_set() {}
        // the SVD erroneously marks swrst as write-only, so we
        // need to manually read the bit here
        while mode.ctrla.read().bits() & 1 != 0 {}

        mode.ctrlbset.write(|w| {
            // Count up when the direction bit is zero
            w.dir().clear_bit();
            // Periodic
            w.oneshot().clear_bit()
        });

        // How many cycles of the clock need to happen to reach our
        // effective value.
        let count = params.effective_freq.0 / timeout.0;
        if count > u16::max_value() as u32 {
            panic!(
                "count {} is out of range for a 16 bit counter (timeout={})",
                count, timeout.0
            );
        }

        // Set TOP value for mfrq mode
        mode.cc[0].write(|w| unsafe { w.cc().bits(count as u16) });

        mode.ctrla.modify(|_, w| {
            match divider {
                1 => w.prescaler().div1(),
                2 => w.prescaler().div2(),
                4 => w.prescaler().div4(),
                8 => w.prescaler().div8(),
                16 => w.prescaler().div16(),
                64 => w.prescaler().div64(),
                256 => w.prescaler().div256(),
                1024 => w.prescaler().div1024(),
                _ => unreachable!(),
            };
            // Enable Match Frequency Waveform generation
            w.wavegen().mfrq();
            w.enable().set_bit()
        });
    }

    fn wait(&mut self) -> nb::Result<(), !> {
        if self.mode().intflag.read().ovf().bit_is_set() {
            // Writing a 1 clears the flag
            self.mode().intflag.modify(|_, w| w.ovf().set_bit());
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl TimerCounter<$TC> {
    pub fn new(clocks: &Clocks, tc: $TC) -> Self {
        Self::power_on();

        Self {
            clocks: clocks.clone(),
            timeout: Hertz(0),
            tc,
        }
    }

    fn power_on() {
        // this is safe because we're constrained to just the tc3 bit
        unsafe {
            (*PM::ptr()).apbcmask.modify(|_, w| w.$pm().set_bit());
        }
    }

    pub fn enable_interrupt(&mut self) {
        self.mode().intenset.write(|w| w.ovf().set_bit());
    }

    pub fn disable_interrupt(&mut self) {
        self.mode().intenclr.write(|w| w.ovf().set_bit());
    }

    #[allow(unused)]
    fn power_off() {
        // this is safe because we're constrained to just the tc3 bit
        unsafe {
            (*PM::ptr()).apbcmask.modify(|_, w| w.$pm().clear_bit());
        }
    }

    fn clock_enable(generator: GenericClockGenerator) {
        // this is potentially unsafe because we may mess with a paired
        // timer!
        unsafe {
            (*GCLK::ptr()).clkctrl.write(|w| {
                w.id().$ctrl();
                w.gen().variant(generator);
                w.clken().set_bit()
            });
        }
    }

    #[allow(unused)]
    fn clock_disable() {
        // this is potentially unsafe because we may mess with a paired
        // timer!
        unsafe {
            (*GCLK::ptr()).clkctrl.write(|w| {
                w.id().$ctrl();
                w.gen().gclk0();
                w.clken().clear_bit()
            });
        }
    }

    fn mode(&mut self) -> &COUNT16 {
        unsafe { &self.tc.count16 }
    }
}
        )+
    }
}

tc! {
    TimerCounter3: (TC3, tc3_, tcc2_tc3),
    TimerCounter4: (TC4, tc4_, tc4_tc5),
    // take care: tc4 and tc5 have linked clock generators!
    // TimerCounter5: (TC5, tc5_, tc4_tc5),
}
