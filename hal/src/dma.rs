//! Direct Memory Access
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
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

mod controller;
mod status;

#[cfg(not(feature = "samd51"))]
use self::controller::samd21::{IRQn, IRQN};
#[cfg(feature = "samd51")]
use self::controller::samd51::{ChannelRegisters, IRQn};
use self::controller::*;
pub use self::status::Status;
#[cfg(feature = "samd51")]
use crate::target_device::dmac::chctrla::{TRIGACTW, TRIGSRCW};
#[cfg(not(feature = "samd51"))]
use crate::target_device::dmac::chctrlb::{TRIGACTW, TRIGSRCW};
use crate::target_device::DMAC;
use core::{
    mem,
    sync::atomic::{self, Ordering},
};
use vcell::VolatileCell;

/// Maximum number of jobs to resume
const MAX_JOB_RESUME_COUNT: usize = 10000;

/// DMA clock source
#[cfg(feature = "samd51")]
type ClockSource = crate::target_device::MCLK;

/// DMA clock source
#[cfg(not(feature = "samd51"))]
type ClockSource = crate::target_device::PM;

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
    pub transfer_error: Option<fn(&mut DMA)>,

    /// Transfer done
    pub transfer_done: Option<fn(&mut DMA)>,

    /// Channel suspended
    pub channel_suspend: Option<fn(&mut DMA)>,
}

/// Direct Memory Access
pub struct DMA<'desc> {
    /// DMA controller
    dmac: DMAC,

    /// Channel
    channel: Option<ChannelId>,

    /// Job status
    job_status: VolatileCell<Status>,

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

pub struct InterruptState {
    /// Bitmask of allocated channels
    channel_mask: VolatileCell<u32>,

    /// IRQ critical section counter
    cpu_irq_critical_section_counter: VolatileCell<u32>,

    /// IRQ previous interrupt state
    cpu_irq_prev_interrupt_state: VolatileCell<u8>,
}

impl Default for InterruptState {
    fn default() -> Self {
        Self {
            channel_mask: VolatileCell::new(0),
            cpu_irq_critical_section_counter: VolatileCell::new(0),
            cpu_irq_prev_interrupt_state: VolatileCell::new(0),
        }
    }
}

/// Enter IRQ critical section
fn cpu_irq_enter_critical(int_state: &InterruptState) {
    atomic::compiler_fence(Ordering::Acquire);

    if int_state.cpu_irq_critical_section_counter.get() == 0 {
        if get_primask() == 0 {
            // IRQ enabled?
            disable_irq(); // Disable it
            dmb();
            int_state.cpu_irq_prev_interrupt_state.set(1);
        } else {
            // Make sure the to save the prev state as false
            int_state.cpu_irq_prev_interrupt_state.set(0);
        }
    }

    int_state
        .cpu_irq_critical_section_counter
        .set(int_state.cpu_irq_critical_section_counter.get() + 1);
    atomic::compiler_fence(Ordering::Acquire);
}

/// Leave IRQ critical section
fn cpu_irq_leave_critical(int_state: &InterruptState) {
    atomic::compiler_fence(Ordering::Acquire);

    // Check if the user is trying to leave a critical section
    // when not in a critical section
    if int_state.cpu_irq_critical_section_counter.get() > 0 {
        int_state
            .cpu_irq_critical_section_counter
            .set(int_state.cpu_irq_critical_section_counter.get() - 1);

        // Only enable global interrupts when the counter
        // reaches 0 and the state of the global interrupt flag
        // was enabled when entering critical state */
        if int_state.cpu_irq_critical_section_counter.get() == 0
            && int_state.cpu_irq_prev_interrupt_state.get() == 1
        {
            dmb();
            enable_irq();
        }
    }

    atomic::compiler_fence(Ordering::Acquire);
}

impl<'desc> DMA<'desc> {
    /// Initialize a new DMA context
    pub fn new(dmac: DMAC, descriptor_list: &'desc DescriptorList) -> Self {
        Self {
            dmac,
            channel: None, // Channel not yet allocated
            job_status: VolatileCell::new(Status::Ok),
            has_descriptors: false, // No descriptors allocated yet
            loop_flag: false,
            peripheral_trigger: TRIGSRCW::DISABLE, // Software trigger only by default
            trigger_action: TRIGACTW::TRANSACTION,
            callbacks: Callbacks::default(),
            descriptor_list,
        }
    }

