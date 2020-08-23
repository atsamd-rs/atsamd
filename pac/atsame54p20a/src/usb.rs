#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_host: [u8; 266usize],
}
impl RegisterBlock {
    #[doc = "0x00 - USB is Host"]
    #[inline(always)]
    pub fn host(&self) -> &HOST {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const HOST) }
    }
    #[doc = "0x00 - USB is Host"]
    #[inline(always)]
    pub fn host_mut(&self) -> &mut HOST {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut HOST) }
    }
    #[doc = "0x00 - USB is Device"]
    #[inline(always)]
    pub fn device(&self) -> &DEVICE {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const DEVICE) }
    }
    #[doc = "0x00 - USB is Device"]
    #[inline(always)]
    pub fn device_mut(&self) -> &mut DEVICE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut DEVICE) }
    }
}
#[doc = r"Register block"]
#[repr(C)]
pub struct DEVICE {
    #[doc = "0x00 - Control A"]
    pub ctrla: self::device::CTRLA,
    _reserved1: [u8; 1usize],
    #[doc = "0x02 - Synchronization Busy"]
    pub syncbusy: self::device::SYNCBUSY,
    #[doc = "0x03 - USB Quality Of Service"]
    pub qosctrl: self::device::QOSCTRL,
    _reserved3: [u8; 4usize],
    #[doc = "0x08 - DEVICE Control B"]
    pub ctrlb: self::device::CTRLB,
    #[doc = "0x0a - DEVICE Device Address"]
    pub dadd: self::device::DADD,
    _reserved5: [u8; 1usize],
    #[doc = "0x0c - DEVICE Status"]
    pub status: self::device::STATUS,
    #[doc = "0x0d - Finite State Machine Status"]
    pub fsmstatus: self::device::FSMSTATUS,
    _reserved7: [u8; 2usize],
    #[doc = "0x10 - DEVICE Device Frame Number"]
    pub fnum: self::device::FNUM,
    _reserved8: [u8; 2usize],
    #[doc = "0x14 - DEVICE Device Interrupt Enable Clear"]
    pub intenclr: self::device::INTENCLR,
    _reserved9: [u8; 2usize],
    #[doc = "0x18 - DEVICE Device Interrupt Enable Set"]
    pub intenset: self::device::INTENSET,
    _reserved10: [u8; 2usize],
    #[doc = "0x1c - DEVICE Device Interrupt Flag"]
    pub intflag: self::device::INTFLAG,
    _reserved11: [u8; 2usize],
    #[doc = "0x20 - DEVICE End Point Interrupt Summary"]
    pub epintsmry: self::device::EPINTSMRY,
    _reserved12: [u8; 2usize],
    #[doc = "0x24 - Descriptor Address"]
    pub descadd: self::device::DESCADD,
    #[doc = "0x28 - USB PAD Calibration"]
    pub padcal: self::device::PADCAL,
    _reserved14: [u8; 214usize],
    #[doc = "0x100 - DEVICE_ENDPOINT\\[%s\\]"]
    pub device_endpoint0: self::device::DEVICE_ENDPOINT,
    _reserved15: [u8; 22usize],
    #[doc = "0x120 - DEVICE_ENDPOINT\\[%s\\]"]
    pub device_endpoint1: self::device::DEVICE_ENDPOINT,
    _reserved16: [u8; 22usize],
    #[doc = "0x140 - DEVICE_ENDPOINT\\[%s\\]"]
    pub device_endpoint2: self::device::DEVICE_ENDPOINT,
    _reserved17: [u8; 22usize],
    #[doc = "0x160 - DEVICE_ENDPOINT\\[%s\\]"]
    pub device_endpoint3: self::device::DEVICE_ENDPOINT,
    _reserved18: [u8; 22usize],
    #[doc = "0x180 - DEVICE_ENDPOINT\\[%s\\]"]
    pub device_endpoint4: self::device::DEVICE_ENDPOINT,
    _reserved19: [u8; 22usize],
    #[doc = "0x1a0 - DEVICE_ENDPOINT\\[%s\\]"]
    pub device_endpoint5: self::device::DEVICE_ENDPOINT,
    _reserved20: [u8; 22usize],
    #[doc = "0x1c0 - DEVICE_ENDPOINT\\[%s\\]"]
    pub device_endpoint6: self::device::DEVICE_ENDPOINT,
    _reserved21: [u8; 22usize],
    #[doc = "0x1e0 - DEVICE_ENDPOINT\\[%s\\]"]
    pub device_endpoint7: self::device::DEVICE_ENDPOINT,
}
#[doc = r"Register block"]
#[doc = "USB is Device"]
pub mod device;
#[doc = r"Register block"]
#[repr(C)]
pub struct HOST {
    #[doc = "0x00 - Control A"]
    pub ctrla: self::host::CTRLA,
    _reserved1: [u8; 1usize],
    #[doc = "0x02 - Synchronization Busy"]
    pub syncbusy: self::host::SYNCBUSY,
    #[doc = "0x03 - USB Quality Of Service"]
    pub qosctrl: self::host::QOSCTRL,
    _reserved3: [u8; 4usize],
    #[doc = "0x08 - HOST Control B"]
    pub ctrlb: self::host::CTRLB,
    #[doc = "0x0a - HOST Host Start Of Frame Control"]
    pub hsofc: self::host::HSOFC,
    _reserved5: [u8; 1usize],
    #[doc = "0x0c - HOST Status"]
    pub status: self::host::STATUS,
    #[doc = "0x0d - Finite State Machine Status"]
    pub fsmstatus: self::host::FSMSTATUS,
    _reserved7: [u8; 2usize],
    #[doc = "0x10 - HOST Host Frame Number"]
    pub fnum: self::host::FNUM,
    #[doc = "0x12 - HOST Host Frame Length"]
    pub flenhigh: self::host::FLENHIGH,
    _reserved9: [u8; 1usize],
    #[doc = "0x14 - HOST Host Interrupt Enable Clear"]
    pub intenclr: self::host::INTENCLR,
    _reserved10: [u8; 2usize],
    #[doc = "0x18 - HOST Host Interrupt Enable Set"]
    pub intenset: self::host::INTENSET,
    _reserved11: [u8; 2usize],
    #[doc = "0x1c - HOST Host Interrupt Flag"]
    pub intflag: self::host::INTFLAG,
    _reserved12: [u8; 2usize],
    #[doc = "0x20 - HOST Pipe Interrupt Summary"]
    pub pintsmry: self::host::PINTSMRY,
    _reserved13: [u8; 2usize],
    #[doc = "0x24 - Descriptor Address"]
    pub descadd: self::host::DESCADD,
    #[doc = "0x28 - USB PAD Calibration"]
    pub padcal: self::host::PADCAL,
    _reserved15: [u8; 214usize],
    #[doc = "0x100 - HOST_PIPE\\[%s\\]"]
    pub host_pipe0: self::host::HOST_PIPE,
    _reserved16: [u8; 22usize],
    #[doc = "0x120 - HOST_PIPE\\[%s\\]"]
    pub host_pipe1: self::host::HOST_PIPE,
    _reserved17: [u8; 22usize],
    #[doc = "0x140 - HOST_PIPE\\[%s\\]"]
    pub host_pipe2: self::host::HOST_PIPE,
    _reserved18: [u8; 22usize],
    #[doc = "0x160 - HOST_PIPE\\[%s\\]"]
    pub host_pipe3: self::host::HOST_PIPE,
    _reserved19: [u8; 22usize],
    #[doc = "0x180 - HOST_PIPE\\[%s\\]"]
    pub host_pipe4: self::host::HOST_PIPE,
    _reserved20: [u8; 22usize],
    #[doc = "0x1a0 - HOST_PIPE\\[%s\\]"]
    pub host_pipe5: self::host::HOST_PIPE,
    _reserved21: [u8; 22usize],
    #[doc = "0x1c0 - HOST_PIPE\\[%s\\]"]
    pub host_pipe6: self::host::HOST_PIPE,
    _reserved22: [u8; 22usize],
    #[doc = "0x1e0 - HOST_PIPE\\[%s\\]"]
    pub host_pipe7: self::host::HOST_PIPE,
}
#[doc = r"Register block"]
#[doc = "USB is Host"]
pub mod host;
