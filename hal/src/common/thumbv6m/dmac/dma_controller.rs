//! # Functions to setup and use the DMA controller as a whole

pub use crate::target_device::dmac::chctrlb::{
    LVL_A as PriorityLevel, TRIGACT_A as TriggerAction, TRIGSRC_A as TriggerSource,
};

use super::{
    channel::{new_chan, Channel, Uninitialized},
    DESCRIPTOR_SECTION, WRITEBACK,
};
use crate::target_device::{DMAC, PM};

/// DMA Controller object. Wrapping the DMAC ensures we have a properly initialized
/// DMAC before attempting a transfer
pub struct DmaController {
    dmac: DMAC,
}

impl DmaController {
    /// Return a mutable reference to the underlying DMAC object exposed by the PAC.
    pub fn as_mut(&mut self) -> &mut DMAC {
        &mut self.dmac
    }
}

impl DmaController {
    /// Initialize the DMAC and return a useable DmaController object useable by [`DmaTransfer`](crate::transfer::DmaTransfer)'s
    pub fn init(dmac: DMAC, pm: &mut PM) -> Self {
        // ----- Initialize DMAC ----- //
        // Enable clocking
        pm.ahbmask.modify(|_, w| w.dmac_().set_bit());
        pm.apbbmask.modify(|_, w| w.dmac_().set_bit());

        // Reset the dmac and wait for reset to complete
        dmac.ctrl.modify(|_, w| w.swrst().set_bit());
        while !dmac.ctrl.read().swrst().bit_is_clear() {}

        // SAFETY this is safe because we write a whole u32 to 32-bit registers,
        // and the descriptor array addesses will never change since they are static.
        // We just need to ensure the writeback and descriptor_section addresses
        // are valid.
        unsafe {
            dmac.baseaddr
                .write(|w| w.baseaddr().bits(DESCRIPTOR_SECTION.as_ptr() as u32));
            dmac.wrbaddr
                .write(|w| w.wrbaddr().bits(WRITEBACK.as_ptr() as u32));
        }

        // ----- Select priority levels ----- //
        // TODO selectively enable priority levels
        // right now we blindly enable all priority levels
        dmac.ctrl.modify(|_, w| {
            w.lvlen3()
                .set_bit()
                .lvlen2()
                .set_bit()
                .lvlen1()
                .set_bit()
                .lvlen0()
                .set_bit()
        });

        // Enable DMA controller
        dmac.ctrl.modify(|_, w| w.dmaenable().set_bit());

        Self { dmac }
    }

    /// Split the DMAC into distinct channels
    pub fn split(&mut self) -> DmacChannels {
        DmacChannels(
            new_chan(),
            new_chan(),
            new_chan(),
            new_chan(),
            new_chan(),
            new_chan(),
            new_chan(),
            new_chan(),
            new_chan(),
            new_chan(),
            new_chan(),
            new_chan(),
        )
    }

    /// Release the DMAC and return the register block
    pub fn release(self, pm: &mut PM) -> DMAC {
        self.dmac.ctrl.modify(|_, w| w.dmaenable().clear_bit());

        // Disable the DMAC clocking
        pm.apbbmask.modify(|_, w| w.dmac_().clear_bit());
        pm.ahbmask.modify(|_, w| w.dmac_().clear_bit());

        // Release the DMAC
        self.dmac
    }
}

/// Struct generating individual handles to each DMA channel
pub struct DmacChannels(
    pub Channel<Uninitialized, 0>,
    pub Channel<Uninitialized, 1>,
    pub Channel<Uninitialized, 2>,
    pub Channel<Uninitialized, 3>,
    pub Channel<Uninitialized, 4>,
    pub Channel<Uninitialized, 5>,
    pub Channel<Uninitialized, 6>,
    pub Channel<Uninitialized, 7>,
    pub Channel<Uninitialized, 8>,
    pub Channel<Uninitialized, 9>,
    pub Channel<Uninitialized, 10>,
    pub Channel<Uninitialized, 11>,
);