    // TODO(adafruit): Drop impl? Should stop job, delete descriptors, free channel.

    /// IRQ handler
    pub fn irq_handler(&mut self, flags: u8) {
        #[allow(unused_variables)]
        let channel = self.channel.expect("no channel ID");

        // 'flags' is initially passed in as channel number,
        // from which we look up the actual interrupt flags...
        let flags = {
            #[cfg(feature = "samd51")]
            {
                self.dmac.chintflag(flags).read().bits()
            }

            #[cfg(not(feature = "samd51"))]
            {
                flags
            }
        };

        if flags & CHINTENCLR_TERR != 0 {
            // Clear error flag
            #[cfg(feature = "samd51")]
            {
                self.dmac
                    .chintflag(channel)
                    .write(|reg| reg.terr().clear_bit());
            }

            #[cfg(not(feature = "samd51"))]
            {
                self.dmac.chintflag.write(|reg| reg.terr().clear_bit());
            }

            self.job_status.set(Status::Io);

            if let Some(cb) = self.callbacks.transfer_error {
                cb(self);
            }
        } else if flags & CHINTENCLR_TCMPL != 0 {
            // Clear transfer complete flag
            #[cfg(feature = "samd51")]
            {
                self.dmac
                    .chintflag(channel)
                    .write(|reg| reg.tcmpl().set_bit());
            }

            #[cfg(not(feature = "samd51"))]
            {
                self.dmac.chintflag.write(|reg| reg.tcmpl().set_bit());
            }

            self.job_status.set(Status::Ok);

            if let Some(cb) = self.callbacks.transfer_done {
                cb(self);
            }
        } else if flags & CHINTENCLR_SUSP != 0 {
            // Clear channel suspend flag
            #[cfg(feature = "samd51")]
            {
                self.dmac
                    .chintflag(channel)
                    .write(|reg| reg.susp().set_bit());
            }

            #[cfg(not(feature = "samd51"))]
            {
                self.dmac.chintflag.write(|reg| reg.susp().set_bit());
            }

            self.job_status.set(Status::Suspend);
            if let Some(cb) = self.callbacks.channel_suspend {
                cb(self)
            }
        }
    }

    // DMA CHANNEL FUNCTIONS ---------------------------------------------------

