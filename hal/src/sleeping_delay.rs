//! Delays with WFI sleep while we wait using a timer
use core::sync::atomic;
use cortex_m::asm;
use fugit::ExtU32;

use crate::ehal::delay::DelayNs;
use crate::ehal_02;
use crate::timer_traits::InterruptDrivenTimer;

const NUM_NS_IN_S: u32 = 1_000_000_000;

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

impl<TIM, TYPE> ehal_02::blocking::delay::DelayUs<TYPE> for SleepingDelay<TIM>
where
    TIM: InterruptDrivenTimer,
    TYPE: Into<u32>,
{
    fn delay_us(&mut self, us: TYPE) {
        <Self as DelayNs>::delay_us(self, us.into());
    }
}

impl<TIM, TYPE> ehal_02::blocking::delay::DelayMs<TYPE> for SleepingDelay<TIM>
where
    TIM: InterruptDrivenTimer,
    TYPE: Into<u32>,
{
    fn delay_ms(&mut self, ms: TYPE) {
        <Self as DelayNs>::delay_ms(self, ms.into());
    }
}

impl<TIM: InterruptDrivenTimer> DelayNs for SleepingDelay<TIM> {
    fn delay_ns(&mut self, ns: u32) {
        // Determine how many cycles we need to run for this delay, if any
        // Avoid timers that run longer than a second because for 48 MHz-based timers,
        //   there is no valid divisor + cycle count greater than ~1.3s, so we'd panic.
        let mut loop_counter: u32 = 1 + (ns / NUM_NS_IN_S);

        // Start the timer and sleep!
        self.timer.start((ns / loop_counter).nanos());
        self.timer.enable_interrupt();
        loop {
            asm::wfi();
            if self.timer.wait().is_ok() || self.interrupt_fired.load(atomic::Ordering::Relaxed) {
                self.interrupt_fired.store(false, atomic::Ordering::Relaxed);
                loop_counter -= 1;
                if loop_counter == 0 {
                    break;
                }
            }
        }
        self.timer.disable_interrupt();
    }
}
