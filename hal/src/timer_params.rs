//! helper struct to calculate divider & cycles settings for timers.
use crate::time::{Hertz, Nanoseconds};

/// Helper type for computing cycles and divider given frequency
#[derive(Debug, Clone, Copy)]
pub struct TimerParams {
    pub divider: u16,
    pub cycles: u32,
}

impl TimerParams {
    /// calculates TimerParams from a given frequency based timeout.
    pub fn new<T>(timeout: T, src_freq: u32) -> Self
    where
        T: Into<Hertz>,
    {
        let timeout = timeout.into();
        let ticks: u32 = src_freq / timeout.0.max(1);
        Self::new_from_ticks(ticks)
    }

    /// calculates TimerParams from a given period based timeout.
    pub fn new_us<T>(timeout: T, src_freq: u32) -> Self
    where
        T: Into<Nanoseconds>,
    {
        let timeout = timeout.into();
        let ticks: u32 = (timeout.0 as u64 * src_freq as u64 / 1_000_000_000_u64) as u32;
        Self::new_from_ticks(ticks)
    }

    fn new_from_ticks(ticks: u32) -> Self {
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

        let cycles: u32 = ticks / divider as u32;

        if cycles > u16::max_value() as u32 {
            panic!("cycles {} is out of range for a 16 bit counter", cycles);
        }

        TimerParams {
            divider: divider as u16,
            cycles,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::time::U32Ext;
    use crate::timer_params::TimerParams;

    #[test]
    fn timer_params_hz_and_us_same_1hz() {
        let tp_from_hz = TimerParams::new(1_u32.hz(), 48_000_000_u32);
        let tp_from_us = TimerParams::new_us(1_000_000_u32.us(), 48_000_000_u32);

        assert_eq!(tp_from_hz.divider, tp_from_us.divider);
        assert_eq!(tp_from_hz.cycles, tp_from_us.cycles);
    }

    #[test]
    fn timer_params_hz_and_us_same_3hz() {
        let tp_from_hz = TimerParams::new(3_u32.hz(), 48_000_000_u32);
        let tp_from_us = TimerParams::new_us(333_333_u32.us(), 48_000_000_u32);

        // There's some rounding error here, but it is extremely small (1 cycle
        // difference)
        assert_eq!(tp_from_hz.divider, tp_from_us.divider);
        assert!((tp_from_hz.cycles as i32 - tp_from_us.cycles as i32).abs() <= 1);
    }
}