    /// Allocates channel for DMA object
    pub fn allocate(
        &mut self,
        clock_source: &mut ClockSource,
        int_state: &InterruptState,
    ) -> Status {
        if self.channel.is_some() {
            return Status::Ok; // Already alloc'd!
        }

        // Find index of first free DMA channel.  As currently written,
        // this "does not play well with others" as it assumes the channel mask
        // is the final arbiter of channels in use (this is true only within
        // this library -- but other DMA-driven code may have allocated its
        // own channel(s) elsewhere, sometimes with an equally broken
        // approach).
        //
        // A possible (untested) alternate approach might be to loop through
        // each channel, set `cmac.chid`'s ID and then test whether `chctrl`'s
        // ENABLE is set.
        let mut channel: ChannelId = 0;

        while channel < NUM_CHANNELS as u8 && (int_state.channel_mask.get() & (1 << channel)) == 1 {
            channel += 1;
        }

        // Doesn't help that code later does a software reset of the DMA
        // controller, which would blow out other DMA-using libraries
        // anyway (or they're just as likely to blow out this one).
        // I think it's just an all-or-nothing affair...use one library
        // for DMA everything, never mix and match.
        if channel >= NUM_CHANNELS as u8 {
            // No free channel!
            return Status::NotFound;
        }

        cpu_irq_enter_critical(int_state);

        if int_state.channel_mask.get() == 0 {
            // No channels allocated yet; initialize DMA!
            clock_source.ahbmask.write(|reg| reg.dmac_().set_bit());

            #[cfg(not(feature = "samd51"))]
            {
                clock_source.apbbmask.write(|reg| reg.dmac_().set_bit());
            }

            // Disable DMA controller
            self.dmac.ctrl.write(|reg| reg.dmaenable().clear_bit());

            // Perform software reset
            self.dmac.ctrl.write(|reg| reg.swrst().set_bit());

            // Initialize descriptor list addresses
            self.dmac.baseaddr.write(|reg| unsafe {
                reg.baseaddr()
                    .bits(self.descriptor_list.descriptor.as_ptr() as u32)
            });

            self.dmac.wrbaddr.write(|reg| unsafe {
                reg.wrbaddr()
                    .bits(self.descriptor_list.writeback.as_ptr() as u32)
            });

            // TODO(tarcieri): Wipe decriptor and writeback tables
            // self.descriptor_list.descriptor[0] = Descriptor::default();
            // self.descriptor_list.descriptor[0] = Descriptor::default();

            // Re-enable DMA controller with all priority levels
            self.dmac.ctrl.write(|reg| {
                reg.dmaenable()
                    .set_bit()
                    .lvlen0()
                    .set_bit()
                    .lvlen1()
                    .set_bit()
                    .lvlen2()
                    .set_bit()
                    .lvlen3()
                    .set_bit()
            });

            // Enable DMA interrupt at lowest priority
            #[cfg(feature = "samd51")]
            {
                // Enable DMAC IRQs
                for irq in &[
                    IRQn::N0,
                    IRQn::N1,
                    IRQn::N2,
                    IRQn::N3,
                    IRQn::N4,
                ] {
                    nvic_enable_irq(*irq);
                    nvic_set_priority(*irq, (1 << NVIC_PRIO_BITS) - 1);
                }
            }

            #[cfg(not(feature = "samd51"))]
            {
                // Enable DMAC IRQ
                nvic_enable_irq(IRQN);
                nvic_set_priority(IRQN, (1 << NVIC_PRIO_BITS) - 1);
            }
        }

        self.channel = Some(channel);
        int_state
            .channel_mask
            .set(int_state.channel_mask.get() | 1 << channel); // Mark channel as allocated

        // Reset the allocated channel
        #[cfg(feature = "samd51")]
        {
            self.dmac
                .chctrla(channel)
                .write(|reg| reg.enable().clear_bit().swrst().set_bit());
        }

        #[cfg(not(feature = "samd51"))]
        unsafe {
            self.dmac.chid.write(|reg| reg.id().bits(channel));
            self.dmac
                .chctrla
                .write(|reg| reg.enable().clear_bit().swrst().set_bit());
        }

        // Clear software trigger
        self.dmac
            .swtrigctrl
            .modify(|reg, value| unsafe { value.bits(reg.bits() & !(1 << channel)) });

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
        #[cfg(feature = "samd51")]
        {
            self.dmac.chprilvl(channel).write(|reg| reg.prilvl().lvl0());
            self.dmac.chctrla(channel).write(|reg| {
                reg.trigsrc()
                    .variant(peripheral_trigger)
                    .trigact()
                    .variant(trigger_action)
                    .burstlen()
                    .single()
            });
        }

        #[cfg(not(feature = "samd51"))]
        {
            self.dmac.chctrlb.write(|reg| reg.lvl().lvl0());
            self.dmac
                .chctrlb
                .write(|reg| reg.trigsrc().variant(peripheral_trigger));
            self.dmac
                .chctrlb
                .write(|reg| reg.trigact().variant(trigger_action));
        }

        cpu_irq_leave_critical(int_state);

        Status::Ok
    }

    /// Set priority

    pub fn set_priority(&mut self, pri: Priority) {
        #[cfg(feature = "samd51")]
        unsafe {
            self.dmac
                .chprilvl(self.channel.expect("no channel ID"))
                .write(|reg| reg.prilvl().bits(pri as u8));
        }

        #[cfg(not(feature = "samd51"))]
        {
            self.dmac.chctrlb.write(|reg| reg.lvl().bits(pri as u8));
        }
    }

