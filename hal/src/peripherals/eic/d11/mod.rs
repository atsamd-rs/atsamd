mod pin;

#[cfg(feature = "async")]
pub(super) mod async_api {
    use crate::async_hal::interrupts::{Binding, Handler, Interrupt, EIC as EicInterrupt};
    use crate::eic::{Eic, EicFuture, NUM_CHANNELS};
    use crate::pac;
    use crate::util::BitIter;

    use core::marker::PhantomData;

    use embassy_sync::waitqueue::AtomicWaker;

    pub struct InterruptHandler {
        _private: (),
    }

    impl crate::typelevel::Sealed for InterruptHandler {}

    impl Handler<EicInterrupt> for InterruptHandler {
        unsafe fn on_interrupt() {
            let eic = pac::Peripherals::steal().eic;

            let pending_interrupts = BitIter(eic.intflag().read().bits());
            for channel in pending_interrupts {
                let mask = 1 << channel;
                // Disable the interrupt but don't clear; will be cleared
                // when future is next polled.
                eic.intenclr().write(|w| w.bits(mask));
                WAKERS[channel as usize].wake();
            }
        }
    }

    impl Eic {
        /// Turn an EIC pin into a pin usable as a [`Future`](core::future::Future).
        /// The correct interrupt source is needed.
        pub fn into_future<I>(self, _irq: I) -> Eic<EicFuture>
        where
            I: Binding<EicInterrupt, InterruptHandler>,
        {
            EicInterrupt::unpend();
            unsafe { EicInterrupt::enable() };

            Eic {
                eic: self.eic,
                _irqs: PhantomData,
            }
        }
    }

    #[allow(clippy::declare_interior_mutable_const)]
    const NEW_WAKER: AtomicWaker = AtomicWaker::new();
    pub(super) static WAKERS: [AtomicWaker; NUM_CHANNELS] = [NEW_WAKER; NUM_CHANNELS];
}
