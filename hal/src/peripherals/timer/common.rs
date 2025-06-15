//! Common items for timer hardware for all chips.
use core::convert::Infallible;
use fugit::NanosDurationU32;

use crate::ehal::delay::DelayNs;
use crate::ehal_02::timer::{CountDown, Periodic};
use crate::time::Nanoseconds;
use crate::timer_params::TimerParams;
use crate::timer_traits::InterruptDrivenTimer;

use super::{Count16Reg, TimerCounter};

/// This is a helper trait to make it easier to make most of the
/// TimerCounter impl generic.  It doesn't make too much sense to
/// to try to implement this trait outside of this module.
pub trait Count16 {
    fn count_16(&self) -> &Count16Reg;
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

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<NanosDurationU32>,
    {
        let params = TimerParams::new_ns(timeout.into(), self.freq);
        self.start_timer(params.divider, params.check_cycles_u16());
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

impl<TC> DelayNs for TimerCounter<TC>
where
    TC: Count16,
{
    fn delay_ns(&mut self, ns: u32) {
        let params = TimerParams::new_ns(NanosDurationU32::nanos(ns), self.freq);

        // The timer is is only 16 bits, so we may need to run it multiple times.
        let mut cycles = params.cycles;
        if cycles > u16::MAX as u32 {
            self.start_timer(params.divider, u16::MAX);
            while cycles > u16::MAX as u32 {
                let _ = nb::block!(InterruptDrivenTimer::wait(self));
                cycles -= u16::MAX as u32;
            }
        }

        // Wait more if there are any leftover cycles
        if cycles > 0 {
            self.start_timer(params.divider, cycles as u16);
            let _ = nb::block!(InterruptDrivenTimer::wait(self));
        }

        self.disable();
    }
}