    /// Deallocate DMA channel
    // TODO(adafruit): should this delete/deallocate the descriptor list?
    pub fn free(
        &mut self,
        clock_source: &mut ClockSource,
        int_state: &InterruptState
    ) -> Status {
        cpu_irq_enter_critical(int_state); // job_status is volatile

        let status = if self.job_status.get() == Status::Busy {
            Status::Busy // Can't leave when busy
        } else if let Some(channel) = self.channel {
            if (int_state.channel_mask.get() & (1 << channel)) != 0 {
                // Valid in-use channel; release it
                int_state
                    .channel_mask
                    .set(int_state.channel_mask.get() & !(1 << channel)); // Clear bit

                if int_state.channel_mask.get() == 0 {
                    // No more channels in use?
                    #[cfg(feature = "samd51")]
                    {
                        // Disable DMA interrupt
                        nvic_disable_irq(IRQn::N0);

                        // Disable DMA
                        self.dmac.ctrl.write(|reg| reg.dmaenable().clear_bit());

                        // Disable DMA clock
                        clock_source.ahbmask.write(|reg| reg.dmac_().clear_bit());
                    }

                    #[cfg(not(feature = "samd51"))]
                    {
                        // Disable DMA interrupt
                        nvic_disable_irq(IRQN);

                        // Disable DMA
                        self.dmac.ctrl.write(|reg| reg.dmaenable().clear_bit());

                        // Disable DMA clocks
                        clock_source.apbbmask.write(|reg| reg.dmac_().clear_bit());
                        clock_source.ahbmask.write(|reg| reg.dmac_().clear_bit());
                    }
                }

                self.channel = None;
                Status::Ok
            } else {
                Status::NotInitialized // Channel not in mask?
            }
        } else {
            Status::NotInitialized // Channel not in use
        };

        cpu_irq_leave_critical(int_state);

        status
    }

    /// Start DMA transfer job.  Channel and descriptors should be allocated
    /// before calling this.
    pub fn start_job(&mut self, int_state: &InterruptState) -> Status {
        cpu_irq_enter_critical(int_state); // Job status is volatile

        let channel = self.channel.expect("no channel ID");
        let status = if self.job_status.get() == Status::Busy {
            Status::Busy // Resource is busy
        } else if self.channel.is_none() {
            Status::NotInitialized // Channel not in use
        } else if !self.has_descriptors || self.descriptor_list.descriptor(channel).btcnt.get() <= 0
        {
            Status::InvalidArg // Bad transfer size
        } else {
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

            self.job_status.set(Status::Busy);

            #[cfg(feature = "samd51")]
            unsafe {
                self.dmac
                    .chintenset(channel)
                    .write(|reg| reg.bits(CHINTENSET_MASK & interrupt_mask));
                self.dmac
                    .chintenclr(channel)
                    .write(|reg| reg.bits(CHINTENCLR_MASK & !interrupt_mask));
                self.dmac
                    .chctrla(channel)
                    .write(|reg| reg.enable().set_bit());
            }

            #[cfg(not(feature = "samd51"))]
            unsafe {
                self.dmac.chid.write(|reg| reg.id().bits(channel));
                self.dmac
                    .chintenset
                    .write(|reg| reg.bits(CHINTENSET_MASK & interrupt_mask));
                self.dmac
                    .chintenclr
                    .write(|reg| reg.bits(CHINTENCLR_MASK & !interrupt_mask));
                self.dmac.chctrla.write(|reg| reg.enable().set_bit());
            }

            Status::Ok
        };

        cpu_irq_leave_critical(int_state);

        status
    }

    /// Set and enable callback function for DMA object. This can be called
    /// before or after channel and/or descriptors are allocated, but needs
    /// to be called before job is started.
    pub fn set_callback(&mut self, cb_type: CallbackType, cb: fn(&mut DMA)) {
        match cb_type {
            CallbackType::TransferError => self.callbacks.transfer_error = Some(cb),
            CallbackType::TransferDone => self.callbacks.transfer_done = Some(cb),
            CallbackType::ChannelSuspend => self.callbacks.channel_suspend = Some(cb),
        }
    }

    // TODO(adafruit): Suspend/resume don't quite do what I thought -- avoid using for now.

    /// Suspend.
    ///
    /// NOTE: Not recommended.
    pub fn suspend(&mut self, int_state: &InterruptState) {
        cpu_irq_enter_critical(int_state);

        let channel = self.channel.expect("no channel ID");

        #[cfg(feature = "samd51")]
        {
            self.dmac.chctrlb(channel).write(|reg| reg.cmd().suspend());
        }

        #[cfg(not(feature = "samd51"))]
        unsafe {
            self.dmac.chid.write(|reg| reg.id().bits(channel));
            self.dmac.chctrlb.write(|reg| reg.cmd().suspend());
        }

        cpu_irq_leave_critical(int_state);
    }

