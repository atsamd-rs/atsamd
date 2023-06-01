//! # Version 2 of the `timer` module
//!
//! High level abstraction for TC (Timer/Counter) peripherals.
//!
//! Available features:
//! - Typesafe construction of all valid combination of timers
//! - Exposed configuration options like frequency/prescaler, direction,
//!   interrupts etc.
//! - `rtic_monotonic::Monotonic` implementation for 32-bit configured timers
//! - Two compare registers (CC0 and CC1) that can trigger an interrupt on match
//!   with a count
//!
//! Currently there is no support for counting events (only clock sources
//! through `Pclk`), capture operations and timer operates in a default, NFRQ
//! mode (normal frequency generation).
//!
//! Because of their structure, timers can be constructed through
//! - [`Timer::disjoint`] which yields a set of two timers
//! limited to 8 and 16 bit counting resolution.
//! - [`Timer::paired`] which yields a single timer capable of 8, 16 and 32 bit
//!   counting resolution.
//!
//! After construction and configuration, timer has to be enabled.
//!
//! Example of how to construct, configure and enable a timer.
//!
//! ```no_run
//! # use atsamd_hal::clock::v2::pclk::Pclk;
//! # use atsamd_hal::timer::v2::{Timer, TimersTupleExt, TimerPrescaler, TimerDirection};
//! # let mut pac = unsafe { atsamd_hal::pac::Peripherals::steal() };
//! let (mut buses, clocks, tokens) = atsamd_hal::clock::v2::clock_system_at_reset(
//!     pac.OSCCTRL,
//!     pac.OSC32KCTRL,
//!     pac.GCLK,
//!     pac.MCLK,
//!     &mut pac.NVMCTRL,
//! );
//! let gclk0 = clocks.gclk0;
//!
//! let tc0 = pac.TC0;
//! let tc1 = pac.TC1;
//! let tc2 = pac.TC2;
//! let tc3 = pac.TC3;
//!
//! let tc0_clk = buses.apb.enable(tokens.apbs.tc0);
//! let tc1_clk = buses.apb.enable(tokens.apbs.tc1);
//! let tc2_clk = buses.apb.enable(tokens.apbs.tc2);
//! let tc3_clk = buses.apb.enable(tokens.apbs.tc3);
//!
//! let (tc0_tc1_pclk, gclk0) = Pclk::enable(tokens.pclks.tc0_tc1, gclk0);
//! let (tc2_tc3_pclk, gclk0) = Pclk::enable(tokens.pclks.tc2_tc3, gclk0);
//!
//! let (pri, sec) = Timer::disjoint(tc0, tc0_clk, tc1, tc1_clk, tc0_tc1_pclk) /* -> (Timer<Tc0>, Timer<Tc1>) */;
//! let (tc0, tc0_clk, tc1, tc1_clk, tc0_tc1_pclk) = (pri, sec).free();
//! let timer = Timer::paired(tc0, tc0_clk, tc1, tc1_clk, tc0_tc1_pclk) /* -> Timer<Tc0Tc1> */;
//! let timer = timer
//!               .with_prescaler(TimerPrescaler::DIV16)
//!               .with_oneshot(true)
//!               .with_direction(TimerDirection::Increment)
//!               .enable();
//! let timer = timer.disable();
//! let (tc0, tc0_clk, tc1, tc1_clk, tc0_tc1_pclk) = timer.free();
//!
//! let (pri, sec) = Timer::disjoint(tc2, tc2_clk, tc3, tc3_clk, tc2_tc3_pclk) /* -> (Timer<Tc2>, Timer<Tc3>) */;
//! let (tc2, tc2_clk, tc3, tc3_clk, tc2_tc3_pclk) = (pri, sec).free();
//! let timer = Timer::paired(tc2, tc2_clk, tc3, tc3_clk, tc2_tc3_pclk) /* -> Timer<Tc2Tc3> */;
//! let timer = timer.into_32_bit().enable().disable();
//! let (tc2, tc2_clk, tc3, tc3_clk, tc2_tc3_pclk) = timer.free();
//! ```
//!
//! In order to use the timer as a RTIC monotonic timer, one can call
//! `Timer::into_monotonic`. This requires a feature flag `rtic` being enabled.

