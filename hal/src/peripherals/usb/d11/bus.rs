// This crate uses standard host-centric USB terminology for transfer
// directions. Therefore an OUT transfer refers to a host-to-device transfer,
// and an IN transfer refers to a device-to-host transfer. This is mainly a
// concern for implementing new USB peripheral drivers and USB classes, and
// people doing that should be familiar with the USB standard. http://ww1.microchip.com/downloads/en/DeviceDoc/60001507E.pdf
// http://ww1.microchip.com/downloads/en/AppNotes/Atmel-42261-SAM-D21-USB_Application-Note_AT06475.pdf

use super::Descriptors;
use crate::calibration::{usb_transn_cal, usb_transp_cal, usb_trim_cal};
use crate::clock;
use crate::gpio::{AlternateG, AnyPin, Pin, PA24, PA25};
use crate::pac::usb::Device;
use crate::pac::{Pm, Usb};
use crate::usb::devicedesc::DeviceDescBank;
use atsamd_hal_macros::{hal_cfg, hal_macro_helper};
use core::cell::{Ref, RefCell, RefMut};
use core::marker::PhantomData;
use core::mem;
use cortex_m::singleton;
use critical_section::{with as disable_interrupts, Mutex};
use usb_device::bus::PollResult;
use usb_device::endpoint::{EndpointAddress, EndpointType};
use usb_device::{Result as UsbResult, UsbDirection, UsbError};

/// EndpointTypeBits represents valid values for the EPTYPE fields in
/// the EPCFGn registers.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum EndpointTypeBits {
    #[default]
    Disabled = 0,
    Control = 1,
    Isochronous = 2,
    Bulk = 3,
    Interrupt = 4,
    #[allow(unused)]
    DualBank = 5,
}

impl From<EndpointType> for EndpointTypeBits {
    fn from(ep_type: EndpointType) -> EndpointTypeBits {
        match ep_type {
            EndpointType::Control => EndpointTypeBits::Control,
            EndpointType::Isochronous { .. } => EndpointTypeBits::Isochronous,
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

    #[allow(clippy::too_many_arguments)]
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
    singleton!(: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE] ).unwrap()
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
        let buf_end = unsafe { start_addr.add(BUFFER_SIZE) };

        // The address must be 32-bit aligned, so allow for that here
        // by offsetting by an appropriate alignment.
        let offset = start_addr.align_offset(mem::align_of::<u32>());
        let start_addr = unsafe { start_addr.add(offset) };

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
    _dm_pad: Pin<PA24, AlternateG>,
    _dp_pad: Pin<PA25, AlternateG>,
    endpoints: RefCell<AllEndpoints>,
    buffers: RefCell<BufferAllocator>,
}

pub struct UsbBus {
    inner: Mutex<RefCell<Inner>>,
}

struct Bank<'a, T> {
    address: EndpointAddress,
    usb: &'a Device,
    desc: RefMut<'a, super::Descriptors>,
    _phantom: PhantomData<T>,
    endpoints: Ref<'a, AllEndpoints>,
}

impl<T> Bank<'_, T> {
    fn usb(&self) -> &Device {
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

impl Bank<'_, InBank> {
    fn desc_bank(&mut self) -> &mut DeviceDescBank {
        let idx = self.index();
        self.desc.bank(idx, 1)
    }

    /// Returns true if Bank 1 is Ready and thus has data that can be written
    #[inline]
    fn is_ready(&self) -> bool {
        self.usb().epstatus(self.index()).read().bk1rdy().bit()
    }

    /// Set Bank 1 Ready.
    /// Ready means that the buffer contains data that can be sent.
    #[inline]
    fn set_ready(&self, ready: bool) {
        if ready {
            self.usb()
                .epstatusset(self.index())
                .write(|w| w.bk1rdy().set_bit());
        } else {
            self.usb()
                .epstatusclr(self.index())
                .write(|w| w.bk1rdy().set_bit());
        }
    }

    /// Acknowledges the signal that the last packet was sent.
    #[inline]
    fn clear_transfer_complete(&self) {
        // Clear bits in epintflag by writing them to 1
        self.usb()
            .epintflag(self.index())
            .write(|w| w.trcpt1().set_bit().trfail1().set_bit());
    }

    /// Indicates if a transfer is complete or pending.
    #[inline]
    fn is_transfer_complete(&self) -> bool {
        self.usb().epintflag(self.index()).read().trcpt1().bit()
    }

    /// Writes out endpoint configuration to its in-memory descriptor.
    fn flush_config(&mut self) {
        let config = *self.config();
        {
            let desc = self.desc_bank();
            desc.set_address(config.addr as *mut u8);
            desc.set_endpoint_size(config.max_packet_size);
            desc.set_multi_packet_size(0);
            desc.set_byte_count(0);
        }
    }

    /// Enables endpoint-specific interrupts.
    fn setup_ep_interrupts(&mut self) {
        self.usb()
            .epintenset(self.index())
            .write(|w| w.trcpt1().set_bit());
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
        self.usb().epintflag(self.index()).read().stall1().bit()
    }

    fn set_stall(&mut self, stall: bool) {
        if stall {
            self.usb()
                .epstatusset(self.index())
                .write(|w| w.stallrq1().set_bit())
        } else {
            self.usb()
                .epstatusclr(self.index())
                .write(|w| w.stallrq1().set_bit())
        }
    }
}

