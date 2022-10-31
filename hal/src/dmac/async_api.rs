use crate::dmac::waker::WAKERS;
use cortex_m::interrupt::InterruptNumber;

// BitIter shamelessly stolen from embassy:
// https://github.com/embassy-rs/embassy/blob/3d1501c02038e5fe6f6d3b72bd18bd7a52595a77/embassy-stm32/src/exti.rs#L67
struct BitIter(u32);

impl Iterator for BitIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.trailing_zeros() {
            32 => None,
            b => {
                self.0 &= !(1 << b);
                Some(b)
            }
        }
    }
}

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
        pub(crate) fn new(interrupt_number: N) -> Self {
            Self {
                _interrupt_number: interrupt_number,
            }
        }
    }

    pub(in super::super) fn on_interrupt() {
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
