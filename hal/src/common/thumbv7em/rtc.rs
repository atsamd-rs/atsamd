//! Real-time clock
// See 21. Real-Time Counter
use crate::target_device::{MCLK, RTC};

/// Datetime represents an RTC clock/calendar value.
#[derive(Debug,Clone,Copy)]
pub struct Datetime {
    pub seconds: u8,
    pub minutes: u8,
    pub hours: u8,
    pub day: u8,
    pub month: u8,
    pub year: u8,
}

type ClockR = crate::target_device::rtc::mode2::clock::R;

impl From<ClockR> for Datetime {
    fn from(clock: ClockR) -> Datetime {
        Datetime{
            seconds: clock.second().bits(),
            minutes: clock.minute().bits(),
            hours: clock.hour().bits(),
            day: clock.day().bits(),
            month: clock.month().bits(),
            year: clock.year().bits(),
         }
    }
}

/// RtcClock represents the RTC configured in clock/calendar mode.
pub struct RtcClock {
    rtc: RTC,
}

impl RtcClock {
    /// Initializes the provided RTC in clock/calendar mode.
    pub fn new(rtc: RTC, mclk: &mut MCLK) -> Self {
        mclk.apbamask.modify(|_, w| w.rtc_().set_bit());

        rtc.mode2().ctrla.modify(|_, w| w.swrst().set_bit());
        while rtc.mode2().syncbusy.read().swrst().bit_is_set() {
            cortex_m::asm::nop();
        }

        rtc.mode2().ctrla.modify(|_, w| {
            w.mode().clock() // enable mode2 (clock)
            .clkrep().clear_bit()
            .matchclr().clear_bit()
            .prescaler().div1024() // 1.024 kHz / 1024 = 1Hz
            .enable().set_bit()
            .clocksync().set_bit() // synchronize the CLOCK register
        });
        while rtc.mode2().syncbusy.read().enable().bit_is_set() {
            cortex_m::asm::nop();
        }

        Self { rtc }
    }

    /// Returns the current clock/calendar value.
    pub fn current_time(&self) -> Datetime {
        self.rtc.mode2().clock.read().into()
    }

    /// Updates the current clock/calendar value.
    pub fn set_time(&mut self, time: Datetime) {
        self.rtc.mode2().clock.write(|w| unsafe {
            w.second().bits(time.seconds)
             .minute().bits(time.minutes)
             .hour().bits(time.hours)
             .day().bits(time.day)
             .month().bits(time.month)
             .year().bits(time.year)
        });

        while self.rtc.mode2().syncbusy.read().clock().bit_is_set() {
            cortex_m::asm::nop();
        }
    }
}
