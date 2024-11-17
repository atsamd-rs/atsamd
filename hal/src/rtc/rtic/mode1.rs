use super::MIN_COMPARE_TICKS;
use crate::pac;
use atsamd_hal_macros::hal_macro_helper;
use core::sync::atomic::Ordering;
use portable_atomic::AtomicU64;
use rtic_time::{
    half_period_counter::calculate_now,
    timer_queue::{TimerQueue, TimerQueueBackend},
};

struct TimerValueU16(u16);
impl rtic_time::half_period_counter::TimerValue for TimerValueU16 {
    const BITS: u32 = 16;
}
impl From<TimerValueU16> for u64 {
    fn from(value: TimerValueU16) -> Self {
        Self::from(value.0)
    }
}

/// RTC mode 1 based [`TimerQueueBackend`].
pub struct RtcBackend;

static RTC_PERIOD_COUNT: AtomicU64 = AtomicU64::new(0);
static RTC_TQ: TimerQueue<RtcBackend> = TimerQueue::new();

#[hal_macro_helper]
impl RtcBackend {
    #[inline]
    fn sync() {
        let rtc = unsafe { &pac::Rtc::steal() };

        #[hal_cfg("rtc-d5x")]
        while rtc.mode1().syncbusy().read().bits() != 0 {}
        #[hal_cfg(any("rtc-d11", "rtc-d21"))]
        while rtc.mode1().status().read().syncbusy().bit_is_set() {}
    }

    pub fn raw_count() -> u16 {
        let rtc = unsafe { &pac::Rtc::steal() };

        #[hal_cfg(any("rtc-d11", "rtc-d21"))]
        {
            rtc.mode1().readreq().modify(|_, w| w.rcont().set_bit());
            Self::sync();
        }
        // NOTE: Sync is automatic on SAMD5x chips.

        rtc.mode1().count().read().bits()
    }

    /// Starts the clock.
    ///
    /// **Do not use this function directly.**
    ///
    /// Use the [`rtc_mode1_monotonic`](crate::rtc_mode1_monotonic) macro
    /// instead and then call `start` on the monotonic.
    pub fn _start<S: super::rtc_clock::RtcClockSetter>(
        rtc: pac::Rtc,
        mclk: &mut pac::Mclk,
        osc32kctrl: &mut pac::Osc32kctrl,
    ) {
        // Enables the APBA clock for the RTC.
        mclk.apbamask().modify(|_, w| w.rtc_().set_bit());

        // It is necessary to disable the RTC before resetting it.
        // NOTE: This register and field are the same in all modes.
        rtc.mode0().ctrla().modify(|_, w| w.enable().clear_bit());
        Self::sync();

        // Reset RTC back to initial settings, which disables it and enters mode 0.
        rtc.mode0().ctrla().modify(|_, w| w.swrst().set_bit());
        // Wait for the reset to complete
        while rtc.mode0().ctrla().read().swrst().bit_is_set() {}

        // Set the RTC clock source.
        osc32kctrl.rtcctrl().write(S::set_source);

        // Set mode 1 (16 bit counter)
        rtc.mode0().ctrla().modify(|_, w| w.mode().count16());

        // Set the mode 1 period
        unsafe { rtc.mode1().per().write(|w| w.bits(0xFFFF)) };

        // Configure the compare registers
        unsafe {
            rtc.mode1().comp(0).write(|w| w.bits(0)); // Dynamic wakeup
            rtc.mode1().comp(1).write(|w| w.bits(0x8000)); // Half-period
        }
        Self::sync();

        // Timing critical, make sure we don't get interrupted.
        critical_section::with(|_| {
            // Start the RTC counter.
            rtc.mode1().ctrla().modify(|_, w| w.enable().set_bit());
            Self::sync();

            // Enable counter sync on SAMx5x, the counter cannot be read otherwise.
            #[hal_cfg("rtc-d5x")]
            {
                rtc.mode1().ctrla().modify(|_, w| w.countsync().set_bit());
                Self::sync();

                // Errata: The first read of the count is incorrect so we need to read it
                // then wait for it to change.
                let count = Self::raw_count();
                while Self::raw_count() == count {}
            }

            // Make sure period counter is synced with the timer value
            RTC_PERIOD_COUNT.store(0, Ordering::SeqCst);

            // Initialize the timer queue
            RTC_TQ.initialize(Self {});

            // Clear the triggered compare flag
            Self::clear_compare_flag();

            // Enable the compare and overflow interrupts.
            rtc.mode1()
                .intenset()
                .write(|w| w.ovf().set_bit().cmp0().set_bit().cmp1().set_bit());

            // Enable the RTC interrupt in the NVIC and set its priority.
            // SAFETY: We take full ownership of the peripheral and interrupt vector,
            // plus we are not using any external shared resources so we won't impact
            // basepri/source masking based critical sections.
            unsafe {
                super::set_monotonic_prio(pac::NVIC_PRIO_BITS, pac::Interrupt::RTC);
                pac::NVIC::unmask(pac::Interrupt::RTC);
            }
        });
    }
}

