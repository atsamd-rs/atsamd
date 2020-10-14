use crate::clock::EicClock;
use crate::target_device;

pub mod pin;

/// An External Interrupt Controller which is being configured.
pub struct ConfigurableEIC {
    eic: target_device::EIC,
}

impl ConfigurableEIC {
    fn new(eic: target_device::EIC) -> Self {
        Self { eic }
    }

    /// button_debounce_pins enables debouncing for the
    /// specified pins, with a configuration appropriate
    /// for debouncing physical buttons.
    pub fn button_debounce_pins(&mut self, debounce_pins: &[pin::ExternalInterruptID]) {
        self.eic.dprescaler.modify(|_, w| {
            w.tickon().set_bit()    // Use the 32k clock for debouncing.
            .states0().set_bit()    // Require 7 0 samples to see a falling edge.
            .states1().set_bit()    // Require 7 1 samples to see a rising edge.
            .prescaler0().div16()
            .prescaler1().div16()
        });

        let mut debounceen: u32 = 0;
        for pin in debounce_pins {
            debounceen |= 1 << *pin as u32;
        }
        self.eic.debouncen.write(|w| unsafe { w.bits(debounceen) });
    }

    /// finalize enables the EIC.
    pub fn finalize(self) -> EIC {
        self.into()
    }
}

/// init_with_ulp32k initializes the EIC and wires it up to the
/// ultra-low-power 32kHz clock source. finalize() must be called
/// before the EIC is ready for use.
pub fn init_with_ulp32k(
    mclk: &mut target_device::MCLK,
    _clock: EicClock,
    eic: target_device::EIC,
) -> ConfigurableEIC {
    mclk.apbamask.modify(|_, w| w.eic_().set_bit());

    eic.ctrla.modify(|_, w| w.swrst().set_bit());
    while eic.syncbusy.read().swrst().bit_is_set() {
        cortex_m::asm::nop();
    }

    // Use the low-power 32k clock.
    eic.ctrla.modify(|_, w| w.cksel().set_bit());

    ConfigurableEIC::new(eic)
}

/// A configured External Interrupt Controller.
pub struct EIC {
    _eic: target_device::EIC,
}

impl From<ConfigurableEIC> for EIC {
    fn from(eic: ConfigurableEIC) -> Self {
        eic.eic.ctrla.modify(|_, w| w.enable().set_bit());
        while eic.eic.syncbusy.read().enable().bit_is_set() {
            cortex_m::asm::nop();
        }

        Self { _eic: eic.eic }
    }
}