impl Bank<'_, OutBank> {
    fn desc_bank(&mut self) -> &mut DeviceDescBank {
        let idx = self.index();
        self.desc.bank(idx, 0)
    }

    /// Returns true if Bank 0 is Ready and thus has data that can be read.
    #[inline]
    fn is_ready(&self) -> bool {
        self.usb().epstatus(self.index()).read().bk0rdy().bit()
    }

    /// Set Bank 0 Ready.
    /// Ready means that the buffer contains data that can be read.
    #[inline]
    fn set_ready(&self, ready: bool) {
        if ready {
            self.usb()
                .epstatusset(self.index())
                .write(|w| w.bk0rdy().set_bit());
        } else {
            self.usb()
                .epstatusclr(self.index())
                .write(|w| w.bk0rdy().set_bit());
        }
    }

    /// Acknowledges the signal that data has been received.
    #[inline]
    fn clear_transfer_complete(&self) {
        // Clear bits in epintflag by writing them to 1
        self.usb()
            .epintflag(self.index())
            .write(|w| w.trcpt0().set_bit().trfail0().set_bit());
    }

    /// Returns true if a Received Setup interrupt has occurred.
    /// This indicates that the read buffer holds a SETUP packet.
    #[inline]
    fn received_setup_interrupt(&self) -> bool {
        self.usb().epintflag(self.index()).read().rxstp().bit()
    }

    /// Acknowledges the signal that a SETUP packet was received
    /// successfully.
    #[inline]
    fn clear_received_setup_interrupt(&self) {
        // Clear bits in epintflag by writing them to 1
        self.usb()
            .epintflag(self.index())
            .write(|w| w.rxstp().set_bit());
    }

    /// Writes out endpoint configuration to its in-memory descriptor.
    fn flush_config(&mut self) {
        let config = *self.config();
        {
            let desc = self.desc_bank();
            desc.set_address(config.addr as *mut u8);
            desc.set_endpoint_size(config.max_packet_size);
            desc.set_multi_packet_size(0);
            desc.set_byte_count(0);
        }
    }

    /// Enables endpoint-specific interrupts.
    fn setup_ep_interrupts(&mut self) {
        self.usb()
            .epintenset(self.index())
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
        self.usb().epintflag(self.index()).read().stall0().bit()
    }

    fn set_stall(&mut self, stall: bool) {
        if stall {
            self.usb()
                .epstatusset(self.index())
                .write(|w| w.stallrq0().set_bit())
        } else {
            self.usb()
                .epstatusclr(self.index())
                .write(|w| w.stallrq0().set_bit())
        }
    }
}