impl TimerQueueBackend for RtcBackend {
    type Ticks = u64;

    #[hal_macro_helper]
    fn now() -> Self::Ticks {
        calculate_now(
            || RTC_PERIOD_COUNT.load(Ordering::Relaxed),
            || TimerValueU16(Self::raw_count()),
        )
    }

    fn enable_timer() {
        let rtc = unsafe { pac::Rtc::steal() };
        rtc.mode1().ctrla().modify(|_, w| w.enable().set_bit());
        Self::sync();
    }

    fn disable_timer() {
        let rtc = unsafe { pac::Rtc::steal() };
        rtc.mode1().ctrla().modify(|_, w| w.enable().clear_bit());
        Self::sync();
    }

    fn on_interrupt() {
        let rtc: pac::Rtc = unsafe { pac::Rtc::steal() };
        let intflag = rtc.mode1().intflag().read();

        if intflag.ovf().bit_is_set() {
            // This was an overflow interrupt
            rtc.mode1().intflag().modify(|_, w| w.ovf().set_bit());
            let prev = RTC_PERIOD_COUNT.fetch_add(1, Ordering::Relaxed);
            assert!(prev % 2 == 1, "Monotonic must have skipped an interrupt!");
        }
        if intflag.cmp1().bit_is_set() {
            // This was half-period interrupt
            rtc.mode1().intflag().modify(|_, w| w.cmp1().set_bit());
            let prev = RTC_PERIOD_COUNT.fetch_add(1, Ordering::Relaxed);
            assert!(prev % 2 == 0, "Monotonic must have skipped an interrupt!");
        }

        // For some strange reason that was unable to be determined, often the
        // interrupt will trigger, but the count will be less the value that is
        // supposed to trigger the interrupt, even after syncing. This causes
        // now() to return an incorrect time, which causes the TimerQueue lots
        // of problems.
        //
        // Testing showed that usually the count is only one less than the
        // expected value. We correct for this here by waiting until the counter
        // reaches the compare value.
        let expected_compare = if intflag.ovf().bit_is_set() {
            0
        } else if intflag.cmp1().bit_is_set() {
            rtc.mode1().comp(1).read().bits()
        } else {
            // The cmp0 flag has already been cleared by this point, but if it
            // is neither of the others, than the interrupt must have been triggered
            // by a cmp0
            rtc.mode1().comp(0).read().bits()
        };

        loop {
            let counter = Self::raw_count();

            if expected_compare < 0x1000 {
                // Wait for the counter to roll over
                if counter < 0x8000 && counter >= expected_compare {
                    break;
                }
            } else {
                // Wait for the counter to catch up to the compare value
                if counter >= expected_compare {
                    break;
                }
            }
        }
    }

    fn set_compare(mut instant: Self::Ticks) {
        let rtc = unsafe { pac::Rtc::steal() };

        const MAX: u64 = 0xFFFF;

        // Disable interrupts because this section is timing critical.
        // We rely on the fact that this entire section runs within one
        // RTC clock tick. (which it will do easily if it doesn't get
        // interrupted)
        critical_section::with(|_| {
            let now = Self::now();
            // wrapping_sub deals with the u64 overflow corner case
            let diff = instant.wrapping_sub(now);
            let val = if diff <= MAX {
                // Now we know `instant` will happen within one `MAX` time duration.

                // Evidently the compare interrupt will not trigger if the instant is within
                // a couple of ticks, so delay it a bit if it is too
                // close. This is not mentioned in the documentation
                // or errata, but is known to be an issue for other
                // microcontrollers as well (e.g. nRF family).
                if diff < MIN_COMPARE_TICKS.into() {
                    instant = instant.wrapping_add(MIN_COMPARE_TICKS.into())
                }

                (instant & MAX) as u16
            } else {
                0
            };

            unsafe { rtc.mode1().comp(0).write(|w| w.comp().bits(val)) };
            Self::sync();
        });
    }

    fn clear_compare_flag() {
        let rtc = unsafe { pac::Rtc::steal() };
        rtc.mode1().intflag().write(|w| w.cmp0().set_bit());
        // NOTE: Should not need to sync here
    }

    fn pend_interrupt() {
        pac::NVIC::pend(pac::Interrupt::RTC);
    }

    fn timer_queue() -> &'static TimerQueue<Self> {
        &RTC_TQ
    }
}
