use super::*;

pub trait IntoRaw<T: AbstractTimerId, I, TW, S> {
    fn into_raw(self) -> RawTimer<T, I, TW, S>;
}

// TODO: These `pub(super)` are somewhat smelly; especially `reg` and `to`
// Maybe `Builder` should have access to a better API inside Timer, maybe at
// least unsafe Or maybe they should not be sibling modules, if builder needs to
// access raw, then maybe builder should be just submodule of raw.

/// General timer abstraction. WiP.
pub struct RawTimer<T: AbstractTimerId, I, TW = u16, S = state::Disabled> {
    pub(super) reg: T::Reg, // pub(super) for the builder
    pclk_freq: Hertz,
    __: PhantomData<(I, S, TW)>,
}

impl<T: AbstractTimerId, I> RawTimer<T, I> {
    // pub(super) for the builder
    pub(super) fn new(reg: T::Reg, pclk_freq: Hertz) -> Self {
        reg.count8().ctrla.modify(|_, w| w.swrst().set_bit());
        while reg.count8().syncbusy.read().swrst().bit_is_set() {}
        let mut s = Self {
            reg,
            pclk_freq,
            __: PhantomData,
        };
        s.set_debug_run(false); // Not controlled by SWRST, setting explicitly
        s
    }
}

impl<T: AbstractTimerId, I, TW, S> RawTimer<T, I, TW, S> {
    // pub(super) for the builder
    pub(super) fn to<TW2, S2>(self) -> RawTimer<T, I, TW2, S2> {
        RawTimer {
            reg: self.reg,
            pclk_freq: self.pclk_freq,
            __: PhantomData,
        }
    }

    fn sync_ctrlbset_write(&mut self, write: impl WriteClosure<CtrlBSet>) {
        self.reg.count8().ctrlbset.write(write);
        while self.reg.count8().syncbusy.read().ctrlb().bit_is_set() {}
    }

    fn sync_ctrlbclr_write(&mut self, write: impl WriteClosure<CtrlBClr>) {
        self.reg.count8().ctrlbclr.write(write);
        while self.reg.count8().syncbusy.read().ctrlb().bit_is_set() {}
    }

    pub fn set_debug_run(&mut self, value: bool) -> &mut Self {
        self.reg.count8().dbgctrl.write(|w| w.dbgrun().bit(value));
        self
    }

    // To re-engage the timer one must call [`Timer::retrigger`]
    pub fn set_oneshot(&mut self, value: bool) -> &mut Self {
        if value {
            self.sync_ctrlbset_write(|w| w.oneshot().set_bit());
        } else {
            self.sync_ctrlbclr_write(|w| w.oneshot().set_bit());
        }
        self
    }

    pub fn set_ondemand(&mut self, value: bool) -> &mut Self {
        self.reg
            .count8()
            .ctrla
            .modify(|_, w| w.ondemand().bit(value));
        self
    }

    pub fn set_runstdby(&mut self, value: bool) -> &mut Self {
        self.reg
            .count8()
            .ctrla
            .modify(|_, w| w.runstdby().bit(value));
        self
    }

    pub fn set_direction(&mut self, dir: TimerDirection) -> &mut Self {
        match dir {
            TimerDirection::Increment => self.sync_ctrlbclr_write(|w| w.dir().set_bit()),
            TimerDirection::Decrement => self.sync_ctrlbset_write(|w| w.dir().set_bit()),
        }
        self
    }

    pub fn direction(&self) -> TimerDirection {
        match self.reg.count8().ctrlbset.read().dir().bit_is_set() {
            true => TimerDirection::Decrement,
            false => TimerDirection::Increment,
        }
    }

    pub fn enable_interrupts(&mut self, interrupt_set: TimerInterruptSet) -> &mut Self {
        self.reg
            .count8()
            .intenset
            .write(|w| unsafe { w.bits(interrupt_set.0) });
        self
    }

    pub fn disable_interrupts(&mut self, interrupt_set: TimerInterruptSet) -> &mut Self {
        self.reg
            .count8()
            .intenclr
            .write(|w| unsafe { w.bits(interrupt_set.0) });
        self
    }

