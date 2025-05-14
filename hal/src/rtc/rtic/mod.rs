//! [`Monotonic`](rtic_time::Monotonic) implementations using the Real Time
//! Clock (RTC).
//!
//! Enabling the `rtic` feature is required to use this module.
//!
//! For RTIC v1, the old [`rtic_monotonic::Monotonic`] trait is implemented for
//! [`Rtc`](crate::rtc::Rtc) in [`Count32Mode`](crate::rtc::Count32Mode) in the
//! [`v1`] module. A monotonic for RTIC v2 is provided here.
//!
//! # RTC clock selection
//!
//! Prior to starting the monotonic, the RTC clock source must be configured
//! using [`clocks`](crate::clock). On SAMD11/21 platforms, the RTC clock must
//! be setup as a [generic clock](crate::clock::GenericClockController).
//! On SAMx5x platforms the RTC clock must be selected from either the 1.1024
//! kHz clock or the 32.768 kHz clock, either of which can be internal or
//! external.
//!
//! **NOTE: Eventually, starting the monotonic will require proof that the RTC
//! clock has been configured. However, this requires v2 of the clock API for
//! SAMx5x chips, which is not yet fully supported in the rest of the HAL.**
//!
//! # RTC modes
//!
//! The RTC on all chip variants has two counter modes: mode 0 features a 32-bit
//! hardware counter, and mode 1 features a a 16-bit hardware counter but some
//! additional features. Part of the [`Monotonic`](rtic_time::Monotonic)
//! contract is that the monotonic should always count up and never roll over
//! back to time zero. However, even the 32-bit hardware counter will overflow
//! after about 36 hours using the faster clock rate, which is not acceptable.
//!
//! A technique known as [half-period counting
//! (HPC)](rtic_time::half_period_counter) is used to effectively increase the
//! montononic counter to be 64 bits wide in either mode. The result is a
//! monotonic that effectively counts up forever without rolling over. This
//! technique requires two compare registers, one for waking RTIC tasks and one
//! for HPC. The number of compare registers available on ATSAMD chips is as
//! follows:
//!
//! |            | SAMD11/21 | SAMx5x |
//! | -----------| --------- | ------ |
//! | **Mode 0** | 1         | 2      |
//! | **Mode 1** | 2         | 4      |
//!
//! As a result, HPC can be done in mode 0 for SAMx5x chips but requires mode 1
//! for SAMD11/21 variants. The monotonic provided for each variant uses the
//! appropriate RTC mode.
//!
//! The monotonics have the following specifications:
//!
//! |                                      | 1 kHz clock        | 32 kHz clock        |
//! | ------------------------------------ | ------------------ | ------------------- |
//! | **Rollover period**                  | ~571 million years | ~17.8 million years |
//! | **HPC interrupt period (SAMD11/21)** | 32 seconds         | 1 second            |
//! | **HPC interrupt period (SAMx5x)**    | ~24 days           | ~18 hours           |
//! | **Time resolution**                  | ~977 μs            | ~31 μs              |
//!
//! # Usage
//!
//! The monotonic should be created using the
//! [macro](crate::rtc_monotonic). The first macro argument is the name of
//! the global structure that will implement
//! [`Monotonic`](rtic_time::Monotonic). The RTC clock rate must be
//! known at compile time, and so the appropriate type from [`rtc_clock`] must
//! be passed to the macro as the second argument.
//!
//! Sometime during initialization, the monotonic also must be started by
//! calling the `start` method on the created monotonic. The
//! [`Rtc`](crate::pac::Rtc) peripheral struct must be passed to `start` to
//! ensure that the monotonic has complete control of the RTC.
//!
//! Note that the macro creates the RTC interrupt handler, and starting the
//! monotonic enables RTC interrupts in the NVIC, so that this does not need to
//! be done manually.
//!
//! # Example
//!
//! ```
//! use atsamd_hal::prelude::*;
//! use atsamd_hal::rtc::rtic::rtc_clock;
//!
//! // Create the monotonic struct named `Mono`
//! rtc_monotonic!(Mono, rtc_clock::Clock32k);
//!
//! // Uncomment if not using the RTIC RTOS:
//! // #[no_mangle]
//! // static RTIC_ASYNC_MAX_LOGICAL_PRIO: u8 = 1;
//! //
//! // This tells the monotonic driver the maximum interrupt
//! // priority it's allowed to use. RTIC sets it automatically,
//! // but you need to set it manually if you're writing
//! // a RTIC-less app.
//!
//! fn init() {
//!     # // This is normally provided by the selected PAC
//!     # let rtc = unsafe { core::mem::transmute(()) };
//!     # let mut mclk = unsafe { core::mem::transmute(()) };
//!     # let mut osc32kctrl = unsafe { core::mem::transmute(()) };
//!     // Here the RTC clock source should be configured using the clocks API
//!
//!     // Start the monotonic
//!     Mono::start(rtc);
//! }
//!
//! async fn usage() {
//!     loop {
//!          // Use the monotonic
//!          let timestamp = Mono::now();
//!
//!          Mono::delay_until(timestamp + 2u32.secs()).await;
//!          Mono::delay(100u32.millis()).await;
//!     }
//! }
//! ```
//!
//! # Other notes
//!
//! The number returned by
//! [`Monotonic::now().ticks()`](rtic_monotonic::Monotonic::now) will always
//! increase (barring monotonic rollover). However, due to the register
//! [synchronization delay](https://onlinedocs.microchip.com/oxy/GUID-F5813793-E016-46F5-A9E2-718D8BCED496-en-US-14/GUID-0C52DB00-4BF6-4F41-85B5-B76529875364.html),
//! the number returned may not always increment by one every time it changes.
//! In fact, testing shows that it typically increments by four every time it
//! changes. This is true regardless of the clock rate used, as the
//! synchronization delay scales along with the clock period.

