use core::ops::Deref;

#[hal_cfg("clock-d11")]
use crate::pac::tc1 as tc;
#[hal_cfg("clock-d21")]
use crate::pac::tc3 as tc;

use crate::pac::Pm;
use atsamd_hal_macros::hal_cfg;
use tc::count16::evctrl::Evactselect;

use super::evsys::{EvSysChannel, GenReady};

/// Counter settings
/// Default options
///
/// * Stop counting on max value reached (65535), and do not overflow
/// * Allow running in standby mode
pub struct CounterSettings {
    stop_on_overflow: bool,
    run_in_standby: bool,
}

impl Default for CounterSettings {
    fn default() -> Self {
        Self {
            stop_on_overflow: true,
            run_in_standby: true,
        }
    }
}

impl CounterSettings {
    pub fn run_in_standby(mut self, b: bool) -> Self {
        self.run_in_standby = b;
        self
    }

    pub fn stop_on_overflow(mut self, b: bool) -> Self {
        self.stop_on_overflow = b;
        self
    }
}

pub trait CounterInstance {
    const EVSYS_USR_EVT_ID: usize;
    type Instance: Deref<Target = tc::RegisterBlock>;
    type Clock;
    fn enable_pm(pm: &mut Pm);
    fn disable_pm(pm: &mut Pm);
}

pub struct PulseCounter<T: CounterInstance, EvId: crate::evsys::ChId> {
    instance: T::Instance,
    evsys_channel: EvSysChannel<EvId, crate::evsys::Ready>,
}

impl<T: CounterInstance, EvId: crate::evsys::ChId> PulseCounter<T, EvId> {
    /// Creates a new pulse counter out of a TC peripheral
    ///
    /// This consumes the evsys channel that monitors the GPIO that is to be monitored
    ///
    /// Once created, call [`PulseCounter::start`] to start monitoring
    ///
    /// ## Limitations
    /// 1. Can only work in 16 bit mode, so when monitoring very high frequency signals,
    ///    you will need to query the counter very frequently
    ///
    /// Returns the pulse counter instance
    pub fn new(
        tc: T::Instance,
        _clock: &T::Clock,
        settings: CounterSettings,
        pm: &mut Pm,
        evsys_channel: EvSysChannel<EvId, GenReady>,
    ) -> Self {
        // !!IMPORTANT!!
        // It is undocumented, but the peripherals user register MUST be set BEFORE enabling
        // the peripherals mclk - otherwise the user register becomes locked!
        let ready_channel = evsys_channel.register_user(T::EVSYS_USR_EVT_ID);
        T::enable_pm(pm);
        // Register the evsys channel to this peripheral's receiver
        {
            // Scoped whilst we are modifying count16
            let instance = tc.count16();
            // Then trigger SWRST
            instance.ctrla().write(|w| w.swrst().set_bit());
            while instance.status().read().syncbusy().bit_is_set() {}

            // Enable reading off the EVSYS channel
            instance.evctrl().write(|w| {
                w.tcei().set_bit(); // Enable incomming events
                w.evact().variant(Evactselect::Count) // Count up on event
            });
            instance.count().reset();
            if settings.stop_on_overflow {
                instance.ctrlbclr().write(|w| w.oneshot().set_bit()); // Clear oneshot (Allow overflow)
            } else {
                instance.ctrlbset().write(|w| w.oneshot().set_bit()); // Set oneshhot (Stop on overflow)
            }
            while instance.status().read().syncbusy().bit_is_set() {}
            instance.ctrla().write(|w| {
                if settings.run_in_standby {
                    w.runstdby().set_bit();
                }
                w.prescaler().div1()
            });
        }
        let s = Self {
            instance: tc,
            evsys_channel: ready_channel,
        };
        s
    }

    /// Stop pulse counting and release the TC peripheral and EVSYS channel
    pub fn release(self, pm: &mut Pm) -> (T::Instance, EvSysChannel<EvId, GenReady>) {
        T::disable_pm(pm);
        let instance = self.instance.count16();
        instance.ctrla().modify(|_, w| w.enable().clear_bit());
        while instance.status().read().syncbusy().bit_is_set() {}
        instance.ctrla().modify(|_, w| w.swrst().set_bit());
        while instance.status().read().syncbusy().bit_is_set() {}
        let unhooked = self.evsys_channel.deregister_user(T::EVSYS_USR_EVT_ID);
        (self.instance, unhooked)
    }

    /// Retreives the current number of pulses counted.
    ///
    /// This does not clear the counted value, to do that, call [PulseCounter::clear]
    pub fn count(&self) -> u16 {
        let instance = self.instance.count16();
        instance.count().read().bits()
    }

    /// Set the counter value back to 0
    pub fn clear(&self) {
        let instance = self.instance.count16();
        instance.count().reset();
        self.sync();
    }

    fn sync(&self) {
        let count16 = self.instance.count16();
        while count16.status().read().syncbusy().bit_is_set() {
            core::hint::spin_loop();
        }
    }

    /// Stop pulse counting
    pub fn stop(&self) {
        self.instance
            .count16()
            .ctrla()
            .write(|w| w.enable().clear_bit());
    }

    /// Start pulse counting
    pub fn start(&self) {
        self.instance
            .count16()
            .ctrla()
            .write(|w| w.enable().set_bit());
    }
}

macro_rules! tc_pulse_counter {
    ($($TYPE:ident: ($TC:ident, $mclk:ident, $clock:ident, $apmask:ident, $evsysuser:literal),)+) => {
        $(
        pub struct $TYPE;

        impl CounterInstance for $TYPE {
            const EVSYS_USR_EVT_ID: usize = $evsysuser;
            type Clock = crate::clock::$clock;
            type Instance = crate::pac::$TC;

            fn enable_pm(pm: &mut Pm) {
                pm.$apmask().write(|w| w.$mclk().set_bit());
            }

            fn disable_pm(pm: &mut Pm) {
                pm.$apmask().write(|w| w.$mclk().clear_bit());
            }
        }
        )+
    };
}

// D11
#[hal_cfg(all("clock-d11", "tc1"))]
tc_pulse_counter! {Tc1PulseCounter: (Tc1, tc1_, Tc1Tc2Clock, apbcmask, 0x0A),}
#[hal_cfg(all("clock-d11", "tc2"))]
tc_pulse_counter! {Tc2PulseCounter: (Tc2, tc2_, Tc1Tc2Clock, apbcmask, 0x0B),}

// D21
#[hal_cfg(all("clock-d21", "tc3"))]
tc_pulse_counter! {Tc3PulseCounter: (Tc3, tc3_, Tcc2Tc3Clock, apbcmask, 0x12),}

#[hal_cfg(all("clock-d21", "tc4"))]
tc_pulse_counter! {Tc4PulseCounter: (Tc4, tc4_, Tc4Tc5Clock, apbcmask, 0x13),}

#[hal_cfg(all("clock-d21", "tc5"))]
tc_pulse_counter! {Tc5PulseCounter: (Tc5, tc5_, Tc4Tc5Clock, apbcmask, 0x14),}

#[hal_cfg(all("clock-d21", "tc6"))]
tc_pulse_counter! {Tc6PulseCounter: (Tc6, tc6_, Tc6Tc7Clock, apbcmask, 0x15),}

#[hal_cfg(all("clock-d21", "tc7"))]
tc_pulse_counter! {Tc7PulseCounter: (Tc7, tc7_, Tc6Tc7Clock, apbcmask, 0x16),}
