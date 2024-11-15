//! [`Monotonic`](rtic_time::Monotonic) implementations for the Real Time
//! Clock (RTC).
//!
//! TODO: Somewhere make a note about u32 implementation limited to 36 hours and
//! 24 minutes
//!
//! TODO: Mention v1 and v2.
//! TODO: Have prelude for monotonic that includes extension traits like in
//! `rtic-monotonics`?
//!
//! # Example
//!
//! ```
//! // TODO: Update this
//! use atsamd_hal::rtc::rtic::prelude::*;
//! rtc_monotonic!(Mono);
//!
//! fn init() {
//!     # // This is normally provided by the selected PAC
//!     # let rtc = unsafe { core::mem::transmute(()) };
//!     # let mut mclk = unsafe { core::mem::transmute(()) };
//!     // Start the monotonic
//!     Mono::start(rtc, &mut mclk);
//! }
//!
//! async fn usage() {
//!     loop {
//!          // Use the monotonic
//!          let timestamp = Mono::now();
//!          Mono::delay(100.millis()).await;
//!     }
//! }
//! ```

use super::{Count32Mode, Rtc};

// TODO: how to handle this?
pub use v2::*;

mod v1 {
    use super::*;
    use rtic_monotonic::Monotonic;

    /// The RTC clock frequency in Hz.
    pub const CLOCK_FREQ: u32 = 32_768;

    type Instant = fugit::Instant<u32, 1, CLOCK_FREQ>;
    type Duration = fugit::Duration<u32, 1, CLOCK_FREQ>;

    impl Monotonic for Rtc<Count32Mode> {
        type Instant = Instant;
        type Duration = Duration;
        unsafe fn reset(&mut self) {
            // Since reset is only called once, we use it to enable the interrupt generation
            // bit.
            self.mode0().intenset().write(|w| w.cmp0().set_bit());
        }

        fn now(&mut self) -> Self::Instant {
            Self::Instant::from_ticks(self.count32())
        }

        fn zero() -> Self::Instant {
            Self::Instant::from_ticks(0)
        }

        fn set_compare(&mut self, instant: Self::Instant) {
            unsafe {
                self.mode0()
                    .comp(0)
                    .write(|w| w.comp().bits(instant.ticks()))
            }
        }

        fn clear_compare_flag(&mut self) {
            self.mode0().intflag().write(|w| w.cmp0().set_bit());
        }
    }
}

pub mod v2 {
    use crate::pac;
    use atsamd_hal_macros::hal_macro_helper;
    use rtic_time::timer_queue::{TimerQueue, TimerQueueBackend};

    const MIN_COMPARE_TICKS: u32 = 5;

