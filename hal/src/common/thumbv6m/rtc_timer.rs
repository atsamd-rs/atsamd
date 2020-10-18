//! RTC based 32 bit timer
use crate::clock;
use crate::target_device::{rtc::MODE0, PM};
use crate::time::{Hertz, Nanoseconds};
use crate::timer_traits::InterruptDrivenTimer;

use hal::timer::{CountDown, Periodic};
use void::Void;

/// Implements the 32 bit timer mode for the Real Time Counter for
/// SAMD11 and SAMD21.
pub struct RealTimeCounterTimer<'a> {
    freq: Hertz,
    rtc: &'a MODE0,
}

impl<'a> RealTimeCounterTimer<'a> {
    /// Creates a mode-0 RTC timer with the given clock and RTC peripheral
    pub fn new(
        rtc_source_clock: &clock::GClock,
        pm: &mut PM,
        clocks: &mut clock::GenericClockController,
        rtc: &'a MODE0,
    ) -> Self {
        // enable the RTC block in the power manager. Safe because we're only touching
        // the RTC bit
        pm.apbamask.modify(|_, w| w.rtc_().set_bit());
        let rtc_clock = clocks.rtc(rtc_source_clock).unwrap();

        let new_rtc = Self {
            freq: rtc_clock.freq(),
            rtc,
        };

        // disable and return
        new_rtc.enable(false);
        new_rtc
    }

    #[inline]
    fn reset(&self) {
        self.rtc.ctrl.modify(|_, w| w.swrst().set_bit());
        self.sync();
    }

    #[inline]
    fn sync(&self) {
        while self.rtc.status.read().syncbusy().bit_is_set() {}
    }

    #[inline]
    fn enable(&self, enable: bool) {
        if enable {
            self.rtc.ctrl.modify(|_, w| w.enable().set_bit());
        } else {
            self.rtc.ctrl.modify(|_, w| w.enable().clear_bit());
        }
        self.sync();
    }
}

impl Periodic for RealTimeCounterTimer<'_> {}
impl CountDown for RealTimeCounterTimer<'_> {
    type Time = Nanoseconds;

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Self::Time>,
    {
        let params = TimerParams::new_us(timeout, self.freq.0);
        let divider = params.divider;
        let cycles = params.cycles;

        // Disable the timer while we reconfigure it
        self.enable(false);

        // 32 bit counter mode plz
        self.rtc.ctrl.modify(|_, w| w.mode().count32());

        // Now that we have a clock routed to the peripheral, we
        // can ask it to perform a reset.
        self.reset();

        // the SVD erroneously marks swrst as write-only, so we
        // need to manually read the bit here
        while self.rtc.ctrl.read().bits() & 1 != 0 {}

        // set cycles to compare to...
        self.rtc.comp[0].write(|w| unsafe { w.comp().bits(cycles) });
        self.rtc.ctrl.modify(|_, w| {
            // set clock divider...
            match divider {
                1 => w.prescaler().div1(),
                2 => w.prescaler().div2(),
                4 => w.prescaler().div4(),
                8 => w.prescaler().div8(),
                16 => w.prescaler().div16(),
                32 => w.prescaler().div32(),
                64 => w.prescaler().div64(),
                128 => w.prescaler().div128(),
                256 => w.prescaler().div256(),
                512 => w.prescaler().div512(),
                1024 => w.prescaler().div1024(),
                _ => unreachable!(),
            };
            // clear timer on match...
            w.matchclr().set_bit();
            // and enable RTC.
            w.enable().set_bit()
        });
    }

    fn wait(&mut self) -> nb::Result<(), Void> {
        if self.rtc.intflag.read().cmp0().bit_is_set() {
            // Writing a 1 clears the flag
            self.rtc.intflag.modify(|_, w| w.cmp0().set_bit());
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl InterruptDrivenTimer for RealTimeCounterTimer<'_> {
    /// Enable the interrupt generation for this hardware timer.
    /// This method only sets the clock configuration to trigger
    /// the interrupt; it does not configure the interrupt controller
    /// or define an interrupt handler.
    fn enable_interrupt(&mut self) {
        self.rtc.intenset.write(|w| w.cmp0().set_bit());
    }

    /// Disables interrupt generation for this hardware timer.
    /// This method only sets the clock configuration to prevent
    /// triggering the interrupt; it does not configure the interrupt
    /// controller.
    fn disable_interrupt(&mut self) {
        self.rtc.intenclr.write(|w| w.cmp0().set_bit());
    }
}

/// Helper type for computing cycles and divider given frequency
#[derive(Debug, Clone, Copy)]
pub struct TimerParams {
    pub divider: u16,
    pub cycles: u32,
}

impl TimerParams {
    /// calculates RTC timer paramters based on the input frequency-based
    /// timeout.
    pub fn new<T>(timeout: T, src_freq: u32) -> Self
    where
        T: Into<Hertz>,
    {
        let timeout = timeout.into();
        let ticks: u32 = src_freq / timeout.0.max(1);
        TimerParams::new_from_ticks(ticks)
    }

    /// calculates RTC timer paramters based on the input period-based timeout.
    pub fn new_us<T>(timeout: T, src_freq: u32) -> Self
    where
        T: Into<Nanoseconds>,
    {
        let timeout = timeout.into();
        let ticks: u32 = (timeout.0 as u64 * src_freq as u64 / 1_000_000_000_u64) as u32;
        Self::new_from_ticks(ticks)
    }

    /// Common helper function that gets the best divider & calculates cycles
    /// with that divider.
    fn new_from_ticks(ticks: u32) -> Self {
        let mut divider = ((ticks >> 16) + 1).next_power_of_two();
        if divider > 1024 {
            // lame. Can we catch this at compile time? Probably??
            divider = 1024;
        }

        let cycles: u32 = ticks / divider as u32;

        TimerParams {
            divider: divider as u16,
            cycles,
        }
    }
}
