#[doc(hidden)]
#[macro_export]
macro_rules! __internal_backend_methods {
    {
        mode = $mode:ty;
        rtic_int = $rtic_int:ty;
        rtc_pac = $rtc_pac: ident;
        init_compares = $init_compares:block
        statics = $statics:block
        enable_interrupts = $enable_interrupts:block
    }
    => {
        /// RTC interrupt handler called before control passes to the
        /// [`TimerQueue`
        /// handler`](rtic_time::timer_queue::TimerQueue::on_monotonic_interrupt).
        ///
        /// # Safety
        /// This should only be called from the RTC interrupt handler.
        #[inline]
        pub unsafe fn interrupt_handler() {
            let rtc = pac::Rtc::steal();

            /// Returns whether a < b, taking wrapping into account and assuming
            /// that the difference is less than a half period.
            #[inline]
            fn less_than_with_wrap(
                a: <$mode as RtcMode>::Count,
                b: <$mode as RtcMode>::Count,
            ) -> bool {
                let d = a.wrapping_sub(b);

                d >= <$mode as RtcModeMonotonic>::HALF_PERIOD
            }

            // Ensure that the COUNT is at least the compare value
            // Due to syncing delay this may not be the case initially
            // Note that this has to be done here because RTIC will clear the cmp0 flag
            // before `RtcBackend::on_interrupt` is called.
            if <$mode>::check_interrupt_flag::<$rtic_int>(&rtc) {
                let compare = <$mode>::get_compare(&rtc, 0);
                while less_than_with_wrap(<$mode>::count(&rtc), compare) {}
            }

            Self::timer_queue().on_monotonic_interrupt();
        }

        /// Starts the clock.
        ///
        /// **Do not use this function directly.**
        ///
        /// Use the crate level macros instead, then call `start` on the monotonic.
        pub fn _start($rtc_pac: pac::Rtc) {
            // Disable the RTC.
            <$mode>::disable(&$rtc_pac);

            // Reset RTC back to initial settings, which disables it and enters mode 0.
            <$mode>::reset(&$rtc_pac);

            unsafe {
                // Set the RTC mode
                <$mode>::set_mode(&$rtc_pac);

                $init_compares
            }

            // Timing critical, make sure we don't get interrupted.
            critical_section::with(|_| {
                // Start the timer and initialize it
                <$mode>::start_and_initialize(&$rtc_pac);

                // Clear the triggered compare flag
                <$mode>::clear_interrupt_flag::<$rtic_int>(&$rtc_pac);

                // Enable the compare interrupt
                <$mode>::enable_interrupt::<$rtic_int>(&$rtc_pac);

                $statics

                $enable_interrupts

                // Enable the RTC interrupt in the NVIC and set its priority.
                // SAFETY: We take full ownership of the peripheral and interrupt vector,
                // plus we are not using any external shared resources so we won't impact
                // basepri/source masking based critical sections.
                unsafe {
                    $crate::rtc::rtic::set_monotonic_prio(pac::NVIC_PRIO_BITS, pac::Interrupt::RTC);
                    pac::NVIC::unmask(pac::Interrupt::RTC);
                }
            });
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __internal_basic_backend {
    ($name:ident, $mode:ty, $mode_num:literal, $rtic_int:ty) => {
        use atsamd_hal_macros::hal_macro_helper;
        use rtic_time::timer_queue::{TimerQueue, TimerQueueBackend};
        use $crate::pac;
        use $crate::rtc::modes::RtcMode;

        #[doc = concat!("Basic RTC-based [`TimerQueueBackend`] without period counting that uses the RTC in mode ", stringify!($mode_num), ".")]
        pub struct $name;

        static RTC_TQ: TimerQueue<$name> = TimerQueue::new();

        #[hal_macro_helper]
        impl $name {
            $crate::__internal_backend_methods! {
                mode = $mode;
                rtic_int = $rtic_int;
                rtc_pac = rtc;
                init_compares = {
                    // Set the the initial compare
                    <$mode>::set_compare(&rtc, 0, 0);
                }
                statics = {
                    // Initialize the timer queue
                    RTC_TQ.initialize(Self);
                }
                enable_interrupts = {
                    // Enable the compare interrupt
                    <$mode>::enable_interrupt::<$rtic_int>(&rtc);
                }
            }
        }

        impl TimerQueueBackend for $name {
            type Ticks = <$mode as RtcMode>::Count;

            #[hal_macro_helper]
            fn now() -> Self::Ticks {
                <$mode>::count(unsafe { &pac::Rtc::steal() })
            }

            fn enable_timer() {
                <$mode>::enable(unsafe { &pac::Rtc::steal() });
            }

            fn disable_timer() {
                <$mode>::disable(unsafe { &pac::Rtc::steal() });
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
                if instant.saturating_sub(Self::now())
                    < <$mode as RtcModeMonotonic>::MIN_COMPARE_TICKS
                {
                    instant = instant.wrapping_add(<$mode as RtcModeMonotonic>::MIN_COMPARE_TICKS)
                }

                unsafe { <$mode>::set_compare(&rtc, 0, instant) };
            }

            fn clear_compare_flag() {
                <$mode>::clear_interrupt_flag::<$rtic_int>(unsafe { &pac::Rtc::steal() });
            }

            fn pend_interrupt() {
                pac::NVIC::pend(pac::Interrupt::RTC);
            }

            fn timer_queue() -> &'static TimerQueue<Self> {
                &RTC_TQ
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __internal_half_period_counting_backend {
    ($name:ident, $mode:ty, $mode_num:literal, $rtic_int:ty, $half_period_int:ty, $overflow_int:ty) => {
        use atsamd_hal_macros::hal_macro_helper;
        use core::sync::atomic::Ordering;
        use portable_atomic::AtomicU64;
        use rtic_time::{
            half_period_counter::calculate_now,
            timer_queue::{TimerQueue, TimerQueueBackend},
        };
        use $crate::pac;

        struct TimerValue(<$mode as RtcMode>::Count);
        impl rtic_time::half_period_counter::TimerValue for TimerValue {
            const BITS: u32 = <$mode as RtcMode>::Count::BITS;
        }
        impl From<TimerValue> for u64 {
            fn from(value: TimerValue) -> Self {
                Self::from(value.0)
            }
        }

        #[doc = concat!("An RTC-based [`TimerQueueBackend`] using [half-period counting](rtic_time::half_period_counter) that uses the RTC in mode ", stringify!($mode_num), ".")]
        pub struct $name;

        static RTC_PERIOD_COUNT: AtomicU64 = AtomicU64::new(0);
        static RTC_TQ: TimerQueue<$name> = TimerQueue::new();

        #[hal_macro_helper]
        impl $name {
            $crate::__internal_backend_methods! {
                mode = $mode;
                rtic_int = $rtic_int;
                rtc_pac = rtc;
                init_compares = {
                    // Configure the compare registers
                    <$mode>::set_compare(&rtc, 0, 0);
                    <$mode>::set_compare(&rtc, 1, <$mode as RtcModeMonotonic>::HALF_PERIOD);
                }
                statics = {
                    // Make sure period counter is synced with the timer value
                    RTC_PERIOD_COUNT.store(0, Ordering::SeqCst);

                    // Initialize the timer queue
                    RTC_TQ.initialize(Self);
                }
                enable_interrupts = {
                    // Enable the compare and overflow interrupts.
                    <$mode>::enable_interrupt::<$rtic_int>(&rtc);
                    <$mode>::enable_interrupt::<$half_period_int>(&rtc);
                    <$mode>::enable_interrupt::<$overflow_int>(&rtc);
                }
            }
        }

        impl TimerQueueBackend for RtcBackend {
            type Ticks = u64;

            #[hal_macro_helper]
            fn now() -> Self::Ticks {
                calculate_now(
                    || RTC_PERIOD_COUNT.load(Ordering::Relaxed),
                    || TimerValue(<$mode>::count(unsafe { &pac::Rtc::steal() })),
                )
            }

            fn enable_timer() {
                <$mode>::enable(unsafe { &pac::Rtc::steal() });
            }

            fn disable_timer() {
                <$mode>::disable(unsafe { &pac::Rtc::steal() });
            }

            fn on_interrupt() {
                let rtc: pac::Rtc = unsafe { pac::Rtc::steal() };

                // NOTE: The cmp0 flag is cleared when RTIC calls `clear_compare_flag`.
                if <$mode>::check_interrupt_flag::<$half_period_int>(&rtc) {
                    <$mode>::clear_interrupt_flag::<$half_period_int>(&rtc);
                    let prev = RTC_PERIOD_COUNT.fetch_add(1, Ordering::Relaxed);
                    assert!(prev % 2 == 0, "Monotonic must have skipped an interrupt!");

                    // Ensure that the COUNT has crossed
                    // Due to syncing delay this may not be the case initially
                    while <$mode>::count(&rtc) < <$mode as RtcModeMonotonic>::HALF_PERIOD {}
                }
                if <$mode>::check_interrupt_flag::<$overflow_int>(&rtc) {
                    <$mode>::clear_interrupt_flag::<$overflow_int>(&rtc);
                    let prev = RTC_PERIOD_COUNT.fetch_add(1, Ordering::Relaxed);
                    assert!(prev % 2 == 1, "Monotonic must have skipped an interrupt!");

                    // Ensure that the COUNT has wrapped
                    // Due to syncing delay this may not be the case initially
                    while <$mode>::count(&rtc) > <$mode as RtcModeMonotonic>::HALF_PERIOD {}
                }
            }

            fn set_compare(mut instant: Self::Ticks) {
                let rtc = unsafe { pac::Rtc::steal() };

                const MAX: u64 = <$mode as RtcMode>::Count::MAX as u64;

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
                        if diff < <$mode as RtcModeMonotonic>::MIN_COMPARE_TICKS.into() {
                            instant = instant
                                .wrapping_add(<$mode as RtcModeMonotonic>::MIN_COMPARE_TICKS.into())
                        }

                        (instant & MAX) as <$mode as RtcMode>::Count
                    } else {
                        0
                    };

                    unsafe { <$mode>::set_compare(&rtc, 0, val) };
                });
            }

            fn clear_compare_flag() {
                <$mode>::clear_interrupt_flag::<$rtic_int>(unsafe { &pac::Rtc::steal() });
            }

            fn pend_interrupt() {
                pac::NVIC::pend(pac::Interrupt::RTC);
            }

            fn timer_queue() -> &'static TimerQueue<Self> {
                &RTC_TQ
            }
        }
    };
}
