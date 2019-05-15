//! Direct Memory Access channels
//!
//! Adapted from the Adafruit ZeroDMA project:
//!
//! <https://github.com/adafruit/Adafruit_ZeroDMA>

// The MIT License (MIT)
//
// Copyright (c) 2016 Adafruit Industries
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use super::controller::*;
use super::descriptor::{Descriptor, DescriptorList};
use super::error::Error;
#[cfg(feature = "samd51")]
use crate::target_device::dmac::chctrla::{TRIGACTW, TRIGSRCW};
#[cfg(not(feature = "samd51"))]
use crate::target_device::dmac::chctrlb::{TRIGACTW, TRIGSRCW};
use crate::target_device::NVIC;
use core::mem;
use vcell::VolatileCell;

/// (DMAC_BTCTRL) Source Address Increment Enable
const BTCTRL_SRCINC: u16 = 1 << 10;

/// (DMAC_BTCTRL) Destination Address Increment Enable
const BTCTRL_DSTINC: u16 = 1 << 11;

/// (DMAC_BTCTRL) Step Selection
const BTCTRL_STEPSEL: u16 = 1 << 12;

/// (DMAC_BTCTRL) Beat Size Mask
const BTCTRL_BEATSIZE_MASK: u16 = 3 << BTCTRL_BEATSIZE_POS;

/// (DMAC_BTCTRL) Beat Size Mask
const BTCTRL_BEATSIZE_POS: u16 = 8;

/// (DMAC_BTCTRL) Step Size Mask
const BTCTRL_STEPSIZE_MASK: u16 = 7 << BTCTRL_STEPSIZE_POS;

/// (DMAC_BTCTRL) Step Size Mask
const BTCTRL_STEPSIZE_POS: u16 = 13;

/// (DMAC_BTCTRL) Descriptor Valid
const BTCTRL_VALID: u16 = 1;

/// Channel Transfer Error Interrupt Enable
const CHINTENCLR_TERR: u8 = 1;

/// Channel Transfer Complete Interrupt Enable
const CHINTENCLR_TCMPL: u8 = 1 << 1;

/// Channel Suspend Interrupt Enable
const CHINTENCLR_SUSP: u8 = 1 << 2;

/// Disable event output
const EVENT_OUTPUT_DISABLE: u16 = 0;

/// Channel IDs
pub type Id = u8;

/// Channel mask values
pub(super) type Mask = u32;

/// DMA callback types
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CallbackType {
    /// Transfer errors are flagged if a bus error is detected during an AHB
    /// access or when he DMAC fetches an invalid descriptor.
    TransferError,

    /// Transfer finished successfully
    TransferDone,

    /// Channel suspended
    ChannelSuspend,
}

/// DMA callbacks
#[derive(Copy, Clone, Default)]
pub struct Callbacks {
    /// Transfer error
    pub transfer_error: Option<fn(&mut Channel)>,

    /// Transfer done
    pub transfer_done: Option<fn(&mut Channel)>,

    /// Channel suspended
    pub channel_suspend: Option<fn(&mut Channel)>,
}

/// Direct Memory Access channel
pub struct Channel<'dmac, 'desc> {
    /// DMA controller
    controller: &'dmac Controller,

    /// Channel ID
    channel_id: Id,

    /// Job status
    job_status: VolatileCell<Result<(), Error>>,

    /// Has descriptors?
    has_descriptors: bool,

    /// Loop flag
    loop_flag: bool,

    /// Peripheral trigger
    peripheral_trigger: TRIGSRCW,

    /// Trigger action
    trigger_action: TRIGACTW,

    /// Callbacks
    callbacks: Callbacks,

    /// Descriptor list
    descriptor_list: &'desc DescriptorList,
}

impl<'dmac, 'desc> Channel<'dmac, 'desc> {
    /// Initialize a DMA channel
    pub fn new(
        controller: &'dmac Controller,
        channel_id: Id,
        descriptor_list: &'desc DescriptorList,
    ) -> Result<Self, Error> {
        let mut dma = Self {
            controller,
            channel_id,
            job_status: VolatileCell::new(Ok(())),
            has_descriptors: false, // No descriptors allocated yet
            loop_flag: false,
            peripheral_trigger: TRIGSRCW::DISABLE, // Software trigger only by default
            trigger_action: TRIGACTW::TRANSACTION,
            callbacks: Callbacks::default(),
            descriptor_list,
        };

        dma.allocate()?;
        Ok(dma)
    }

    // TODO(adafruit): Drop impl? Should stop job, delete descriptors, free channel.

