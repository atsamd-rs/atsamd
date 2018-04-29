//! USB Device support

use atsamd21g18a::usb::DEVICE;
use atsamd21g18a::{PM, USB};
use calibration::{usb_transn_cal, usb_transp_cal, usb_trim_cal};
use clock;
use core::ptr::null_mut;
use gpio;

/// Emit SOF at 1Khz on this pin when configured as function G
pub type SofPad = gpio::Pa23<gpio::PfG>;

/// USB D- is connected here
pub type DmPad = gpio::Pa24<gpio::PfG>;

/// USB D+ is connected here
pub type DpPad = gpio::Pa25<gpio::PfG>;

pub struct UsbDevice<'a> {
    desc: &'a mut Descriptors,
    dm_pad: DmPad,
    dp_pad: DpPad,
    usb: USB,
}

#[repr(packed)]
pub struct DeviceDescBank {
    /// endpoint data buffer, must be 32-bit aligned
    addr: *mut u8,
    pcksize: u32,
    extreg: u16,
    status_bk: u8,
    _reserved: [u8; 5],
}

impl DeviceDescBank {
    fn new() -> Self {
        Self {
            addr: null_mut(),
            pcksize: 0,
            extreg: 0,
            status_bk: 0,
            _reserved: [0, 0, 0, 0, 0],
        }
    }

    /// This bit defines the automatic Zero Length Packet mode of the endpoint.
    /// When enabled, the USB module will manage the ZLP handshake by hardware.
    /// This bit is for IN endpoints only. When disabled the handshake should be
    /// managed by firmware.
    pub fn set_auto_zlp(&mut self, enable: bool) {
        let val = if enable { 1u32 << 31 } else { 0 };
        self.pcksize = (self.pcksize & !(1 << 31)) | val;
    }

    /// These bits contains the maximum packet size of the endpoint.
    pub fn set_endpoint_size(&mut self, size: u16) {
        let size = match size {
            8 => 0u32,
            16 => 1,
            32 => 2,
            64 => 3,
            128 => 4,
            256 => 5,
            512 => 6,
            1023 => 7,
            _ => 0,
        };
        self.pcksize = (self.pcksize & !(7 << 28)) | (size << 28);
    }

    /// For IN endpoints, MULTI_PACKET_SIZE holds the total number of bytes
    /// sent. MULTI_PACKET_SIZE should be written to zero when setting up a new
    /// transfer.  For OUT endpoints, MULTI_PACKET_SIZE holds the total data
    /// size for the complete transfer. This value must be a multiple of the
    /// maximum packet size.
    pub fn set_multi_packet_size(&mut self, size: u16) {
        self.pcksize = (self.pcksize & !(0b11_1111_1111_1111 << 14)) | ((size as u32) << 14);
    }

    /// For IN endpoints, BYTE_COUNT holds the number of bytes to be sent in the
    /// next IN transaction.  For OUT endpoint or SETUP endpoints, BYTE_COUNT
    /// holds the number of bytes received upon the last OUT or SETUP
    /// transaction.
    pub fn set_byte_count(&mut self, size: u16) {
        self.pcksize = (self.pcksize & !0b11_1111_1111_1111) | (size as u32);
    }

    pub fn link_state(&self) -> u8 {
        // every value except 1 (L1 sleep) is reserved
        (self.extreg >> 4) as u8 & 0b1111
    }

    /// best effort service latency
    pub fn besl(&self) -> u8 {
        (self.extreg >> 8) as u8 & 0b1111
    }

    pub fn remote_wake(&self) -> bool {
        ((self.extreg >> 12) & 1) != 0
    }

    /// These bits define the SUBPID field of a received extended token. These
    /// bits are updated when the USB has answered by an handshake token
    /// ACK to a LPM transaction
    pub fn subpid(&self) -> u8 {
        self.extreg as u8 & 4
    }

    /// This bit defines the Error Flow Status.  This bit is set when a Error
    /// Flow has been detected during transfer from/towards this bank.  For OUT
    /// transfer, a NAK handshake has been sent.  For Isochronous OUT transfer,
    /// an overrun condition has occurred.  For IN transfer, this bit is not
    /// valid. EPSTATUS.TRFAIL0 and EPSTATUS.TRFAIL1 should reflect the flow
    /// errors.
    pub fn error_flow(&self) -> bool {
        (self.status_bk & 0b10) != 0
    }

    /// This bit defines the CRC Error Status.  This bit is set when a CRC
    /// error has been detected in an isochronous OUT endpoint bank
    pub fn crc_error(&self) -> bool {
        (self.status_bk & 1) != 0
    }
}

#[repr(packed)]
pub struct DeviceDescriptor {
    bank: [DeviceDescBank; 2],
}

impl DeviceDescriptor {
    fn new() -> Self {
        Self {
            bank: [DeviceDescBank::new(), DeviceDescBank::new()],
        }
    }
}

#[repr(packed)]
pub struct Descriptors {
    desc: [DeviceDescriptor; 8],
}

impl Descriptors {
    pub fn new() -> Self {
        Self {
            desc: [
                DeviceDescriptor::new(),
                DeviceDescriptor::new(),
                DeviceDescriptor::new(),
                DeviceDescriptor::new(),
                DeviceDescriptor::new(),
                DeviceDescriptor::new(),
                DeviceDescriptor::new(),
                DeviceDescriptor::new(),
            ],
        }
    }
}

impl<'a> UsbDevice<'a> {
    pub fn new(
        _clock: &clock::UsbClock,
        pm: &mut PM,
        dm_pad: DmPad,
        dp_pad: DpPad,
        usb: USB,
        desc: &'a mut Descriptors,
    ) -> Self {
        pm.apbbmask.modify(|_, w| w.usb_().set_bit());
        Self {
            dm_pad,
            dp_pad,
            usb,
            desc,
        }
    }

    fn usb(&self) -> &DEVICE {
        unsafe { &self.usb.device }
    }

    /// Reset the USB hardware.
    pub fn reset(&mut self) {
        let addr = self.desc as *mut _ as u32;
        let usb = self.usb();
        usb.ctrla.modify(|_, w| w.swrst().set_bit());
        while usb.syncbusy.read().swrst().bit_is_set() {}

        usb.descadd.write(|w| unsafe { w.descadd().bits(addr) });
        usb.padcal.modify(|_, w| unsafe {
            w.transn().bits(usb_transn_cal());
            w.transp().bits(usb_transp_cal());
            w.trim().bits(usb_trim_cal())
        });
        usb.ctrla.modify(|_, w| {
            w.mode().device();
            w.runstdby().set_bit()
        });
        // full speed
        usb.ctrlb.modify(|_, w| w.spdconf().fs());

        usb.ctrla.modify(|_, w| w.enable().set_bit());
    }

    pub fn free(self) -> (DmPad, DpPad, USB) {
        (self.dm_pad, self.dp_pad, self.usb)
    }
}
