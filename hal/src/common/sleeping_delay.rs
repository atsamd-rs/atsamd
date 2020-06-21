//! Delays with WFI sleep while we wait using a timer
use core::sync::atomic;
use cortex_m::asm;

use crate::time::U32Ext;
use crate::timer_traits::InterruptDrivenTimer;
use hal::blocking::delay::{DelayMs, DelayUs};

const NUM_US_IN_S: u32 = 1_000_000;

/// Delay and sleep while we do (WFI) using a timer
pub struct SleepingDelay<TIM> {
    timer: TIM,
    interrupt_fired: &'static atomic::AtomicBool,
}

impl<TIM> SleepingDelay<TIM>
where
    TIM: InterruptDrivenTimer,
{
    /// Initializes a new SleepingDelay struct
    pub fn new(timer: TIM, interrupt_fired: &'static atomic::AtomicBool) -> SleepingDelay<TIM>
    where
        TIM: InterruptDrivenTimer,
    {
        SleepingDelay {
            timer,
            interrupt_fired,
        }
    }

    /// Releases the timer resource
    pub fn free(self) -> TIM {
        self.timer
    }
}

impl<TIM, TYPE> DelayUs<TYPE> for SleepingDelay<TIM>
where
    TIM: InterruptDrivenTimer,
    TYPE: Into<u32>,
{
    fn delay_us(&mut self, us: TYPE) {
        let mut us: u32 = us.into();
        if us == 0 {
            panic!("Invalid delay duration");
        }

        // Determine how many cycles we need to run for this delay, if any
        let mut count: u32 = 1 + us / NUM_US_IN_S;
        if count > 1 {
            us /= count - 1;
        }

        // Start the timer and sleep!
        self.timer.start((NUM_US_IN_S / us).hz());
        self.timer.enable_interrupt();
        loop {
            asm::wfi();
            if self.timer.wait().is_ok() || self.interrupt_fired.load(atomic::Ordering::Relaxed) {
                self.interrupt_fired.store(false, atomic::Ordering::Relaxed);
                count -= 1;
                if count == 0 {
                    break;
                }
            }
        }
        self.timer.disable_interrupt();
    }
}

impl<TIM, TYPE> DelayMs<TYPE> for SleepingDelay<TIM>
where
    TIM: InterruptDrivenTimer,
    TYPE: Into<u32>,
{
    fn delay_ms(&mut self, ms: TYPE) {
        self.delay_us(ms.into() * 1_000_u32);
    }
}
