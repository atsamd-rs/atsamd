use super::{Descriptors, DmPad, DpPad};
use crate::calibration::{usb_transn_cal, usb_transp_cal, usb_trim_cal};
use crate::clock;
use crate::target_device;
use crate::target_device::usb::DEVICE;
use crate::target_device::{MCLK, USB};
use crate::usb::devicedesc::DeviceDescBank;
use core::cell::{Ref, RefCell, RefMut};
use core::marker::PhantomData;
use core::mem;
use cortex_m::interrupt::{free as disable_interrupts, Mutex};
use cortex_m::singleton;
use usb_device;
use usb_device::bus::PollResult;
use usb_device::endpoint::{EndpointAddress, EndpointType};
use usb_device::{Result as UsbResult, UsbDirection, UsbError};

use crate::dbgprint;
#[cfg(feature = "use_uart_debug")]
use crate::uart_debug;

/// EndpointTypeBits represents valid values for the EPTYPE fields in
/// the EPCFGn registers.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EndpointTypeBits {
    Disabled = 0,
    Control = 1,
    Isochronous = 2,
    Bulk = 3,
    Interrupt = 4,
    #[allow(unused)]
    DualBank = 5,
}

impl Default for EndpointTypeBits {
    fn default() -> Self {
        EndpointTypeBits::Disabled
    }
}

impl From<EndpointType> for EndpointTypeBits {
    fn from(ep_type: EndpointType) -> EndpointTypeBits {
        match ep_type {
            EndpointType::Control => EndpointTypeBits::Control,
            EndpointType::Isochronous => EndpointTypeBits::Isochronous,
            EndpointType::Bulk => EndpointTypeBits::Bulk,
            EndpointType::Interrupt => EndpointTypeBits::Interrupt,
        }
    }
}

/// EPConfig tracks the desired configuration for one side of an endpoint.
#[derive(Default, Clone, Copy)]
struct EPConfig {
    ep_type: EndpointTypeBits,
    allocated_size: u16,
    max_packet_size: u16,
    addr: usize,
}

impl EPConfig {
    fn new(
        ep_type: EndpointType,
        allocated_size: u16,
        max_packet_size: u16,
        buffer_addr: *mut u8,
    ) -> Self {
        Self {
            ep_type: ep_type.into(),
            allocated_size,
            max_packet_size,
            addr: buffer_addr as usize,
        }
    }
}

// EndpointInfo represents the desired configuration for an endpoint pair.
#[derive(Default)]
struct EndpointInfo {
    bank0: EPConfig,
    bank1: EPConfig,
}

impl EndpointInfo {
    fn new() -> Self {
        Default::default()
    }
}

/// AllEndpoints tracks the desired configuration of all endpoints managed
/// by the USB peripheral.
struct AllEndpoints {
    endpoints: [EndpointInfo; 8],
}

impl AllEndpoints {
    fn new() -> Self {
        Self {
            endpoints: [
                EndpointInfo::new(),
                EndpointInfo::new(),
                EndpointInfo::new(),
                EndpointInfo::new(),
                EndpointInfo::new(),
                EndpointInfo::new(),
                EndpointInfo::new(),
                EndpointInfo::new(),
            ],
        }
    }

    fn find_free_endpoint(&self, dir: UsbDirection) -> UsbResult<usize> {
        // start with 1 because 0 is reserved for Control
        for idx in 1..8 {
            let ep_type = match dir {
                UsbDirection::Out => self.endpoints[idx].bank0.ep_type,
                UsbDirection::In => self.endpoints[idx].bank1.ep_type,
            };
            if ep_type == EndpointTypeBits::Disabled {
                return Ok(idx);
            }
        }
        Err(UsbError::EndpointOverflow)
    }