    pub unsafe fn registers(&mut self) -> &T::Reg {
        &self.reg
    }

    fn sync_run_command(&mut self, cmd: TimerCommand) -> &mut Self {
        self.sync_ctrlbset_write(|w| w.cmd().variant(cmd));
        while !self.reg.count8().ctrlbset.read().cmd().is_none() {}
        self
    }
}

impl<T: AbstractTimerId, I, TW> RawTimer<T, I, TW> {
    // TODO: Restoring the builder is dangerous if new configuration in
    // "raw" mode violated some invariant that other "specific timer"
    // might have made.
    pub unsafe fn into_builder(self) -> TimerBuilder<T, I, TW> {
        TimerBuilder::new(self)
    }

    pub fn reset_into_builder(self) -> TimerBuilder<T, I> {
        TimerBuilder::new(RawTimer::new(self.reg, self.pclk_freq))
    }

    pub fn enable(self) -> RawTimer<T, I, TW, state::Enabled> {
        self.reg.count8().ctrla.modify(|_, w| w.enable().set_bit());
        while self.reg.count8().syncbusy.read().enable().bit_is_set() {}
        self.to()
    }

    pub fn with_ondemand(mut self, value: bool) -> Self {
        self.set_ondemand(value);
        self
    }

    pub fn with_runstdby(mut self, value: bool) -> Self {
        self.set_runstdby(value);
        self
    }

    pub fn with_oneshot(mut self, value: bool) -> Self {
        self.set_oneshot(value);
        self
    }

    pub fn with_direction(mut self, dir: TimerDirection) -> Self {
        self.set_direction(dir);
        self
    }

    /// Disables all interrupts first
    pub fn with_interrupts(mut self, interrupt_set: TimerInterruptSet) -> Self {
        // TODO: If monotonic/countdown decays into raw then we should disable
        // interrupts first so it's more idempotent But then, maybe
        // monotonic/countdown should do the same? :thinking:
        // self.disable_interrupts(InterruptSet::full());
        self.enable_interrupts(interrupt_set);
        self
    }

    /// Mutually exclusive with [`Self::with_prescaler`]
    pub fn with_frequency(self, frequency: Hertz) -> Result<Self, TimerError> {
        let input_freq = self.pclk_freq.to_Hz();
        let output_freq = frequency.to_Hz();
        let prescaler = match input_freq.checked_rem(output_freq) {
            Some(0) => match input_freq / output_freq {
                1 => TimerPrescaler::DIV1,
                2 => TimerPrescaler::DIV2,
                4 => TimerPrescaler::DIV4,
                8 => TimerPrescaler::DIV8,
                16 => TimerPrescaler::DIV16,
                64 => TimerPrescaler::DIV64,
                256 => TimerPrescaler::DIV256,
                1024 => TimerPrescaler::DIV1024,
                _ => {
                    return Err(TimerError::NoValidPrescaler(
                        TimerPrescalerError::InvalidPrescaler,
                    ))
                }
            },
            Some(_) => {
                return Err(TimerError::NoValidPrescaler(
                    TimerPrescalerError::FrequenciesNotDivisible,
                ))
            }
            None => {
                return Err(TimerError::NoValidPrescaler(
                    TimerPrescalerError::OutputFrequencyIsZero,
                ))
            }
        };
        Ok(self.with_prescaler(prescaler))
    }

    /// Mutually exclusive with [`Self::with_frequency`]
    pub fn with_prescaler(self, prescaler: TimerPrescaler) -> Self {
        self.reg
            .count8()
            .ctrla
            .modify(|_, w| w.prescaler().variant(prescaler));
        self
    }

    pub fn with_mode(self, mode: TimerMode) -> Self {
        self.reg.count8().wave.write(|w| w.wavegen().variant(mode));
        self
    }
}

impl<T: AbstractTimerId, I, TW> RawTimer<T, I, TW, state::Enabled> {
    pub fn disable(self) -> RawTimer<T, I, TW> {
        self.reg.count8().ctrla.modify(|_, w| w.enable().set_bit());
        while self.reg.count8().syncbusy.read().enable().bit_is_set() {}
        self.to()
    }

