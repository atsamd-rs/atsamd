use super::*;

pub trait IntoCountdown<T: AbstractTimerId, I, TW> {
    fn into_countdown<const TIMER_HZ: u32>(
        self,
    ) -> Result<UnreadyCountdownTimer<TIMER_HZ, T, I, TW>, TimerError>;
}

impl<T: AbstractTimerId, I, TW: timer_width::TimerWidth> IntoCountdown<T, I, TW>
    for TimerBuilder<T, I, TW>
{
    fn into_countdown<const TIMER_HZ: u32>(
        self,
    ) -> Result<UnreadyCountdownTimer<TIMER_HZ, T, I, TW>, TimerError> {
        let raw = self
            .into_raw()
            .with_frequency(Hertz::Hz(TIMER_HZ))?
            .with_mode(TimerMode::MFRQ)
            .with_direction(TimerDirection::Decrement)
            .with_interrupts(TimerInterruptSet::empty());
        Ok(UnreadyCountdownTimer(CountdownTimer { raw }))
    }
}

pub struct UnreadyCountdownTimer<const TIMER_HZ: u32, T: AbstractTimerId, I, TW>(
    CountdownTimer<TIMER_HZ, T, I, TW>,
);

impl<const TIMER_HZ: u32, T: AbstractTimerId, I, TW: timer_width::TimerWidth>
    UnreadyCountdownTimer<TIMER_HZ, T, I, TW>
{
    pub fn with_period(
        mut self,
        period: fugit::TimerDurationU32<TIMER_HZ>,
    ) -> Result<CountdownTimer<TIMER_HZ, T, I, TW>, CountdownTimerError> {
        self.0.set_period(period)?;
        Ok(self.0)
    }

    pub fn with_saturated_period(
        mut self,
        period: fugit::TimerDurationU32<TIMER_HZ>,
    ) -> CountdownTimer<TIMER_HZ, T, I, TW> {
        self.0.set_saturated_period(period);
        self.0
    }
}

pub struct CountdownTimer<const TIMER_HZ: u32, T: AbstractTimerId, I, TW, S = state::Disabled> {
    raw: RawTimer<T, I, TW, S>,
}

#[derive(Copy, Clone, Debug)]
pub enum CountdownTimerError {
    OutOfRange,
}

impl<const TIMER_HZ: u32, T: AbstractTimerId, I, TW: timer_width::TimerWidth>
    CountdownTimer<TIMER_HZ, T, I, TW>
{
    pub fn enable(self) -> CountdownTimer<TIMER_HZ, T, I, TW, state::Enabled> {
        CountdownTimer {
            raw: self.raw.enable(),
        }
    }

    pub fn with_oneshot(mut self, value: bool) -> Self {
        self.set_oneshot(value);
        self
    }

    pub fn with_interrupt(mut self) -> Self {
        self.enable_interrupt();
        self
    }
}

impl<const TIMER_HZ: u32, T: AbstractTimerId, I, TW: timer_width::TimerWidth>
    CountdownTimer<TIMER_HZ, T, I, TW, state::Enabled>
{
    pub fn disable(self) -> CountdownTimer<TIMER_HZ, T, I, TW> {
        CountdownTimer {
            raw: self.raw.disable(),
        }
    }

    pub fn retrigger(&mut self) -> &mut Self {
        self.raw.retrigger();
        self
    }

    pub fn wait(&mut self) -> nb::Result<(), Infallible> {
        if self.raw.interrupt_flags().overflow() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    pub fn clear_interrupt_flag(&mut self) -> &mut Self {
        let _ = self.raw.interrupt_flags();
        self
    }
}

impl<const TIMER_HZ: u32, T: AbstractTimerId, I, TW: timer_width::TimerWidth, S>
    CountdownTimer<TIMER_HZ, T, I, TW, S>
{
    // TODO: set_period -> try_set_period ? Provide guaranteed `set_period` for u32
    // timer Or maybe just set_saturating_period to avoid all that hassle
    pub fn set_period(
        &mut self,
        period: fugit::TimerDurationU32<TIMER_HZ>,
    ) -> Result<&mut Self, CountdownTimerError> {
        match TW::from_u32(period.ticks()) {
            Some(ticks) => {
                self.raw.set_compare(TimerCompareRegister::Zero, ticks);
                self.raw.set_count(ticks);
                Ok(self)
            }
            None => Err(CountdownTimerError::OutOfRange),
        }
    }

    pub fn set_saturated_period(&mut self, period: fugit::TimerDurationU32<TIMER_HZ>) -> &mut Self {
        let period = match TW::from_u32(period.ticks() - 1) {
            Some(ticks) => ticks,
            None => TW::max_value(),
        };
        self.raw.set_compare(TimerCompareRegister::Zero, period);
        self.raw.set_count(period);
        self
    }

    pub fn set_oneshot(&mut self, value: bool) -> &mut Self {
        self.raw.set_oneshot(value);
        self
    }

    pub fn enable_interrupt(&mut self) -> &mut Self {
        self.raw.enable_interrupts(TimerInterrupt::Overflow.into());
        self
    }

    pub fn disable_interrupt(&mut self) -> &mut Self {
        self.raw.disable_interrupts(TimerInterrupt::Overflow.into());
        self
    }

    // TODO: Clean it up
    /// # Safety
    /// API of the `CountdownTimer` full ownership over underlying timer and
    /// makes assumptions about its state
    /// - direction: decrement
    /// - interrupts: all but overflow should be disabled; overflow is _do not
    ///   care_
    /// - mode: mfrq (must wrap on CC0, NOT on TOP)
    /// - frequency: matches TIMER_HZ
    pub unsafe fn inner_raw(&mut self) -> &mut RawTimer<T, I, TW, S> {
        &mut self.raw
    }
}

impl<const TIMER_HZ: u32, T: AbstractTimerId, I, TW, S> IntoRaw<T, I, TW, S>
    for CountdownTimer<TIMER_HZ, T, I, TW, S>
{
    fn into_raw(self) -> RawTimer<T, I, TW, S> {
        self.raw
    }
}