    // TODO: docs or suppress? Can we move to enum type programming?
    /// The exact 1 kHz clock frequency in Hz
    pub const CLOCK_FREQ_1K: u32 = 1_024;
    /// The exact 32 kHz clock frequency in Hz
    pub const CLOCK_FREQ_32K: u32 = 32_768;

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __internal_create_rtc_interrupt {
        ($mode:ident) => {
            #[no_mangle]
            #[allow(non_snake_case)]
            unsafe extern "C" fn RTC() {
                use $crate::rtic_time::timer_queue::TimerQueueBackend;
                $crate::rtc::rtic::$mode::RtcBackend::timer_queue().on_monotonic_interrupt();
            }
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __internal_create_rtc_struct {
        ($name:ident, $mode:ident, $clock_rate:expr, $clock_source:ident) => {
            /// A `Monotonic` based on the RTC peripheral.
            pub struct $name;

            impl $name {
                /// Starts the `Monotonic`.
                ///
                /// This method must be called only once.
                pub fn start(
                    rtc: $crate::pac::Rtc,
                    mclk: &mut $crate::pac::Mclk,
                    osc32kctrl: &mut $crate::pac::Osc32kctrl,
                ) {
                    $crate::__internal_create_rtc_interrupt!($mode);

                    $crate::rtc::rtic::$mode::RtcBackend::_start(
                        rtc,
                        mclk,
                        osc32kctrl,
                        $crate::rtc::rtic::ClockSource::$clock_source,
                    );
                }
            }

            impl $crate::rtic_time::monotonic::TimerQueueBasedMonotonic for $name {
                type Backend = $crate::rtc::rtic::$mode::RtcBackend;
                type Instant = $crate::fugit::Instant<
                    <Self::Backend as $crate::rtic_time::timer_queue::TimerQueueBackend>::Ticks,
                    1,
                    { $clock_rate },
                >;
                type Duration = $crate::fugit::Duration<
                    <Self::Backend as $crate::rtic_time::timer_queue::TimerQueueBackend>::Ticks,
                    1,
                    { $clock_rate },
                >;
            }

            $crate::rtic_time::impl_embedded_hal_delay_fugit!($name);
            $crate::rtic_time::impl_embedded_hal_async_delay_fugit!($name);
        };
    }

    // TODO: Have separate macros, or have additional type arguments?
    /* #[doc = r#"This is

    a multiline string"#] */

    /* /// Create an RTC based monotonic for RTIC v2 and register the RTC
     * interrupt for it with a 1.024 kHz internal clock.
     *
     * - **Total monotonic period:** ~48 days
     * - **Time precision:** ~977 μs
     *
     * See the [`rtc`](crate::rtc) module for more details. */
    #[macro_export]
    macro_rules! rtc_mode0_monotonic_1k_int {
        ($name:ident) => {
            $crate::__internal_create_rtc_struct!(
                $name,
                mode0,
                $crate::rtc::rtic::v2::CLOCK_FREQ_1K,
                Int1k
            );
        };
    }

    /// Create an RTC based monotonic for RTIC v2 and register the RTC interrupt
    /// for it with a 1.024 kHz internal clock.
    ///
    /// - **Total monotonic period:** ~48 days
    /// - **Time precision:** ~977 μs
    ///
    /// See the [`rtc`](crate::rtc) module for more details.
    #[macro_export]
    macro_rules! rtc_mode0_monotonic_1k_ext {
        ($name:ident) => {
            $crate::__internal_create_rtc_struct!(
                $name,
                mode0,
                $crate::rtc::rtic::v2::CLOCK_FREQ_1K,
                Ext1k
            );
        };
    }

    /// Create an RTC based monotonic for RTIC v2 and register the RTC interrupt
    /// for it with a 32.768 kHz internal clock.
    ///
    /// - **Total monotonic period:** ~36 hours
    /// - **Time precision:** ~31 μs
    ///
    /// See the [`rtc`](crate::rtc) module for more details.
    #[macro_export]
    macro_rules! rtc_mode0_monotonic_32k_int {
        ($name:ident) => {
            $crate::__internal_create_rtc_struct!(
                $name,
                mode0,
                $crate::rtc::rtic::v2::CLOCK_FREQ_32K,
                Int32k
            );
        };
    }

    /// Create an RTC based monotonic for RTIC v2 and register the RTC interrupt
    /// for it with a 32.768 kHz internal clock.
    ///
    /// - **Total monotonic period:** ~36 hours
    /// - **Time precision:** ~31 μs
    ///
    /// See the [`rtc`](crate::rtc) module for more details.
    #[macro_export]
    macro_rules! rtc_mode0_monotonic_32k_ext {
        ($name:ident) => {
            $crate::__internal_create_rtc_struct!(
                $name,
                mode0,
                $crate::rtc::rtic::v2::CLOCK_FREQ_32K,
                Ext32k
            );
        };
    }

    /// Create an RTC based monotonic for RTIC v2 and register the RTC interrupt
    /// for it with a 1.024 kHz internal clock.
    ///
    /// - **Total monotonic period:** ~571 million years
    /// - **Time precision:** ~977 μs
    ///
    /// See the [`rtc`](crate::rtc) module for more details.
    #[macro_export]
    macro_rules! rtc_mode1_monotonic_1k_int {
        ($name:ident) => {
            $crate::__internal_create_rtc_struct!(
                $name,
                mode1,
                $crate::rtc::rtic::v2::CLOCK_FREQ_1K,
                Int1k
            );
        };
    }

    /// Create an RTC based monotonic for RTIC v2 and register the RTC interrupt
    /// for it with a 1.024 kHz external clock.
    ///
    /// - **Total monotonic period:** ~571 million years
    /// - **Time precision:** ~977 μs
    ///
    /// See the [`rtc`](crate::rtc) module for more details.
    #[macro_export]
    macro_rules! rtc_mode1_monotonic_1k_ext {
        ($name:ident) => {
            $crate::__internal_create_rtc_struct!(
                $name,
                mode1,
                $crate::rtc::rtic::v2::CLOCK_FREQ_1K,
                Ext1k
            );
        };
    }

    /// Create an RTC based monotonic for RTIC v2 and register the RTC interrupt
    /// for it with a 32.768 kHz internal clock.
    ///
    /// - **Total monotonic period:** ~17.8 million years
    /// - **Time precision:**  ~31 μs
    ///
    /// See the [`rtc`](crate::rtc) module for more details.
    #[macro_export]
    macro_rules! rtc_mode1_monotonic_32k_int {
        ($name:ident) => {
            $crate::__internal_create_rtc_struct!(
                $name,
                mode1,
                $crate::rtc::rtic::v2::CLOCK_FREQ_32K,
                Int32k
            );
        };
    }

    /// Create an RTC based monotonic for RTIC v2 and register the RTC interrupt
    /// for it with a 32.768 kHz external clock.
    ///
    /// - **Total monotonic period:** ~17.8 million years
    /// - **Time precision:**  ~31 μs
    ///
    /// See the [`rtc`](crate::rtc) module for more details.
    #[macro_export]
    macro_rules! rtc_mode1_monotonic_32k_ext {
        ($name:ident) => {
            $crate::__internal_create_rtc_struct!(
                $name,
                mode1,
                $crate::rtc::rtic::v2::CLOCK_FREQ_32K,
                Ext32k
            );
        };
    }

    // TODO: Suppress doc probably
    // TODO: Move register write here
    // TODO: Can we use type programming enum instead?
    pub enum ClockSource {
        Int1k,
        Ext1k,
        Int32k,
        Ext32k,
    }

    pub mod mode0 {
        use super::*;

        /// RTC based [`TimerQueueBackend`].
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
            /// TODO: Update
            /// Use the [`rtc_monotonic`] macro instead.
            pub fn _start(
                rtc: pac::Rtc,
                mclk: &mut pac::Mclk,
                osc32kctrl: &mut pac::Osc32kctrl,
                clock_source: ClockSource,
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
                match clock_source {
                    ClockSource::Int1k => osc32kctrl.rtcctrl().write(|w| w.rtcsel().ulp1k()),
                    ClockSource::Ext1k => osc32kctrl.rtcctrl().write(|w| w.rtcsel().xosc1k()),
                    ClockSource::Int32k => osc32kctrl.rtcctrl().write(|w| w.rtcsel().ulp32k()),
                    ClockSource::Ext32k => osc32kctrl.rtcctrl().write(|w| w.rtcsel().xosc32k()),
                }

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

                    // Enable the RTC interrupt at the highest priority in the NVIC.
                    unsafe {
                        set_monotonic_prio(pac::NVIC_PRIO_BITS, pac::Interrupt::RTC);
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
    }

    pub mod mode1 {
        use core::sync::atomic::Ordering;

        use super::*;
        use portable_atomic::AtomicU64;
        use rtic_time::half_period_counter::calculate_now;

        struct TimerValueU16(u16);
        impl rtic_time::half_period_counter::TimerValue for TimerValueU16 {
            const BITS: u32 = 16;
        }
        impl From<TimerValueU16> for u64 {
            fn from(value: TimerValueU16) -> Self {
                Self::from(value.0)
            }
        }

        /// RTC based [`TimerQueueBackend`].
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
            /// TODO: Update
            /// Use the [`rtc_monotonic`] macro instead.
            pub fn _start(
                rtc: pac::Rtc,
                mclk: &mut pac::Mclk,
                osc32kctrl: &mut pac::Osc32kctrl,
                clock_source: ClockSource,
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
                match clock_source {
                    ClockSource::Int1k => osc32kctrl.rtcctrl().write(|w| w.rtcsel().ulp1k()),
                    ClockSource::Ext1k => osc32kctrl.rtcctrl().write(|w| w.rtcsel().xosc1k()),
                    ClockSource::Int32k => osc32kctrl.rtcctrl().write(|w| w.rtcsel().ulp32k()),
                    ClockSource::Ext32k => osc32kctrl.rtcctrl().write(|w| w.rtcsel().xosc32k()),
                }

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
                        set_monotonic_prio(pac::NVIC_PRIO_BITS, pac::Interrupt::RTC);
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
    }

    const fn cortex_logical2hw(logical: u8, nvic_prio_bits: u8) -> u8 {
        ((1 << nvic_prio_bits) - logical) << (8 - nvic_prio_bits)
    }

    unsafe fn set_monotonic_prio(
        prio_bits: u8,
        interrupt: impl cortex_m::interrupt::InterruptNumber,
    ) {
        extern "C" {
            static RTIC_ASYNC_MAX_LOGICAL_PRIO: u8;
        }

        let max_prio = RTIC_ASYNC_MAX_LOGICAL_PRIO.max(1).min(1 << prio_bits);
        let hw_prio = cortex_logical2hw(max_prio, prio_bits);

        // We take ownership of the entire IRQ and all settings to it, we only change
        // settings for the IRQ we control.
        // This will also compile-error in case the NVIC changes in size.
        let mut nvic: cortex_m::peripheral::NVIC = core::mem::transmute(());

        nvic.set_priority(interrupt, hw_prio);
    }
}

// TODO: Delete?
/* mod v2 {
    use super::*;
    use crate::pac;
    use portable_atomic::{AtomicU64, Ordering};
    use rtic_time::{
        half_period_counter::calculate_now,
        timer_queue::{TimerQueue, TimerQueueBackend},
    };

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __internal_create_rtc_interrupt {
        () => {
            #[no_mangle]
            #[allow(non_snake_case)]
            unsafe extern "C" fn RTC() {
                use $crate::rtic_time::timer_queue::TimerQueueBackend;
                $crate::rtc::rtic::RtcBackend::timer_queue().on_monotonic_interrupt();
            }
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __internal_create_rtc_struct {
        ($name:ident, $clock_rate:literal, $clock_source:ident) => {
            /// A `Monotonic` based on the RTC peripheral.
            pub struct $name;

            impl $name {
                /// Starts the `Monotonic`.
                ///
                /// This method must be called only once.
                pub fn start(
                    rtc: $crate::pac::Rtc,
                    mclk: &mut $crate::pac::Mclk,
                    osc32kctrl: &mut $crate::pac::Osc32kctrl,
                ) {
                    $crate::__internal_create_rtc_interrupt!();

                    $crate::rtc::rtic::RtcBackend::_start(
                        rtc,
                        mclk,
                        osc32kctrl,
                        $crate::rtc::rtic::ClockSource::$clock_source,
                    );
                }
            }

            impl $crate::rtic_time::monotonic::TimerQueueBasedMonotonic for $name {
                type Backend = $crate::rtc::rtic::RtcBackend;
                type Instant = $crate::fugit::Instant<
                    <Self::Backend as $crate::rtic_time::timer_queue::TimerQueueBackend>::Ticks,
                    1,
                    $clock_rate,
                >;
                type Duration = $crate::fugit::Duration<
                    <Self::Backend as $crate::rtic_time::timer_queue::TimerQueueBackend>::Ticks,
                    1,
                    $clock_rate,
                >;
            }

            $crate::rtic_time::impl_embedded_hal_delay_fugit!($name);
            $crate::rtic_time::impl_embedded_hal_async_delay_fugit!($name);
        };
    }


} */