    pub fn retrigger(&mut self) {
        self.sync_run_command(TimerCommand::RETRIGGER);
    }

    // Clears the interrupts
    #[must_use]
    pub fn interrupt_flags(&mut self) -> TimerInterruptSet {
        let interrupts = self.reg.count8().intflag.read().bits();
        // Clear the interrupts
        self.reg
            .count8()
            .intflag
            .write(|w| unsafe { w.bits(interrupts) });
        TimerInterruptSet(interrupts)
    }
}

// TODO: Make sure that count && CCs can be interacted with when timer is
// disabled as well as when enabled Especially if `sync_run_command` is fine
// when disabled
impl<T: AbstractTimerId, I, TW: timer_width::TimerWidth, S> RawTimer<T, I, TW, S> {
    pub fn compare(&self, index: TimerCompareRegister) -> TW {
        // Compare register value must be <= TW::MAX, this is noop
        TW::truncated_from_u32(self.reg.count32().cc[index as usize].read().cc().bits())
    }

    pub fn set_compare(&mut self, index: TimerCompareRegister, compare: TW) -> &mut Self {
        // Writing over reserved bits for an 8/16 bit timer, o-la-la.
        // Hopefully that's fine.
        self.reg.count32().cc[index as usize].write(|w| unsafe { w.cc().bits(compare.as_()) });
        match index {
            TimerCompareRegister::Zero => {
                while self.reg.count8().syncbusy.read().cc0().bit_is_set() {}
            }
            TimerCompareRegister::One => {
                while self.reg.count8().syncbusy.read().cc1().bit_is_set() {}
            }
        }
        self
    }

    pub fn count(&mut self) -> TW {
        self.sync_run_command(TimerCommand::READSYNC);
        // Count value must be <= TW::MAX, this is noop
        TW::truncated_from_u32(self.reg.count32().count.read().count().bits())
    }

    pub fn set_count(&mut self, count: TW) -> &mut Self {
        // Writing over reserved bits for an 8/16 bit timer, o-la-la.
        // Hopefully that's fine.
        self.reg
            .count32()
            .count
            .write(|w| unsafe { w.count().bits(count.as_()) });
        while self.reg.count8().syncbusy.read().count().bit_is_set() {}
        self
    }
}

impl<T: PrimaryTimerId, I: PclkSourceId, TW1, TW2> FreeTimerResourcesExt<T, I>
    for (RawTimer<T, I, TW1>, RawTimer<SecondaryTimer<T>, I, TW2>)
{
    fn free(
        self,
    ) -> (
        T::Reg,
        ApbClk<T>,
        Reg<SecondaryTimer<T>>,
        ApbClk<SecondaryTimer<T>>,
        Pclk<T::CombinedTimer, I>,
    ) {
        (
            self.0.reg,
            ApbClk::new(unsafe { ApbToken::new() }),
            self.1.reg,
            ApbClk::new(unsafe { ApbToken::new() }),
            Pclk::new(unsafe { PclkToken::new() }, self.0.pclk_freq),
        )
    }
}

impl<T: CombinedTimerId, I: PclkSourceId, TW> FreeTimerResourcesExt<T::PrimaryTimer, I>
    for RawTimer<T, I, TW>
where
    T::Reg: Into<(Reg<T::PrimaryTimer>, Reg<T::SecondaryTimer>)>,
{
    fn free(
        self,
    ) -> (
        Reg<T::PrimaryTimer>,
        ApbClk<T::PrimaryTimer>,
        Reg<T::SecondaryTimer>,
        ApbClk<T::SecondaryTimer>,
        Pclk<T, I>,
    ) {
        let (pri_reg, sec_reg) = self.reg.into();
        (
            pri_reg,
            ApbClk::new(unsafe { ApbToken::new() }),
            sec_reg,
            ApbClk::new(unsafe { ApbToken::new() }),
            Pclk::new(unsafe { PclkToken::new() }, self.pclk_freq),
        )
    }
}
