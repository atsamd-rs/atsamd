use num_traits::FromPrimitive;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug, Eq, num_derive::FromPrimitive)]
pub enum TimerInterrupt {
    MatchOrCaptureChannel1 = 1 << 5,
    MatchOrCaptureChannel0 = 1 << 4,
    Error = 1 << 1,
    Overflow = 1 << 0,
}

bitfield::bitfield! {
    /// Raw userpage POD struct that exposes bitfields via methods
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct TimerInterruptSet(u8);
    impl Debug;
    u8;
    pub overflow, set_overflow: 0;
    pub error, set_error: 1;
    pub match_or_capture_channel_0, set_match_or_capture_channel_0: 4;
    pub match_or_capture_channel_1, set_match_or_capture_channel_1: 5;
}

impl TimerInterruptSet {
    pub fn full() -> Self {
        Self::from_iter(Self(u8::MAX))
    }

    pub fn empty() -> Self {
        Self(0)
    }
}

impl From<TimerInterrupt> for TimerInterruptSet {
    fn from(value: TimerInterrupt) -> Self {
        Self(value as _)
    }
}

impl FromIterator<TimerInterrupt> for TimerInterruptSet {
    fn from_iter<T: IntoIterator<Item = TimerInterrupt>>(iter: T) -> Self {
        Self(iter.into_iter().fold(0, |l, r| l | r as u8))
    }
}

impl Iterator for TimerInterruptSet {
    type Item = TimerInterrupt;

    fn next(&mut self) -> Option<Self::Item> {
        let raw_interrupt_set = &mut self.0;
        while *raw_interrupt_set != 0 {
            // Count of trailing zeros is equal to a bit index of an interrupt
            let raw_interrupt = 1 << raw_interrupt_set.trailing_zeros();
            // Clear the bit
            *raw_interrupt_set &= !raw_interrupt;
            match TimerInterrupt::from_u8(raw_interrupt) {
                None => continue,
                Some(i) => return Some(i),
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_full_interrupt_set() {
        let interrupts = [
            TimerInterrupt::MatchOrCaptureChannel0,
            TimerInterrupt::MatchOrCaptureChannel1,
            TimerInterrupt::Error,
            TimerInterrupt::Overflow,
        ];
        let full_interrupt_set_from_u8_max = TimerInterruptSet::full();
        let full_interrupt_set_from_summed_variants = TimerInterruptSet(
            TimerInterrupt::MatchOrCaptureChannel0 as u8
                | TimerInterrupt::MatchOrCaptureChannel1 as u8
                | TimerInterrupt::Error as u8
                | TimerInterrupt::Overflow as u8,
        );
        let full_interrupt_set = TimerInterruptSet::from_iter(interrupts);
        assert_eq!(full_interrupt_set_from_u8_max, full_interrupt_set);
        assert_eq!(full_interrupt_set_from_summed_variants, full_interrupt_set);
        assert_eq!(full_interrupt_set.count(), interrupts.len());
    }
    #[test]
    fn test_interrupt_set_with_single_interrupts() {
        for interrupt in TimerInterruptSet(u8::MAX) {
            let is_from = TimerInterruptSet::from(interrupt);
            let is_from_iter = TimerInterruptSet::from_iter([interrupt]);
            assert_eq!(is_from, is_from_iter);
        }
    }
    #[test]
    fn test_empty_interrupt_set() {
        let interrupt_set_from_0 = TimerInterruptSet(0);
        let interrupt_set_from_empty_iterator = TimerInterruptSet::from_iter([]);
        assert_eq!(0, interrupt_set_from_0.count());
        assert_eq!(interrupt_set_from_0, interrupt_set_from_empty_iterator);
    }
}