    /// IRQ handler
    pub fn irq_handler(&mut self, flags: u8) {
        // TODO(tarcieri): the original ZeroDMA didn't lock here. Is it ok?
        let dmac = self.controller.lock();

        // On SAMD51, 'flags' is initially passed in as channel number,
        // from which we look up the actual interrupt flags...
        let (flags, channel_id) = dmac.read_channel_id_and_flags(flags);

        // TODO(tarcieri): support for more than one DMA channel
        assert_eq!(
            self.channel_id, channel_id,
            "received event for a different DMA channel: {}",
            channel_id
        );

        if flags & CHINTENCLR_TERR != 0 {
            dmac.clear_transfer_error_flag(channel_id);
            self.job_status.set(Err(Error::Io));

            if let Some(cb) = self.callbacks.transfer_error {
                cb(self);
            }
        } else if flags & CHINTENCLR_TCMPL != 0 {
            dmac.clear_transfer_complete_flag(channel_id);
            self.job_status.set(Ok(()));

            if let Some(cb) = self.callbacks.transfer_done {
                cb(self);
            }
        } else if flags & CHINTENCLR_SUSP != 0 {
            dmac.clear_suspend_flag(channel_id);
            self.job_status.set(Err(Error::Suspend));

            if let Some(cb) = self.callbacks.channel_suspend {
                cb(self)
            }
        }
    }

    /// Allocates channel for DMA object
    pub fn allocate(&mut self) -> Result<(), Error> {
        let dmac = self.controller.lock();

        // As currently written, this "does not play well with others" as it
        // assumes the channel mask is the final arbiter of channels in use
        // (this is true only within this code -- but other DMA-driven code
        // may have allocated its own channel(s) elsewhere, sometimes with an
        // equally broken approach).
        //
        // A possible (untested) alternate approach might be to loop through
        // each channel, set `cmac.chid`'s ID and then test whether `chctrl`'s
        // ENABLE is set.
        dmac.allocate_channel(self.channel_id)?;

        // Clear software trigger
        dmac.registers()
            .swtrigctrl
            .modify(|reg, value| unsafe { value.bits(reg.bits() & !(1 << self.channel_id)) });

        // This type unfortunately does not impl Copy or Clone
        let peripheral_trigger = match self.peripheral_trigger {
            TRIGSRCW::DISABLE => TRIGSRCW::DISABLE,
        };

        // Ditto for this type
        let trigger_action = match self.trigger_action {
            #[cfg(not(feature = "samd51"))]
            TRIGACTW::BEAT => TRIGACTW::BEAT,
            TRIGACTW::BLOCK => TRIGACTW::BLOCK,
            #[cfg(feature = "samd51")]
            TRIGACTW::BURST => TRIGACTW::BURST,
            TRIGACTW::TRANSACTION => TRIGACTW::TRANSACTION,
        };

        // Configure default behaviors
        dmac.configure_default_behaviors(self.channel_id, peripheral_trigger, trigger_action);

        Ok(())
    }

    /// Set DMA priority
    pub fn set_priority(&mut self, priority: Priority) {
        // TODO(tarcieri): confirm it's ok to acquire/hold a lock here
        // The original Adafruit code did not
        let dmac = self.controller.lock();
        dmac.set_priority(self.channel_id, priority);
    }

    /// Deallocate DMA channel
    // TODO(adafruit): should this delete/deallocate the descriptor list?
    pub fn free(&mut self, clock_source: &mut ClockSource, nvic: &mut NVIC) -> Result<(), Error> {
        // job_status is volatile
        let dmac = self.controller.lock();

        // Can't leave when busy
        if self.job_status.get() == Err(Error::Busy) {
            return Err(Error::Busy);
        }

        dmac.release_channel(self.channel_id, clock_source, nvic)
    }

    /// Start DMA transfer job.
    pub fn start_job(&mut self) -> Result<(), Error> {
        // job_status is volatile
        let dmac = self.controller.lock();

        if self.job_status.get() == Err(Error::Busy) {
            // Resource is busy
            return Err(Error::Busy);
        }

        if !self.has_descriptors
            || self.descriptor_list.descriptor(self.channel_id).btcnt.get() <= 0
        {
            return Err(Error::InvalidArg); // Bad transfer size
        }

        let mut interrupt_mask = 0;

        if self.callbacks.transfer_error.is_some() {
            interrupt_mask |= 1;
        }

        if self.callbacks.transfer_done.is_some() {
            interrupt_mask |= 1 << 1;
        }

        if self.callbacks.channel_suspend.is_some() {
            interrupt_mask |= 1 << 2;
        }

        self.job_status.set(Err(Error::Busy));
        dmac.set_interrupt_mask(self.channel_id, interrupt_mask);

        Ok(())
    }

