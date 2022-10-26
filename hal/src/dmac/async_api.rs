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
        interrupt_number: N,
    }

    impl<N> Interrupts<N>
    where
        N: InterruptNumber,
    {
        pub(crate) fn new(interrupt_number: N) -> Self {
            Self { interrupt_number }
        }
    }

    pub(in super::super) fn on_interrupt() {
        // SAFETY: Here we can't go through the `with_chid` method to safely access
        // the different channel interrupt flags. Instead, we read the ID in a short
        // critical section, and make sure to RESET the CHID field to whatever
        // it was before this function ran.
        let dmac = unsafe { crate::pac::Peripherals::steal().DMAC };

        cortex_m::interrupt::free(|_| {
            let intpend = &dmac.intpend;
            let old_id = intpend.read().id().bits();
            let pending_interrupts = BitIter(dmac.intstatus.read().bits());

            // TODO notify task that there is an error?

            // Iterate over channels and check their interrupt status
            for pend_channel in pending_interrupts {
                unsafe { intpend.modify(|_, w| w.id().bits(pend_channel as u8)) };

                let wake = if intpend.read().tcmpl().bit_is_set() {
                    // Transfer complete
                    intpend.modify(|_, w| w.tcmpl().set_bit());
                    true
                } else if intpend.read().terr().bit_is_set() {
                    // Transfer error
                    intpend.modify(|_, w| w.terr().set_bit());
                    true
                } else {
                    false
                };

                if wake {
                    WAKERS[pend_channel as usize].wake();
                }
            }

            // Reset the INTPEND.ID register
            unsafe {
                intpend.write(|w| w.id().bits(old_id));
            }
        });
    }
}

#[cfg(any(feature = "samd11", feature = "samd21"))]
pub use thumbv6m::*;
