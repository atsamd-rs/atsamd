use core::marker::PhantomData;

use crate::async_hal::interrupts::Handler;


pub(super) mod waker {
    use embassy_sync::waitqueue::AtomicWaker;

    #[allow(clippy::declare_interior_mutable_const)]
    const NEW_WAKER: AtomicWaker = AtomicWaker::new();
    pub static ADC_WAKERS: [AtomicWaker; super::super::NUM_ADC] = [NEW_WAKER; super::super::NUM_ADC];
}

/// Interrupt handler for the ADC peripheral.
pub struct InterruptHandler<A: super::Adc> {
    _private: (),
    _adc: PhantomData<A>
}

impl<A: super::Adc> crate::typelevel::Sealed for InterruptHandler<A>{}

impl<A: super::Adc> Handler<A::Interrupt> for InterruptHandler<A> {
    unsafe fn on_interrupt() {
        let mut peripherals = unsafe { crate::pac::Peripherals::steal() };
        let adc = A::reg_block(&mut peripherals);
        critical_section::with(|_| {
            // Just check if result ready is set. Todo - Handle overrun and other interrupt reasons
            if adc.intflag().read().resrdy().bit_is_set() {
                adc.intflag().modify(|_, w| w.resrdy().set_bit());
                // Wake up!
                A::waker().wake();
            } else {
                // Handle other cases
            }
        })
    }
}