    /// Resume.
    ///
    /// NOTE: Not recommended.
    pub fn resume(&mut self, int_state: &InterruptState) {
        cpu_irq_enter_critical(int_state); // job_status is volatile

        if self.job_status.get() == Status::Suspend {
            if let Some(channel) = self.channel {
                let bit_mask = 1 << channel;

                #[cfg(feature = "samd51")]
                {
                    self.dmac.chctrlb(channel).write(|reg| reg.cmd().resume());
                }

                #[cfg(not(feature = "samd51"))]
                unsafe {
                    self.dmac.chid.write(|reg| reg.id().bits(channel));
                    self.dmac.chctrlb.write(|reg| reg.cmd().resume())
                }

                let mut count = 0;

                while count < MAX_JOB_RESUME_COUNT
                    && !(self.dmac.busych.read().bits() & bit_mask) != 0
                {
                    count += 1;
                }

                if count < MAX_JOB_RESUME_COUNT {
                    self.job_status.set(Status::Busy);
                } else {
                    self.job_status.set(Status::Timeout);
                }
            }
        }

        cpu_irq_leave_critical(int_state);
    }

    /// Abort
    pub fn abort(&mut self, int_state: &InterruptState) {
        if let Some(channel) = self.channel {
            cpu_irq_enter_critical(int_state);

            #[cfg(feature = "samd51")]
            unsafe {
                self.dmac.chctrla(channel).write(|reg| reg.bits(0));
            }

            #[cfg(not(feature = "samd51"))]
            unsafe {
                // Select channel
                self.dmac.chid.write(|reg| reg.bits(channel));

                // Disable
                self.dmac.chctrla.write(|reg| reg.bits(0));
            }

            self.job_status.set(Status::Aborted);

            cpu_irq_leave_critical(int_state);
        }
    }

    /// Set DMA peripheral trigger.
    /// This can be done before or after channel is allocated.
    pub fn set_trigger(&mut self, trigger: TRIGSRCW, int_state: &InterruptState) {
        // Save value for `allocate()`
        self.peripheral_trigger = match self.peripheral_trigger {
            TRIGSRCW::DISABLE => TRIGSRCW::DISABLE,
        };

        // If channel already allocated, configure peripheral trigger
        // (old lib required configure before alloc -- either way OK now)
        if let Some(channel) = self.channel {
            cpu_irq_enter_critical(int_state);

            #[cfg(feature = "samd51")]
            {
                self.dmac
                    .chctrla(channel)
                    .write(|reg| reg.trigsrc().variant(trigger));
            }

            #[cfg(not(feature = "samd51"))]
            unsafe {
                self.dmac.chid.write(|reg| reg.id().bits(channel));
                self.dmac
                    .chctrlb
                    .write(|reg| reg.trigsrc().variant(trigger));
            }

            cpu_irq_leave_critical(int_state);
        }
    }

    /// Set DMA trigger action.
    /// This can be done before or after channel is allocated.
    pub fn set_action(&mut self, action: TRIGACTW, int_state: &InterruptState) {
        // Save value for `allocate()`. Hax because TRIGACTW is non-Copy/Clone
        self.trigger_action = match action {
            #[cfg(not(feature = "samd51"))]
            TRIGACTW::BEAT => TRIGACTW::BEAT,
            TRIGACTW::BLOCK => TRIGACTW::BLOCK,
            #[cfg(feature = "samd51")]
            TRIGACTW::BURST => TRIGACTW::BURST,
            TRIGACTW::TRANSACTION => TRIGACTW::TRANSACTION,
        };

        // If channel already allocated, configure trigger action
        // (old lib required configure before alloc -- either way OK now)
        if let Some(channel) = self.channel {
            cpu_irq_enter_critical(int_state);

            #[cfg(feature = "samd51")]
            {
                self.dmac
                    .chctrla(channel)
                    .write(|reg| reg.trigact().variant(action));
            }

            #[cfg(not(feature = "samd51"))]
            unsafe {
                self.dmac.chid.write(|reg| reg.id().bits(channel));
                self.dmac.chctrlb.write(|reg| reg.trigact().variant(action));
            }

            cpu_irq_leave_critical(int_state);
        }
    }

