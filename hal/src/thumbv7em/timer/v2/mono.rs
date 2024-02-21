use super::*;

pub trait IntoMonotonic<T: AbstractTimerId, I> {
    fn into_monotonic<const TIMER_HZ: u32>(
        self,
    ) -> Result<MonotonicTimer<TIMER_HZ, T, I>, TimerError>;
}

impl<T: CombinedTimerId, I, TW> IntoMonotonic<T, I> for TimerBuilder<T, I, TW> {
    fn into_monotonic<const TIMER_HZ: u32>(
        self,
    ) -> Result<MonotonicTimer<TIMER_HZ, T, I>, TimerError> {
        let raw = self
            .into_32_bit()
            .into_raw()
            .with_frequency(Hertz::Hz(TIMER_HZ))?
            .with_mode(TimerMode::NFRQ)
            .with_oneshot(false)
            .with_direction(TimerDirection::Increment)
            .with_interrupts(TimerInterrupt::Overflow.into());
        Ok(MonotonicTimer { raw, high_bits: 0 })
    }
}

pub struct MonotonicTimer<const TIMER_HZ: u32, T: AbstractTimerId, I, S = state::Disabled> {
    raw: RawTimer<T, I, u32, S>,
    high_bits: u32,
}

impl<const TIMER_HZ: u32, T: AbstractTimerId, I> MonotonicTimer<TIMER_HZ, T, I> {
    pub fn enable(self) -> MonotonicTimer<TIMER_HZ, T, I, state::Enabled> {
        MonotonicTimer {
            raw: self.raw.enable(),
            high_bits: self.high_bits,
        }
    }
}

impl<const TIMER_HZ: u32, T: AbstractTimerId, I> MonotonicTimer<TIMER_HZ, T, I, state::Enabled> {
    pub unsafe fn inner_raw(&mut self) -> &mut RawTimer<T, I, u32, state::Enabled> {
        &mut self.raw
    }

    pub fn disable(self) -> MonotonicTimer<TIMER_HZ, T, I> {
        MonotonicTimer {
            raw: self.raw.disable(),
            high_bits: self.high_bits,
        }
    }
}

impl<const TIMER_HZ: u32, T: AbstractTimerId, I, S> IntoRaw<T, I, u32, S>
    for MonotonicTimer<TIMER_HZ, T, I, S>
{
    fn into_raw(self) -> RawTimer<T, I, u32, S> {
        self.raw
    }
}

impl<const TIMER_HZ: u32, T: AbstractTimerId, I> rtic_monotonic::Monotonic
    for MonotonicTimer<TIMER_HZ, T, I, state::Enabled>
{
    const DISABLE_INTERRUPT_ON_EMPTY_QUEUE: bool = false;

    type Instant = fugit::TimerInstantU64<TIMER_HZ>;

    type Duration = fugit::TimerDurationU64<TIMER_HZ>;

    fn now(&mut self) -> Self::Instant {
        let ticks = u64::from(self.high_bits) << u32::BITS | u64::from(self.raw.count());
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
            let timer = &mut self.raw;
            timer.enable_interrupts(TimerInterrupt::MatchOrCaptureChannel0.into());
            timer.set_compare(TimerCompareRegister::Zero, instant.ticks() as u32);
        }
    }

    fn clear_compare_flag(&mut self) {
        let timer = &mut self.raw;
        let interrupts = timer.interrupt_flags();
        for interrupt in interrupts {
            match interrupt {
                TimerInterrupt::Overflow => self.high_bits += 1,
                TimerInterrupt::MatchOrCaptureChannel0 => {
                    timer.disable_interrupts(TimerInterrupt::MatchOrCaptureChannel0.into());
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
        self.raw.set_count(0);
    }
}
