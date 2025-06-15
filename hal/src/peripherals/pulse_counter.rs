//! # EIC Pulse counter
//!
//! This module combines an EIC channel, an EVSYS channel, and a TC peripheral
//! in order to count pulses form an EIC channel, whilst the CPU remains
//! asleep.
//!
//! Theoretically, using this system, it is possible to count pulses up to 50Mhz,
//! all without any CPU intervension.
//!
//! ## Example usage:
//!
//! ```no_run
//! let evsys = EvSysController::new(&mut mclk, pac_peripherals.evsys);
//! let evsys_channels = evsys.split();
//! let mut eic = Eic::new(&mut mclk, (&eic_clock).into(), pac_peripherals.eic);
//! let eic_channels = eic.split();
//!
//! let mut eic_pin = eic_channels.0.with_pin(pins.eic_pin.into_pull_down_interrupt());
//! eic_pin.sense(pac::eic::config::Sense0select::Fall); // Count on falling edge of signal
//! let (eic_pin, evsys_channel_eic_pin) = eic_pin.enable_event(evsys_channels.1); // Register EIC Event output
//!
//! let mut pulse_counter = CounterSettings::default()
//!     .stop_on_overflow(true) // Stops counting when 16bit limit reached
//!     .run_in_standby(true) // Continue counting when sleeping
//!     .build(pac_peripherals.tc_2, &pclk_tc2_tc3, &mut mclk, evsys_channel_eic_pin);
//!
//! pulse_counter.start();
//!
//! loop {
//!     let count = pulse_counter.count();
//!     pulse_counter.clear(); // Reset count back to 0
//! }
//!
//! pulse_counter.stop();
//! ```

use core::ops::Deref;

use crate::{
    evsys::{EvSysGenerator, EvSysUser},
    pac::{
        tc0::{self, count16::evctrl::Evactselect},
        Mclk,
    },
};
use atsamd_hal_macros::hal_cfg;

use super::{
    clock::v2::pclk::Pclk,
    evsys::{EvSysChannel, GenReady},
};

/// Counter settings
/// Default options
///
/// * Stop counting on max value reached (65535), and do not overflow
/// * Allow running in standby mode
pub struct PulseCounterBuilder {
    stop_on_overflow: bool,
    run_in_standby: bool,
}

impl Default for PulseCounterBuilder {
    fn default() -> Self {
        Self {
            stop_on_overflow: true,
            run_in_standby: true,
        }
    }
}

impl PulseCounterBuilder {
    pub fn run_in_standby(mut self, b: bool) -> Self {
        self.run_in_standby = b;
        self
    }

    pub fn stop_on_overflow(mut self, b: bool) -> Self {
        self.stop_on_overflow = b;
        self
    }

    /// Creates a new pulse counter out of a TC peripheral
    ///
    /// This consumes the evsys channel that monitors the GPIO that is to be monitored
    ///
    /// Once created, call [`PulseCounter::start`] to start monitoring
    ///
    /// ## Limitations
    /// 1. Can only work in 16 bit mode, so when monitoring very high frequency signals,
    ///    you will need to query the counter very frequently to avoid an overflow
    ///
    /// Returns the pulse counter instance
    pub fn build<PS, T, EvId, EvSrc>(
        self,
        tc: T::Instance,
        clock: &Pclk<T::ClockId, PS>,
        mclk: &mut Mclk,
        evsys_channel: EvSysChannel<EvId, GenReady<EvSrc>>,
    ) -> PulseCounter<T, EvId, EvSrc>
    where
        PS: crate::clock::v2::pclk::PclkSourceId,
        T: CounterInstance,
        EvId: crate::evsys::ChId,
        EvSrc: EvSysGenerator,
    {
        PulseCounter::new(tc, clock, self, mclk, evsys_channel)
    }
}

pub trait CounterInstance {
    const EVSYS_USR_EVT_ID: usize;
    type ClockId: crate::clock::v2::pclk::PclkId;
    type Instance: Deref<Target = tc0::RegisterBlock>;
    fn enable_mclk(mclk: &mut Mclk);
    fn disable_mclk(mclk: &mut Mclk);
}

impl<CNT: CounterInstance> EvSysUser for CNT {
    const USER_ID: usize = CNT::EVSYS_USR_EVT_ID;
}

pub struct PulseCounter<T: CounterInstance, EvId: crate::evsys::ChId, EvSrc: EvSysGenerator> {
    instance: T::Instance,
    evsys_channel: EvSysChannel<EvId, crate::evsys::Ready<EvSrc, T>>,
}