    /// Issue software trigger.
    ///
    /// Channel must be allocated and descriptors added!
    pub fn trigger(&mut self) {
        if let Some(channel) = self.channel {
            if self.has_descriptors {
                unsafe {
                    self.dmac
                        .swtrigctrl
                        .modify(|r, w| w.bits(r.bits() | 1 << channel));
                }
            }
        }
    }

    /// Get the channel ID for this channel
    pub fn get_channel(&self) -> Option<ChannelId> {
        self.channel
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
        // Channel must be allocated first
        let channel = self.channel?;

        // Can't do while job's busy
        if self.job_status.get() == Status::Busy {
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
            // while prev.descaddr.reg && prev.descaddr.reg != unsafe { &DESCRIPTOR[channel] } {
            //    prev = prev.descaddr.reg;
            // }
            //
            // prev.descaddr.reg = desc;
            // desc
            unimplemented!();
        } else {
            self.descriptor_list.descriptor(channel)
        };

        self.has_descriptors = true;

        // Beat transfer size IN BYTES
        let bytes_per_beat = match size {
            BeatSize::Byte => 1,
            BeatSize::HWord => 2,
            BeatSize::Word => 4,
        };

        // TODO(tarcieri): verify all of these reprs are correct
        let mut btctrl = BTCTRL_VALID
            | EVENT_OUTPUT_DISABLE
            | btctrl_beatsize(size as u16)
            | btctrl_stepsize(step_size as u16);

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

        if src_inc {
            if step_sel {
                desc.srcaddr
                    .set(desc.srcaddr.get() + (bytes_per_beat * count) as u32 * (1 << step_size));
            } else {
                desc.srcaddr
                    .set(desc.srcaddr.get() + (bytes_per_beat * count) as u32);
            }
        }

        desc.dstaddr.set(dst as u32);

        if dst_inc {
            if !step_sel {
                desc.dstaddr
                    .set(desc.dstaddr.get() + (bytes_per_beat * count) as u32 * (1 << step_size));
            } else {
                desc.dstaddr
                    .set(desc.dstaddr.get() + (bytes_per_beat * count) as u32);
            }
        }

        if self.loop_flag {
            desc.descaddr
                .set((self.descriptor_list.descriptor(channel) as *const Descriptor) as u32);
        } else {
            desc.descaddr.set(0);
        };

        Some(desc)
    }

    /// Modify DMA descriptor with a new source address, destination address &
    /// block transfer count.  All other attributes (including increment enables,
    /// etc.) are unchanged.  Mostly for changing the data being pushed to a
    /// peripheral (DAC, SPI, whatev.)
    pub fn change_descriptor(
        &mut self,
        desc: &mut Descriptor,
        src: *const u8,
        dst: *mut u8,
        count: u16,
    ) {
        // Read the current beat transfer size
        let size = (desc.btctrl.get() & BTCTRL_BEATSIZE_MASK) >> BTCTRL_BEATSIZE_POS;

        // Beat transfer size IN BYTES
        let bytes_per_beat = if size == BeatSize::HWord as u16 {
            2
        } else if size == BeatSize::Word as u16 {
            4
        } else {
            1
        };

        if count > 0 {
            desc.btcnt.set(count);
        }

        if !src.is_null() {
            desc.srcaddr.set(src as u32);

            if desc.btctrl.get() & BTCTRL_SRCINC != 0 {
                if desc.btctrl.get() & BTCTRL_STEPSEL != 0 {
                    let step_size =
                        (desc.btctrl.get() & BTCTRL_STEPSIZE_MASK) >> BTCTRL_STEPSIZE_POS;

                    desc.srcaddr.set(
                        desc.srcaddr.get()
                            + (desc.btcnt.get() * bytes_per_beat) as u32 * (1 << step_size),
                    );
                } else {
                    desc.srcaddr
                        .set(desc.srcaddr.get() + (desc.btcnt.get() * bytes_per_beat) as u32);
                }
            }
        }

        if !dst.is_null() {
            desc.dstaddr.set(dst as u32);

            if desc.btctrl.get() & BTCTRL_DSTINC != 0 {
                if desc.btctrl.get() & BTCTRL_STEPSEL != 0 {
                    let step_size =
                        (desc.btctrl.get() & BTCTRL_STEPSIZE_MASK) >> BTCTRL_STEPSIZE_POS;

                    desc.dstaddr.set(
                        desc.dstaddr.get()
                            + (desc.btcnt.get() * bytes_per_beat) as u32 * (1 << step_size),
                    );
                } else {
                    desc.dstaddr
                        .set(desc.dstaddr.get() + (desc.btcnt.get() * bytes_per_beat) as u32);
                }
            }
        }

        // TODO(adafruit): I think this code is here by accident -- disabling for now.
        // cpu_irq_enter_critical();
        // self.job_status.set(Status::DmaStatusOk);
        // let channel = self.channel.unwrap();
        //
        // #[cfg(feature = "samd51")]
        // DMAC.chctrla(channel).bit.ENABLE = 1;
        //
        // #[cfg(not(feature = "samd51"))]
        // {
        //    CHID.bit.ID = channel;
        //    CHCTRLA.reg |= CHCTRLA_ENABLE;
        // }
        //
        // cpu_irq_leave_critical();
    }

