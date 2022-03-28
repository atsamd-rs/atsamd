use crate::ehal::timer::{CountDown, Periodic};
use crate::time;

/// Trait for timers that can enable & disable an interrupt that fires
/// when the timer expires
pub trait InterruptDrivenTimer: CountDown<Time = time::Nanoseconds> + Periodic {
    /// Enable the timer interrupt
    fn enable_interrupt(&mut self);

    /// Disable the timer interrupt
    fn disable_interrupt(&mut self);
}
