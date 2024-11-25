//! [`Monotonic`](rtic_time::Monotonic) implementations for the Real Time
//! Clock (RTC).
//!
//! Enabling the `rtic` feature is required to use this module.
//!
//! For RTIC v1, the old [`rtic_monotonic::Monotonic`] trait is implemented for
//! [`Rtc`](crate::rtc::Rtc) in [`Count32Mode`](crate::rtc::Count32Mode).
//!
//! The items here provide monotonics for RTIC v2. Two monotonics are provided:
//! one that uses the RTC in mode 0, and another that uses the RTC in mode 1.
//! The mode 0 monotonic uses a 32-bit hardware counter with no [half-period
//! counting](rtic_time::half_period_counter), whereas the mode 1 monotonic uses
//! a 16-bit counter but with half-period counting.
//!
//! The mode 0 monotonic has the advantage that the only interrupts that occur
//! are for the RTIC tasks, but the disadvantage is that the lack of half-period
//! counting means that the monotonic wraps in a much shorter time than the mode
//! 1 monotonic. The mode 1 monotonic, however is less efficient in the sense
//! that additional interrupts are required for half-period counting, which can
//! be quite frequent depending on the selected clock rate. The monotonic should
//! be chosen based on the specific use case, bearing in mind that monotonics
//! are never supposed to actually roll over back to zero time (i.e. they are
//! supposed to count time _monotonically_). Doing so may result in strange
//! behavior for tasks scheduled near the time of the rollover.
//!
//! The overall monotonic period (i.e. the time before the monotonic rolls over
//! back to zero time) is as follows:
//!
//! |            | 1 kHz clock        | 32 kHz clock        |
//! | ---------- | ------------------ | ------------------- |
//! | **Mode 0** | ~48 days           | ~36 hours           |
//! | **Mode 1** | ~571 million years | ~17.8 million years |
//!
//! The time precision (i.e. the RTC tick time and shortest delay time) is as
//! follows:
//!
//! |              | 1 kHz clock | 32 kHz clock |
//! | ------------ | ----------- | ------------ |
//! | **Any mode** | ~977 μs     | ~31 μs       |
//!
//! A monotonic using the desired RTC mode should be created with the
//! appropriate [macro](crate::rtc::rtic::prelude). The RTC clock rate and
//! source must also be specified when calling the macro, using the types in
//! [`rtc_clock`]. The first macro argument is the name of the global structure
//! that will implement [`Monotonic`](rtic_time::Monotonic). The second argument
//! must be a clock rate type that implements
//! [`RtcClockRate`](rtc_clock::RtcClockRate), and the third argument must be a
//! type clock source implementing
//! [`RtcClockSource`](rtc_clock::RtcClockSource). See below for an example.
//!
//! NOTE: These monotonics currently live in the HAL for testing and refinement
//! purposes. The intention is to eventually move them to the
//! [`rtic-monotonics`](https://docs.rs/rtic-monotonics/latest/rtic_monotonics/) crate, which is a central location for peripheral-based
//! RTIC monotonics for various microcontroller families. As such, be aware that
//! this module could disappear at any time in the future.
//!
//! NOTE: Some SAMD chips will support half-period counting in mode 0, which
//! would greatly extend the overall monotonic period to that of the mode 1
//! monotonic, while also still being very interrupt-efficient. It is planned to
//! add this to the mode 0 monotonic for chips that support it.
//!
//! NOTE: The mode 1 monotonic currently has a robustness issue that is being
//! worked on. Refer to [Issue #765](https://github.com/atsamd-rs/atsamd/issues/765) for details.
//!
//! # Example
//!
//! ```
//! use atsamd_hal::prelude::*;
//! rtc_mode0_monotonic!(Mono, rtc_clock::Clock32k, rtc_clock::ClockInternal);
//!
//! fn init() {
//!     # // This is normally provided by the selected PAC
//!     # let rtc = unsafe { core::mem::transmute(()) };
//!     # let mut mclk = unsafe { core::mem::transmute(()) };
//!     # let mut osc32kctrl = unsafe { core::mem::transmute(()) };
//!     // Start the monotonic
//!     Mono::start(rtc, &mut mclk, &mut osc32kctrl);
//! }
//!
//! async fn usage() {
//!     loop {
//!          // Use the monotonic
//!          let timestamp = Mono::now();
//!          Mono::delay(100u32.millis()).await;
//!     }
//! }
//! ```

// TODO: Before HAL PR
// - Make sure every chip at least compiles

mod v1 {
    use crate::rtc::{Count32Mode, Rtc};
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

mod mode0;
mod mode1;

pub use mode0::RtcBackend as RtcMode0Backend;
pub use mode1::RtcBackend as RtcMode1Backend;

const MIN_COMPARE_TICKS: u32 = 5;

pub mod prelude {
    pub use super::rtc_clock;
    pub use crate::{rtc_mode0_monotonic, rtc_mode1_monotonic};
    pub use rtic_time::Monotonic;

    pub use fugit::{self, ExtU32, ExtU32Ceil, ExtU64, ExtU64Ceil};
}

/// Types used to specify the RTC clock source at compile time when creating the
/// monotonics.
///
/// These types utilize [type-level programming](crate::typelevel)
/// techniques and are passed to the [monotonic creation
/// macros](crate::rtc::rtic::prelude)
///
/// NOTE: This could probably be accomplished using the items from
/// [`clock::v2::rtcosc`](crate::clock::v2::rtcosc), but we want to avoid
/// dependencies from other parts of the HAL since this will move to [`rtic-monotonics`](https://docs.rs/rtic-monotonics/latest/rtic_monotonics/).
pub mod rtc_clock {
    use crate::pac::{generic::W, osc32kctrl::rtcctrl::RtcctrlSpec};
    use core::marker::PhantomData;

