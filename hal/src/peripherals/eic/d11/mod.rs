use crate::clock::EicClock;
use crate::pac;

pub mod pin;

pub struct EIC {
    eic: pac::Eic,
}

impl EIC {
    pub fn init(pm: &mut pac::Pm, _clock: EicClock, eic: pac::Eic) -> Self {
        pm.apbamask().modify(|_, w| w.eic_().set_bit());

        eic.ctrl().modify(|_, w| w.enable().set_bit());
        while eic.status().read().syncbusy().bit_is_set() {
            cortex_m::asm::nop();
        }

        EIC { eic }
    }
}
