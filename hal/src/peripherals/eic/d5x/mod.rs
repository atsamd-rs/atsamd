use crate::{clock::EicClock, pac};

pub mod pin;

/// An External Interrupt Controller which is being configured.
pub struct ConfigurableEIC {
    eic: pac::Eic,
}

impl ConfigurableEIC {
    fn new(eic: pac::Eic) -> Self {
        Self { eic }
    }

    /// button_debounce_pins enables debouncing for the
    /// specified pins, with a configuration appropriate
    /// for debouncing physical buttons.
    pub fn button_debounce_pins(&mut self, debounce_pins: &[pin::ExternalInterruptID]) {
        self.eic.dprescaler().modify(|_, w| {
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
        self.eic
            .debouncen()
            .write(|w| unsafe { w.bits(debounceen) });
    }

    /// finalize enables the EIC.
    pub fn finalize(self) -> EIC {
        self.into()
    }
}

/// init_with_ulp32k initializes the EIC and wires it up to the
/// ultra-low-power 32kHz clock source. finalize() must be called
/// before the EIC is ready for use.
pub fn init_with_ulp32k(mclk: &mut pac::Mclk, _clock: EicClock, eic: pac::Eic) -> ConfigurableEIC {
    mclk.apbamask().modify(|_, w| w.eic_().set_bit());

    eic.ctrla().modify(|_, w| w.swrst().set_bit());
    while eic.syncbusy().read().swrst().bit_is_set() {
        cortex_m::asm::nop();
    }

    // Use the low-power 32k clock.
    eic.ctrla().modify(|_, w| w.cksel().set_bit());

    ConfigurableEIC::new(eic)
}

/// A configured External Interrupt Controller.
pub struct EIC {
    eic: pac::Eic,
}

impl EIC {
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

impl From<ConfigurableEIC> for EIC {
    fn from(eic: ConfigurableEIC) -> Self {
        eic.eic.ctrla().modify(|_, w| w.enable().set_bit());
        while eic.eic.syncbusy().read().enable().bit_is_set() {
            cortex_m::asm::nop();
        }

        Self { eic: eic.eic }
    }
}

#[cfg(feature = "async")]
mod async_api {
    use super::pin::NUM_CHANNELS;
    use super::*;
    use crate::async_hal::interrupts::Handler;
    use crate::util::BitIter;
    use embassy_sync::waitqueue::AtomicWaker;

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

#[cfg(feature = "async")]
pub use async_api::*;
