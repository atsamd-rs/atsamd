use crate::{dmac::waker::WAKERS, util::BitIter};
use cortex_m::interrupt::InterruptNumber;
use cortex_m_interrupt::NvicInterruptRegistration;

#[cfg(any(feature = "samd11", feature = "samd21"))]
mod thumbv6m {
    use super::*;

    pub struct Interrupts<N>
    where
        N: InterruptNumber,
    {
        _interrupt_number: N,
    }

    impl<N> Interrupts<N>
    where
        N: InterruptNumber,
    {
        pub fn new<I: NvicInterruptRegistration<N>>(interrupt: I) -> Self {
            let interrupt_number = interrupt.number();
            interrupt.occupy(on_interrupt);
            unsafe { cortex_m::peripheral::NVIC::unmask(interrupt_number) };
            Self {
                _interrupt_number: interrupt_number,
            }
        }
    }

    fn on_interrupt() {
        // SAFETY: Here we can't go through the `with_chid` method to safely access
        // the different channel interrupt flags. Instead, we read the ID in a short
        // critical section, and make sure to RESET the CHID field to whatever
        // it was before this function ran.
        let dmac = unsafe { crate::pac::Peripherals::steal().DMAC };

        cortex_m::interrupt::free(|_| {
            let old_id = dmac.chid.read().id().bits();
            let pending_interrupts = BitIter(dmac.intstatus.read().bits());

            // Iterate over channels and check their interrupt status
            for pend_channel in pending_interrupts {
                unsafe { dmac.chid.modify(|_, w| w.id().bits(pend_channel as u8)) };

                let wake = if dmac.chintflag.read().tcmpl().bit_is_set() {
                    // Transfer complete. Don't clear the flag, but
                    // disable the interrupt. Flag will be cleared when polled
                    dmac.chintenclr.modify(|_, w| w.tcmpl().set_bit());
                    true
                } else if dmac.chintflag.read().terr().bit_is_set() {
                    // Transfer error
                    dmac.chintenclr.modify(|_, w| w.terr().set_bit());
                    true
                } else {
                    false
                };

                if wake {
                    dmac.chctrla.modify(|_, w| w.enable().clear_bit());
                    WAKERS[pend_channel as usize].wake();
                }
            }

            // Reset the CHID.ID register
            unsafe {
                dmac.chid.write(|w| w.id().bits(old_id));
            }
        });
    }
}

#[cfg(any(feature = "samd11", feature = "samd21"))]
pub use thumbv6m::*;

#[cfg(feature = "min-samd51g")]
mod thumbv7em {
    use super::*;

    pub struct Interrupts<N>
    where
        N: InterruptNumber,
    {
        _interrupt_0: N,
        _interrupt_1: N,
        _interrupt_2: N,
        _interrupt_3: N,
        _interrupt_other: N,
    }

    impl<N> Interrupts<N>
    where
        N: InterruptNumber,
    {
        pub fn new<N0, N1, N2, N3, NOther>(
            dmac_0: N0,
            dmac_1: N1,
            dmac_2: N2,
            dmac_3: N3,
            dmac_other: NOther,
        ) -> Self
        where
            N0: NvicInterruptRegistration<N>,
            N1: NvicInterruptRegistration<N>,
            N2: NvicInterruptRegistration<N>,
            N3: NvicInterruptRegistration<N>,
            NOther: NvicInterruptRegistration<N>,
        {
            let n_0 = dmac_0.number();
            dmac_0.occupy(on_interrupt_0);
            unsafe { cortex_m::peripheral::NVIC::unmask(n_0) };

            let n_1 = dmac_1.number();
            dmac_1.occupy(on_interrupt_1);
            unsafe { cortex_m::peripheral::NVIC::unmask(n_1) };

            let n_2 = dmac_2.number();
            dmac_2.occupy(on_interrupt_2);
            unsafe { cortex_m::peripheral::NVIC::unmask(n_2) };

            let n_3 = dmac_3.number();
            dmac_3.occupy(on_interrupt_3);
            unsafe { cortex_m::peripheral::NVIC::unmask(n_3) };

            let n_other = dmac_other.number();
            dmac_other.occupy(on_interrupt_other);
            unsafe { cortex_m::peripheral::NVIC::unmask(n_other) };

            Self {
                _interrupt_0: n_0,
                _interrupt_1: n_1,
                _interrupt_2: n_2,
                _interrupt_3: n_3,
                _interrupt_other: n_other,
            }
        }
    }

    fn on_interrupt(channel: usize) {
        let dmac = unsafe { crate::pac::Peripherals::steal().DMAC };

        let wake = if dmac.channel[channel].chintflag.read().tcmpl().bit_is_set() {
            // Transfer complete. Don't clear the flag, but
            // disable the interrupt. Flag will be cleared when polled
            dmac.channel[channel]
                .chintenclr
                .modify(|_, w| w.tcmpl().set_bit());
            true
        } else if dmac.channel[channel].chintflag.read().terr().bit_is_set() {
            // Transfer error
            dmac.channel[channel]
                .chintenclr
                .modify(|_, w| w.terr().set_bit());
            true
        } else {
            false
        };

        if wake {
            dmac.channel[channel]
                .chctrla
                .modify(|_, w| w.enable().clear_bit());
            WAKERS[channel].wake();
        }
    }

    fn on_interrupt_0() {
        const CHANNEL: usize = 0;
        on_interrupt(CHANNEL);
    }

    fn on_interrupt_1() {
        const CHANNEL: usize = 1;
        on_interrupt(CHANNEL);
    }

    fn on_interrupt_2() {
        const CHANNEL: usize = 2;
        on_interrupt(CHANNEL);
    }

    fn on_interrupt_3() {
        const CHANNEL: usize = 3;
        on_interrupt(CHANNEL);
    }

    fn on_interrupt_other() {
        let dmac = unsafe { crate::pac::Peripherals::steal().DMAC };

        // Get pending channels, but ignore first 4 since they're handled by other
        // interrupts.
        let pending_channels = BitIter(dmac.intstatus.read().bits() & !0b1111);
        for pend_chan in pending_channels {
            on_interrupt(pend_chan as usize);
        }
    }
}

#[cfg(feature = "min-samd51g")]
pub use thumbv7em::*;