    fn allocate_endpoint(
        &mut self,
        dir: UsbDirection,
        idx: usize,
        ep_type: EndpointType,
        allocated_size: u16,
        max_packet_size: u16,
        _interval: u8,
        buffer_addr: *mut u8,
    ) -> UsbResult<EndpointAddress> {
        let bank = match dir {
            UsbDirection::Out => &mut self.endpoints[idx].bank0,
            UsbDirection::In => &mut self.endpoints[idx].bank1,
        };
        if bank.ep_type != EndpointTypeBits::Disabled {
            return Err(UsbError::EndpointOverflow);
        }

        *bank = EPConfig::new(ep_type, allocated_size, max_packet_size, buffer_addr);

        Ok(EndpointAddress::from_parts(idx, dir))
    }
}

// FIXME: replace with more general heap?
const BUFFER_SIZE: usize = 2048;
fn buffer() -> &'static mut [u8; BUFFER_SIZE] {
    singleton!(: [u8; BUFFER_SIZE] = unsafe{ mem::MaybeUninit::uninit().assume_init() }).unwrap()
}

struct BufferAllocator {
    buffers: &'static mut [u8; BUFFER_SIZE],
    next_buf: u16,
}

impl BufferAllocator {
    fn new() -> Self {
        Self {
            next_buf: 0,
            buffers: buffer(),
        }
    }

    fn allocate_buffer(&mut self, size: u16) -> UsbResult<*mut u8> {
        debug_assert!(size & 1 == 0);

        let start_addr = &mut self.buffers[self.next_buf as usize] as *mut u8;
        let buf_end = unsafe { start_addr.offset(BUFFER_SIZE as isize) };

        // The address must be 32-bit aligned, so allow for that here
        // by offsetting by an appropriate alignment.
        let offset = start_addr.align_offset(mem::align_of::<u32>());
        let start_addr = unsafe { start_addr.offset(offset as isize) };

        if start_addr >= buf_end {
            return Err(UsbError::EndpointMemoryOverflow);
        }

        let end_addr = unsafe { start_addr.offset(size as isize) };
        if end_addr > buf_end {
            return Err(UsbError::EndpointMemoryOverflow);
        }

        self.next_buf = unsafe { end_addr.sub(self.buffers.as_ptr() as usize) as u16 };

        Ok(start_addr)
    }
}

struct Inner {
    desc: RefCell<Descriptors>,
    _dm_pad: DmPad,
    _dp_pad: DpPad,
    endpoints: RefCell<AllEndpoints>,
    buffers: RefCell<BufferAllocator>,
}

pub struct UsbBus {
    inner: Mutex<RefCell<Inner>>,
}

/// Generate a method that allows returning the endpoint register
/// for a given endpoint index.  This helps very slightly with
/// two inconvenient issues:
/// - the SVD file translation generates a sequence of elements like ecfg0,
///   efcg1 rather than an array, so we have to manually translate the indices
/// - rust doesn't currently have a great solution for generating identifier
///   names, so we have to pass in a list of the possible names.
macro_rules! ep {
    ($name:ident, $type:ident) => {
        #[allow(unused)]
        #[inline]
        fn $name(&self, endpoint: usize) -> &target_device::usb::device::device_endpoint::$type {
            match endpoint {
                0 => &self.usb().device_endpoint0.$name,
                1 => &self.usb().device_endpoint1.$name,
                2 => &self.usb().device_endpoint2.$name,
                3 => &self.usb().device_endpoint3.$name,
                4 => &self.usb().device_endpoint4.$name,
                5 => &self.usb().device_endpoint5.$name,
                6 => &self.usb().device_endpoint6.$name,
                7 => &self.usb().device_endpoint7.$name,
                _ => unreachable!(),
            }
        }
    };
}

struct Bank<'a, T> {
    address: EndpointAddress,
    usb: &'a DEVICE,
    desc: RefMut<'a, super::Descriptors>,
    _phantom: PhantomData<T>,
    endpoints: Ref<'a, AllEndpoints>,
}

impl<'a, T> Bank<'a, T> {
    fn usb(&self) -> &DEVICE {
        self.usb
    }

    #[inline]
    fn index(&self) -> usize {
        self.address.index()
    }