/// Items for RTIC v1.
///
/// This mainly implements [`rtic_monotonic::Monotonic`] for
/// [`Rtc<Count32Mode>`](crate::rtc::Rtc).
///
/// This will be removed in a future release, users should migrate to RTIC v2.
#[deprecated]
pub mod v1 {
    use crate::rtc::{
        modes::{
            mode0::{Compare0, RtcMode0},
            RtcMode,
        },
        Count32Mode, Rtc,
    };
    use rtic_monotonic::Monotonic;

    /// The RTC clock frequency in Hz.
    pub const CLOCK_FREQ: u32 = 32_768;

    /// The [`fugit`] time instant.
    pub type Instant = fugit::Instant<u32, 1, CLOCK_FREQ>;
    /// The [`fugit`] time duration.
    pub type Duration = fugit::Duration<u32, 1, CLOCK_FREQ>;

    impl Monotonic for Rtc<Count32Mode> {
        type Instant = Instant;
        type Duration = Duration;
        unsafe fn reset(&mut self) {
            // Since reset is only called once, we use it to enable the interrupt generation
            // bit.
            RtcMode0::enable_interrupt::<Compare0>(&self.rtc);
        }

        fn now(&mut self) -> Self::Instant {
            Self::Instant::from_ticks(self.count32())
        }

        fn zero() -> Self::Instant {
            Self::Instant::from_ticks(0)
        }

        fn set_compare(&mut self, instant: Self::Instant) {
            RtcMode0::set_compare(&self.rtc, 0, instant.ticks());
        }

        fn clear_compare_flag(&mut self) {
            RtcMode0::clear_interrupt_flag::<Compare0>(&self.rtc);
        }
    }
}

mod backends;

use super::modes::{mode0::RtcMode0, mode1::RtcMode1, RtcMode};
use crate::interrupt::{Priority, NVIC_PRIO_BITS};
use atsamd_hal_macros::hal_cfg;

/// Types used to specify the RTC clock rate at compile time when creating the
/// monotonics.
///
/// These types utilize [type-level programming](crate::typelevel)
/// techniques and are passed to the [monotonic creation
/// macro](crate::rtc_monotonic).
/// The RTC clock rate must be specified at compile time so that the `Instant`
/// and `Duration` types in
/// [`TimerQueueBasedMonotonic`](rtic_time::monotonic::TimerQueueBasedMonotonic)
/// can be specified.
pub mod rtc_clock {
    /// Type-level enum for available RTC clock rates.
    pub trait RtcClockRate {
        const RATE_HZ: u32;
    }

    /// Type level [`RtcClockRate`] variant for the 32.768 kHz clock rate.
    pub enum Clock32k {}
    impl RtcClockRate for Clock32k {
        const RATE_HZ: u32 = 32_768;
    }

    /// Type level [`RtcClockRate`] variant for the 1.024 kHz clock rate.
    pub enum Clock1k {}
    impl RtcClockRate for Clock1k {
        const RATE_HZ: u32 = 1_024;
    }

    /// Type level [`RtcClockRate`] variant for a custom clock rate
    pub enum ClockCustom<const RATE_HZ: u32> {}
    impl<const RATE_HZ: u32> RtcClockRate for ClockCustom<RATE_HZ> {
        const RATE_HZ: u32 = RATE_HZ;
    }
}

