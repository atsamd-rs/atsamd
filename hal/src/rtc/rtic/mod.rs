//! [`Monotonic`](rtic_time::Monotonic) implementations using the Real Time
//! Clock (RTC).
//!
//! Enabling the `rtic` feature is required to use this module.
//!
//! For RTIC v1, the old [`rtic_monotonic::Monotonic`] trait is implemented for
//! [`Rtc`](crate::rtc::Rtc) in [`Count32Mode`](crate::rtc::Count32Mode).
//!
//! The items here provide monotonics for RTIC v2. Two monotonics are provided:
//! one that uses the RTC in mode 0 (32-bit hardware counter), and another that
//! uses the RTC in mode 1 (16-bit hardware counter).
//!
//! Which mode is used can influence the total monotonic period before the its
//! time counter overflows and rolls back to zero time. This rollover violates
//! the contract of the monotonic, which is supposed to count time
//! _monotonically_. As such, the rollover can cause strange behavior for tasks
//! scheduled near the time of the rollover. Hence, the monotonic should be
//! chosen to match the use case such that the monotonic will never roll over
//! during the expected total duration of program execution.
//!
//! For all chip variants, the mode 1 monotonic uses [half-period
//! counting](rtic_time::half_period_counter) to extend the monotonic counter to
//! 64-bits. This is enabled by the fact that the RTC for every variant has
//! at least two compare registers in mode 1. This greatly extends the total
//! monotonic period. However, in this mode, half-period counting interrupts
//! occur more frequently (see below) due to the hardware counter being only 16
//! bits wide, so it is less efficient in that regard.
//!
//! For SAMD11/21 chips, the mode 0 monotonic only has a single compare register
//! so that half-period counting is not possible. This significantly reduces the
//! total monotonic period. The SAMx5x chips, however, feature two compare
//! registers in mode 0 so that half-period counting can be done. In the latter
//! case, the mode 0 monotonic has extremely infrequent half-period counting
//! interrupts and so is more efficient. In fact, the mode 1 monotonic on SAMx5x
//! platforms offers nothing except more undesirable more frequent interrupts,
//! so its use is not recommended.
//!
//! # Choosing a mode
//!
//! The overall monotonic period (i.e. the time before the monotonic rolls over
//! back to zero time) is as follows:
//!
//! |                        | 1 kHz clock        | 32 kHz clock        |
//! | ---------------------- | ------------------ | ------------------- |
//! | **Mode 0 (SAMD11/21)** | ~48 days           | ~36 hours           |
//! | **Mode 0 (SAMx5x)**    | ~571 million years | ~17.8 million years |
//! | **Mode 1**             | ~571 million years | ~17.8 million years |
//!
//! The half-period counting interrupt periods for the monotonics are:
//!
//! |                        | 1 kHz clock | 32 kHz clock |
//! | ---------------------- | ----------- | ------------ |
//! | **Mode 0 (SAMD11/21)** | Never       | Never        |
//! | **Mode 0 (SAMx5x)**    | ~24 days    | ~18 hours    |
//! | **Mode 1**             | 32 seconds  | 1 second     |
//!
//! The time resolution (i.e. the RTC tick time and shortest delay time) is as
//! follows:
//!
//! |              | 1 kHz clock | 32 kHz clock |
//! | ------------ | ----------- | ------------ |
//! | **Any mode** | ~977 μs     | ~31 μs       |
//!
//! It should be apparent from these numbers that the mode 1 monotonic offers
//! nothing but more frequent interrupts on SAMx5x platforms.
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
//! NOTE: Eventually, starting the monotonic will require proof that the RTC
//! clock has been configured. However, this requires v2 of the clock API for
//! SAMx5x chips, which is not yet fully supported in the rest of the HAL.
//!
//! # Usage
//!
//! A monotonic using the desired RTC mode should be created with the
//! appropriate [macro](crate::rtc::rtic::prelude). The first macro argument is
//! the name of the global structure that will implement
//! [`Monotonic`](rtic_time::Monotonic). The RTC clock rate must be
//! known at compile time, and so the appropriate type from [`rtc_clock`] must
//! be passed to the macro as the second argument.
//!
//! Sometime during initialization, the monotonic also must be started by called
//! the `start` method on the created monotonic. The [`Rtc`](crate::pac::Rtc)
//! peripheral struct must be passed to `start` to ensure that the monotonic
//! has complete control of the RTC.
//!
//! Note that the macro creates the RTC interrupt handler, and starting the
//! monotonic enables RTC interrupts in the NVIC, so that this does not need to
//! be done manually.
//!
//! # Example
//!
//! ```
//! use atsamd_hal::prelude::*;
//!
//! // Create the monotonic struct named `Mono`
//! rtc_mode0_monotonic!(Mono, rtc_clock::Clock32k);
//!
//! fn init() {
//!     # // This is normally provided by the selected PAC
//!     # let rtc = unsafe { core::mem::transmute(()) };
//!     # let mut mclk = unsafe { core::mem::transmute(()) };
//!     # let mut osc32kctrl = unsafe { core::mem::transmute(()) };
//!     // Here the RTC clock source should be configured using the HAL
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