    #[inline]
    fn config(&mut self) -> &EPConfig {
        let ep = &self.endpoints.endpoints[self.address.index()];
        if self.address.is_out() {
            &ep.bank0
        } else {
            &ep.bank1
        }
    }
}

/// InBank represents In direction banks, Bank #1
struct InBank;

/// OutBank represents Out direction banks, Bank #0
struct OutBank;

impl<'a> Bank<'a, InBank> {
    fn desc_bank(&mut self) -> &mut DeviceDescBank {
        let idx = self.index();
        self.desc.bank(idx, 1)
    }

    /// Returns true if Bank 1 is Ready and thus has data that can be written
    #[inline]
    fn is_ready(&self) -> bool {
        self.epstatus(self.index()).read().bk1rdy().bit()
    }

    /// Set Bank 1 Ready.
    /// Ready means that the buffer contains data that can be sent.
    #[inline]
    fn set_ready(&self, ready: bool) {
        if ready {
            self.epstatusset(self.index())
                .write(|w| w.bk1rdy().set_bit());
        } else {
            self.epstatusclr(self.index())
                .write(|w| w.bk1rdy().set_bit());
        }
    }

    /// Acknowledges the signal that the last packet was sent.
    #[inline]
    fn clear_transfer_complete(&self) {
        // Clear bits in epintflag by writing them to 1
        self.epintflag(self.index())
            .write(|w| w.trcpt1().set_bit().trfail1().set_bit());
    }

    /// Indicates if a transfer is complete or pending.
    #[inline]
    fn is_transfer_complete(&self) -> bool {
        self.epintflag(self.index()).read().trcpt1().bit()
    }

    /// Indicates if a transfer failed.
    #[inline]
    fn is_transfer_failed(&self) -> bool {
        self.epintflag(self.index()).read().trfail1().bit()
    }

    /// Resets the state of the endpoint as expected for a USB
    /// protocol reset.
    fn reset(&mut self) {
        let idx = self.index();
        let config = self.config().clone();
        {
            let desc = self.desc_bank();

            desc.set_address(config.addr as *mut u8);
            desc.set_endpoint_size(config.max_packet_size);
            desc.set_multi_packet_size(0);
            desc.set_byte_count(0);
        }

        self.epstatusclr(idx)
            .write(|w| w.stallrq1().set_bit().dtglin().set_bit().bk1rdy().set_bit());
        self.epintenset(idx).write(|w| w.trcpt1().set_bit());
        self.clear_transfer_complete();
    }

    /// Prepares to transfer a series of bytes by copying the data into the
    /// bank1 buffer. The caller must call set_ready() to finalize the
    /// transfer.
    pub fn write(&mut self, buf: &[u8]) -> UsbResult<usize> {
        let size = buf.len().min(self.config().allocated_size as usize);
        let desc = self.desc_bank();

        unsafe {
            buf.as_ptr()
                .copy_to_nonoverlapping(desc.get_address(), size);
        }

        desc.set_multi_packet_size(0);
        desc.set_byte_count(size as u16);

        Ok(size)
    }

    fn is_stalled(&self) -> bool {
        self.epintflag(self.index()).read().stall1().bit()
    }

    fn set_stall(&mut self, stall: bool) {
        if stall {
            self.epstatusset(self.index())
                .write(|w| w.stallrq1().set_bit())
        } else {
            self.epstatusclr(self.index())
                .write(|w| w.stallrq1().set_bit())
        }
    }
}

impl<'a> Bank<'a, OutBank> {
    fn desc_bank(&mut self) -> &mut DeviceDescBank {
        let idx = self.index();
        self.desc.bank(idx, 0)
    }

    /// Returns true if Bank 0 is Ready and thus has data that can be read.
    #[inline]
    fn is_ready(&self) -> bool {
        self.epstatus(self.index()).read().bk0rdy().bit()
    }

