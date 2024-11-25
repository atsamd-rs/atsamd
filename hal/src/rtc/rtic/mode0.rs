use super::{LoopChecker, MIN_COMPARE_TICKS};
use crate::pac;
use atsamd_hal_macros::hal_macro_helper;
use rtic_time::timer_queue::{TimerQueue, TimerQueueBackend};

/// RTC mode 0 based [`TimerQueueBackend`].
pub struct RtcBackend;

static RTC_TQ: TimerQueue<RtcBackend> = TimerQueue::new();

// NOTE: TODO: Move this elsewhere so it is not duplicated?
// As explained in the datasheet, reading a read-synced register may result in
// an old value, which we try to avoid by ensuring that SYNCBUSY is clear before
// reading. A write to a write-synced register will be discarded if syncing is
// happening during the write. As such we also ensure that SYNCBUSY is clear
// before writing to a synced register. Every register access should be prefaced
// by a SYNC comment indicating the required synchronization, which indicates
// that this access was checked and accounted for.
#[hal_macro_helper]
impl RtcBackend {
    /// RTC interrupt handler called before control passes to RTIC.
    #[inline]
    pub unsafe fn interrupt_handler() {
        let rtc = pac::Rtc::steal();

        /// Returns whether a < b, taking wrapping into account and assuming
        /// that the difference is less than 0x80000000.
        #[inline]
        fn less_than_with_wrap(a: u32, b: u32) -> bool {
            let d = a.wrapping_sub(b);

            d >= 0x80000000
        }

        // Ensure that the COUNT is at least the compare value
        // Due to syncing delay this may not be the case initially
        // Note that this has to be done here because RTIC will clear the cmp0 flag
        // before `RtcBackend::on_interrupt` is called.
        if rtc.mode0().intflag().read().cmp0().bit_is_set() {
            let compare = rtc.mode0().comp(0).read().bits();
            while less_than_with_wrap(Self::now(), compare) {}
        }

        Self::timer_queue().on_monotonic_interrupt();
    }

    // TODO: Test code
    fn check_loop(checker: &mut LoopChecker, item: &str) {
        let rtc = unsafe { &pac::Rtc::steal() };

        if checker.too_many() {
            Self::sync();
            panic!(
                "Took too long in '{item}' with count 0x{:X} and cmp 0x{:X}. Int flags: 0x{:X}, Enabled: {:?}",
                rtc.mode0().count().read().bits(),
                rtc.mode0().comp(0).read().bits(),
                rtc.mode0().intflag().read().bits(),
                rtc.mode0().ctrla().read().enable().bit_is_set(),
            );
        }
    }

    /// Waits until the SYNCBUSY register is clear.
    #[inline]
    fn sync() {
        let rtc = unsafe { &pac::Rtc::steal() };

        let mut loop_checker = LoopChecker::default();

        // SYNC: None
        #[hal_cfg("rtc-d5x")]
        while rtc.mode0().syncbusy().read().bits() != 0 {
            Self::check_loop(&mut loop_checker, "sync");
        }
        // SYNC: None
        #[hal_cfg(any("rtc-d11", "rtc-d21"))]
        while rtc.mode0().status().read().syncbusy().bit_is_set() {
            Self::check_loop(&mut loop_checker, "sync");
        }
    }

