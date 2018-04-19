//! Working with timer counter hardware
use core::marker::PhantomData;

use atsamd21g18a::tc3::{COUNT16, COUNT32, COUNT8};
use atsamd21g18a::{TC3, GCLK, PM};
use hal::timer::{CountDown, Periodic};

use clock::{Clocks, GenericClockGenerator};
use nb;
use time::Hertz;

// pub struct Timer<INSTANCE> { }

// TC3 + TC4 can be paired to make a 32-bit counter
// TC5 + TC6 can be paired to make a 32-bit counter

pub struct Width16;
pub struct Width32;

pub struct TimerCounter<TC, WIDTH> {
    clocks: Clocks,
    timeout: Hertz,
    tc: TC,
    _count: PhantomData<WIDTH>,
}

pub type TimerCounter3_16 = TimerCounter<TC3, Width16>;

impl Periodic for TimerCounter<TC3, Width16> {}

impl CountDown for TimerCounter<TC3, Width16> {
    type Time = Hertz;

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Hertz>,
    {
        let timeout = timeout.into();
        let params = self.clocks.clock_params(timeout);
        let divider = params.divider;
        let mode = self.mode();

        // Disable the timer while we reconfigure it
        mode.ctrla.modify(|_, w| w.enable().clear_bit());
        while mode.status.read().syncbusy().bit_is_set() {}

        mode.ctrlbset.write(|w| {
            // Count up when the direction bit is zero
            w.dir().clear_bit();
            // Periodic
            w.oneshot().clear_bit()
        });

        // Select an appropriate clock source based on the chosen
        // frequency.
        Self::clock_enable(params.generator);

        // How many cycles of the clock need to happen to reach our
        // effective value.
        let count = params.effective_freq.0 / timeout.0;

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

impl TimerCounter<TC3, Width16> {
    pub fn new(clocks: Clocks, tc: TC3) -> Self {
        Self::power_on();

        // Disable and reset.  We need to do both separately
        // to avoid undefined behavior.
        // We need the unsafe block because of the union
        unsafe {
            tc.count16.ctrla.modify(|_, w| w.enable().clear_bit());
            while tc.count16.status.read().syncbusy().bit_is_set() {}

            // I wanted to perform a software reset here, but something
            // about this screws things up in a way that the JLink
            // cannot see into, so it is a bit of a mystery
            //tc.count16.ctrla.modify(|_, w| w.swrst().set_bit());
            //while tc.count16.status.read().syncbusy().bit_is_set() {}

            tc.count16.ctrla.write(|w| w.mode().count16());
        }

        Self {
            clocks,
            timeout: Hertz(0),
            tc,
            _count: PhantomData,
        }
    }

    fn power_on() {
        // this is safe because we're constrained to just the tc3 bit
        unsafe {
            (*PM::ptr()).apbcmask.modify(|_, w| w.tc3_().set_bit());
        }
    }

    fn power_off() {
        // this is safe because we're constrained to just the tc3 bit
        unsafe {
            (*PM::ptr()).apbcmask.modify(|_, w| w.tc3_().clear_bit());
        }
    }

    fn clock_enable(generator: GenericClockGenerator) {
        // this is safe because we're constrained to just tc3
        unsafe {
            (*GCLK::ptr()).clkctrl.write(|w| {
                w.id().tcc2_tc3();
                w.gen().variant(generator);
                w.clken().set_bit()
            });
        }
    }

    fn clock_disable() {
        // this is safe because we're constrained to just tc3
        unsafe {
            (*GCLK::ptr()).clkctrl.write(|w| {
                w.id().tcc2_tc3();
                w.gen().gclk0();
                w.clken().clear_bit()
            });
        }
    }

    fn mode(&mut self) -> &COUNT16 {
        unsafe { &self.tc.count16 }
    }
}
