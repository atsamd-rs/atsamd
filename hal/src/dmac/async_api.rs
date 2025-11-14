//! APIs for async DMAC operations.

use atsamd_hal_macros::hal_cfg;
use core::sync::atomic;

use crate::{
    async_hal::interrupts::{DMAC, Handler},
    dmac::{TriggerSource, waker::WAKERS},
    util::BitIter,
};

/// Interrupt handler for the DMAC peripheral.
pub struct InterruptHandler {
    _private: (),
}

impl crate::typelevel::Sealed for InterruptHandler {}

#[hal_cfg(any("dmac-d11", "dmac-d21"))]
impl Handler<DMAC> for InterruptHandler {
    unsafe fn on_interrupt() {
        // SAFETY: Here we can't go through the `with_chid` method to safely access
        // the different channel interrupt flags. Instead, we read the ID in a short
        // critical section, and make sure to RESET the CHID field to whatever
        // it was before this function ran.
        let dmac = unsafe { crate::pac::Peripherals::steal().dmac };

        critical_section::with(|_| {
            let old_id = dmac.chid().read().id().bits();
            let pending_interrupts = BitIter(dmac.intstatus().read().bits());

            // Iterate over channels and check their interrupt status
            for pend_channel in pending_interrupts {
                unsafe { dmac.chid().modify(|_, w| w.id().bits(pend_channel as u8)) };

                let wake = if dmac.chintflag().read().tcmpl().bit_is_set() {
                    // Transfer complete. Don't clear the flag, but
                    // disable the interrupt. Flag will be cleared when polled
                    dmac.chintenclr().modify(|_, w| w.tcmpl().set_bit());
                    true
                } else if dmac.chintflag().read().terr().bit_is_set() {
                    // Transfer error
                    dmac.chintenclr().modify(|_, w| w.terr().set_bit());
                    true
                } else {
                    false
                };

                if wake {
                    dmac.chctrla().modify(|_, w| w.enable().clear_bit());
                    dmac.chctrlb()
                        .modify(|_, w| w.trigsrc().variant(TriggerSource::Disable));

                    while dmac.chctrla().read().enable().bit_is_set() {
                        core::hint::spin_loop();
                    }

                    // Prevent the compiler from re-ordering read/write
                    // operations beyond this fence.
                    // (see https://docs.rust-embedded.org/embedonomicon/dma.html#compiler-misoptimizations)
                    atomic::fence(atomic::Ordering::Acquire); // ▼

                    WAKERS[pend_channel as usize].wake();
                }
            }

            // Reset the CHID.ID register
            unsafe {
                dmac.chid().write(|w| w.id().bits(old_id));
            }
        });
    }
}

#[hal_cfg("dmac-d5x")]
impl Handler<DMAC> for InterruptHandler {
    unsafe fn on_interrupt() {
        let dmac = unsafe { crate::pac::Peripherals::steal().dmac };

        let pending_channels = BitIter(dmac.intstatus().read().bits());
        for channel in pending_channels.map(|c| c as usize) {
            let wake = if dmac
                .channel(channel)
                .chintflag()
                .read()
                .tcmpl()
                .bit_is_set()
            {
                // Transfer complete. Don't clear the flag, but
                // disable the interrupt. Flag will be cleared when polled
                dmac.channel(channel)
                    .chintenclr()
                    .modify(|_, w| w.tcmpl().set_bit());
                true
            } else if dmac.channel(channel).chintflag().read().terr().bit_is_set() {
                // Transfer error
                dmac.channel(channel)
                    .chintenclr()
                    .modify(|_, w| w.terr().set_bit());
                true
            } else {
                false
            };

            if wake {
                dmac.channel(channel).chctrla().modify(|_, w| {
                    w.enable().clear_bit();
                    w.trigsrc().variant(TriggerSource::Disable)
                });

                while dmac.channel(channel).chctrla().read().enable().bit_is_set() {
                    core::hint::spin_loop();
                }

                // Prevent the compiler from re-ordering read/write
                // operations beyond this fence.
                // (see https://docs.rust-embedded.org/embedonomicon/dma.html#compiler-misoptimizations)
                atomic::fence(atomic::Ordering::Acquire); // ▼

                WAKERS[channel].wake();
            }
        }
    }
}