    /// Set and enable callback function for DMA object. This can be called
    /// before or after channel and/or descriptors are allocated, but needs
    /// to be called before job is started.
    pub fn set_callback(&mut self, cb_type: CallbackType, cb: fn(&mut Channel)) {
        match cb_type {
            CallbackType::TransferError => self.callbacks.transfer_error = Some(cb),
            CallbackType::TransferDone => self.callbacks.transfer_done = Some(cb),
            CallbackType::ChannelSuspend => self.callbacks.channel_suspend = Some(cb),
        }
    }

    /// Abort
    pub fn abort(&mut self) {
        let dmac = self.controller.lock();
        dmac.abort(self.channel_id);
        self.job_status.set(Err(Error::Aborted));
    }

    /// Set DMA peripheral trigger.
    /// This can be done before or after channel is allocated.
    pub fn set_trigger(&mut self, trigger: TRIGSRCW) {
        let dmac = self.controller.lock();

        // Save value for `allocate()`
        self.peripheral_trigger = match self.peripheral_trigger {
            TRIGSRCW::DISABLE => TRIGSRCW::DISABLE,
        };

        dmac.set_trigger(self.channel_id, trigger);
    }

    /// Set DMA trigger action.
    /// This can be done before or after channel is allocated.
    pub fn set_action(&mut self, action: TRIGACTW) {
        let dmac = self.controller.lock();

        // Save value for `allocate()`. Hax because TRIGACTW is non-Copy/Clone
        self.trigger_action = match action {
            #[cfg(not(feature = "samd51"))]
            TRIGACTW::BEAT => TRIGACTW::BEAT,
            TRIGACTW::BLOCK => TRIGACTW::BLOCK,
            #[cfg(feature = "samd51")]
            TRIGACTW::BURST => TRIGACTW::BURST,
            TRIGACTW::TRANSACTION => TRIGACTW::TRANSACTION,
        };

        dmac.set_action(self.channel_id, action);
    }

    /// Issue software trigger.
    ///
    /// Channel must be allocated and descriptors added!
    pub fn trigger(&mut self) {
        if self.has_descriptors {
            // TODO(tarcieri): confirm it's ok to acquire/hold a lock here
            // The original Adafruit code did not
            let dmac = self.controller.lock();

            unsafe {
                dmac.registers()
                    .swtrigctrl
                    .modify(|r, w| w.bits(r.bits() | 1 << self.channel_id));
            }
        }
    }

    /// Get the channel ID for this channel
    pub fn get_channel_id(&self) -> Id {
        self.channel_id
    }

    // DMA DESCRIPTOR FUNCTIONS ------------------------------------------------

    // Allocates a new DMA descriptor (if needed) and appends it to the
    // channel's descriptor list.  Returns pointer to Descriptor,
    // or NULL on various errors.  You'll want to keep the pointer for
    // later if you need to modify or free the descriptor.
    // Channel must be allocated first!
    pub fn add_descriptor(
        &mut self,
        src: *const u8,
        dst: *mut u8,
        count: u16,
        size: BeatSize,
        src_inc: bool,
        dst_inc: bool,
        step_size: u32,
        step_sel: bool,
    ) -> Option<&Descriptor> {
        // Can't do while job's busy
        if self.job_status.get() == Err(Error::Busy) {
            return None;
        }

        // Scan descriptor list to find last entry.  If an entry's
        // DESCADDR value is 0, that's the end of the list and it's
        // currently un-looped.  If the DESCADDR value is the same
        // as the first entry, that's the end of the list and it's
        // looped.  Either way, set the last entry's DESCADDR value
        // to the new descriptor, and the descriptor's own DESCADDR
        // will be set later either to 0 or the list head.
        let desc = if self.has_descriptors {
            // TODO(tarcieri): allocate new descriptor
            // let desc = [...];
            // let mut prev = unsafe { &DESCRIPTOR[channel] };
            //
            // while prev.descaddr.reg && prev.descaddr.reg != unsafe { &DESCRIPTOR[channel]
            // } {    prev = prev.descaddr.reg;
            // }
            //
            // prev.descaddr.reg = desc;
            // desc
            unimplemented!();
        } else {
            self.descriptor_list.descriptor(self.channel_id)
        };

        self.has_descriptors = true;

        // TODO(tarcieri): verify all of these reprs are correct
        let mut btctrl = BTCTRL_VALID
            | EVENT_OUTPUT_DISABLE
            | ((0x3 << 8) & ((size as u16) << 8)) as u16
            | ((0x7 << 13) & (step_size << 13)) as u16;

        if src_inc {
            btctrl |= BTCTRL_SRCINC;
        }

        if dst_inc {
            btctrl |= BTCTRL_DSTINC;
        }

        if step_sel {
            btctrl |= BTCTRL_STEPSEL;
        }

        desc.btctrl.set(btctrl);
        desc.btcnt.set(count);
        desc.srcaddr.set(src as u32);

        let mut srcaddr = src as u32;

        if src_inc {
            let mut offset = size.bytes_per_beat() as u16 * count;

            if step_sel {
                offset *= 1 << step_size;
            }

            srcaddr += offset as u32;
        }

        desc.srcaddr.set(srcaddr);

        let mut dstaddr = dst as u32;

        if dst_inc {
            let mut offset = size.bytes_per_beat() as u16 * count;

            if !step_sel {
                offset *= 1 << step_size;
            }

            dstaddr += offset as u32;
        }

        desc.dstaddr.set(dstaddr);

        desc.descaddr.set(if self.loop_flag {
            (self.descriptor_list.descriptor(self.channel_id) as *const Descriptor) as u32
        } else {
            0
        });

        Some(desc)
    }