use core::marker::PhantomData;

#[cfg(feature = "rtic")]
mod mono;
#[cfg(feature = "rtic")]
pub use mono::MonotonicTimer;

mod interrupt;
pub use interrupt::{Interrupt, InterruptSet};

mod types;
pub use types::*;

use crate::{
    clock::v2::{
        apb::{ApbClk, ApbToken},
        pclk::{Pclk, PclkSourceId, PclkToken},
    },
    time::Hertz,
};

/// General timer abstraction. WiP.
pub struct Timer<T: AbstractTimerId, I, TW = u16, S = state::Disabled> {
    reg: T::Reg,
    pclk_freq: Hertz,
    __: PhantomData<(I, S, TW)>,
}

impl<T: AbstractTimerId, I> Timer<T, I> {
    fn new(reg: T::Reg, pclk_freq: Hertz) -> Self {
        reg.count8().ctrla.modify(|_, w| w.swrst().set_bit());
        while reg.count8().syncbusy.read().swrst().bit_is_set() {}
        Self {
            reg,
            pclk_freq,
            __: PhantomData,
        }
    }
}

impl<T: AbstractTimerId, I, TW, S> Timer<T, I, TW, S> {
    fn to<TW2, S2>(self) -> Timer<T, I, TW2, S2> {
        Timer {
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

    pub fn set_oneshot(&mut self, value: bool) -> &mut Self {
        if value {
            self.sync_ctrlbset_write(|w| w.oneshot().set_bit());
        } else {
            self.sync_ctrlbclr_write(|w| w.oneshot().set_bit());
        }
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

    pub fn enable_interrupts(&mut self, interrupt_set: InterruptSet) -> &mut Self {
        self.reg
            .count8()
            .intenset
            .write(|w| unsafe { w.bits(interrupt_set.0) });
        self
    }

    pub fn disable_interrupts(&mut self, interrupt_set: InterruptSet) -> &mut Self {
        self.reg
            .count8()
            .intenclr
            .write(|w| unsafe { w.bits(interrupt_set.0) });
        self
    }

    pub unsafe fn registers(&self) -> &T::Reg {
        &self.reg
    }
}

impl<T: AbstractTimerId, I, TW> Timer<T, I, TW, state::Enabled> {
    pub fn disable(self) -> Timer<T, I, TW> {
        self.reg.count8().ctrla.modify(|_, w| w.enable().set_bit());
        while self.reg.count8().syncbusy.read().enable().bit_is_set() {}
        self.to()
    }

    fn sync_run_command(&mut self, cmd: TimerCommand) -> &mut Self {
        self.sync_ctrlbset_write(|w| w.cmd().variant(cmd));
        while !self.reg.count8().ctrlbset.read().cmd().is_none() {}
        self
    }
}

impl<T: AbstractTimerId, I, TW> Timer<T, I, TW> {
    pub fn enable(self) -> Timer<T, I, TW, state::Enabled> {
        self.reg.count8().ctrla.modify(|_, w| w.enable().set_bit());
        while self.reg.count8().syncbusy.read().enable().bit_is_set() {}
        self.to()
    }

    pub fn into_8_bit(self) -> Timer<T, I, u8> {
        self.reg.count8().ctrla.modify(|_, w| w.mode().count8());
        self.to()
    }

    pub fn into_16_bit(self) -> Timer<T, I, u16> {
        self.reg.count8().ctrla.modify(|_, w| w.mode().count16());
        self.to()
    }

    pub fn set_ondemand(&mut self, value: bool) -> &mut Self {
        self.reg
            .count8()
            .ctrla
            .modify(|_, w| w.ondemand().bit(value));
        self
    }

    pub fn with_ondemand(mut self, value: bool) -> Self {
        self.set_ondemand(value);
        self
    }

    pub fn set_runstdby(&mut self, value: bool) -> &mut Self {
        self.reg
            .count8()
            .ctrla
            .modify(|_, w| w.runstdby().bit(value));
        self
    }

    pub fn with_runstdby(mut self, value: bool) -> Self {
        self.set_runstdby(value);
        self
    }

    pub fn with_debug_run(mut self, value: bool) -> Self {
        self.set_debug_run(value);
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

    pub fn with_interrupts(mut self, interrupt_set: InterruptSet) -> Self {
        self.disable_interrupts(InterruptSet::full());
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
}

impl<T: CombinedTimerId, I, TW> Timer<T, I, TW> {
    pub fn into_32_bit(self) -> Timer<T, I, u32> {
        self.reg.count8().ctrla.modify(|_, w| w.mode().count32());
        self.to()
    }
}

impl<T: AbstractTimerId, I, TW: timer_width::TimerWidth> Timer<T, I, TW, state::Enabled> {
    pub fn compare(&self, index: TimerCompareRegister) -> TW {
        TW::from_u32(self.reg.count32().cc[index as usize].read().cc().bits())
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
        TW::from_u32(self.reg.count32().count.read().count().bits())
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

    // Clears the interrupts
    pub fn interrupt_flags(&self) -> InterruptSet {
        let interrupts = self.reg.count8().intflag.read().bits();
        // Clear the interrupts
        self.reg
            .count8()
            .intflag
            .write(|w| unsafe { w.bits(interrupts) });
        InterruptSet(interrupts)
    }
}

impl<T: PrimaryTimerId, I: PclkSourceId> Timer<T, I> {
    // Create the disjoint pair of timers. They support 8-bit and 16-bit counters
    pub fn disjoint(
        primary_reg: T::Reg,
        primary_clk: ApbClk<T>,
        secondary_reg: Reg<SecondaryTimer<T>>,
        secondary_clk: ApbClk<SecondaryTimer<T>>,
        common_pclk: Pclk<T::CombinedTimer, I>,
    ) -> (Timer<T, I>, Timer<SecondaryTimer<T>, I>) {
        let _ = primary_clk;
        let _ = secondary_clk;
        let pclk_freq = common_pclk.freq();
        let _ = common_pclk;

        (
            Timer::new(primary_reg, pclk_freq),
            Timer::new(secondary_reg, pclk_freq),
        )
    }
}

pub trait TimersTupleExt<T: PrimaryTimerId, I: PclkSourceId> {
    fn free(
        self,
    ) -> (
        T::Reg,
        ApbClk<T>,
        Reg<SecondaryTimer<T>>,
        ApbClk<SecondaryTimer<T>>,
        Pclk<T::CombinedTimer, I>,
    );
}

impl<T: PrimaryTimerId, I: PclkSourceId, TW1, TW2> TimersTupleExt<T, I>
    for (Timer<T, I, TW1>, Timer<SecondaryTimer<T>, I, TW2>)
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

impl<T: PairableTimerId, I: PclkSourceId> Timer<T, I>
where
    Reg<T::CombinedTimer>: From<(Reg<T>, Reg<SecondaryTimer<T>>)>,
{
    // Create the paired timer. It supports 32-bit counter
    pub fn paired(
        primary_reg: T::Reg,
        primary_clk: ApbClk<T>,
        secondary_reg: Reg<SecondaryTimer<T>>,
        secondary_clk: ApbClk<SecondaryTimer<T>>,
        common_pclk: Pclk<T::CombinedTimer, I>,
    ) -> Timer<T::CombinedTimer, I> {
        let _ = primary_clk;
        let _ = secondary_clk;
        let pclk_freq = common_pclk.freq();
        let _ = common_pclk;
        Timer::new((primary_reg, secondary_reg).into(), pclk_freq)
    }
}

impl<T: CombinedTimerId, I: PclkSourceId, TW> Timer<T, I, TW>
where
    T::Reg: Into<(Reg<T::PrimaryTimer>, Reg<T::SecondaryTimer>)>,
{
    pub fn free(
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
