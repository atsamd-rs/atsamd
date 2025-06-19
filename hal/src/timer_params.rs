//! helper struct to calculate divider & cycles settings for timers.
use crate::time::{Hertz, Nanoseconds};

/// Helper type for computing cycles and divider given frequency
#[derive(Debug, Clone, Copy)]
pub struct TimerParams {
    pub divider: u16,
    pub cycles: u32,
}

impl TimerParams {
    /// Calculates the [`TimerParams`] from a given frequency based timeout.
    ///
    /// Panics if the combination of `timeout` and `src_freq` cannot be done
    /// with a 16-bit timer.
    pub fn new(timeout: Hertz, src_freq: Hertz) -> Self {
        let ticks: u32 = src_freq.to_Hz() / timeout.to_Hz().max(1);
        let ret = Self::new_from_ticks(ticks);
        ret.check_cycles_u16();
        ret
    }

    /// Calculates the [`TimerParams`] from a given period based timeout.
    ///
    /// Panics if the combination of `timeout` and `src_freq` cannot be done
    /// with a 16-bit timer.
    pub fn new_ns(timeout: Nanoseconds, src_freq: Hertz) -> Self {
        let ticks: u32 =
            (timeout.to_nanos() as u64 * src_freq.to_Hz() as u64 / 1_000_000_000_u64) as u32;
        let ret = Self::new_from_ticks(ticks);
        ret.check_cycles_u16();
        ret
    }

    pub(crate) fn new_from_ticks(ticks: u32) -> Self {
        let divider = ((ticks >> 16) + 1).next_power_of_two();
        let divider = match divider {
            1 | 2 | 4 | 8 | 16 | 64 | 256 | 1024 => divider,
            // There are a couple of gaps, so we round up to the next largest
            // divider; we'll need to count twice as many but it will work.
            32 => 64,
            128 => 256,
            512 => 1024,
            // Catch all case; this is lame.  Would be great to detect this
            // and fail at compile time.
            _ => 1024,
        };

        let cycles: u32 = ticks / divider;

        TimerParams {
            divider: divider as u16,
            cycles,
        }
    }

    /// Returns the number of required `cycles` as a `u16` and panics if the
    /// number is too high to fit.
    pub(crate) fn check_cycles_u16(&self) -> u16 {
        match u16::try_from(self.cycles) {
            Ok(c) => c,
            Err(_) => panic!(
                "cycles {} is out of range for a 16 bit counter",
                self.cycles
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fugit::{ExtU32, RateExtU32};
    use crate::timer_params::TimerParams;

    #[test]
    fn timer_params_hz_and_us_same_1hz() {
        let tp_from_hz = TimerParams::new(1.Hz(), 48.MHz());
        let tp_from_us = TimerParams::new_ns(1_000_000.micros(), 48.MHz());

        assert_eq!(tp_from_hz.divider, tp_from_us.divider);
        assert_eq!(tp_from_hz.cycles, tp_from_us.cycles);
    }

    #[test]
    fn timer_params_hz_and_us_same_3hz() {
        let tp_from_hz = TimerParams::new(3.Hz(), 48.MHz());
        let tp_from_us = TimerParams::new_ns(333_333.micros(), 48.MHz());

        // There's some rounding error here, but it is extremely small (1 cycle
        // difference)
        assert_eq!(tp_from_hz.divider, tp_from_us.divider);
        assert!((tp_from_hz.cycles as i32 - tp_from_us.cycles as i32).abs() <= 1);
    }
}