    /// Modify DMA descriptor with a new source address, destination address &
    /// block transfer count.  All other attributes (including increment
    /// enables, etc.) are unchanged.  Mostly for changing the data being
    /// pushed to a peripheral (DAC, SPI, whatev.)
    pub fn change_descriptor(
        &mut self,
        desc: &mut Descriptor,
        src: *const u8,
        dst: *mut u8,
        count: u16,
    ) {
        // Read the current beat transfer size
        let size =
            BeatSize::from((desc.btctrl.get() & BTCTRL_BEATSIZE_MASK) >> BTCTRL_BEATSIZE_POS);

        if count > 0 {
            desc.btcnt.set(count);
        }

        if !src.is_null() {
            let mut srcaddr = src as u32;

            if desc.btctrl.get() & BTCTRL_SRCINC != 0 {
                let mut offset = desc.btcnt.get() * size.bytes_per_beat() as u16;

                if desc.btctrl.get() & BTCTRL_STEPSEL != 0 {
                    let step_size =
                        (desc.btctrl.get() & BTCTRL_STEPSIZE_MASK) >> BTCTRL_STEPSIZE_POS;

                    offset *= 1 << step_size;
                }

                srcaddr += offset as u32;
            }

            desc.srcaddr.set(srcaddr);
        }

        if !dst.is_null() {
            let mut dstaddr = dst as u32;

            if desc.btctrl.get() & BTCTRL_DSTINC != 0 {
                let mut offset = desc.btcnt.get() * size.bytes_per_beat() as u16;

                if desc.btctrl.get() & BTCTRL_STEPSEL != 0 {
                    let step_size =
                        (desc.btctrl.get() & BTCTRL_STEPSIZE_MASK) >> BTCTRL_STEPSIZE_POS;

                    offset *= 1 << step_size;
                }

                dstaddr += offset as u32;
            }

            desc.dstaddr.set(dstaddr);
        }
    }

    // TODO(adafruit): delete descriptor, delete whole descriptor chain

    /// Select whether channel's descriptor list should repeat or not.
    /// This can be done before or after channel & any descriptors are
    /// allocated.
    pub fn enable_loop(&mut self, flag: bool) {
        // The loop selection is 'sticky' -- that is, you can enable or
        // disable looping before a descriptor list is built, or after
        // the fact.  This requires some extra steps in the library code
        // but avoids a must-do-in-X-order constraint on user.
        self.loop_flag = flag;

        if self.has_descriptors {
            // Scan descriptor list to find last entry.  If an entry's
            // DESCADDR value is 0, that's the end of the list and it's
            // currently un-looped.  If the DESCADDR value is the same
            // as the first entry, that's the end of the list and it's
            // already looped.
            let mut desc = self.descriptor_list.descriptor(self.channel_id);

            loop {
                let next_desc = desc.descaddr.get();

                if next_desc != 0
                    && next_desc
                        != (self.descriptor_list.descriptor(self.channel_id) as *const Descriptor)
                            as u32
                {
                    desc = unsafe { mem::transmute(next_desc) };
                } else {
                    break;
                }
            }

            // Loop or unloop descriptor list as appropriate
            if self.loop_flag {
                desc.descaddr.set(
                    (self.descriptor_list.descriptor(self.channel_id) as *const Descriptor) as u32,
                );
            } else {
                desc.descaddr.set(0);
            };
        }
    }

    /// Is this DMA currently active?
    pub fn is_active(&self) -> bool {
        self.descriptor_list.writeback(self.channel_id).btctrl.get() & BTCTRL_VALID != 0
    }
}