// TODO: Need to revisit this and either modernize (e.g. it does not use period
// counting) it or remove it.
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

mod backends;

use super::modes::{mode0::RtcMode0, mode1::RtcMode1, RtcMode};
use atsamd_hal_macros::hal_macro_helper;

pub mod prelude {
    pub use super::rtc_clock;
    pub use crate::{rtc_mode0_monotonic, rtc_mode1_monotonic};
    pub use rtic_time::Monotonic;

    pub use fugit::{self, ExtU32, ExtU32Ceil, ExtU64, ExtU64Ceil};
}

/// Types used to specify the RTC clock rate at compile time when creating the
/// monotonics.
///
/// These types utilize [type-level programming](crate::typelevel)
/// techniques and are passed to the [monotonic creation
/// macros](crate::rtc::rtic::prelude).
/// The clock rate must be specified at compile time so that the `Instant` and
/// `Duration` types in
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
    const MIN_COMPARE_TICKS: Self::Count = 5;
}
impl RtcModeMonotonic for RtcMode1 {
    const HALF_PERIOD: Self::Count = 0x8000;
    const MIN_COMPARE_TICKS: Self::Count = 5;
}

#[hal_macro_helper]
mod mode0 {
    use super::*;
    use crate::rtc::modes::mode0::Compare0;
    #[hal_cfg("rtc-d5x")]
    use crate::rtc::modes::mode0::{Compare1, Overflow};

    // The SAMD11/21 chips do not feature a second compare in MODE0.
    #[hal_cfg(any("rtc-d11", "rtc-d21"))]
    crate::__internal_basic_backend!(RtcBackend, RtcMode0, 0, Compare0);
    #[hal_cfg("rtc-d5x")]
    crate::__internal_half_period_counting_backend!(
        RtcBackend, RtcMode0, 0, Compare0, Compare1, Overflow
    );
}

mod mode1 {
    use super::*;
    use crate::rtc::modes::mode1::{Compare0, Compare1, Overflow};

    crate::__internal_half_period_counting_backend!(
        RtcBackend, RtcMode1, 1, Compare0, Compare1, Overflow
    );
}

pub use mode0::RtcBackend as RtcMode0Backend;
pub use mode1::RtcBackend as RtcMode1Backend;

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

/// Create an RTIC v2 monotonic that uses the RTC in mode 0.
///
/// See the [`rtic`](crate::rtc::rtic) module for details.
#[macro_export]
macro_rules! rtc_mode0_monotonic {
    ($name:ident, $clock_rate: ty) => {
        $crate::__internal_create_rtc_struct!($name, RtcMode0Backend, $clock_rate);
    };
}

/// Create an RTIC v2 monotonic that uses the RTC in mode 1.
///
/// See the [`rtic`](crate::rtc::rtic) module for details.
#[macro_export]
macro_rules! rtc_mode1_monotonic {
    ($name:ident, $clock_rate: ty) => {
        $crate::__internal_create_rtc_struct!($name, RtcMode1Backend, $clock_rate);
    };
}

/// This function was copied from the private function in `rtic-monotonics`,
/// so that should be used when the monotonics move there.
const fn cortex_logical2hw(logical: u8, nvic_prio_bits: u8) -> u8 {
    ((1 << nvic_prio_bits) - logical) << (8 - nvic_prio_bits)
}

/// This function was copied from the private function in `rtic-monotonics`.
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
