use crate::ehal::watchdog;
use crate::pac::WDT;

/// WatchdogTimeout enumerates usable values for configuring
/// the timeout of the watchdog peripheral.
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum WatchdogTimeout {
    Cycles8 = 0,
    Cycles16,
    Cycles32,
    Cycles64,
    Cycles128,
    Cycles256,
    Cycles512,
    Cycles1K,
    Cycles2K,
    Cycles4K,
    Cycles8K,
    Cycles16K,
}

pub struct Watchdog {
    wdt: WDT,
}

impl Watchdog {
    pub fn new(wdt: WDT) -> Self {
        Self { wdt }
    }
}

impl watchdog::Watchdog for Watchdog {
    /// Feeds an existing watchdog to ensure the processor isn't reset.
    /// Sometimes commonly referred to as "kicking" or "refreshing".
    fn feed(&mut self) {
        self.wdt.clear.write(|w| unsafe { w.clear().bits(0xA5) });
    }
}

/// Disables a running watchdog timer so the processor won't be reset.
impl watchdog::WatchdogDisable for Watchdog {
    fn disable(&mut self) {
        // Disable the watchdog timer.
        self.wdt.ctrla.write(|w| w.enable().clear_bit());
        // Wait for watchdog timer to be disabled.
        while self.wdt.syncbusy.read().enable().bit_is_set() {}
    }
}

impl watchdog::WatchdogEnable for Watchdog {
    type Time = u8;

    /// Enables a watchdog timer to reset the processor if software is frozen
    /// or stalled.
    fn start<T>(&mut self, period: T)
    where
        T: Into<Self::Time>,
    {
        // Write the timeout configuration.
        self.wdt
            .config
            .write(|w| unsafe { w.per().bits(period.into()) });
        // Enable the watchdog timer.
        self.wdt.ctrla.write(|w| w.enable().set_bit());
        // Wait for watchdog timer to be enabled.
        while self.wdt.syncbusy.read().enable().bit_is_set() {}
    }
}
