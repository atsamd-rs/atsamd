use crate::clock::EicClock;
use crate::target_device;

pub mod pin;

pub struct EIC {
    eic: target_device::EIC,
}

impl EIC {
    pub fn init(pm: &mut target_device::PM, _clock: EicClock, eic: target_device::EIC) -> Self {
        pm.apbamask.modify(|_, w| w.eic_().set_bit());

        eic.ctrl.modify(|_, w| w.enable().set_bit());
        while eic.status.read().syncbusy().bit_is_set() {
            cortex_m::asm::nop();
        }

        EIC { eic }
    }
}