impl<T: CounterInstance, EvId: crate::evsys::ChId, EvSrc: EvSysGenerator>
    PulseCounter<T, EvId, EvSrc>
{
    pub(crate) fn new<PS: crate::clock::v2::pclk::PclkSourceId>(
        tc: T::Instance,
        _clock: &Pclk<T::ClockId, PS>,
        settings: PulseCounterBuilder,
        mclk: &mut Mclk,
        evsys_channel: EvSysChannel<EvId, GenReady<EvSrc>>,
    ) -> Self {
        // !!IMPORTANT!!
        // It is undocumented, but the peripherals user register MUST be set BEFORE enabling
        // the peripherals mclk - otherwise the user register becomes locked!
        let ready_channel = evsys_channel.register_user();
        T::enable_mclk(mclk);
        // Register the evsys channel to this peripheral's receiver
        {
            // Scoped whilst we are modifying count16
            let instance = tc.count16();
            // Then trigger SWRST
            instance.ctrla().write(|w| w.swrst().set_bit());
            while instance.syncbusy().read().bits() != 0 {}

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
            while instance.syncbusy().read().bits() != 0 {}
            instance.ctrla().modify(|_, w| {
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
    pub fn release(self, mclk: &mut Mclk) -> (T::Instance, EvSysChannel<EvId, GenReady<EvSrc>>) {
        T::disable_mclk(mclk);
        let instance = self.instance.count16();
        instance.ctrla().modify(|_, w| w.enable().clear_bit());
        while instance.syncbusy().read().bits() != 0 {}
        instance.ctrla().modify(|_, w| w.swrst().set_bit());
        while instance.syncbusy().read().bits() != 0 {}
        let unhooked = self.evsys_channel.deregister_user();
        (self.instance, unhooked)
    }

    /// Retreives the current number of pulses counted.
    ///
    /// This does not clear the counted value, to do that, call [PulseCounter::clear]
    pub fn count(&self) -> u16 {
        let instance = self.instance.count16();
        instance.ctrlbset().write(|w| w.cmd().readsync());
        self.sync();
        while instance.ctrlbset().read().cmd().bits() != 0 {
            core::hint::spin_loop();
        }
        let res = instance.count().read().bits();
        res
    }

    /// Set the counter value back to 0
    pub fn clear(&self) {
        let instance = self.instance.count16();
        instance.count().reset();
        self.sync();
    }

    fn sync(&self) {
        let count16 = self.instance.count16();
        while count16.syncbusy().read().bits() != 0 {
            core::hint::spin_loop();
        }
    }

    /// Stop pulse counting
    pub fn stop(&self) {
        self.instance
            .count16()
            .ctrla()
            .modify(|_, w| w.enable().clear_bit());
    }

    /// Start pulse counting
    pub fn start(&self) {
        self.instance
            .count16()
            .ctrla()
            .modify(|_, w| w.enable().set_bit());
    }
}

macro_rules! tc_pulse_counter {
    ($($TYPE:ident: ($TC:ident, $mclk:ident, $clock:ident, $apmask:ident, $evsysuser:literal),)+) => {
        $(
        pub struct $TYPE;

        impl CounterInstance for $TYPE {
            const EVSYS_USR_EVT_ID: usize = $evsysuser;
            type ClockId = crate::clock::v2::pclk::ids::$clock;
            type Instance = crate::pac::$TC;

            fn enable_mclk(mclk: &mut Mclk) {
                mclk.$apmask().write(|w| w.$mclk().set_bit());
            }

            fn disable_mclk(mclk: &mut Mclk) {
                mclk.$apmask().write(|w| w.$mclk().clear_bit());
            }
        }
        )+
    };
}

tc_pulse_counter! {
    Tc0PulseCounter: (Tc0, tc0_, Tc0Tc1, apbamask, 44),
    Tc1PulseCounter: (Tc1, tc1_, Tc0Tc1, apbamask, 45),
    Tc2PulseCounter: (Tc2, tc2_, Tc2Tc3, apbbmask, 46),
    Tc3PulseCounter: (Tc3, tc3_, Tc2Tc3, apbbmask, 47),
}

#[hal_cfg(all("tc4", "tc5"))]
tc_pulse_counter! {
    Tc4PulseCounter: (Tc4, tc4_, Tc4Tc5, apbcmask, 48),
    Tc5PulseCounter: (Tc5, tc5_, Tc4Tc5, apbcmask, 49),
}
#[hal_cfg(all("tc6", "tc7"))]
tc_pulse_counter! {
    Tc6PulseCounter: (Tc6, tc6_, Tc6Tc7, apbdmask, 50),
    Tc7PulseCounter: (Tc7, tc7_, Tc6Tc7, apbdmask, 51),
}
