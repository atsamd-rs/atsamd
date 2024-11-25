use super::{LoopChecker, MIN_COMPARE_TICKS};
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
        /// that the difference is less than 0x8000.
        #[inline]
        fn less_than_with_wrap(a: u16, b: u16) -> bool {
            let d = a.wrapping_sub(b);

            d >= 0x8000
        }

        // Ensure that the COUNT is at least the compare value
        // Due to syncing delay this may not be the case initially
        // Note that this has to be done here because RTIC will clear the cmp0 flag
        // before `RtcBackend::on_interrupt` is called.
        if rtc.mode1().intflag().read().cmp0().bit_is_set() {
            let compare = rtc.mode1().comp(0).read().bits();
            while less_than_with_wrap(Self::raw_count(), compare) {}
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
                rtc.mode1().count().read().bits(),
                rtc.mode1().comp(0).read().bits(),
                rtc.mode1().intflag().read().bits(),
                rtc.mode1().ctrla().read().enable().bit_is_set(),
            );
        }
    }

    #[inline]
    fn sync() {
        let rtc = unsafe { &pac::Rtc::steal() };

        let mut loop_checker = LoopChecker::default();

        // SYNC: None
        #[hal_cfg("rtc-d5x")]
        while rtc.mode1().syncbusy().read().bits() != 0 {
            Self::check_loop(&mut loop_checker, "sync");
        }
        // SYNC: None
        #[hal_cfg(any("rtc-d11", "rtc-d21"))]
        while rtc.mode1().status().read().syncbusy().bit_is_set() {
            Self::check_loop(&mut loop_checker, "sync");
        }
    }

    // TODO: This will ultimately not need to be public
    pub fn raw_count() -> u16 {
        let rtc = unsafe { &pac::Rtc::steal() };

        #[hal_cfg(any("rtc-d11", "rtc-d21"))]
        {
            // SYNC: TODO: Need to check for these other chips
            rtc.mode1().readreq().modify(|_, w| w.rcont().set_bit());
            Self::sync();
        }

        // SYNC: Read/Write
        Self::sync();
        rtc.mode1().count().read().bits()
    }

    // TODO: Test?
    fn wait_for_count_change() -> u16 {
        let rtc = unsafe { &pac::Rtc::steal() };

        let mut last_count = Self::raw_count();

        // TODO: If the clock is disabled then just continue.
        // This can happen if a new task pends the interrupt while the queue is empty so
        // that the timer is disabled, which can otherwise result in waiting
        // forever.
        // SYNC: Write (we just read though)
        if !rtc.mode1().ctrla().read().enable().bit_is_set() {
            return last_count;
        }

        let mut loop_checker = LoopChecker::default();
        loop {
            let count = Self::raw_count();

            if count != last_count {
                if last_count > 0 && count != last_count.wrapping_add(4) {
                    // TODO: Test code
                    // SYNC: Write (we just read though)
                    let compare = rtc.mode1().comp(0).read().bits();

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
    /// Use the [`rtc_mode1_monotonic`](crate::rtc_mode1_monotonic) macro
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

        // Set mode 1 (16 bit counter)
        // SYNC: Write
        Self::sync();
        rtc.mode0().ctrla().modify(|_, w| w.mode().count16());

        // TODO: Test code
        // Set the prescalar
        //rtc.mode1().ctrla().modify(|_, w| w.prescaler().div4());

        // Set the mode 1 period
        // SYNC: Write
        Self::sync();
        unsafe { rtc.mode1().per().write(|w| w.bits(0xFFFF)) };

        // Configure the compare registers
        unsafe {
            // RTIC tasks waker
            // SYNC: Write
            Self::sync();
            rtc.mode1().comp(0).write(|w| w.bits(0));
            // Half period
            // SYNC: Write
            Self::sync();
            rtc.mode1().comp(1).write(|w| w.bits(0x8000));
        }

        // Timing critical, make sure we don't get interrupted.
        critical_section::with(|_| {
            // Start the RTC counter.
            // SYNC: Write
            Self::sync();
            rtc.mode1().ctrla().modify(|_, w| w.enable().set_bit());

            // Enable counter sync on SAMx5x, the counter cannot be read otherwise.
            #[hal_cfg("rtc-d5x")]
            {
                // Enable counter synchronization
                // SYNC: Write
                Self::sync();
                rtc.mode1().ctrla().modify(|_, w| w.countsync().set_bit());

                // Errata: The first read of the count is incorrect so we need to read it
                // then wait for it to change.
                Self::wait_for_count_change();
            }

            // Clear the triggered compare flag
            Self::clear_compare_flag();

            // Make sure period counter is synced with the timer value
            RTC_PERIOD_COUNT.store(0, Ordering::SeqCst);

            // Initialize the timer queue
            RTC_TQ.initialize(Self {});

            // Enable the compare and overflow interrupts.
            // SYNC: None
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
        // SYNC: Write
        Self::sync();
        rtc.mode1().ctrla().modify(|_, w| w.enable().set_bit());
    }

    fn disable_timer() {
        let rtc = unsafe { pac::Rtc::steal() };
        // SYNC: Write
        Self::sync();
        rtc.mode1().ctrla().modify(|_, w| w.enable().clear_bit());
    }

    fn on_interrupt() {
        let rtc: pac::Rtc = unsafe { pac::Rtc::steal() };

        // Capture the interrupt flags
        // SYNC: None
        let intflag = rtc.mode1().intflag().read();

        // NOTE: The cmp0 flag is cleared when RTIC calls `clear_compare_flag`.
        if intflag.cmp1().bit_is_set() {
            // This was half-period interrupt
            // SYNC: None
            rtc.mode1().intflag().modify(|_, w| w.cmp1().set_bit());
            let prev = RTC_PERIOD_COUNT.fetch_add(1, Ordering::Relaxed);
            assert!(prev % 2 == 0, "Monotonic must have skipped an interrupt!");

            // Ensure that the COUNT has crossed
            // Due to syncing delay this may not be the case initially
            while Self::raw_count() < 0x8000 {}
        }
        if intflag.ovf().bit_is_set() {
            // This was an overflow interrupt
            // SYNC: None
            rtc.mode1().intflag().modify(|_, w| w.ovf().set_bit());
            let prev = RTC_PERIOD_COUNT.fetch_add(1, Ordering::Relaxed);
            assert!(prev % 2 == 1, "Monotonic must have skipped an interrupt!");

            // Ensure that the COUNT has wrapped
            // Due to syncing delay this may not be the case initially
            while Self::raw_count() > 0x8000 {}
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

            // Wrapping_sub deals with the u64 overflow corner case
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

            // SYNC: Write
            Self::sync();
            unsafe { rtc.mode1().comp(0).write(|w| w.comp().bits(val)) };
        });
    }

    fn clear_compare_flag() {
        let rtc = unsafe { pac::Rtc::steal() };
        // SYNC: None
        rtc.mode1().intflag().write(|w| w.cmp0().set_bit());
    }

    fn pend_interrupt() {
        pac::NVIC::pend(pac::Interrupt::RTC);
    }

    fn timer_queue() -> &'static TimerQueue<Self> {
        &RTC_TQ
    }
}
