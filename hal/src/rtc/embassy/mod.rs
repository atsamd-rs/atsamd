use core::cell::RefCell;
use core::task::Waker;

use cortex_m_semihosting::hprintln;
use crate::pac::{NVIC, interrupt, Rtc, Interrupt};
use crate::rtc::modes::{mode0::{RtcMode0, Compare0}, RtcMode};
use critical_section::{CriticalSection, Mutex};
use embassy_time_driver::Driver;
use embassy_time_queue_utils::Queue;

embassy_time_driver::time_driver_impl!(static DRIVER: AtmelDriver = AtmelDriver{
    queue: Mutex::new(RefCell::new(Queue::new()))
});

struct AtmelDriver {
    queue: Mutex<RefCell<Queue>>,
}

impl AtmelDriver {
    fn set_alarm(&self, _cs: &CriticalSection, at: u64, rtc: &Rtc) -> bool {
        // SAFETY: inside a CriticalSection
        let rtc = unsafe {
            Rtc::steal()
        };

        let at = match u32::try_from(at) {
            Ok(at) => at,
            Err(_) => return false,
        };

        RtcMode0::set_compare(&rtc, 0, at);
        true

    }
}

fn handle_interrupt() {
    hprintln!("interrupt handled");
    
    // safety: inside a critical section
    let rtc = unsafe {
        Rtc::steal()
    };
    if RtcMode0::check_interrupt_flag::<Compare0>(&rtc) {
        RtcMode0::clear_interrupt_flag::<Compare0>(&rtc);
        let now = RtcMode0::count(&rtc) as u64;

        critical_section::with(|cs| {
            let next = DRIVER.queue.borrow_ref_mut(cs).next_expiration(now);
            DRIVER.set_alarm(&cs, next, &rtc);
        });
    }
}

#[interrupt]
fn RTC() {
    handle_interrupt()
}

impl Driver for AtmelDriver {
    fn now(&self) -> u64 {
        critical_section::with(|cs| {
            let rtc = unsafe {
                Rtc::steal()
            };
            let now =  RtcMode0::count(&rtc) as u64;
            now
        })
    }

    fn schedule_wake(&self, at: u64, waker: &Waker) {
        critical_section::with(|cs| {
            let rtc = unsafe {
                Rtc::steal()
            };
            let mut queue = self.queue.borrow(cs).borrow_mut();
            if queue.schedule_wake(at, waker) {
                let next = queue.next_expiration(self.now());
                // We can only handle one alarm at a time right now
                self.set_alarm(&cs, next, &rtc);
            }
        });
    }
}

pub unsafe fn init() {
    // TODO: ensure CLK_RTC_APB is started in MCLK, and the prescaler is set to 1
    let rtc = Rtc::steal();
    RtcMode0::disable(&rtc);
    RtcMode0::reset(&rtc);
    RtcMode0::set_mode(&rtc);


    critical_section::with(|_| {
        RtcMode0::start_and_initialize(&rtc);
        RtcMode0::clear_interrupt_flag::<Compare0>(&rtc);
        RtcMode0::enable_interrupt::<Compare0>(&rtc);
        rtc.mode0().evctrl().write(|w| w.cmpeo0().set_bit());

        unsafe {
            let mut nvic: cortex_m::peripheral::NVIC = core::mem::transmute(());
            nvic.set_priority(Interrupt::RTC, 128);
            NVIC::unmask(Interrupt::RTC);
        }

        RtcMode0::enable(&rtc);
    });
}
