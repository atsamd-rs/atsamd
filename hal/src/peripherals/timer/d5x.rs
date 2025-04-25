//! Working with timer counter hardware
use atsamd_hal_macros::hal_cfg;

use crate::pac::tc0::Count16 as Count16Reg;
use crate::pac::{Mclk, Tc2, Tc3};
#[hal_cfg(all("tc4", "tc5"))]
use crate::pac::{Tc4, Tc5};
#[hal_cfg(all("tc6", "tc7"))]
use crate::pac::{Tc6, Tc7};

use crate::clock;
use crate::time::Hertz;

mod common;
pub use common::Count16;

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
impl<TC> TimerCounter<TC>
where
    TC: Count16,
{
    /// Starts the 16-bit timer, counting up in periodic mode.
    fn start_timer(&mut self, divider: u16, cycles: u16) {
        // Disable the timer while we reconfigure it
        self.disable();

        let count = self.tc.count_16();

        // Now that we have a clock routed to the peripheral, we
        // can ask it to perform a reset.
        count.ctrla().write(|w| w.swrst().set_bit());
        while count.syncbusy().read().swrst().bit_is_set() {}

        count.ctrlbset().write(|w| {
            // Count up when the direction bit is zero
            w.dir().clear_bit();
            // Periodic
            w.oneshot().clear_bit()
        });

        // Set TOP value for mfrq mode
        count.cc(0).write(|w| unsafe { w.cc().bits(cycles) });

        // Enable Match Frequency Waveform generation
        count.wave().modify(|_, w| w.wavegen().mfrq());

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
            w.enable().set_bit();
            w.runstdby().set_bit()
        });
    }

    /// Disable the timer
    fn disable(&mut self) {
        let count = self.tc.count_16();

        count.ctrla().modify(|_, w| w.enable().clear_bit());
        while count.syncbusy().read().enable().bit_is_set() {}
    }
}

macro_rules! tc {
    ($($TYPE:ident: ($TC:ident, $mclk:ident, $clock:ident, $apmask:ident),)+) => {
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
    pub fn $mclk(clock: &clock::$clock, tc: $TC, mclk: &mut Mclk) -> Self {
        // this is safe because we're constrained to just the tc3 bit
        mclk.$apmask().modify(|_, w| w.$mclk().set_bit());
        {
            let count = tc.count16();

            // Disable the timer while we reconfigure it
            count.ctrla().modify(|_, w| w.enable().clear_bit());
            while count.syncbusy().read().enable().bit_is_set()  {}
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

tc! {
    TimerCounter2: (Tc2, tc2_, Tc2Tc3Clock, apbbmask),
    TimerCounter3: (Tc3, tc3_, Tc2Tc3Clock, apbbmask),
}

#[hal_cfg(all("tc4", "tc5"))]
tc! {
    TimerCounter4: (Tc4, tc4_, Tc4Tc5Clock, apbcmask),
    TimerCounter5: (Tc5, tc5_, Tc4Tc5Clock, apbcmask),
}

#[hal_cfg(all("tc6", "tc7"))]
tc! {
    TimerCounter6: (Tc6, tc6_, Tc6Tc7Clock, apbdmask),
    TimerCounter7: (Tc7, tc7_, Tc6Tc7Clock, apbdmask),
}
