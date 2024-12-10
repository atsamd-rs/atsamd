//! Delays

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::SYST;

use crate::clock::GenericClockController;
use crate::ehal::delay::DelayNs;
use crate::ehal_02;
use crate::time::Hertz;
use crate::typelevel::Increment;
use crate::clock::v2::Source;

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

    pub fn new_with_source<S>(mut syst: SYST, source: S) -> (Self, S::Inc)
    where S: Source + Increment {
        syst.set_clock_source(SystClkSource::Core);
        (
            Delay {
                syst,
                sysclock: source.freq(),
            },
            source.inc()
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

impl ehal_02::blocking::delay::DelayMs<u32> for Delay {
    fn delay_ms(&mut self, ms: u32) {
        <Self as DelayNs>::delay_us(self, ms * 1_000);
    }
}

impl ehal_02::blocking::delay::DelayMs<u16> for Delay {
    fn delay_ms(&mut self, ms: u16) {
        <Self as ehal_02::blocking::delay::DelayMs<u32>>::delay_ms(self, ms as u32);
    }
}

impl ehal_02::blocking::delay::DelayMs<u8> for Delay {
    fn delay_ms(&mut self, ms: u8) {
        <Self as ehal_02::blocking::delay::DelayMs<u32>>::delay_ms(self, ms as u32);
    }
}

impl ehal_02::blocking::delay::DelayUs<u32> for Delay {
    fn delay_us(&mut self, us: u32) {
        <Self as DelayNs>::delay_us(self, us);
    }
}

impl ehal_02::blocking::delay::DelayUs<u16> for Delay {
    fn delay_us(&mut self, us: u16) {
        <Self as ehal_02::blocking::delay::DelayUs<u32>>::delay_us(self, us as u32);
    }
}

impl ehal_02::blocking::delay::DelayUs<u8> for Delay {
    fn delay_us(&mut self, us: u8) {
        <Self as ehal_02::blocking::delay::DelayUs<u32>>::delay_us(self, us as u32);
    }
}