trait RtcModeMonotonic: RtcMode {
    /// The COUNT value representing a half period.
    const HALF_PERIOD: Self::Count;
    /// The minimum number of ticks that compares need to be ahead of the COUNT
    /// in order to trigger.
    const MIN_COMPARE_TICKS: Self::Count;
}
impl RtcModeMonotonic for RtcMode0 {
    const HALF_PERIOD: Self::Count = 0x8000_0000;
    const MIN_COMPARE_TICKS: Self::Count = 8;
}
impl RtcModeMonotonic for RtcMode1 {
    const HALF_PERIOD: Self::Count = 0x8000;
    const MIN_COMPARE_TICKS: Self::Count = 8;
}

mod backend {
    use super::*;

    // For SAMD11/21 chips mode 1 is the only sensible option
    #[hal_cfg(any("rtc-d11", "rtc-d21"))]
    use crate::rtc::modes::mode1::{Compare0, Compare1, Overflow};

    #[hal_cfg(any("rtc-d11", "rtc-d21"))]
    crate::__internal_half_period_counting_backend!(
        RtcBackend, RtcMode1, 1, Compare0, Compare1, Overflow
    );

    // For SAMx5x mode 0 is the best option
    #[hal_cfg("rtc-d5x")]
    use crate::rtc::modes::mode0::{Compare0, Compare1, Overflow};

    #[hal_cfg("rtc-d5x")]
    crate::__internal_half_period_counting_backend!(
        RtcBackend, RtcMode0, 0, Compare0, Compare1, Overflow
    );
}

pub use backend::RtcBackend;

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
    ($name:ident, $backend:ident, $clock_rate:ty) => {
        /// A `Monotonic` based on the RTC peripheral.
        pub struct $name;

        impl $name {
            /// This method must be called only once.
            pub fn start(rtc: $crate::pac::Rtc) {
                use $crate::rtc::rtic::rtc_clock::*;
                $crate::__internal_create_rtc_interrupt!($backend);

                $crate::rtc::rtic::$backend::_start(rtc);
            }
        }

        use $crate::rtc::rtic::rtc_clock::RtcClockRate;

        impl $crate::rtic_time::monotonic::TimerQueueBasedMonotonic for $name {
            type Backend = $crate::rtc::rtic::$backend;
            type Instant = $crate::fugit::Instant<
                <Self::Backend as $crate::rtic_time::timer_queue::TimerQueueBackend>::Ticks,
                1,
                { <$clock_rate>::RATE_HZ },
            >;
            type Duration = $crate::fugit::Duration<
                <Self::Backend as $crate::rtic_time::timer_queue::TimerQueueBackend>::Ticks,
                1,
                { <$clock_rate>::RATE_HZ },
            >;
        }

        $crate::rtic_time::impl_embedded_hal_delay_fugit!($name);
        $crate::rtic_time::impl_embedded_hal_async_delay_fugit!($name);
    };
}

/// Create an RTIC v2 monotonic that uses the RTC.
///
/// See the [`rtic`](crate::rtc::rtic) module for details.
#[macro_export]
macro_rules! rtc_monotonic {
    ($name:ident, $clock_rate: ty) => {
        $crate::__internal_create_rtc_struct!($name, RtcBackend, $clock_rate);
    };
}

/// This function was modified from the private function in `rtic-monotonics`,
/// part of the [`rtic`](https://github.com/rtic-rs/rtic) project.
///
/// Note that this depends on the static variable `RTIC_ASYNC_MAX_LOGICAL_PRIO`
/// defined as part of RTIC. Refer to the example in the [`rtic`
/// module](crate::rtc::rtic) documentation for more details.
///
/// See LICENSE-MIT and LICENSE-APACHE for the licenses.
unsafe fn set_monotonic_prio(interrupt: impl cortex_m::interrupt::InterruptNumber) {
    extern "C" {
        static RTIC_ASYNC_MAX_LOGICAL_PRIO: u8;
    }

    let max_prio = RTIC_ASYNC_MAX_LOGICAL_PRIO.clamp(1, 1 << NVIC_PRIO_BITS);
    let hw_prio = Priority::from_numeric(max_prio).unwrap().logical2hw();

    // We take ownership of the entire IRQ and all settings to it, we only change
    // settings for the IRQ we control.
    // This will also compile-error in case the NVIC changes in size.
    let mut nvic: cortex_m::peripheral::NVIC = core::mem::transmute(());

    nvic.set_priority(interrupt, hw_prio);
}
