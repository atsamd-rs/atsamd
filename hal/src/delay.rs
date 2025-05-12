//! Delays

use atsamd_hal_macros::hal_cfg;
use cortex_m::peripheral::SYST;
use cortex_m::peripheral::syst::SystClkSource;

use crate::clock::GenericClockController;
use crate::ehal::delay::DelayNs;
use crate::time::Hertz;

#[hal_cfg("rtc-d5x")]
use crate::typelevel::Increment;

#[hal_cfg("rtc-d5x")]
use crate::clock::v2::{Source, gclk::Gclk0Id};

/// System timer (SysTick) as a delay provider
pub struct Delay {
    sysclock: Hertz,
    syst: SYST,
}

impl Delay {
    /// Configures the system timer (SysTick) as a delay provider
    pub fn new(mut syst: SYST, clocks: &mut GenericClockController) -> Self {
        syst.set_clock_source(SystClkSource::Core);

        Delay {
            syst,
            sysclock: clocks.gclk0().into(),
        }
    }

    #[hal_cfg("rtc-d5x")]
    /// Configures the system timer (SysTick) as a delay provide, compatible
    /// with the V2 clocking API
    pub fn new_with_source<S>(mut syst: SYST, gclk0: S) -> (Self, S::Inc)
    where
        S: Source<Id = Gclk0Id> + Increment,
    {
        syst.set_clock_source(SystClkSource::Core);
        (
            Delay {
                syst,
                sysclock: gclk0.freq(),
            },
            gclk0.inc(),
        )
    }

    /// Releases the system timer (SysTick) resource
    pub fn free(self) -> SYST {
        self.syst
    }
}

impl DelayNs for Delay {
    // The default method is delay_ns. If we don't provide implementations for
    // delay_us and delay_ms, the trait impl will use delay_ns to implement the
    // other two methods. As the delay implementation is actually defined in terms
    // of microseconds, we need to provide implementations for all three methods.
    fn delay_ns(&mut self, ns: u32) {
        self.delay_us(ns / 1000);
    }

    fn delay_us(&mut self, us: u32) {
        // The SysTick Reload Value register supports values between 1 and 0x00FFFFFF.
        const MAX_RVR: u32 = 0x00FF_FFFF;

        let mut total_rvr = us * (self.sysclock.to_Hz() / 1_000_000);

        while total_rvr != 0 {
            let current_rvr = if total_rvr <= MAX_RVR {
                total_rvr
            } else {
                MAX_RVR
            };

            self.syst.set_reload(current_rvr);
            self.syst.clear_current();
            self.syst.enable_counter();

            // Update the tracking variable while we are waiting...
            total_rvr -= current_rvr;

            while !self.syst.has_wrapped() {}

            self.syst.disable_counter();
        }
    }

    fn delay_ms(&mut self, ms: u32) {
        self.delay_us(ms * 1000);
    }
}
