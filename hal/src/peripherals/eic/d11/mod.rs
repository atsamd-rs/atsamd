use super::{ChId, Channel};
use crate::pac;
mod pin;

impl<Id: ChId, F> Channel<Id, F> {
    /// Run the provided closure with the EIC peripheral disabled. The
    /// enable-protected registers, such as CONFIGx, should be accessed through
    /// this method.
    ///
    /// # Caution
    ///
    /// You should not re-enable the provided EIC PAC object inside the provided
    /// closure.
    fn with_disable(&mut self, fun: impl Fn(&mut pac::Eic)) {
        self.eic.ctrl().modify(|_, w| w.enable().clear_bit());
        self.enable_sync();
        fun(&mut self.eic);
        self.eic.ctrl().modify(|_, w| w.enable().set_bit());
        self.enable_sync();
    }

    /// Busy-wait until SYNCBUSY.ENABLE clears
    fn enable_sync(&mut self) {
        while self.eic.status().read().syncbusy().bit_is_set() {
            core::hint::spin_loop();
        }
    }
}

#[cfg(feature = "async")]
pub(super) mod async_api {
    use crate::async_hal::interrupts::{Binding, EIC as EicInterrupt, Handler, Interrupt};
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
            let eic = unsafe { pac::Peripherals::steal().eic };

            let pending_interrupts = BitIter(eic.intflag().read().bits());
            for channel in pending_interrupts {
                let mask = 1 << channel;
                // Disable the interrupt but don't clear; will be cleared
                // when future is next polled.
                eic.intenclr().write(|w| unsafe { w.bits(mask) });
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
