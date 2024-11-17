use super::MIN_COMPARE_TICKS;
use crate::pac;
use atsamd_hal_macros::hal_macro_helper;
use rtic_time::timer_queue::{TimerQueue, TimerQueueBackend};

/// RTC mode 0 based [`TimerQueueBackend`].
pub struct RtcBackend;

static RTC_TQ: TimerQueue<RtcBackend> = TimerQueue::new();

#[hal_macro_helper]
impl RtcBackend {
    #[inline]
    fn sync() {
        let rtc = unsafe { &pac::Rtc::steal() };

        #[hal_cfg("rtc-d5x")]
        while rtc.mode0().syncbusy().read().bits() != 0 {}
        #[hal_cfg(any("rtc-d11", "rtc-d21"))]
        while rtc.mode0().status().read().syncbusy().bit_is_set() {}
    }

    pub fn raw_count() -> u32 {
        let rtc = unsafe { &pac::Rtc::steal() };

        #[hal_cfg(any("rtc-d11", "rtc-d21"))]
        {
            rtc.mode0().readreq().modify(|_, w| w.rcont().set_bit());
            Self::sync();
        }
        // NOTE: Sync is automatic on SAMD5x chips.
        rtc.mode0().count().read().bits()
    }

    /// Starts the clock.
    ///
    /// **Do not use this function directly.**
    ///
    /// Use the [`rtc_mode0_monotonic`](crate::rtc_mode0_monotonic) macro
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

        // Initialize the timer queue
        RTC_TQ.initialize(Self {});

        // Reset RTC back to initial settings, which disables it and enters mode 0.
        rtc.mode0().ctrla().modify(|_, w| w.swrst().set_bit());
        // Wait for the reset to complete
        while rtc.mode0().ctrla().read().swrst().bit_is_set() {}

        // Set the RTC clock source.
        osc32kctrl.rtcctrl().write(S::set_source);

        // Set the the initial compare
        unsafe {
            rtc.mode0().comp(0).write(|w| w.comp().bits(0));
        }
        Self::sync();

        // Timing critical, make sure we don't get interrupted.
        critical_section::with(|_| {
            // Start the RTC counter.
            rtc.mode0().ctrla().modify(|_, w| w.enable().set_bit());
            Self::sync();

            // Enable counter sync on SAMx5x, the counter cannot be read otherwise.
            #[hal_cfg("rtc-d5x")]
            {
                rtc.mode0().ctrla().modify(|_, w| w.countsync().set_bit());

                // Errata: The first read of the count is incorrect so we need to read it
                // then wait for it to change.
                let count = Self::now();
                while Self::now() == count {}

                // Clear the triggered compare flag
                Self::clear_compare_flag();

                // Clear the compare flag and enable the interrupt
                rtc.mode0().intenset().write(|w| w.cmp0().set_bit());
            }

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
    type Ticks = u32;

    #[hal_macro_helper]
    fn now() -> Self::Ticks {
        Self::raw_count()
    }

    fn enable_timer() {
        let rtc = unsafe { pac::Rtc::steal() };
        rtc.mode0().ctrla().modify(|_, w| w.enable().set_bit());
        Self::sync();
    }

    fn disable_timer() {
        let rtc = unsafe { pac::Rtc::steal() };
        rtc.mode0().ctrla().modify(|_, w| w.enable().clear_bit());
        Self::sync();
    }

    fn on_interrupt() {
        let rtc = unsafe { pac::Rtc::steal() };

        // For some strange reason that was unable to be determined, often the compare
        // interrupt will trigger, but the count will be less than the compare value,
        // even after syncing, which causes the TimerQueue to not register that the
        // timeout is up. There is nothing about this in the chip errata or
        // documentation.
        //
        // Testing showed that usually the count is only one less than
        // the compare. We correct for this here by waiting until the counter reaches
        // the compare value.
        let compare = rtc.mode0().comp(0).read().bits();
        loop {
            let counter = Self::raw_count();

            if counter >= compare {
                break;
            }
        }
    }

    fn set_compare(mut instant: Self::Ticks) {
        let rtc = unsafe { pac::Rtc::steal() };

        // Evidently the compare interrupt will not trigger if the instant is within a
        // couple of ticks, so delay it a bit if it is too close.
        // This is not mentioned in the documentation or errata, but is known to be an
        // issue for other microcontrollers as well (e.g. nRF family).
        if instant.saturating_sub(Self::now()) < MIN_COMPARE_TICKS {
            instant = instant.wrapping_add(MIN_COMPARE_TICKS)
        }

        unsafe { rtc.mode0().comp(0).write(|w| w.comp().bits(instant as u32)) };
        Self::sync();
    }

    fn clear_compare_flag() {
        let rtc = unsafe { pac::Rtc::steal() };
        rtc.mode0().intflag().modify(|_, w| w.cmp0().set_bit());
        // NOTE: Should not need to sync here
    }

    fn pend_interrupt() {
        pac::NVIC::pend(pac::Interrupt::RTC);
    }

    fn timer_queue() -> &'static TimerQueue<Self> {
        &RTC_TQ
    }
}
