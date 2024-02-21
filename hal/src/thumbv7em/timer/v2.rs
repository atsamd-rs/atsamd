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
//! # let mut pac = unsafe { atsamd_hal::pac::Peripherals::steal() };
//! use atsamd_hal::prelude::*;
//! use atsamd_hal::clock::v2::pclk::Pclk;
//! use atsamd_hal::timer::v2 as timer;
//! use timer::{TimerBuilder, TimerPrescaler, TimerDirection, TimerInterrupt, TimerInterruptSet};
//!
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
//! let (pri, sec) = TimerBuilder::disjoint(tc0, tc0_clk, tc1, tc1_clk, tc0_tc1_pclk); /* -> (TimerBuilder<Tc0>, TimerBuilder<Tc1>) */
//! let pri = pri
//!         .into_8_bit()
//!         .with_ondemand(true)
//!         .into_raw()
//!         .with_interrupts(TimerInterruptSet::from_iter(
//!         [
//!             TimerInterrupt::Overflow,
//!             TimerInterrupt::MatchOrCaptureChannel0,
//!             TimerInterrupt::Error,
//!         ]
//!         ))
//!         .with_frequency(48.MHz())
//!         .expect("oops")
//!         .enable();
//! let (tc0, tc0_clk, tc1, tc1_clk, tc0_tc1_pclk) = (pri.disable(), sec.into_raw()).free();
//! let timer = TimerBuilder::paired(tc0, tc0_clk, tc1, tc1_clk, tc0_tc1_pclk); /* -> TimerBuilder<Tc0Tc1> */
//! let timer = timer /* -> RawTimer<Tc0Tc1, _, u16, state::Enabled> */
//!           .into_raw()
//!           .with_interrupts(TimerInterrupt::Overflow.into())
//!           .with_prescaler(TimerPrescaler::DIV16)
//!           .with_oneshot(true)
//!           .with_direction(TimerDirection::Increment)
//!           .enable();
//! let timer = timer.disable();
//! let timer = timer
//!           .reset_into_builder()
//!           .into_monotonic::<48_000_000>()
//!           .expect("oops")
//!           .enable();
//! let (tc0, tc0_clk, tc1, tc1_clk, tc0_tc1_pclk) = timer.disable().into_raw().free();
//!
//! let (pri, sec) = TimerBuilder::disjoint(tc2, tc2_clk, tc3, tc3_clk, tc2_tc3_pclk) /* -> (TimerBuilder<Tc2>, TimerBuilder<Tc3>) */;
//! let (tc2, tc2_clk, tc3, tc3_clk, tc2_tc3_pclk) = (pri, sec).free();
//! let timer = TimerBuilder::paired(tc2, tc2_clk, tc3, tc3_clk, tc2_tc3_pclk) /* -> TimerBuilder<Tc2Tc3> */;
//! let mut timer = timer /* -> CountdownTimer<Tc2Tc3, _, u8, state::Enabled> */
//!           .into_8_bit()
//!           .into_countdown::<32_768>()
//!           .expect("oops")
//!           .enable();
//! timer.retrigger_with_period(2.secs());
//! timer.enable_interrupt();
//! timer.disable_interrupt();
//! let (tc2, tc2_clk, tc3, tc3_clk, tc2_tc3_pclk) = timer.into_raw().disable().free();
//! ```
//!
//! In order to use the timer as a RTIC monotonic timer, one can call
//! `Timer::into_monotonic`. This requires a feature flag `rtic` being enabled.

use core::{convert::Infallible, marker::PhantomData};

pub mod prelude;

mod builder;
pub use builder::*;

mod raw;
pub use raw::*;

#[cfg(feature = "rtic")]
mod mono;
#[cfg(feature = "rtic")]
pub use mono::*;

mod countdown;
pub use countdown::*;

mod interrupt;
pub use interrupt::*;

mod types;
pub use types::*;

use crate::{
    clock::v2::{
        apb::{ApbClk, ApbToken},
        pclk::{Pclk, PclkSourceId, PclkToken},
    },
    time::Hertz,
};

impl<T: PrimaryTimerId, I: PclkSourceId> TimerBuilder<T, I> {
    // Create the disjoint pair of timers. They support 8-bit and 16-bit counters
    pub fn disjoint(
        primary_reg: T::Reg,
        primary_clk: ApbClk<T>,
        secondary_reg: Reg<SecondaryTimer<T>>,
        secondary_clk: ApbClk<SecondaryTimer<T>>,
        common_pclk: Pclk<T::CombinedTimer, I>,
    ) -> (TimerBuilder<T, I>, TimerBuilder<SecondaryTimer<T>, I>) {
        let _ = primary_clk;
        let _ = secondary_clk;
        let pclk_freq = common_pclk.freq();
        let _ = common_pclk;

        (
            TimerBuilder::new(RawTimer::new(primary_reg, pclk_freq)),
            TimerBuilder::new(RawTimer::new(secondary_reg, pclk_freq)),
        )
    }
}

impl<T: PairableTimerId, I: PclkSourceId> TimerBuilder<T, I>
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
    ) -> TimerBuilder<T::CombinedTimer, I> {
        let _ = primary_clk;
        let _ = secondary_clk;
        let pclk_freq = common_pclk.freq();
        let _ = common_pclk;
        TimerBuilder::new(RawTimer::new(
            (primary_reg, secondary_reg).into(),
            pclk_freq,
        ))
    }
}
