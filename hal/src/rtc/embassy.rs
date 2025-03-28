//! This is a driver for embassy-time
//! It must be instantiated with embassy_time!()
//! which will configure the chip's RTC

use core::cell::RefCell;
use core::task::Waker;

use crate::pac::{Interrupt, Rtc, NVIC};
use crate::rtc::modes::{
    mode0::{Compare0, RtcMode0},
    RtcMode,
};
use atsamd_hal_macros::hal_cfg;
use critical_section::{CriticalSection, Mutex};
use embassy_time_driver::Driver;
use embassy_time_queue_utils::Queue;

/// Used internally by the embassy time driver.
/// You shouldn't need this
pub struct EmbassyBackend {
    queue: Mutex<RefCell<Queue>>,
}

impl EmbassyBackend {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            queue: Mutex::new(RefCell::new(Queue::new())),
        }
    }

    fn set_alarm(&self, _cs: &CriticalSection, at: u64, rtc: &Rtc) -> bool {
        // Embassy uses u64::MAX as a "no upcoming interrupt" sentinel
        let at = match u32::try_from(at) {
            Ok(at) => at,
            _ if at == u64::MAX => u32::MAX,
            Err(_) => return false,
        };

        RtcMode0::set_compare(rtc, 0, at);
        true
    }

    pub fn handle_interrupt(&self, rtc: &Rtc, cs: CriticalSection) {
        if RtcMode0::check_interrupt_flag::<Compare0>(rtc) {
            // Due to synchronization delay, the RTC may be slightly behind
            // Assume the current time is the time the interrupt is set for
            let now = RtcMode0::get_compare(rtc, 0) as u64;
            let next = self.queue.borrow_ref_mut(cs).next_expiration(now);
            self.set_alarm(&cs, next, rtc);
            RtcMode0::clear_interrupt_flag::<Compare0>(rtc);
        }
    }

    /// # Safety
    ///
    /// This enables interrupts, which can break out of critical sections
    pub unsafe fn init() {
        let rtc = Rtc::steal();

        RtcMode0::disable(&rtc);
        RtcMode0::reset(&rtc);
        RtcMode0::set_mode(&rtc);

        RtcMode0::start_and_initialize(&rtc);
        RtcMode0::clear_interrupt_flag::<Compare0>(&rtc);
        RtcMode0::enable_interrupt::<Compare0>(&rtc);

        RtcMode0::enable(&rtc);
        unsafe {
            cortex_m::interrupt::enable();
            let mut nvic: cortex_m::peripheral::NVIC = core::mem::transmute(());
            nvic.set_priority(Interrupt::RTC, 1);
            NVIC::unmask(Interrupt::RTC);
        }
    }
}

impl Driver for EmbassyBackend {
    fn now(&self) -> u64 {
        critical_section::with(|_cs| {
            let rtc = unsafe { Rtc::steal() };
            RtcMode0::count(&rtc) as u64
        })
    }

    fn schedule_wake(&self, at: u64, waker: &Waker) {
        critical_section::with(|cs| {
            let rtc = unsafe { Rtc::steal() };
            let mut queue = self.queue.borrow(cs).borrow_mut();
            if queue.schedule_wake(at, waker) {
                let next = queue.next_expiration(self.now());
                // We can only handle one alarm at a time right now
                self.set_alarm(&cs, next, &rtc);
            }
        });
    }
}

pub trait EmbassyRtcSource {}

#[hal_cfg(any("clock-d11", "clock-d21"))]
mod rtc_src {
    impl super::EmbassyRtcSource for crate::clock::RtcClock {}
}

#[hal_cfg("clock-d5x")]
mod rtc_src {
    use crate::clock::v2::{osculp32k::OscUlp32kId, rtcosc::RtcOsc, xosc32k::Xosc32kId};
    impl super::EmbassyRtcSource for RtcOsc<OscUlp32kId> {}
    impl super::EmbassyRtcSource for RtcOsc<Xosc32kId> {}
}

#[allow(unused_imports)]
use rtc_src::*;

/// Create an embassy-time compliant driver
/// This driver should be called outside any function
/// The driver must be started by calling init() on the created struct
/// with an RTC token to indicate proper configuration
/// ```invalid
/// rtc::embassy::embassy_time!(Driver);
///
/// #[embassy_executor::main]
/// async fn main(_s: embassy_executor::Spawner) {
///     let rtc_token = set_up_clock!();
///     /// Safety: called outside a critical section
///     unsafe {
///       Driver::init(rtc_token);
///     }
/// }
/// ```
#[macro_export]
macro_rules! embassy_time {
    ($name: ident) => {

        use $crate::pac::interrupt;
        use $crate::{embassy_time_driver, rtc::embassy::{EmbassyBackend, EmbassyRtcSource}};

        embassy_time_driver::time_driver_impl!(static DRIVER: EmbassyBackend = EmbassyBackend::new());

        #[$crate::pac::interrupt]
        fn RTC() {
            critical_section::with(|cs| {
                let rtc = unsafe { $crate::pac::Rtc::steal() };
                DRIVER.handle_interrupt(&rtc, cs)
            });
        }

        pub struct $name;

        impl $name {
            unsafe fn init<SRC>(_rtc: SRC) where SRC: EmbassyRtcSource {
                EmbassyBackend::init();
            }
        }
    }
}
