use crate::target_device::WDT;
use hal::watchdog;

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

fn feed(&mut wdt: WDT) {
    wdt.clear.write(|w| unsafe { w.clear().bits(0xA5) });
}
fn disable(&mut wdt: WDT) {
    // Disable the watchdog timer.
    wdt.ctrla.write(|w| w.enable().clear_bit());
    // Wait for watchdog timer to be disabled.
    while wdt.syncbusy.read().enable().bit_is_set() {}
}
fn start<T: Into<u8>>(&mut wdt: WDT, period: T) {
    // Write the timeout configuration.
    wdt.config.write(|w| unsafe { w.per().bits(period.into()) });
    // Enable the watchdog timer.
    wdt.ctrla.write(|w| w.enable().set_bit());
    // Wait for watchdog timer to be enabled.
    while wdt.syncbusy.read().enable().bit_is_set() {}
}

#[cfg(not(feature = "unproven"))]
impl Watchdog {
    /// Feeds an existing watchdog to ensure the processor isn't reset.
    /// Sometimes commonly referred to as "kicking" or "refreshing".
    fn feed(&mut self) {
        feed(&mut self.wdt);
    }
    /// Disables a running watchdog timer so the processor won't be reset.
    fn disable(&mut self) {
        disable(&mut self.wdt);
    }
    /// Enables a watchdog timer to reset the processor if software is frozen
    /// or stalled.
    fn start<T: Into<u8>>(&mut self, period: T) {
        start(&mut self.wdt, period);
    }
}

#[cfg(feature = "unproven")]
impl watchdog::Watchdog for Watchdog {
    /// Feeds an existing watchdog to ensure the processor isn't reset.
    /// Sometimes commonly referred to as "kicking" or "refreshing".
    fn feed(&mut self) {
        feed(&mut self.wdt);
    }
}

#[cfg(feature = "unproven")]
impl watchdog::WatchdogDisable for Watchdog {
    /// Disables a running watchdog timer so the processor won't be reset.
    fn disable(&mut self) {
        disable(&mut self.wdt);
    }
}

#[cfg(feature = "unproven")]
impl watchdog::WatchdogEnable for Watchdog {
    type Time = u8;

    /// Enables a watchdog timer to reset the processor if software is frozen
    /// or stalled.
    fn start<T>(&mut self, period: T)
    where
        T: Into<Self::Time>,
    {
        start(&mut self.wdt, period);
    }
}