    /// Set Bank 0 Ready.
    /// Ready means that the buffer contains data that can be read.
    #[inline]
    fn set_ready(&self, ready: bool) {
        if ready {
            self.epstatusset(self.index())
                .write(|w| w.bk0rdy().set_bit());
        } else {
            self.epstatusclr(self.index())
                .write(|w| w.bk0rdy().set_bit());
        }
    }

    /// Acknowledges the signal that data has been received.
    #[inline]
    fn clear_transfer_complete(&self) {
        // Clear bits in epintflag by writing them to 1
        self.epintflag(self.index())
            .write(|w| w.trcpt0().set_bit().trfail0().set_bit());
    }

    /// Checks if data has been received. Returns true for failed transfers
    /// as well as successful transfers.
    #[inline]
    fn is_transfer_complete(&self) -> bool {
        self.epintflag(self.index()).read().trcpt0().bit()
    }

    /// Returns true if the transfer failed.
    #[inline]
    fn is_transfer_failed(&self) -> bool {
        self.epintflag(self.index()).read().trfail0().bit()
    }

    /// Returns true if a Received Setup interrupt has occurred.
    /// This indicates that the read buffer holds a SETUP packet.
    #[inline]
    fn received_setup_interrupt(&self) -> bool {
        self.epintflag(self.index()).read().rxstp().bit()
    }

    /// Acknowledges the signal that a SETUP packet was received
    /// successfully.
    #[inline]
    fn clear_received_setup_interrupt(&self) {
        // Clear bits in epintflag by writing them to 1
        self.epintflag(self.index()).write(|w| w.rxstp().set_bit());
    }

    /// Resets the state of the endpoint as expected for a USB
    /// protocol reset.
    fn reset(&mut self) {
        let idx = self.index();
        let config = self.config().clone();

        {
            let desc = self.desc_bank();

            desc.set_address(config.addr as *mut u8);
            desc.set_endpoint_size(config.max_packet_size);
            desc.set_multi_packet_size(0);
            desc.set_byte_count(0);
        }

        self.epstatusclr(idx).write(|w| {
            w.stallrq0()
                .set_bit()
                .dtglout()
                .set_bit()
                .bk0rdy()
                .set_bit()
        });
        self.epintenset(idx)
            .write(|w| w.rxstp().set_bit().trcpt0().set_bit());
    }

    /// Copies data from the bank0 buffer to the provided array. The caller
    /// must call set_ready to indicate the buffer is free for the next
    /// transfer.
    pub fn read(&mut self, buf: &mut [u8]) -> UsbResult<usize> {
        let desc = self.desc_bank();
        let size = desc.get_byte_count() as usize;

        if size > buf.len() {
            return Err(UsbError::BufferOverflow);
        }
        unsafe {
            desc.get_address()
                .copy_to_nonoverlapping(buf.as_mut_ptr(), size);
        }

        desc.set_byte_count(0);
        desc.set_multi_packet_size(0);

        Ok(size)
    }

    fn is_stalled(&self) -> bool {
        self.epintflag(self.index()).read().stall0().bit()
    }

    fn set_stall(&mut self, stall: bool) {
        if stall {
            self.epstatusset(self.index())
                .write(|w| w.stallrq0().set_bit())
        } else {
            self.epstatusclr(self.index())
                .write(|w| w.stallrq0().set_bit())
        }
    }
}

impl<'a, T> Bank<'a, T> {
    ep!(epcfg, EPCFG);
    ep!(epstatusclr, EPSTATUSCLR);
    ep!(epstatusset, EPSTATUSSET);
    ep!(epstatus, EPSTATUS);
    ep!(epintflag, EPINTFLAG);
    ep!(epintenclr, EPINTENCLR);
    ep!(epintenset, EPINTENSET);
}

impl Inner {
    ep!(epcfg, EPCFG);
    ep!(epstatus, EPSTATUS);
    ep!(epintflag, EPINTFLAG);

