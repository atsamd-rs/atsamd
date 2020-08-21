//! Time units

// Frequency based

/// Bits per second
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Bps(pub u32);

/// Hertz
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Hertz(pub u32);

/// KiloHertz
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct KiloHertz(pub u32);

/// MegaHertz
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MegaHertz(pub u32);

// Period based

/// Seconds
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Seconds(pub u32);

/// Milliseconds
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Milliseconds(pub u32);

/// Microseconds
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Microseconds(pub u32);

/// Nanoseconds
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Nanoseconds(pub u32);

/// Extension trait that adds convenience methods to the `u32` type
pub trait U32Ext {
    /// Wrap in `Bps`
    fn bps(self) -> Bps;

    /// Wrap in `Hertz`
    fn hz(self) -> Hertz;

    /// Wrap in `KiloHertz`
    fn khz(self) -> KiloHertz;

    /// Wrap in `MegaHertz`
    fn mhz(self) -> MegaHertz;

    /// Wrap in `Seconds`
    fn s(self) -> Seconds;

    /// Wrap in `Milliseconds`
    fn ms(self) -> Milliseconds;

    /// Wrap in `Microseconds`
    fn us(self) -> Microseconds;

    /// Wrap in `NanoSeconds`
    fn ns(self) -> Nanoseconds;
}

impl U32Ext for u32 {
    // Frequency based

    fn bps(self) -> Bps {
        Bps(self)
    }

    fn hz(self) -> Hertz {
        Hertz(self)
    }

    fn khz(self) -> KiloHertz {
        KiloHertz(self)
    }

    fn mhz(self) -> MegaHertz {
        MegaHertz(self)
    }

    // Period based

    fn s(self) -> Seconds {
        Seconds(self)
    }

    fn ms(self) -> Milliseconds {
        Milliseconds(self)
    }

    fn us(self) -> Microseconds {
        Microseconds(self)
    }

    fn ns(self) -> Nanoseconds {
        Nanoseconds(self)
    }
}

// Frequency based

impl Into<Hertz> for KiloHertz {
    fn into(self) -> Hertz {
        Hertz(self.0 * 1_000)
    }
}

impl Into<Hertz> for MegaHertz {
    fn into(self) -> Hertz {
        Hertz(self.0 * 1_000_000)
    }
}

impl Into<KiloHertz> for MegaHertz {
    fn into(self) -> KiloHertz {
        KiloHertz(self.0 * 1_000)
    }
}

impl Into<KiloHertz> for Hertz {
    fn into(self) -> KiloHertz {
        KiloHertz(self.0 / 1_000)
    }
}

impl Into<MegaHertz> for Hertz {
    fn into(self) -> MegaHertz {
        MegaHertz(self.0 / 1_000_000)
    }
}

impl Into<MegaHertz> for KiloHertz {
    fn into(self) -> MegaHertz {
        MegaHertz(self.0 / 1_000)
    }
}

// Period based

impl Into<Milliseconds> for Seconds {
    fn into(self) -> Milliseconds {
        Milliseconds(self.0 * 1_000)
    }
}
impl Into<Microseconds> for Seconds {
    fn into(self) -> Microseconds {
        Microseconds(self.0 * 1_000_000)
    }
}

impl Into<Nanoseconds> for Seconds {
    fn into(self) -> Nanoseconds {
        Nanoseconds(self.0 * 1_000_000_000)
    }
}

impl Into<Microseconds> for Milliseconds {
    fn into(self) -> Microseconds {
        Microseconds(self.0 * 1_000)
    }
}

impl Into<Nanoseconds> for Microseconds {
    fn into(self) -> Nanoseconds {
        Nanoseconds(self.0 * 1_000)
    }
}

impl Into<Seconds> for Milliseconds {
    fn into(self) -> Seconds {
        Seconds(self.0 / 1_000)
    }
}

impl Into<Seconds> for Microseconds {
    fn into(self) -> Seconds {
        Seconds(self.0 / 1_000_000)
    }
}

impl Into<Milliseconds> for Microseconds {
    fn into(self) -> Milliseconds {
        Milliseconds(self.0 / 1_000)
    }
}

impl Into<Nanoseconds> for Milliseconds {
    fn into(self) -> Nanoseconds {
        Nanoseconds(self.0 * 1_000_000)
    }
}

// Frequency <-> Period

impl Into<Hertz> for Nanoseconds {
    fn into(self) -> Hertz {
        Hertz(1_000_000_000_u32 / self.0)
    }
}

impl Into<Hertz> for Microseconds {
    fn into(self) -> Hertz {
        Hertz(1_000_000_u32 / self.0)
    }
}

impl Into<KiloHertz> for Nanoseconds {
    fn into(self) -> KiloHertz {
        KiloHertz(1_000_000_u32 / self.0)
    }
}

impl Into<MegaHertz> for Nanoseconds {
    fn into(self) -> MegaHertz {
        MegaHertz(1_000_u32 / self.0)
    }
}

impl Into<Microseconds> for Hertz {
    fn into(self) -> Microseconds {
        Microseconds(1_000_000_u32 / self.0)
    }
}

impl Into<Nanoseconds> for Hertz {
    fn into(self) -> Nanoseconds {
        Nanoseconds(1_000_000_000u32 / self.0)
    }
}

impl Into<Nanoseconds> for KiloHertz {
    fn into(self) -> Nanoseconds {
        Nanoseconds(1_000_000u32 / self.0)
    }
}

impl Into<Nanoseconds> for MegaHertz {
    fn into(self) -> Nanoseconds {
        Nanoseconds(1_000u32 / self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::time::*;

    #[test]
    fn convert_us_to_hz() {
        let as_us: Microseconds = 3.hz().into();
        assert_eq!(as_us.0, 333_333_u32);
    }

    #[test]
    fn convert_ms_to_us() {
        let as_us: Microseconds = 3.ms().into();
        assert_eq!(as_us.0, 3_000_u32);
    }

    #[test]
    fn convert_mhz_to_hz() {
        let as_hz: Hertz = 48.mhz().into();
        assert_eq!(as_hz.0, 48_000_000_u32);
    }

    #[test]
    fn convert_hz_to_ns() {
        let as_ns: Nanoseconds = 3.mhz().into();
        assert_eq!(as_ns.0, 333_u32);
    }

    #[test]
    fn convert_hz_to_ns_even() {
        let as_ns: Nanoseconds = 2.mhz().into();
        assert_eq!(as_ns.0, 500_u32);
    }
}
