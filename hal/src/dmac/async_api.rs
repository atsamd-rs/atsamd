//! APIs for async DMAC operations.

use crate::{
    async_hal::interrupts::Handler,
    dmac::{waker::WAKERS, TriggerSource},
    util::BitIter,
};

#[cfg(feature = "thumbv6")]
use crate::async_hal::interrupts::DMAC;

#[cfg(feature = "thumbv7")]
use crate::async_hal::interrupts::{DMAC_0, DMAC_1, DMAC_2, DMAC_3, DMAC_OTHER};

// Interrupt handler for the DMAC peripheral.
pub struct InterruptHandler {
    _private: (),
}

#[cfg(feature = "thumbv6")]
impl Handler<DMAC> for InterruptHandler {
    unsafe fn on_interrupt() {
        // SAFETY: Here we can't go through the `with_chid` method to safely access
        // the different channel interrupt flags. Instead, we read the ID in a short
        // critical section, and make sure to RESET the CHID field to whatever
        // it was before this function ran.
        let dmac = unsafe { crate::pac::Peripherals::steal().DMAC };

        critical_section::with(|_| {
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
                    dmac.chctrlb
                        .modify(|_, w| w.trigsrc().variant(TriggerSource::DISABLE));
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

#[cfg(feature = "thumbv7")]
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
        dmac.channel[channel].chctrla.modify(|_, w| {
            w.enable().clear_bit();
            w.trigsrc().variant(TriggerSource::DISABLE)
        });
        WAKERS[channel].wake();
    }
}

#[cfg(feature = "thumbv7")]
impl Handler<DMAC_0> for InterruptHandler {
    unsafe fn on_interrupt() {
        on_interrupt(0);
    }
}

#[cfg(feature = "thumbv7")]
impl Handler<DMAC_1> for InterruptHandler {
    unsafe fn on_interrupt() {
        on_interrupt(1);
    }
}

#[cfg(feature = "thumbv7")]
impl Handler<DMAC_2> for InterruptHandler {
    unsafe fn on_interrupt() {
        on_interrupt(2);
    }
}

#[cfg(feature = "thumbv7")]
impl Handler<DMAC_3> for InterruptHandler {
    unsafe fn on_interrupt() {
        on_interrupt(3);
    }
}

#[cfg(feature = "thumbv7")]
impl Handler<DMAC_OTHER> for InterruptHandler {
    unsafe fn on_interrupt() {
        let dmac = unsafe { crate::pac::Peripherals::steal().DMAC };

        // Get pending channels, but ignore first 4 since they're handled by other
        // interrupts.
        let pending_channels = BitIter(dmac.intstatus.read().bits() & !0b1111);
        for pend_chan in pending_channels {
            on_interrupt(pend_chan as usize);
        }
    }
}