    fn bank0<'a>(&'a self, ep: EndpointAddress) -> UsbResult<Bank<'a, OutBank>> {
        if ep.is_in() {
            return Err(UsbError::InvalidEndpoint);
        }
        let endpoints = self.endpoints.borrow();

        if endpoints.endpoints[ep.index()].bank0.ep_type == EndpointTypeBits::Disabled {
            return Err(UsbError::InvalidEndpoint);
        }
        Ok(Bank {
            address: ep,
            usb: self.usb(),
            desc: self.desc.borrow_mut(),
            endpoints,
            _phantom: PhantomData,
        })
    }

    fn bank1<'a>(&'a self, ep: EndpointAddress) -> UsbResult<Bank<'a, InBank>> {
        if ep.is_out() {
            return Err(UsbError::InvalidEndpoint);
        }
        let endpoints = self.endpoints.borrow();

        if endpoints.endpoints[ep.index()].bank1.ep_type == EndpointTypeBits::Disabled {
            return Err(UsbError::InvalidEndpoint);
        }
        Ok(Bank {
            address: ep,
            usb: self.usb(),
            desc: self.desc.borrow_mut(),
            endpoints,
            _phantom: PhantomData,
        })
    }

    #[inline]
    fn received_end_of_reset_interrupt(&self) -> bool {
        self.usb().intflag.read().eorst().bit_is_set()
    }

    #[inline]
    fn clear_end_of_reset(&self) {
        // Clear by writing a 1
        self.usb().intflag.modify(|_, w| w.eorst().set_bit());
    }

    #[inline]
    fn received_suspend_interrupt(&self) -> bool {
        self.usb().intflag.read().suspend().bit_is_set()
    }

    #[inline]
    fn clear_suspend(&self) {
        // Clear by writing a 1
        self.usb().intflag.modify(|_, w| w.suspend().set_bit());
    }
}

impl UsbBus {
    pub fn new(
        _clock: &clock::UsbClock,
        mclk: &mut MCLK,
        dm_pad: DmPad,
        dp_pad: DpPad,
        _usb: USB,
    ) -> Self {
        dbgprint!("******** UsbBus::new\n");
        mclk.ahbmask.modify(|_, w| w.usb_().set_bit());
        mclk.apbbmask.modify(|_, w| w.usb_().set_bit());

        let desc = RefCell::new(Descriptors::new());

        let inner = Inner {
            _dm_pad: dm_pad,
            _dp_pad: dp_pad,
            desc,
            buffers: RefCell::new(BufferAllocator::new()),
            endpoints: RefCell::new(AllEndpoints::new()),
        };

        Self {
            inner: Mutex::new(RefCell::new(inner)),
        }
    }
}

impl Inner {
    fn usb(&self) -> &DEVICE {
        unsafe { &(*USB::ptr()).device() }
    }

    fn set_stall<EP: Into<EndpointAddress>>(&self, ep: EP, stall: bool) {
        let ep = ep.into();
        let idx = ep.index();
        let dir = ep.direction();
        dbgprint!("UsbBus::stall={} for {:?} {}\n", stall, dir, idx);
        if ep.is_out() {
            if let Ok(mut bank) = self.bank0(ep) {
                bank.set_stall(stall);
            }
        } else if let Ok(mut bank) = self.bank1(ep) {
            bank.set_stall(stall);
        }
    }

