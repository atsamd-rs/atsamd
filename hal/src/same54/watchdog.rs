use crate::target_device::WDT;

/// WatchdogTimeout enumerates usable values for configuring
/// the timeout of the watchdog peripheral.
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum WatchdogTimeout {
    Timeout8ms = 0,
    Timeout16ms,
    Timeout32ms,
    Timeout64ms,
    Timeout128ms,
    Timeout256ms,
    Timeout512ms,
    Timeout1024ms,
    Timeout2048ms,
}

pub struct Watchdog {
    wdt: WDT,
}

impl Watchdog {
    /// Enables the watchdog in normal (timeout) mode, with
    /// the specified timeout. The caller must invoke reset() within
    /// the timeout duration to avoid a reset.
    pub fn new_with_timeout(wdt: WDT, timeout: WatchdogTimeout) -> Self {
        // Write the timeout configuration.
        wdt.config.write(|w| unsafe { w.per().bits(timeout as u8) });
        // Enable the watchdog timer.
        wdt.ctrla.write(|w| w.enable().set_bit());
        // Wait for watchdog timer to be enabled.
        while wdt.syncbusy.read().enable().bit_is_set() {}

        Self { wdt }
    }

    pub fn clear(&self) {
        self.wdt.clear.write(|w| unsafe { w.clear().bits(0xA5) });
    }
}
