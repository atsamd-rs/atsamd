// Sketching out clock configuration while I figure out
// how all the pieces fit together and how to model them.
use time::Hertz;

/// Clock configuration builder
pub struct Configuration {
    /// Configure the frequencies of the clock generators
    clocks: [Hertz; 8],
    /// Specifies the desired clock frequency for USB
    // TODO: will also need to configure PM.APBBMASK to
    // power up USB.  That isn't directly related to clocks.
    usb: Option<Hertz>,
}

/// Frozen clock configuration record
pub struct Clocks {
    usb: Hertz,
}

impl Configuration {
    /// Find a clock entry that hasn't been set to a frequency,
    /// and reserve it.  panic if there are no remaining
    /// slots that we can use.
    fn alloc_freq(&mut self, hz: Hertz) {
        for mut clk in &mut self.clocks {
            if *clk == Hertz(0) {
                *clk = hz;
                return;
            }
        }
        panic!("too many different clock configurations");
    }

    /// Enable and specify the USB clock frequency
    pub fn usb<F: Into<Hertz>>(mut self, freq: F) -> Self {
        let freq: Hertz = freq.into();
        self.alloc_freq(freq);
        self.usb = Some(freq);
        self
    }

    /// Freeze the configuration builder and apply it to
    /// the appropriate peripherals.
    pub fn freeze(self) -> Clocks {
        // TODO: pass in peripheral and apply the configs!

        Clocks {
            usb: self.usb.unwrap_or(Hertz(0)),
        }
    }
}

impl Clocks {
    pub fn usb(&self) -> Hertz {
        self.usb
    }
}
