use crate::{clock::EicClock, pac};

pub mod pin;

pub struct EIC {
    eic: pac::EIC,
}

impl EIC {
    pub fn init(pm: &mut pac::PM, _clock: EicClock, eic: pac::EIC) -> Self {
        pm.apbamask.modify(|_, w| w.eic_().set_bit());

        eic.ctrl.modify(|_, w| w.enable().set_bit());
        while eic.status.read().syncbusy().bit_is_set() {
            cortex_m::asm::nop();
        }

        EIC { eic }
    }
}

#[cfg(feature = "async")]
mod async_api {
    use super::pin::NUM_CHANNELS;
    use super::*;
    use crate::async_hal::interrupts::{Binding, Handler, Interrupt, EIC as EicInterrupt};
    use crate::util::BitIter;
    use embassy_sync::waitqueue::AtomicWaker;

    pub struct InterruptHandler {
        _private: (),
    }

    impl Handler<EicInterrupt> for InterruptHandler {
        unsafe fn on_interrupt() {
            let eic = pac::Peripherals::steal().EIC;

            let pending_interrupts = BitIter(eic.intflag.read().bits());
            for channel in pending_interrupts {
                let mask = 1 << channel;
                // Disable the interrupt but don't clear; will be cleared
                // when future is next polled.
                eic.intenclr.write(|w| w.bits(mask));
                WAKERS[channel as usize].wake();
            }
        }
    }

    impl EIC {
        pub fn into_future<I>(self, _irq: I) -> EIC
        where
            I: Binding<EicInterrupt, InterruptHandler>,
        {
            EicInterrupt::unpend();
            unsafe { EicInterrupt::enable() };

            EIC { eic: self.eic }
        }
    }

    #[allow(clippy::declare_interior_mutable_const)]
    const NEW_WAKER: AtomicWaker = AtomicWaker::new();
    pub(super) static WAKERS: [AtomicWaker; NUM_CHANNELS] = [NEW_WAKER; NUM_CHANNELS];
}

pub use async_api::*;
