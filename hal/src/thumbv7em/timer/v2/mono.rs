use super::*;

impl<T: AbstractTimerId, I> Timer<T, I, u32> {
    pub fn into_monotonic<const TIMER_HZ: u32>(
        self,
    ) -> Result<MonotonicTimer<TIMER_HZ, T, I>, TimerError> {
        MonotonicTimer::enable(self)
    }
}

pub struct MonotonicTimer<const TIMER_HZ: u32, T: AbstractTimerId, I> {
    inner_timer: Timer<T, I, u32, state::Enabled>,
    high_bits: u32,
}

impl<const TIMER_HZ: u32, T: AbstractTimerId, I> MonotonicTimer<TIMER_HZ, T, I> {
    fn enable(inner: Timer<T, I, u32>) -> Result<Self, TimerError> {
        let inner_timer = inner
            .with_frequency(Hertz::Hz(TIMER_HZ))?
            .with_direction(TimerDirection::Increment)
            .with_interrupts(Interrupt::Overflow.into())
            .enable();
        Ok(Self {
            inner_timer,
            high_bits: 0,
        })
    }

    pub fn disable(self) -> Timer<T, I, u32> {
        self.inner_timer.disable()
    }
}

impl<const TIMER_HZ: u32, T: AbstractTimerId, I> rtic_monotonic::Monotonic
    for MonotonicTimer<TIMER_HZ, T, I>
{
    const DISABLE_INTERRUPT_ON_EMPTY_QUEUE: bool = false;

    type Instant = fugit::TimerInstantU64<TIMER_HZ>;

    type Duration = fugit::TimerDurationU64<TIMER_HZ>;

    fn now(&mut self) -> Self::Instant {
        let ticks = u64::from(self.high_bits) << u32::BITS | u64::from(self.inner_timer.count());
        Self::Instant::from_ticks(ticks)
    }

    fn set_compare(&mut self, instant: Self::Instant) {
        // Set comparator value and enable related interrupt only if requested `instant`
        // is currently in range for a timer (smaller than u32::MAX). `set_compare` will
        // be attempted every time timer overflows until it is finally in range.
        if instant
            .checked_duration_since(self.now())
            .map_or(0, |d| d.ticks())
            <= u32::MAX.into()
        {
            let timer = &mut self.inner_timer;
            timer.enable_interrupts(Interrupt::MatchOrCaptureChannel0.into());
            timer.set_compare(TimerCompareRegister::Zero, instant.ticks() as u32);
        }
    }

    fn clear_compare_flag(&mut self) {
        let timer = &mut self.inner_timer;
        let interrupts = timer.interrupt_flags();
        for interrupt in interrupts {
            match interrupt {
                Interrupt::Overflow => self.high_bits += 1,
                Interrupt::MatchOrCaptureChannel0 => {
                    timer.disable_interrupts(Interrupt::MatchOrCaptureChannel0.into());
                    timer.set_compare(TimerCompareRegister::Zero, 0);
                }
                _ => {}
            }
        }
    }

    fn zero() -> Self::Instant {
        Self::Instant::from_ticks(0)
    }

    unsafe fn reset(&mut self) {
        self.high_bits = 0;
        self.inner_timer.set_count(0);
    }
}
