use bitfield::bitfield;
use core::fmt::{Debug, Error as FmtError, Formatter};
use core::mem;
use core::ptr::null_mut;

type FmtResult = Result<(), FmtError>;

bitfield! {
    struct PckSize(u32);
    impl Debug;
    pub byte_count, set_byte_count: 13, 0;
    pub multi_packet_size, set_multi_packet_size: 27, 14;
    pub size, set_size: 30, 28;
    pub auto_zlp, set_auto_zlp : 31;
}

bitfield! {
    struct ExtReg(u16);
    impl Debug;
    pub subpid, set_subpid: 3, 0;
    pub link_state, set_link_state: 7, 4;
    pub besl, set_besl: 11, 8;
    pub remote_wake, set_remote_wake: 12;
}

bitfield! {
    struct StatusBk(u8);
    impl Debug;
    pub crc_error, set_crc_error: 0;
    pub error_flow, set_error_flow: 1;
}

#[repr(C)]
#[derive(Debug)]
pub struct DeviceDescBank {
    /// endpoint data buffer, must be 32-bit aligned
    addr: *mut u8,
    pcksize: PckSize,
    extreg: ExtReg,
    status_bk: StatusBk,
    _reserved: [u8; 5],
}

impl DeviceDescBank {
    fn new() -> Self {
        debug_assert_eq!(16, mem::size_of::<DeviceDescBank>());
        Self {
            addr: null_mut(),
            pcksize: PckSize(0),
            extreg: ExtReg(0),
            status_bk: StatusBk(0),
            _reserved: [0, 0, 0, 0, 0],
        }
    }

    /// This bit defines the automatic Zero Length Packet mode of the endpoint.
    /// When enabled, the USB module will manage the ZLP handshake by hardware.
    /// This bit is for IN endpoints only. When disabled the handshake should be
    /// managed by firmware.
    #[allow(unused)]
    pub fn set_auto_zlp(&mut self, enable: bool) {
        self.pcksize.set_auto_zlp(enable);
    }

    /// These bits contains the maximum packet size of the endpoint.
    ///
    /// The maximum packet size is encoded in 3 bits; this method takes any u16
    /// below 1024B and rounds up to the lowest endpoint size value which will
    /// accommodate `size`.  Panics if a `size` > 1023 is supplied.
    pub fn set_endpoint_size(&mut self, size: u16) {
        let size = match size {
            1..=8 => 0u32,
            9..=16 => 1,
            17..=32 => 2,
            33..=64 => 3,
            65..=128 => 4,
            129..=256 => 5,
            257..=512 => 6,
            513..=1023 => 7,
            _ => unreachable!(),
        };
        self.pcksize.set_size(size);
    }

    #[allow(unused)]
    pub fn get_endpoint_size(&self) -> u16 {
        let bits = self.pcksize.size();
        match bits {
            0 => 8,
            1 => 16,
            2 => 32,
            3 => 64,
            4 => 128,
            5 => 256,
            6 => 512,
            7 => 1023,
            _ => unreachable!(),
        }
    }

    /// For IN endpoints, MULTI_PACKET_SIZE holds the total number of bytes
    /// sent. MULTI_PACKET_SIZE should be written to zero when setting up a new
    /// transfer.
    pub fn set_multi_packet_size(&mut self, size: u16) {
        self.pcksize.set_multi_packet_size(size.into());
    }

    /// For OUT endpoints, MULTI_PACKET_SIZE holds the total data
    /// size for the complete transfer. This value must be a multiple of the
    /// maximum packet size.
    #[allow(dead_code)]
    pub fn get_multi_packet_size(&self) -> u16 {
        self.pcksize.multi_packet_size() as u16
    }

    /// For IN endpoints, BYTE_COUNT holds the number of bytes to be sent in the
    /// next IN transaction.
    /// For OUT endpoint or SETUP endpoints, BYTE_COUNT
    /// holds the number of bytes received upon the last OUT or SETUP
    /// transaction.
    pub fn set_byte_count(&mut self, size: u16) {
        self.pcksize.set_byte_count(size.into());
    }

    /// For IN endpoints, BYTE_COUNT holds the number of bytes to be sent in the
    /// next IN transaction.
    /// For OUT endpoint or SETUP endpoints, BYTE_COUNT
    /// holds the number of bytes received upon the last OUT or SETUP
    /// transaction.
    pub fn get_byte_count(&self) -> u16 {
        self.pcksize.byte_count() as u16
    }

    #[allow(unused)]
    pub fn link_state(&self) -> u8 {
        // every value except 1 (L1 sleep) is reserved
        self.extreg.link_state() as u8
    }

    /// best effort service latency
    #[allow(unused)]
    pub fn besl(&self) -> u8 {
        self.extreg.besl() as u8
    }

    #[allow(unused)]
    pub fn remote_wake(&self) -> bool {
        self.extreg.remote_wake()
    }

    /// These bits define the SUBPID field of a received extended token. These
    /// bits are updated when the USB has answered by an handshake token
    /// ACK to a LPM transaction
    #[allow(unused)]
    pub fn subpid(&self) -> u8 {
        self.extreg.subpid() as u8
    }

    /// This bit defines the Error Flow Status.  This bit is set when a Error
    /// Flow has been detected during transfer from/towards this bank.  For OUT
    /// transfer, a NAK handshake has been sent.  For Isochronous OUT transfer,
    /// an overrun condition has occurred.  For IN transfer, this bit is not
    /// valid. EPSTATUS.TRFAIL0 and EPSTATUS.TRFAIL1 should reflect the flow
    /// errors.
    #[allow(unused)]
    pub fn error_flow(&self) -> bool {
        self.status_bk.error_flow()
    }

    /// This bit defines the CRC Error Status.  This bit is set when a CRC
    /// error has been detected in an isochronous OUT endpoint bank
    #[allow(unused)]
    pub fn crc_error(&self) -> bool {
        self.status_bk.crc_error()
    }

    pub fn set_address(&mut self, address: *mut u8) {
        self.addr = address;
    }

    pub fn get_address(&self) -> *mut u8 {
        self.addr
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct DeviceDescriptor {
    bank: [DeviceDescBank; 2],
}

impl DeviceDescriptor {
    fn new() -> Self {
        debug_assert_eq!(32, mem::size_of::<DeviceDescriptor>());
        Self {
            bank: [DeviceDescBank::new(), DeviceDescBank::new()],
        }
    }
}

pub struct Descriptors {
    desc: [DeviceDescriptor; 8],
}

impl Debug for Descriptors {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        for ep in 0..8 {
            write!(fmt, "\nep{}: {:?}", ep, &self.desc[ep])?;
        }
        Ok(())
    }
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

    pub fn address(&self) -> u32 {
        &self.desc as *const _ as u32
    }

    pub fn bank(&mut self, idx: usize, bank: usize) -> &mut DeviceDescBank {
        &mut self.desc[idx].bank[bank]
    }
}

unsafe impl Send for DeviceDescBank {}
