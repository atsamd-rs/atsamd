//! Working with timer counter hardware
use atsamd21g18a::tc3::COUNT16;
#[allow(unused)]
use atsamd21g18a::{TC3, TC4, TC5, GCLK, PM};
use hal::timer::{CountDown, Periodic};

use clock::{wait_for_gclk_sync, ClockGenerator, Clocks};
use nb;
use time::Hertz;

// Note:
// TC3 + TC4 can be paired to make a 32-bit counter
// TC5 + TC6 can be paired to make a 32-bit counter

pub struct TimerCounter<TC> {
    clocks: Clocks,
    tc: TC,
    generator: ClockGenerator,
}

pub trait Count16 {
    fn count16(&self) -> &COUNT16;
    fn configure(
        &self,
        clocks: &Clocks,
        gclk: &mut GCLK,
        pm: &mut PM,
        desired_freq: Hertz,
    ) -> ClockGenerator;
}

impl<TC> Periodic for TimerCounter<TC> {}
impl<TC> CountDown for TimerCounter<TC>
where
    TC: Count16,
{
    type Time = Hertz;

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Hertz>,
    {
        let timeout = timeout.into();
        let params = self.clocks
            .constrained_clock_params(self.generator, timeout);
        let divider = params.divider;
        let count = self.tc.count16();

        // Disable the timer while we reconfigure it
        count.ctrla.modify(|_, w| w.enable().clear_bit());
        while count.status.read().syncbusy().bit_is_set() {}

        // Now that we have a clock routed to the peripheral, we
        // can ask it to perform a reset.
        count.ctrla.write(|w| w.swrst().set_bit());
        while count.status.read().syncbusy().bit_is_set() {}
        // the SVD erroneously marks swrst as write-only, so we
        // need to manually read the bit here
        while count.ctrla.read().bits() & 1 != 0 {}

        count.ctrlbset.write(|w| {
            // Count up when the direction bit is zero
            w.dir().clear_bit();
            // Periodic
            w.oneshot().clear_bit()
        });

        // How many cycles of the clock need to happen to reach our
        // effective value.
        let cycles = params.effective_freq.0 / timeout.0;
        if cycles > u16::max_value() as u32 {
            panic!(
                "cycles {} is out of range for a 16 bit counter (timeout={})",
                cycles, timeout.0
            );
        }

        // Set TOP value for mfrq mode
        count.cc[0].write(|w| unsafe { w.cc().bits(cycles as u16) });

        count.ctrla.modify(|_, w| {
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
        let count = self.tc.count16();
        if count.intflag.read().ovf().bit_is_set() {
            // Writing a 1 clears the flag
            count.intflag.modify(|_, w| w.ovf().set_bit());
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl<TC> TimerCounter<TC>
where
    TC: Count16,
{
    pub fn enable_interrupt(&mut self) {
        self.tc.count16().intenset.write(|w| w.ovf().set_bit());
    }

    pub fn disable_interrupt(&mut self) {
        self.tc.count16().intenclr.write(|w| w.ovf().set_bit());
    }

    pub fn new<T: Into<Hertz>>(
        clocks: &Clocks,
        tc: TC,
        gclk: &mut GCLK,
        pm: &mut PM,
        desired_freq: T,
    ) -> Self {
        let generator = tc.configure(clocks, gclk, pm, desired_freq.into());
        Self {
            clocks: clocks.clone(),
            tc,
            generator,
        }
    }
}

macro_rules! tc {
    ($($TYPE:ident: ($TC:ident, $pm:ident, $ctrl:ident),)+) => {
        $(
pub type $TYPE = TimerCounter<$TC>;

impl Count16 for $TC {
    fn count16(&self) -> &COUNT16 {
        unsafe {
            &self.count16
        }
    }

    fn configure(&self, clocks: &Clocks, gclk: &mut GCLK, pm: &mut PM, desired_freq: Hertz) -> ClockGenerator {
        // this is safe because we're constrained to just the tc3 bit
        pm.apbcmask.modify(|_, w| w.$pm().set_bit());

        // Select an appropriate clock source based on the chosen
        // frequency.
        let freq = desired_freq.into();
        let params = clocks.clock_params(freq);

        {
            let count = self.count16();

            // Disable the timer while we reconfigure it
            count.ctrla.modify(|_, w| w.enable().clear_bit());
            while count.status.read().syncbusy().bit_is_set() {}

            gclk.clkctrl.write(|w| {
                w.id().$ctrl();
                w.gen().variant(params.generator.into());
                w.clken().set_bit()
            });
            wait_for_gclk_sync(gclk);
        }

        params.generator
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