    fn print_epstatus(&self, ep: usize, label: &str) {
        let status = self.epstatus(ep).read();
        let epint = self.epintflag(ep).read();
        let intflag = self.usb().intflag.read();

        let mut desc = self.desc.borrow_mut();

        dbgprint!("ep{} status {}:\n    bk1rdy={} stallrq1={} stall1={} trcpt1={} trfail1={} byte_count1={} multi_packet_size1={}\n    bk0rdy={} stallrq0={} stall0={} trcpt0={} trfail0={} byte_count0={} multi_packet_size0={}\n    curbk={} dtglin={} dtglout={} rxstp={}   lpmsusp={} lpmnyet={} ramacer={} uprsm={} eorsm={} wakeup={} eorst={} sof={} suspend={}\n",
            ep, label,
            status.bk1rdy().bit() as u8,
            status.stallrq1().bit() as u8,
            epint.stall1().bit() as u8,
            epint.trcpt1().bit() as u8,
            epint.trfail1().bit() as u8,
            desc.bank(ep, 1).get_byte_count(),
            desc.bank(ep, 1).get_multi_packet_size(),
            status.bk0rdy().bit() as u8,
            status.stallrq0().bit() as u8,
            epint.stall0().bit() as u8,
            epint.trcpt0().bit() as u8,
            epint.trfail0().bit() as u8,
            desc.bank(ep, 0).get_byte_count(),
            desc.bank(ep, 0).get_multi_packet_size(),
            status.curbk().bit() as u8,
            status.dtglin().bit() as u8,
            status.dtglout().bit() as u8,
            epint.rxstp().bit() as u8,
            intflag.lpmsusp().bit() as u8,
            intflag.lpmnyet().bit() as u8,
            intflag.ramacer().bit() as u8,
            intflag.uprsm().bit() as u8,
            intflag.eorsm().bit() as u8,
            intflag.wakeup().bit() as u8,
            intflag.eorst().bit() as u8,
            intflag.sof().bit() as u8,
            intflag.suspend().bit() as u8,
        );
    }
}

impl Inner {
    fn enable(&mut self) {
        dbgprint!("UsbBus::enable\n");
        let usb = self.usb();
        usb.ctrla.modify(|_, w| w.swrst().set_bit());
        while usb.syncbusy.read().swrst().bit_is_set() {}

        let addr = self.desc.borrow().address();
        usb.descadd.write(|w| unsafe { w.descadd().bits(addr) });
        usb.padcal.modify(|_, w| unsafe {
            w.transn().bits(usb_transn_cal());
            w.transp().bits(usb_transp_cal());
            w.trim().bits(usb_trim_cal())
        });
        usb.qosctrl.modify(|_, w| unsafe {
            w.dqos().bits(0b11);
            w.cqos().bits(0b11)
        });
        usb.ctrla.modify(|_, w| {
            w.mode().device();
            w.runstdby().set_bit()
        });
        // full speed
        usb.ctrlb.modify(|_, w| w.spdconf().fs());

        usb.ctrla.modify(|_, w| w.enable().set_bit());
        while usb.syncbusy.read().enable().bit_is_set() {}

        usb.intenset
            .write(|w| w.eorst().set_bit().eorsm().set_bit().suspend().set_bit());

        // Configure the endpoints before we attach, as hosts may enumerate
        // before attempting a USB protocol reset.
        self.flush_eps();

        usb.ctrlb.modify(|_, w| w.detach().clear_bit());
    }

    fn flush_eps(&self) {
        for idx in 0..8 {
            let cfg = self.epcfg(idx);
            let info = &self.endpoints.borrow().endpoints[idx];

            if let Ok(mut bank) = self.bank0(EndpointAddress::from_parts(idx, UsbDirection::Out)) {
                bank.reset();
            }
            if let Ok(mut bank) = self.bank1(EndpointAddress::from_parts(idx, UsbDirection::In)) {
                bank.reset();
            }

            cfg.modify(|_, w| unsafe {
                w.eptype0()
                    .bits(info.bank0.ep_type as u8)
                    .eptype1()
                    .bits(info.bank1.ep_type as u8)
            });
        }
    }

    fn reset(&self) {
        dbgprint!("UsbBus::reset\n");
        self.flush_eps();
        self.usb().ctrlb.modify(|_, w| w.detach().clear_bit());
    }

    fn suspend(&self) {
        dbgprint!("UsbBus::suspend\n");
    }
    fn resume(&self) {
        dbgprint!("UsbBus::resume\n");
        let usb = self.usb();
        usb.ctrla.modify(|_, w| w.enable().set_bit());
        usb.ctrlb.modify(|_, w| w.detach().clear_bit());
    }

