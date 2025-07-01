use crate::pac::Wdt;
use atsamd_hal_macros::hal_macro_helper;

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
    wdt: Wdt,
}

impl Watchdog {
    pub fn new(wdt: Wdt) -> Self {
        Self { wdt }
    }
}

impl Watchdog {
    /// Feeds an existing watchdog to ensure the processor isn't reset.
    /// Sometimes commonly referred to as "kicking" or "refreshing".
    pub fn feed(&mut self) {
        self.wdt.clear().write(|w| unsafe { w.clear().bits(0xA5) });
    }

    /// Disables a running watchdog timer so the processor won't be reset.
    #[hal_macro_helper]
    pub fn disable(&mut self) {
        #[hal_cfg(any("wdt-d11", "wdt-d21"))]
        {
            // Disable the watchdog timer.
            self.wdt.ctrl().write(|w| w.enable().clear_bit());
            // Wait for watchdog timer to be disabled.
            while self.wdt.status().read().syncbusy().bit_is_set() {}
        }
        #[hal_cfg("wdt-d5x")]
        {
            // Disable the watchdog timer.
            self.wdt.ctrla().write(|w| w.enable().clear_bit());
            // Wait for watchdog timer to be disabled.
            while self.wdt.syncbusy().read().enable().bit_is_set() {}
        }
    }

    /// Enables a watchdog timer to reset the processor if software is frozen
    /// or stalled. Pass [`WatchdogTimeout`] as the period.
    /// 
    /// As WDT is driven by a 1024Hz clock, the time until timeout can be calculated
    /// as `(1 second/1024)*period`
    /// 
    /// EG:
    /// `Timeout of 2048 cycles = (1/1024)*2048 = 2 seconds`
    #[hal_macro_helper]
    pub fn start<T>(&mut self, period: T)
    where
        T: Into<u8>,
    {
        // Write the timeout configuration.
        self.wdt
            .config()
            .write(|w| unsafe { w.per().bits(period.into()) });
        #[hal_cfg(any("wdt-d11", "wdt-d21"))]
        {
            // Enable the watchdog timer.
            self.wdt.ctrl().write(|w| w.enable().set_bit());
            // Wait for watchdog timer to be enabled.
            while self.wdt.status().read().syncbusy().bit_is_set() {}
        }

        #[hal_cfg("wdt-d5x")]
        {
            // Enable the watchdog timer.
            self.wdt.ctrla().write(|w| w.enable().set_bit());
            // Wait for watchdog timer to be enabled.
            while self.wdt.syncbusy().read().enable().bit_is_set() {}
        }
    }
}
