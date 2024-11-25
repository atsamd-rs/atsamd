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
        self.eic.ctrla().modify(|_, w| w.enable().clear_bit());
        self.enable_sync();
        fun(&mut self.eic);
        self.eic.ctrla().modify(|_, w| w.enable().set_bit());
        self.enable_sync();
    }

    /// Busy-wait until SYNCBUSY.ENABLE clears
    fn enable_sync(&mut self) {
        while self.eic.syncbusy().read().enable().bit_is_set() {
            core::hint::spin_loop();
        }
    }
}

#[cfg(feature = "async")]
pub(super) mod async_api {
    use super::*;
    use crate::async_hal::interrupts::Handler;
    use crate::eic::NUM_CHANNELS;
    use crate::util::BitIter;
    use embassy_sync::waitqueue::AtomicWaker;

    /// Interrupt handler used for `async` operations.
    pub struct InterruptHandler {
        _private: (),
    }

    impl crate::typelevel::Sealed for InterruptHandler {}

    seq_macro::seq!(N in 0..=15 {
        paste::paste! {

            impl Handler<crate::async_hal::interrupts::[<EIC_EXTINT_ ~N>]> for InterruptHandler {
                unsafe fn on_interrupt() {
                    let eic = unsafe { pac::Peripherals::steal().eic };

                    let pending_interrupts = BitIter(eic.intflag().read().bits());
                    for channel in pending_interrupts {
                        let mask = 1 << channel;
                        // Disable the interrupt but don't clear; will be cleared
                        // when future is next polled.
                        unsafe { eic.intenclr().write(|w| w.bits(mask)) };
                        WAKERS[channel as usize].wake();
                    }
                }
            }
        }
    });

    #[allow(clippy::declare_interior_mutable_const)]
    const NEW_WAKER: AtomicWaker = AtomicWaker::new();
    pub(super) static WAKERS: [AtomicWaker; NUM_CHANNELS] = [NEW_WAKER; NUM_CHANNELS];
}
