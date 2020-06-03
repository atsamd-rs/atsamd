//! Delays with WFI sleep while we wait using a timer
use core::sync::atomic;
use cortex_m::asm;

use crate::time::U32Ext;
use crate::timer_traits::InterruptDrivenTimer;
use hal::blocking::delay::{DelayMs, DelayUs};

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

impl<TIM> DelayUs<u32> for SleepingDelay<TIM>
where
    TIM: InterruptDrivenTimer,
{
    fn delay_us(&mut self, us: u32) {
        self.timer.start((1_000_000 / us).hz());
        self.timer.enable_interrupt();
        loop {
            asm::wfi();
            if self.timer.wait().is_ok() || self.interrupt_fired.load(atomic::Ordering::Relaxed) {
                self.interrupt_fired.store(false, atomic::Ordering::Relaxed);
                break;
            }
        }
        self.timer.disable_interrupt();
    }
}

impl<TIM> DelayUs<u16> for SleepingDelay<TIM>
where
    TIM: InterruptDrivenTimer,
{
    fn delay_us(&mut self, us: u16) {
        self.delay_us(us as u32)
    }
}

impl<TIM> DelayUs<u8> for SleepingDelay<TIM>
where
    TIM: InterruptDrivenTimer,
{
    fn delay_us(&mut self, us: u8) {
        self.delay_us(us as u32)
    }
}

impl<TIM> DelayMs<u32> for SleepingDelay<TIM>
where
    TIM: InterruptDrivenTimer,
{
    fn delay_ms(&mut self, ms: u32) {
        self.delay_us(ms * 1_000);
    }
}

impl<TIM> DelayMs<u16> for SleepingDelay<TIM>
where
    TIM: InterruptDrivenTimer,
{
    fn delay_ms(&mut self, ms: u16) {
        self.delay_ms(ms as u32);
    }
}

impl<TIM> DelayMs<u8> for SleepingDelay<TIM>
where
    TIM: InterruptDrivenTimer,
{
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(ms as u32);
    }
}
