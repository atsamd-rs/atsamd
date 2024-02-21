use super::*;

pub struct TimerBuilder<T: AbstractTimerId, I, TW = u16>(RawTimer<T, I, TW>);

impl<T: AbstractTimerId, I, TW> TimerBuilder<T, I, TW> {
    pub(super) fn new(inner: RawTimer<T, I, TW>) -> Self {
        Self(inner)
    }

    pub fn into_8_bit(self) -> TimerBuilder<T, I, u8> {
        self.0.reg.count8().ctrla.modify(|_, w| w.mode().count8());
        TimerBuilder(self.0.to())
    }

    pub fn into_16_bit(self) -> TimerBuilder<T, I, u16> {
        self.0.reg.count8().ctrla.modify(|_, w| w.mode().count16());
        TimerBuilder(self.0.to())
    }

    pub fn with_ondemand(mut self, value: bool) -> Self {
        self.0.set_ondemand(value);
        self
    }

    pub fn with_runstdby(mut self, value: bool) -> Self {
        self.0.set_runstdby(value);
        self
    }

    pub fn with_debug_run(mut self, value: bool) -> Self {
        self.0.set_debug_run(value);
        self
    }
}

impl<T: AbstractTimerId, I, TW> IntoRaw<T, I, TW, state::Disabled> for TimerBuilder<T, I, TW> {
    fn into_raw(self) -> RawTimer<T, I, TW> {
        self.0
    }
}

impl<T: CombinedTimerId, I, TW> TimerBuilder<T, I, TW> {
    pub fn into_32_bit(self) -> TimerBuilder<T, I, u32> {
        self.0.reg.count8().ctrla.modify(|_, w| w.mode().count32());
        TimerBuilder(self.0.to())
    }
}

impl<T: CombinedTimerId, I: PclkSourceId, TW> FreeTimerResourcesExt<T::PrimaryTimer, I>
    for TimerBuilder<T, I, TW>
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
        self.0.free()
    }
}

impl<T: PrimaryTimerId, I: PclkSourceId, TW1, TW2> FreeTimerResourcesExt<T, I>
    for (
        TimerBuilder<T, I, TW1>,
        TimerBuilder<SecondaryTimer<T>, I, TW2>,
    )
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
        (self.0 .0, self.1 .0).free()
    }
}