    fn alloc_ep(
        &mut self,
        dir: UsbDirection,
        addr: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval: u8,
    ) -> UsbResult<EndpointAddress> {
        let allocated_size = max_packet_size.max(64);

        let buffer = self.buffers.borrow_mut().allocate_buffer(allocated_size)?;

        dbgprint!(
            "UsbBus::alloc_ep dir={:?} addr={:?} type={:?} max_packet_size={} interval={}\n",
            dir,
            addr,
            ep_type,
            max_packet_size,
            interval
        );

        let mut endpoints = self.endpoints.borrow_mut();

        let idx = match addr {
            None => endpoints.find_free_endpoint(dir)?,
            Some(addr) => EndpointAddress::from(addr).index(),
        };

        let addr = endpoints.allocate_endpoint(
            dir,
            idx,
            ep_type,
            allocated_size,
            max_packet_size,
            interval,
            buffer,
        )?;

        dbgprint!("alloc_ep -> {:?}\n", addr);

        Ok(addr.into())
    }

    fn set_device_address(&self, addr: u8) {
        dbgprint!("UsbBus::set_device_address addr={}\n", addr);
        self.usb()
            .dadd
            .write(|w| unsafe { w.dadd().bits(addr).adden().set_bit() });
        if let Ok(bank) = self.bank0(EndpointAddress::from_parts(0, UsbDirection::Out)) {
            bank.clear_received_setup_interrupt();
        }
    }

    fn poll(&self) -> PollResult {
        // Clear flags we are not concerned about.
        self.usb()
            .intflag
            .write(|w| w.wakeup().set_bit().sof().set_bit().eorsm().set_bit());

        if self.received_suspend_interrupt() {
            self.clear_suspend();
            dbgprint!("PollResult::Suspend\n");
            return PollResult::Suspend;
        }
        if self.received_end_of_reset_interrupt() {
            self.clear_end_of_reset();
            dbgprint!("PollResult::Reset\n");
            return PollResult::Reset;
        }

        let intbits = self.usb().epintsmry.read().bits();
        if intbits == 0 {
            return PollResult::None;
        }

        let mut ep_out = 0;
        let mut ep_in_complete = 0;
        let mut ep_setup = 0;

        for ep in 0..8u16 {
            let mask = 1 << ep;
            if (intbits & mask) == 0 {
                continue;
            }

            let idx = ep as usize;

            let bank1 = self
                .bank1(EndpointAddress::from_parts(idx, UsbDirection::In))
                .unwrap();
            if bank1.is_transfer_complete() && bank1.is_transfer_failed() {
                //self.print_epstatus(idx, "WRITE FAIL");
                ep_in_complete |= mask;
                bank1.clear_transfer_complete();
                continue;
            }
            if bank1.is_transfer_complete() {
                dbgprint!("ep {} WRITE DONE\n", ep);
                ep_in_complete |= mask;
                bank1.clear_transfer_complete();
                continue;
            }
            drop(bank1);

            let bank0 = self
                .bank0(EndpointAddress::from_parts(idx, UsbDirection::Out))
                .unwrap();
            if bank0.received_setup_interrupt() {
                dbgprint!("ep {} GOT SETUP\n", ep);
                ep_setup |= mask;
                bank0.clear_received_setup_interrupt();
                break;
            }

            if bank0.is_transfer_complete() {
                dbgprint!("ep {} READABLE\n", ep);
                ep_out |= mask;
            }
            if bank0.is_transfer_failed() {
                drop(bank0);
                self.print_epstatus(idx, "READ FAIL");
                ep_out |= mask;
            }
        }

        PollResult::Data {
            ep_out,
            ep_in_complete,
            ep_setup,
        }
    }

