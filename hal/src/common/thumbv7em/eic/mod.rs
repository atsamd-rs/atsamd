use crate::clock::EicClock;
use crate::target_device;

pub mod pin;

pub struct EIC {
    eic: target_device::EIC,
}

impl EIC {
    pub fn init(mclk: &mut target_device::MCLK, _clock: EicClock, eic: target_device::EIC) -> Self {
        mclk.apbamask.modify(|_, w| w.eic_().set_bit());

        eic.ctrla.modify(|_, w| w.enable().set_bit());
        while eic.syncbusy.read().enable().bit_is_set() {
            cortex_m::asm::nop();
        }

        EIC { eic }
    }
}