    // TODO(adafruit): delete descriptor, delete whole descriptor chain

    /// Select whether channel's descriptor list should repeat or not.
    /// This can be done before or after channel & any descriptors are allocated.
    pub fn enable_loop(&mut self, flag: bool) {
        // The loop selection is 'sticky' -- that is, you can enable or
        // disable looping before a descriptor list is built, or after
        // the fact.  This requires some extra steps in the library code
        // but avoids a must-do-in-X-order constraint on user.
        self.loop_flag = flag;

        if self.has_descriptors {
            // Descriptor list already started?
            let channel = self.channel.expect("no channel ID");

            // Scan descriptor list to find last entry.  If an entry's
            // DESCADDR value is 0, that's the end of the list and it's
            // currently un-looped.  If the DESCADDR value is the same
            // as the first entry, that's the end of the list and it's
            // already looped.
            let mut desc = self.descriptor_list.descriptor(channel);

            loop {
                let next_desc = desc.descaddr.get();

                if next_desc != 0
                    && next_desc
                        != (self.descriptor_list.descriptor(channel) as *const Descriptor) as u32
                {
                    desc = unsafe { mem::transmute(next_desc) };
                } else {
                    break;
                }
            }

            // Loop or unloop descriptor list as appropriate
            if self.loop_flag {
                desc.descaddr
                    .set((self.descriptor_list.descriptor(channel) as *const Descriptor) as u32);
            } else {
                desc.descaddr.set(0);
            };
        }
    }

    /// Is this DMA currently active?
    pub fn is_active(&self) -> bool {
        if let Some(channel) = self.channel {
            self.descriptor_list.writeback(channel).btctrl.get() & BTCTRL_VALID != 0
        } else {
            false
        }
    }
}

/// Thunk for `__get_PRIMASK()`
fn get_primask() -> u32 {
    // TODO(tarcieri): implement this!
    unimplemented!();
}

/// Thunk for `__enable_irq()`
fn enable_irq() {
    // TODO(tarcieri): implement this!
    unimplemented!();
}

/// Thunk for `__disable_irq()`
fn disable_irq() {
    // TODO(tarcieri): implement this!
    unimplemented!();
}

/// Thunk for `__DMB()`
fn dmb() {
    // TODO(tarcieri): implement this!
    unimplemented!();
}

/// Thunk for `NVIC_EnableIRQ`
fn nvic_enable_irq(_irq: IRQn) {
    // TODO(tarcieri): implement this!
    unimplemented!();
}

/// Thunk for `NVIC_DisableIRQ`
fn nvic_disable_irq(_irq: IRQn) {
    // TODO(tarcieri): implement this!
    unimplemented!();
}

/// Thunk for `NVIC_SetPriority`
fn nvic_set_priority(_irq: IRQn, _priority: u16) {
    // TODO(tarcieri): implement this!
    unimplemented!();
}
