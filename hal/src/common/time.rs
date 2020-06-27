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

/// Miliseconds
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Miliseconds(pub u32);

/// Microseconds
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Microseconds(pub u32);

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

    /// Wrap in `Miliseconds`
    fn ms(self) -> Miliseconds;

    /// Wrap in `Microseconds`
    fn us(self) -> Microseconds;
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

    fn ms(self) -> Miliseconds {
        Miliseconds(self)
    }

    fn us(self) -> Microseconds {
        Microseconds(self)
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

impl Into<Miliseconds> for Seconds {
    fn into(self) -> Miliseconds {
        Miliseconds(self.0 * 1_000)
    }
}

impl Into<Microseconds> for Seconds {
    fn into(self) -> Microseconds {
        Microseconds(self.0 * 1_000_000)
    }
}

impl Into<Microseconds> for Miliseconds {
    fn into(self) -> Microseconds {
        Microseconds(self.0 * 1_000)
    }
}

impl Into<Seconds> for Miliseconds {
    fn into(self) -> Seconds {
        Seconds(self.0 / 1_000)
    }
}

impl Into<Seconds> for Microseconds {
    fn into(self) -> Seconds {
        Seconds(self.0 / 1_000_000)
    }
}

impl Into<Miliseconds> for Microseconds {
    fn into(self) -> Miliseconds {
        Miliseconds(self.0 / 1_000)
    }
}

// Frequency <-> Period

impl Into<Hertz> for Microseconds {
    fn into(self) -> Hertz {
        Hertz(1_000_000_u32 / self.0)
    }
}

impl Into<Microseconds> for Hertz {
    fn into(self) -> Microseconds {
        Microseconds(1_000_000_u32 / self.0)
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
}