    // TODO: Test?
    #[inline]
    fn wait_for_count_change() -> u32 {
        let rtc = unsafe { &pac::Rtc::steal() };

        let mut last_count = Self::now();

        // TODO: It sounds like we need to ensure everywhere that
        // a) Sync read registered are ensured not to be syncing before reading
        // b) Sync write registers are also ensured not to be syncing before writing
        // since this will discard the write.

        // TODO: Also the count by 4 of the COUNT may be due to synchronization delay:
        // https://onlinedocs.microchip.com/oxy/GUID-F5813793-E016-46F5-A9E2-718D8BCED496-en-US-13/GUID-0C52DB00-4BF6-4F41-85B5-B76529875364.html#GUID-0C52DB00-4BF6-4F41-85B5-B76529875364
        // Though according to calculations this should only be between 2 and 3 RTC
        // clock periods, not 4? But could take a little over 3, hence 4. It says 2 is
        // typical though.

        // TODO: If the clock is disabled then just continue.
        // This can happen if a new task pends the interrupt while the queue is empty so
        // that the timer is disabled, which can otherwise result in waiting
        // forever.
        // SYNC: Write (we just read though)
        if !rtc.mode0().ctrla().read().enable().bit_is_set() {
            return last_count;
        }

        let mut loop_checker = LoopChecker::default();
        loop {
            let count = Self::now();

            if count != last_count {
                if last_count > 0 && count != last_count.wrapping_add(4) {
                    // TODO: Test code
                    // SYNC: Write (we just read though)
                    let compare = rtc.mode0().comp(0).read().bits();

                    panic!("Clock anomaly at 0x{count:X} with previous step at 0x{last_count:X} with cmp 0x{compare:X}");
                }

                break count;
            }

            Self::check_loop(&mut loop_checker, "wait_for_count_change");

            last_count = count;
        }
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
        // Disable the RTC.
        // NOTE: This register and field are the same in all modes.
        // SYNC: Write
        Self::sync();
        rtc.mode0().ctrla().modify(|_, w| w.enable().clear_bit());

        // Set the RTC clock source.
        osc32kctrl.rtcctrl().write(S::set_source);

        // Enable the APBA clock for the RTC.
        mclk.apbamask().modify(|_, w| w.rtc_().set_bit());

        // Reset RTC back to initial settings, which disables it and enters mode 0.
        // NOTE: This register and field are the same in all modes.
        // SYNC: Write
        Self::sync();
        rtc.mode0().ctrla().modify(|_, w| w.swrst().set_bit());

        // Wait for the reset to complete
        let mut loop_checker = LoopChecker::default();
        // SYNC: Write (we just read though)
        while rtc.mode0().ctrla().read().swrst().bit_is_set() {
            Self::check_loop(&mut loop_checker, "reset");
        }

        // TODO: Test code
        // Set the prescalar
        //rtc.mode0().ctrla().modify(|_, w| w.prescaler().div4());

        // Set the the initial compare
        // SYNC: Write
        Self::sync();
        unsafe {
            rtc.mode0().comp(0).write(|w| w.comp().bits(0));
        }

        // Timing critical, make sure we don't get interrupted.
        critical_section::with(|_| {
            // Start the RTC counter.
            // SYNC: Write
            Self::sync();
            rtc.mode0().ctrla().modify(|_, w| w.enable().set_bit());

            // Enable counter sync on SAMx5x, the counter cannot be read otherwise.
            #[hal_cfg("rtc-d5x")]
            {
                // Enable counter synchronization
                // SYNC: Write
                Self::sync();
                rtc.mode0().ctrla().modify(|_, w| w.countsync().set_bit());

                // Errata: The first read of the count is incorrect so we need to read it
                // then wait for it to change.
                Self::wait_for_count_change();
            }

            // Clear the triggered compare flag
            Self::clear_compare_flag();

            // Enable the compare interrupt
            // SYNC: None
            rtc.mode0().intenset().write(|w| w.cmp0().set_bit());

            // Initialize the timer queue
            RTC_TQ.initialize(Self {});

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
        let rtc = unsafe { &pac::Rtc::steal() };

        #[hal_cfg(any("rtc-d11", "rtc-d21"))]
        {
            // SYNC: TODO: Need to check for these other chips
            rtc.mode0().readreq().modify(|_, w| w.rcont().set_bit());
        }

        // SYNC: Read/Write
        Self::sync();
        rtc.mode0().count().read().bits()
    }

    fn enable_timer() {
        let rtc = unsafe { pac::Rtc::steal() };
        // SYNC: Write
        Self::sync();
        rtc.mode0().ctrla().modify(|_, w| w.enable().set_bit());
    }

    fn disable_timer() {
        let rtc = unsafe { pac::Rtc::steal() };
        // SYNC: Write
        Self::sync();
        rtc.mode0().ctrla().modify(|_, w| w.enable().clear_bit());
    }

    fn on_interrupt() {
        // There is nothing we need to do here
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

        // Set the compare register and wait for sync
        // SYNC: Write
        Self::sync();
        unsafe { rtc.mode0().comp(0).write(|w| w.comp().bits(instant)) };
    }

    fn clear_compare_flag() {
        let rtc = unsafe { pac::Rtc::steal() };

        // SYNC: None
        rtc.mode0().intflag().modify(|_, w| w.cmp0().set_bit());
    }

    fn pend_interrupt() {
        pac::NVIC::pend(pac::Interrupt::RTC);
    }

    fn timer_queue() -> &'static TimerQueue<Self> {
        &RTC_TQ
    }
}