impl Inner {
    fn bank0(&'_ self, ep: EndpointAddress) -> UsbResult<Bank<'_, OutBank>> {
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

    fn bank1(&'_ self, ep: EndpointAddress) -> UsbResult<Bank<'_, InBank>> {
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
}

impl UsbBus {
    pub fn new(
        _clock: &clock::UsbClock,
        pm: &mut Pm,
        dm_pad: impl AnyPin<Id = PA24>,
        dp_pad: impl AnyPin<Id = PA25>,
        _usb: Usb,
    ) -> Self {
        pm.apbbmask().modify(|_, w| w.usb_().set_bit());

        let desc = RefCell::new(Descriptors::new());

        let inner = Inner {
            _dm_pad: dm_pad.into().into_mode::<AlternateG>(),
            _dp_pad: dp_pad.into().into_mode::<AlternateG>(),
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
    #[hal_cfg("usb-d11")]
    fn usb(&self) -> &Device {
        unsafe { (*Usb::ptr()).device() }
    }

    #[hal_cfg("usb-d21")]
    fn usb(&self) -> &Device {
        unsafe { (*Usb::ptr()).device() }
    }

    fn set_stall<EP: Into<EndpointAddress>>(&self, ep: EP, stall: bool) {
        let ep = ep.into();
        if ep.is_out() {
            if let Ok(mut bank) = self.bank0(ep) {
                bank.set_stall(stall);
            }
        } else if let Ok(mut bank) = self.bank1(ep) {
            bank.set_stall(stall);
        }
    }
}

#[derive(Copy, Clone)]
enum FlushConfigMode {
    // Write configuration to all configured endpoints.
    Full,
    // Refresh configuration which was reset due to a bus reset.
    ProtocolReset,
}

impl Inner {
    #[hal_macro_helper]
    fn enable(&mut self) {
        let usb = self.usb();
        usb.ctrla().modify(|_, w| w.swrst().set_bit());
        while usb.syncbusy().read().swrst().bit_is_set() {}

        let addr = self.desc.borrow().address();
        usb.descadd().write(|w| unsafe { w.descadd().bits(addr) });
        usb.padcal().modify(|_, w| unsafe {
            w.transn().bits(usb_transn_cal());
            w.transp().bits(usb_transp_cal());
            w.trim().bits(usb_trim_cal())
        });

        #[hal_cfg("usb-d11")]
        usb.qosctrl().modify(|_, w| unsafe {
            w.dqos().bits(0b11);
            w.cqos().bits(0b11)
        });
        #[hal_cfg("usb-d21")]
        usb.qosctrl().modify(|_, w| unsafe {
            w.dqos().bits(0b11);
            w.cqos().bits(0b11)
        });

        usb.ctrla().modify(|_, w| {
            w.mode().device();
            w.runstdby().set_bit()
        });
        // full speed
        usb.ctrlb().modify(|_, w| w.spdconf().fs());

        usb.ctrla().modify(|_, w| w.enable().set_bit());
        while usb.syncbusy().read().enable().bit_is_set() {}

        // Clear pending.
        usb.intflag()
            .write(|w| unsafe { w.bits(usb.intflag().read().bits()) });
        usb.intenset().write(|w| w.eorst().set_bit());

        // Configure the endpoints before we attach, as hosts may enumerate
        // before attempting a USB protocol reset.
        self.flush_eps(FlushConfigMode::Full);

        usb.ctrlb().modify(|_, w| w.detach().clear_bit());
    }

    /// Enables/disables the Start Of Frame (SOF) interrupt
    fn sof_interrupt(&self, enable: bool) {
        if enable {
            self.usb().intenset().write(|w| w.sof().set_bit());
        } else {
            self.usb().intenclr().write(|w| w.sof().set_bit());
        }
    }

    /// Configures all endpoints based on prior calls to alloc_ep().
    fn flush_eps(&self, mode: FlushConfigMode) {
        for idx in 0..8 {
            match (mode, idx) {
                // A flush due to a protocol reset need not reconfigure endpoint 0,
                // except for enabling its interrupts.
                (FlushConfigMode::ProtocolReset, 0) => {
                    self.setup_ep_interrupts(EndpointAddress::from_parts(idx, UsbDirection::Out));
                    self.setup_ep_interrupts(EndpointAddress::from_parts(idx, UsbDirection::In));
                }
                // A full flush configures all provisioned endpoints + enables interrupts.
                // Endpoints 1-8 have identical behaviour when flushed due to protocol reset.
                (FlushConfigMode::Full, _) | (FlushConfigMode::ProtocolReset, _) => {
                    // Write bank configuration & endpoint type.
                    self.flush_ep(idx);
                    // Endpoint interrupts are configured after the write to EPTYPE, as it appears
                    // writes to EPINTEN*[n] do not take effect unless the
                    // endpoint is already somewhat configured. The datasheet is
                    // ambiguous here, section 38.8.3.7 (Device Interrupt EndPoint Set n)
                    // of the SAM D5x/E5x states:
                    //    "This register is cleared by USB reset or when EPEN[n] is zero"
                    // EPEN[n] is not a register that exists, nor does it align with any other
                    // terminology. We assume this means setting EPCFG[n] to a
                    // non-zero value, but we do interrupt configuration last to
                    // be sure.
                    self.setup_ep_interrupts(EndpointAddress::from_parts(idx, UsbDirection::Out));
                    self.setup_ep_interrupts(EndpointAddress::from_parts(idx, UsbDirection::In));
                }
            }
        }
    }

    /// flush_ep commits bank descriptor information for the endpoint pair,
    /// and enables the endpoint according to its type.
    fn flush_ep(&self, idx: usize) {
        let cfg = self.usb().epcfg(idx);
        let info = &self.endpoints.borrow().endpoints[idx];
        // Write bank descriptors first. We do this so there is no period in
        // which the endpoint is enabled but has an invalid descriptor.
        if let Ok(mut bank) = self.bank0(EndpointAddress::from_parts(idx, UsbDirection::Out)) {
            bank.flush_config();
        }
        if let Ok(mut bank) = self.bank1(EndpointAddress::from_parts(idx, UsbDirection::In)) {
            bank.flush_config();
        }

        // Set the endpoint type. At this point, the endpoint is enabled.
        cfg.modify(|_, w| unsafe {
            w.eptype0()
                .bits(info.bank0.ep_type as u8)
                .eptype1()
                .bits(info.bank1.ep_type as u8)
        });
    }

    /// setup_ep_interrupts enables interrupts for the given endpoint address.
    fn setup_ep_interrupts(&self, ep_addr: EndpointAddress) {
        if ep_addr.is_out() {
            if let Ok(mut bank) = self.bank0(ep_addr) {
                bank.setup_ep_interrupts();
            }
        } else if let Ok(mut bank) = self.bank1(ep_addr) {
            bank.setup_ep_interrupts();
        }
    }

    /// protocol_reset is called by the USB HAL when it detects the host has
    /// performed a USB reset.
    fn protocol_reset(&self) {
        self.flush_eps(FlushConfigMode::ProtocolReset);
    }

    fn suspend(&self) {}

    fn resume(&self) {}

    fn alloc_ep(
        &mut self,
        dir: UsbDirection,
        addr: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval: u8,
    ) -> UsbResult<EndpointAddress> {
        // The USB hardware encodes the maximum packet size in 3 bits, so
        // reserve enough buffer that the hardware won't overwrite it even if
        // the other side issues an overly-long transfer.
        let allocated_size = match max_packet_size {
            1..=8 => 8,
            9..=16 => 16,
            17..=32 => 32,
            33..=64 => 64,
            65..=128 => 128,
            129..=256 => 256,
            257..=512 => 512,
            513..=1023 => 1024,
            _ => return Err(UsbError::Unsupported),
        };

        let buffer = self.buffers.borrow_mut().allocate_buffer(allocated_size)?;

        let mut endpoints = self.endpoints.borrow_mut();

        let idx = match addr {
            None => endpoints.find_free_endpoint(dir)?,
            Some(addr) => addr.index(),
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

        Ok(addr)
    }

    fn set_device_address(&self, addr: u8) {
        self.usb()
            .dadd()
            .write(|w| unsafe { w.dadd().bits(addr).adden().set_bit() });
    }

    fn check_sof_interrupt(&self) -> bool {
        if self.usb().intflag().read().sof().bit() {
            self.usb().intflag().write(|w| w.sof().set_bit());
            return true;
        }
        false
    }

    fn poll(&self) -> PollResult {
        let intflags = self.usb().intflag().read();
        if intflags.eorst().bit() {
            // end of reset interrupt
            self.usb().intflag().write(|w| w.eorst().set_bit());
            return PollResult::Reset;
        }
        // As the suspend & wakup interrupts/states cannot distinguish between
        // unconnected & unsuspended, we do not handle them to avoid spurious
        // transitions.

        let mut ep_out = 0;
        let mut ep_in_complete = 0;
        let mut ep_setup = 0;

        let intbits = self.usb().epintsmry().read().bits();

        for ep in 0..8u16 {
            let mask = 1 << ep;

            let idx = ep as usize;

            if (intbits & mask) != 0 {
                if let Ok(bank1) = self.bank1(EndpointAddress::from_parts(idx, UsbDirection::In)) {
                    if bank1.is_transfer_complete() {
                        bank1.clear_transfer_complete();
                        ep_in_complete |= mask;
                    }
                }
            }

            // Can't test intbits, because bk0rdy doesn't interrupt
            if let Ok(bank0) = self.bank0(EndpointAddress::from_parts(idx, UsbDirection::Out)) {
                if bank0.received_setup_interrupt() {
                    ep_setup |= mask;

                    // The RXSTP interrupt is not cleared here, because doing so
                    // would allow the USB hardware to overwrite the received
                    // data, potentially before it is `read()` - see SAMD21
                    // datasheet "32.6.2.6 Management of SETUP Transactions".
                    // Setup events are only relevant for control endpoints, and
                    // in typical USB devices, endpoint 0 is the only control
                    // endpoint. The usb-device `poll()` method, which calls
                    // this `poll()`, will immediately `read()` endpoint 0 when
                    // its setup bit is set.
                }

                // Clear the transfer complete and transfer failed interrupt flags
                // so that execution leaves the USB interrupt until the host makes
                // another transaction.  The transfer failed flag may have been set
                // if an OUT transaction wasn't read() from the endpoint by the
                // Class; the hardware will have NAKed (unless the endpoint is
                // isochronous) and the host may retry.
                bank0.clear_transfer_complete();

                // Use the bk0rdy flag via is_ready() to indicate that data has been
                // received successfully, rather than the interrupting trcpt0 via
                // is_transfer_ready(), because data may have been received on an
                // earlier poll() which cleared trcpt0.  bk0rdy is cleared in the
                // endpoint read().
                if bank0.is_ready() {
                    ep_out |= mask;
                }
            }
        }

        if ep_out == 0 && ep_in_complete == 0 && ep_setup == 0 {
            PollResult::None
        } else {
            PollResult::Data {
                ep_out,
                ep_in_complete,
                ep_setup,
            }
        }
    }

    fn write(&self, ep: EndpointAddress, buf: &[u8]) -> UsbResult<usize> {
        let mut bank = self.bank1(ep)?;

        if bank.is_ready() {
            // Waiting for the host to pick up the existing data
            return Err(UsbError::WouldBlock);
        }

        let size = bank.write(buf);

        bank.clear_transfer_complete();
        bank.set_ready(true); // ready to be sent

        size
    }

    fn read(&self, ep: EndpointAddress, buf: &mut [u8]) -> UsbResult<usize> {
        let mut bank = self.bank0(ep)?;
        let rxstp = bank.received_setup_interrupt();

        if bank.is_ready() || rxstp {
            let size = bank.read(buf);

            if rxstp {
                bank.clear_received_setup_interrupt();
            }

            bank.clear_transfer_complete();
            bank.set_ready(false);

            size
        } else {
            Err(UsbError::WouldBlock)
        }
    }

    fn is_stalled(&self, ep: EndpointAddress) -> bool {
        if ep.is_out() {
            self.bank0(ep).unwrap().is_stalled()
        } else {
            self.bank1(ep).unwrap().is_stalled()
        }
    }

    fn set_stalled(&self, ep: EndpointAddress, stalled: bool) {
        self.set_stall(ep, stalled);
    }
}

impl UsbBus {
    /// Enables the Start Of Frame (SOF) interrupt
    pub fn enable_sof_interrupt(&self) {
        disable_interrupts(|cs| self.inner.borrow(cs).borrow_mut().sof_interrupt(true))
    }

    /// Disables the Start Of Frame (SOF) interrupt
    pub fn disable_sof_interrupt(&self) {
        disable_interrupts(|cs| self.inner.borrow(cs).borrow_mut().sof_interrupt(false))
    }

    /// Checks, and clears if set, the Start Of Frame (SOF) interrupt flag
    pub fn check_sof_interrupt(&self) -> bool {
        disable_interrupts(|cs| self.inner.borrow(cs).borrow_mut().check_sof_interrupt())
    }
}

impl usb_device::bus::UsbBus for UsbBus {
    fn enable(&mut self) {
        disable_interrupts(|cs| self.inner.borrow(cs).borrow_mut().enable())
    }

    fn reset(&self) {
        disable_interrupts(|cs| self.inner.borrow(cs).borrow().protocol_reset())
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
