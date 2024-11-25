//! Working with timer counter hardware
use core::convert::Infallible;

use atsamd_hal_macros::hal_cfg;
use fugit::NanosDurationU32;

use crate::ehal_02::timer::{CountDown, Periodic};
use crate::pac::Pm;
#[hal_cfg("tc1-d11")]
use crate::pac::{tc1::Count16 as Count16Reg, Tc1};
#[hal_cfg("tc3-d21")]
use crate::pac::{tc3::Count16 as Count16Reg, Tc3, Tc4, Tc5};
use crate::timer_params::TimerParams;

use crate::clock;
use crate::time::{Hertz, Nanoseconds};
use crate::timer_traits::InterruptDrivenTimer;

#[cfg(feature = "async")]
mod async_api;

#[cfg(feature = "async")]
pub use async_api::*;

// Note:
// TC3 + TC4 can be paired to make a 32-bit counter
// TC5 + TC6 can be paired to make a 32-bit counter

/// A generic hardware timer counter.
///
/// The counters are exposed in 16-bit mode only.
/// The hardware allows configuring the 8-bit mode
/// and pairing up some instances to run in 32-bit
/// mode, but that functionality is not currently
/// exposed by this hal implementation.
/// TimerCounter implements both the `Periodic` and
/// the `CountDown` embedded_hal timer traits.
/// Before a hardware timer can be used, it must first
/// have a clock configured.
pub struct TimerCounter<TC> {
    freq: Hertz,
    tc: TC,
}

impl<TC: Count16> TimerCounter<TC> {}

/// This is a helper trait to make it easier to make most of the
/// TimerCounter impl generic.  It doesn't make too much sense to
/// to try to implement this trait outside of this module.
pub trait Count16 {
    fn count_16(&self) -> &Count16Reg;
}

impl<TC> Periodic for TimerCounter<TC> {}
impl<TC> CountDown for TimerCounter<TC>
where
    TC: Count16,
{
    type Time = Nanoseconds;

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Self::Time>,
    {
        <Self as InterruptDrivenTimer>::start(self, timeout);
    }

    fn wait(&mut self) -> nb::Result<(), void::Void> {
        nb::block! {
            <Self as InterruptDrivenTimer>::wait(self)
        }
        .unwrap(); // wait() is Infallible
        Ok(())
    }
}

impl<TC> InterruptDrivenTimer for TimerCounter<TC>
where
    TC: Count16,
{
    /// Enable the interrupt generation for this hardware timer.
    /// This method only sets the clock configuration to trigger
    /// the interrupt; it does not configure the interrupt controller
    /// or define an interrupt handler.
    fn enable_interrupt(&mut self) {
        self.tc.count_16().intenset().write(|w| w.ovf().set_bit());
    }

    fn start<T: Into<NanosDurationU32>>(&mut self, timeout: T) {
        let params = TimerParams::new_ns(timeout.into(), self.freq);
        let divider = params.divider;
        let cycles = params.cycles;

        let count = self.tc.count_16();

        // Disable the timer while we reconfigure it
        count.ctrla().modify(|_, w| w.enable().clear_bit());
        while count.status().read().syncbusy().bit_is_set() {}

        // Now that we have a clock routed to the peripheral, we
        // can ask it to perform a reset.
        count.ctrla().write(|w| w.swrst().set_bit());
        while count.status().read().syncbusy().bit_is_set() {}
        // the SVD erroneously marks swrst as write-only, so we
        // need to manually read the bit here
        while count.ctrla().read().bits() & 1 != 0 {}

        count.ctrlbset().write(|w| {
            // Count up when the direction bit is zero
            w.dir().clear_bit();
            // Periodic
            w.oneshot().clear_bit()
        });

        // Set TOP value for mfrq mode
        count.cc(0).write(|w| unsafe { w.cc().bits(cycles as u16) });

        count.ctrla().modify(|_, w| {
            match divider {
                1 => w.prescaler().div1(),
                2 => w.prescaler().div2(),
                4 => w.prescaler().div4(),
                8 => w.prescaler().div8(),
                16 => w.prescaler().div16(),
                64 => w.prescaler().div64(),
                256 => w.prescaler().div256(),
                1024 => w.prescaler().div1024(),
                _ => unreachable!(),
            };
            // Enable Match Frequency Waveform generation
            w.wavegen().mfrq();
            w.enable().set_bit();
            w.runstdby().set_bit()
        });
    }

    fn wait(&mut self) -> nb::Result<(), Infallible> {
        let count = self.tc.count_16();
        if count.intflag().read().ovf().bit_is_set() {
            // Writing a 1 clears the flag
            count.intflag().modify(|_, w| w.ovf().set_bit());
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    /// Disables interrupt generation for this hardware timer.
    /// This method only sets the clock configuration to prevent
    /// triggering the interrupt; it does not configure the interrupt
    /// controller.
    fn disable_interrupt(&mut self) {
        self.tc.count_16().intenclr().write(|w| w.ovf().set_bit());
    }
}

macro_rules! tc {
    ($($TYPE:ident: ($TC:ident, $pm:ident, $clock:ident),)+) => {
        $(
pub type $TYPE = TimerCounter<$TC>;

impl Count16 for $TC {
    fn count_16(&self) -> &Count16Reg {
        self.count16()
    }
}

impl TimerCounter<$TC>
{
    /// Configure this timer counter instance.
    /// The clock is obtained from the `GenericClockController` instance
    /// and its frequency impacts the resolution and maximum range of
    /// the timeout values that can be passed to the `start` method.
    /// Note that some hardware timer instances share the same clock
    /// generator instance and thus will be clocked at the same rate.
    pub fn $pm(clock: &clock::$clock, tc: $TC, pm: &mut Pm) -> Self {
        // this is safe because we're constrained to just the tc3 bit
        pm.apbcmask().modify(|_, w| w.$pm().set_bit());
        {
            let count = tc.count_16();

            // Disable the timer while we reconfigure it
            count.ctrla().modify(|_, w| w.enable().clear_bit());
            while count.status().read().syncbusy().bit_is_set() {}
        }
        Self {
            freq: clock.freq(),
            tc,
        }
    }
}
        )+
    }
}

// samd11
#[hal_cfg("tc1-d11")]
tc! {
    TimerCounter1: (Tc1, tc1_, Tc1Tc2Clock),
}
// samd21
#[hal_cfg("tc3-d21")]
tc! {
    TimerCounter3: (Tc3, tc3_, Tcc2Tc3Clock),
    TimerCounter4: (Tc4, tc4_, Tc4Tc5Clock),
    TimerCounter5: (Tc5, tc5_, Tc4Tc5Clock),
}