    /// Type-level enum for available RTC clock rates.
    pub trait RtcClockRate {
        const RATE: u32;
    }

    /// Type level [`RtcClockRate`] variant for the 32.768 kHz clock rate.
    pub enum Clock32k {}
    impl RtcClockRate for Clock32k {
        const RATE: u32 = 32_768;
    }

    /// Type level [`RtcClockRate`] variant for the 1.024 kHz clock rate.
    pub enum Clock1k {}
    impl RtcClockRate for Clock1k {
        const RATE: u32 = 1_024;
    }

    /// Type-level enum for available RTC sources.
    pub trait RtcClockSource {}

    /// Type level [`RtcClockSource`] variant for an internal clock source.
    pub enum ClockInternal {}
    impl RtcClockSource for ClockInternal {}

    /// Type level [`RtcClockSource`] variant for an external clock source.
    pub enum ClockExternal {}
    impl RtcClockSource for ClockExternal {}

    /// Types that can configure the RTC clock.
    pub trait RtcClockSetter {
        /// Sets the RTC clock source by modifying the register.
        fn set_source(reg: &mut W<RtcctrlSpec>) -> &mut W<RtcctrlSpec>;
    }

    /// Collection forming a complete RTC clock source, and that can configure
    /// the clock source as an [`RtcClockSetter`].
    pub struct ClockSetter<R: RtcClockRate, S: RtcClockSource> {
        rate: PhantomData<R>,
        source: PhantomData<S>,
    }
    impl RtcClockSetter for ClockSetter<Clock1k, ClockInternal> {
        #[inline]
        fn set_source(reg: &mut W<RtcctrlSpec>) -> &mut W<RtcctrlSpec> {
            reg.rtcsel().ulp1k()
        }
    }
    impl RtcClockSetter for ClockSetter<Clock1k, ClockExternal> {
        #[inline]
        fn set_source(reg: &mut W<RtcctrlSpec>) -> &mut W<RtcctrlSpec> {
            reg.rtcsel().xosc1k()
        }
    }

    impl RtcClockSetter for ClockSetter<Clock32k, ClockInternal> {
        #[inline]
        fn set_source(reg: &mut W<RtcctrlSpec>) -> &mut W<RtcctrlSpec> {
            reg.rtcsel().ulp32k()
        }
    }
    impl RtcClockSetter for ClockSetter<Clock32k, ClockExternal> {
        #[inline]
        fn set_source(reg: &mut W<RtcctrlSpec>) -> &mut W<RtcctrlSpec> {
            reg.rtcsel().xosc32k()
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __internal_create_rtc_interrupt {
    ($backend:ident) => {
        #[no_mangle]
        #[allow(non_snake_case)]
        unsafe extern "C" fn RTC() {
            $crate::rtc::rtic::$backend::interrupt_handler();
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __internal_create_rtc_struct {
    ($name:ident, $backend:ident, $clock_rate:ty, $clock_source:ty) => {
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
                use $crate::rtc::rtic::rtc_clock::*;
                $crate::__internal_create_rtc_interrupt!($backend);

                $crate::rtc::rtic::$backend::_start::<ClockSetter<$clock_rate, $clock_source>>(
                    rtc, mclk, osc32kctrl,
                );
            }
        }

        use $crate::rtc::rtic::rtc_clock::RtcClockRate;

        impl $crate::rtic_time::monotonic::TimerQueueBasedMonotonic for $name {
            type Backend = $crate::rtc::rtic::$backend;
            type Instant = $crate::fugit::Instant<
                <Self::Backend as $crate::rtic_time::timer_queue::TimerQueueBackend>::Ticks,
                1,
                { <$clock_rate>::RATE },
            >;
            type Duration = $crate::fugit::Duration<
                <Self::Backend as $crate::rtic_time::timer_queue::TimerQueueBackend>::Ticks,
                1,
                { <$clock_rate>::RATE },
            >;
        }

        $crate::rtic_time::impl_embedded_hal_delay_fugit!($name);
        $crate::rtic_time::impl_embedded_hal_async_delay_fugit!($name);
    };
}

/// Create an RTIC v2 monotonic using the RTC in mode 0.
///
/// See the [`rtic`](crate::rtc::rtic) module for details.
#[macro_export]
macro_rules! rtc_mode0_monotonic {
    ($name:ident, $clock_rate: ty, $clock_source: ty) => {
        $crate::__internal_create_rtc_struct!($name, RtcMode0Backend, $clock_rate, $clock_source);
    };
}

/// Create an RTIC v2 monotonic based on RTC in mode 1.
///
/// See the [`rtic`](crate::rtc::rtic) module for details.
#[macro_export]
macro_rules! rtc_mode1_monotonic {
    ($name:ident, $clock_rate: ty, $clock_source: ty) => {
        $crate::__internal_create_rtc_struct!($name, RtcMode1Backend, $clock_rate, $clock_source);
    };
}

/// This function was copied from the private function in `rtic-monotonics`,
/// so that should be used when the monotonics move there.
const fn cortex_logical2hw(logical: u8, nvic_prio_bits: u8) -> u8 {
    ((1 << nvic_prio_bits) - logical) << (8 - nvic_prio_bits)
}

/// This function was copied from the private function in `rtic-monotonics`,
/// so that should be used when the monotonics move there.
unsafe fn set_monotonic_prio(prio_bits: u8, interrupt: impl cortex_m::interrupt::InterruptNumber) {
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

// TODO: Test code
#[derive(Default)]
struct LoopChecker {
    count: usize,
}
impl LoopChecker {
    pub fn too_many(&mut self) -> bool {
        self.count += 1;

        self.count > 0x800000
    }
}
