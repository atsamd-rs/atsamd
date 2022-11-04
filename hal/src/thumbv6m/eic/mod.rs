use crate::{clock::EicClock, pac, typelevel::NoneT};

pub mod pin;

pub struct EIC<N = NoneT> {
    eic: pac::EIC,
    _irq_number: N,
}

impl EIC {
    pub fn init(pm: &mut pac::PM, _clock: EicClock, eic: pac::EIC) -> Self {
        pm.apbamask.modify(|_, w| w.eic_().set_bit());

        eic.ctrl.modify(|_, w| w.enable().set_bit());
        while eic.status.read().syncbusy().bit_is_set() {
            cortex_m::asm::nop();
        }

        EIC {
            eic,
            _irq_number: NoneT,
        }
    }
}

#[cfg(feature = "async")]
mod async_api {
    use super::pin::NUM_CHANNELS;
    use super::*;
    use crate::util::BitIter;
    use embassy_sync::waitqueue::AtomicWaker;

    impl EIC {
        pub fn into_future<I, Q>(self, irq: I) -> EIC<Q>
        where
            I: cortex_m_interrupt::NvicInterruptRegistration<Q>,
            Q: cortex_m::interrupt::InterruptNumber,
        {
            let irq_number = irq.number();
            irq.occupy(async_api::on_interrupt);
            unsafe {
                cortex_m::peripheral::NVIC::unmask(irq_number);
            }

            EIC {
                eic: self.eic,
                _irq_number: irq_number,
            }
        }
    }

    pub(super) fn on_interrupt() {
        let eic = unsafe { pac::Peripherals::steal().EIC };

        let pending_interrupts = BitIter(eic.intflag.read().bits());
        for channel in pending_interrupts {
            let mask = 1 << channel;
            // Disable the interrupt but don't clear; will be cleared
            // when future is next polled.
            unsafe { eic.intenclr.write(|w| w.bits(mask)) };
            WAKERS[channel as usize].wake();
        }
    }

    #[allow(clippy::declare_interior_mutable_const)]
    const NEW_WAKER: AtomicWaker = AtomicWaker::new();
    pub(super) static WAKERS: [AtomicWaker; NUM_CHANNELS] = [NEW_WAKER; NUM_CHANNELS];
}