    fn write(&self, ep: EndpointAddress, buf: &[u8]) -> UsbResult<usize> {
        let mut bank = self.bank1(ep.into())?;

        if bank.is_ready() {
            // Waiting for the host to pick up the existing data
            dbgprint!(
                "UsbBus::write {} bytes {:?} to ep {:?} -> BUSY trcpt1={}\n",
                buf.len(),
                buf,
                ep,
                bank.is_transfer_complete()
            );
            return Err(UsbError::WouldBlock);
        }

        let size = bank.write(buf);

        bank.clear_transfer_complete();
        bank.set_ready(true); // ready to be sent

        dbgprint!(
            "UsbBus::write {} bytes {:?} to ep {:?} -> {:?}\n",
            buf.len(),
            buf,
            ep,
            size
        );

        size
    }

    fn read(&self, ep: EndpointAddress, buf: &mut [u8]) -> UsbResult<usize> {
        let mut bank = self.bank0(ep.into())?;

        let bk0rdy = bank.is_ready();
        let rxstp = bank.received_setup_interrupt();

        if bk0rdy || rxstp {
            let size = bank.read(buf);

            // self.print_epstatus(idx, "read");

            bank.clear_transfer_complete();
            bank.set_ready(false);

            drop(bank);

            match size {
                Ok(size) => {
                    //dbgprint!("UsbBus::read {} bytes ok", size);
                    let got = &buf[..size as usize];
                    dbgprint!(
                        "UsbBus::read {} bytes from ep {:?} -> {:?}\n",
                        size,
                        ep,
                        got
                    );
                    Ok(size)
                }
                Err(err) => {
                    dbgprint!("UsbBus::read from ep {:?} -> {:?}\n", ep, err);
                    self.print_epstatus(ep.index(), "after read");
                    Err(err)
                }
            }
        } else {
            Err(UsbError::WouldBlock)
        }
    }

    fn is_stalled(&self, ep: EndpointAddress) -> bool {
        if ep.is_out() {
            self.bank0(ep.into()).unwrap().is_stalled()
        } else {
            self.bank1(ep.into()).unwrap().is_stalled()
        }
    }

    fn set_stalled(&self, ep: EndpointAddress, stalled: bool) {
        self.set_stall(ep, stalled);
    }
}

impl usb_device::bus::UsbBus for UsbBus {
    fn enable(&mut self) {
        disable_interrupts(|cs| self.inner.borrow(cs).borrow_mut().enable())
    }

    fn reset(&self) {
        disable_interrupts(|cs| self.inner.borrow(cs).borrow().reset())
    }

    fn suspend(&self) {
        disable_interrupts(|cs| self.inner.borrow(cs).borrow().suspend())
    }

    fn resume(&self) {
        disable_interrupts(|cs| self.inner.borrow(cs).borrow().resume())
    }

    fn alloc_ep(
        &mut self,
        dir: UsbDirection,
        addr: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval: u8,
    ) -> UsbResult<EndpointAddress> {
        disable_interrupts(|cs| {
            self.inner.borrow(cs).borrow_mut().alloc_ep(
                dir,
                addr,
                ep_type,
                max_packet_size,
                interval,
            )
        })
    }

    fn set_device_address(&self, addr: u8) {
        disable_interrupts(|cs| self.inner.borrow(cs).borrow().set_device_address(addr))
    }

    fn poll(&self) -> PollResult {
        disable_interrupts(|cs| self.inner.borrow(cs).borrow().poll())
    }

    fn write(&self, ep: EndpointAddress, buf: &[u8]) -> UsbResult<usize> {
        disable_interrupts(|cs| self.inner.borrow(cs).borrow().write(ep, buf))
    }

    fn read(&self, ep: EndpointAddress, buf: &mut [u8]) -> UsbResult<usize> {
        disable_interrupts(|cs| self.inner.borrow(cs).borrow().read(ep, buf))
    }

    fn set_stalled(&self, ep: EndpointAddress, stalled: bool) {
        disable_interrupts(|cs| self.inner.borrow(cs).borrow().set_stalled(ep, stalled))
    }

    fn is_stalled(&self, ep: EndpointAddress) -> bool {
        disable_interrupts(|cs| self.inner.borrow(cs).borrow().is_stalled(ep))
    }
}
