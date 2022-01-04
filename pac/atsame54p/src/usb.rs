#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_host: [u8; 0x01ea],
}
impl RegisterBlock {
    #[doc = "0x00..0x1ea - USB is Host"]
    #[inline(always)]
    pub fn host(&self) -> &HOST {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const HOST) }
    }
    #[doc = "0x00..0x1ea - USB is Device"]
    #[inline(always)]
    pub fn device(&self) -> &DEVICE {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const DEVICE) }
    }
}
#[doc = r"Register block"]
#[repr(C)]
pub struct DEVICE {
    #[doc = "0x00 - Control A"]
    pub ctrla: crate::Reg<self::device::ctrla::CTRLA_SPEC>,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - Synchronization Busy"]
    pub syncbusy: crate::Reg<self::device::syncbusy::SYNCBUSY_SPEC>,
    #[doc = "0x03 - USB Quality Of Service"]
    pub qosctrl: crate::Reg<self::device::qosctrl::QOSCTRL_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x08 - DEVICE Control B"]
    pub ctrlb: crate::Reg<self::device::ctrlb::CTRLB_SPEC>,
    #[doc = "0x0a - DEVICE Device Address"]
    pub dadd: crate::Reg<self::device::dadd::DADD_SPEC>,
    _reserved5: [u8; 0x01],
    #[doc = "0x0c - DEVICE Status"]
    pub status: crate::Reg<self::device::status::STATUS_SPEC>,
    #[doc = "0x0d - Finite State Machine Status"]
    pub fsmstatus: crate::Reg<self::device::fsmstatus::FSMSTATUS_SPEC>,
    _reserved7: [u8; 0x02],
    #[doc = "0x10 - DEVICE Device Frame Number"]
    pub fnum: crate::Reg<self::device::fnum::FNUM_SPEC>,
    _reserved8: [u8; 0x02],
    #[doc = "0x14 - DEVICE Device Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::device::intenclr::INTENCLR_SPEC>,
    _reserved9: [u8; 0x02],
    #[doc = "0x18 - DEVICE Device Interrupt Enable Set"]
    pub intenset: crate::Reg<self::device::intenset::INTENSET_SPEC>,
    _reserved10: [u8; 0x02],
    #[doc = "0x1c - DEVICE Device Interrupt Flag"]
    pub intflag: crate::Reg<self::device::intflag::INTFLAG_SPEC>,
    _reserved11: [u8; 0x02],
    #[doc = "0x20 - DEVICE End Point Interrupt Summary"]
    pub epintsmry: crate::Reg<self::device::epintsmry::EPINTSMRY_SPEC>,
    _reserved12: [u8; 0x02],
    #[doc = "0x24 - Descriptor Address"]
    pub descadd: crate::Reg<self::device::descadd::DESCADD_SPEC>,
    #[doc = "0x28 - USB PAD Calibration"]
    pub padcal: crate::Reg<self::device::padcal::PADCAL_SPEC>,
    _reserved14: [u8; 0xd6],
    #[doc = "0x100..0x10a - DEVICE_ENDPOINT\\[%s\\]"]
    pub device_endpoint0: self::device::DEVICE_ENDPOINT,
    _reserved15: [u8; 0x16],
    #[doc = "0x120..0x12a - DEVICE_ENDPOINT\\[%s\\]"]
    pub device_endpoint1: self::device::DEVICE_ENDPOINT,
    _reserved16: [u8; 0x16],
    #[doc = "0x140..0x14a - DEVICE_ENDPOINT\\[%s\\]"]
    pub device_endpoint2: self::device::DEVICE_ENDPOINT,
    _reserved17: [u8; 0x16],
    #[doc = "0x160..0x16a - DEVICE_ENDPOINT\\[%s\\]"]
    pub device_endpoint3: self::device::DEVICE_ENDPOINT,
    _reserved18: [u8; 0x16],
    #[doc = "0x180..0x18a - DEVICE_ENDPOINT\\[%s\\]"]
    pub device_endpoint4: self::device::DEVICE_ENDPOINT,
    _reserved19: [u8; 0x16],
    #[doc = "0x1a0..0x1aa - DEVICE_ENDPOINT\\[%s\\]"]
    pub device_endpoint5: self::device::DEVICE_ENDPOINT,
    _reserved20: [u8; 0x16],
    #[doc = "0x1c0..0x1ca - DEVICE_ENDPOINT\\[%s\\]"]
    pub device_endpoint6: self::device::DEVICE_ENDPOINT,
    _reserved21: [u8; 0x16],
    #[doc = "0x1e0..0x1ea - DEVICE_ENDPOINT\\[%s\\]"]
    pub device_endpoint7: self::device::DEVICE_ENDPOINT,
}
#[doc = r"Register block"]
#[doc = "USB is Device"]
pub mod device;
#[doc = r"Register block"]
#[repr(C)]
pub struct HOST {
    #[doc = "0x00 - Control A"]
    pub ctrla: crate::Reg<self::host::ctrla::CTRLA_SPEC>,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - Synchronization Busy"]
    pub syncbusy: crate::Reg<self::host::syncbusy::SYNCBUSY_SPEC>,
    #[doc = "0x03 - USB Quality Of Service"]
    pub qosctrl: crate::Reg<self::host::qosctrl::QOSCTRL_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x08 - HOST Control B"]
    pub ctrlb: crate::Reg<self::host::ctrlb::CTRLB_SPEC>,
    #[doc = "0x0a - HOST Host Start Of Frame Control"]
    pub hsofc: crate::Reg<self::host::hsofc::HSOFC_SPEC>,
    _reserved5: [u8; 0x01],
    #[doc = "0x0c - HOST Status"]
    pub status: crate::Reg<self::host::status::STATUS_SPEC>,
    #[doc = "0x0d - Finite State Machine Status"]
    pub fsmstatus: crate::Reg<self::host::fsmstatus::FSMSTATUS_SPEC>,
    _reserved7: [u8; 0x02],
    #[doc = "0x10 - HOST Host Frame Number"]
    pub fnum: crate::Reg<self::host::fnum::FNUM_SPEC>,
    #[doc = "0x12 - HOST Host Frame Length"]
    pub flenhigh: crate::Reg<self::host::flenhigh::FLENHIGH_SPEC>,
    _reserved9: [u8; 0x01],
    #[doc = "0x14 - HOST Host Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::host::intenclr::INTENCLR_SPEC>,
    _reserved10: [u8; 0x02],
    #[doc = "0x18 - HOST Host Interrupt Enable Set"]
    pub intenset: crate::Reg<self::host::intenset::INTENSET_SPEC>,
    _reserved11: [u8; 0x02],
    #[doc = "0x1c - HOST Host Interrupt Flag"]
    pub intflag: crate::Reg<self::host::intflag::INTFLAG_SPEC>,
    _reserved12: [u8; 0x02],
    #[doc = "0x20 - HOST Pipe Interrupt Summary"]
    pub pintsmry: crate::Reg<self::host::pintsmry::PINTSMRY_SPEC>,
    _reserved13: [u8; 0x02],
    #[doc = "0x24 - Descriptor Address"]
    pub descadd: crate::Reg<self::host::descadd::DESCADD_SPEC>,
    #[doc = "0x28 - USB PAD Calibration"]
    pub padcal: crate::Reg<self::host::padcal::PADCAL_SPEC>,
    _reserved15: [u8; 0xd6],
    #[doc = "0x100..0x10a - HOST_PIPE\\[%s\\]"]
    pub host_pipe0: self::host::HOST_PIPE,
    _reserved16: [u8; 0x16],
    #[doc = "0x120..0x12a - HOST_PIPE\\[%s\\]"]
    pub host_pipe1: self::host::HOST_PIPE,
    _reserved17: [u8; 0x16],
    #[doc = "0x140..0x14a - HOST_PIPE\\[%s\\]"]
    pub host_pipe2: self::host::HOST_PIPE,
    _reserved18: [u8; 0x16],
    #[doc = "0x160..0x16a - HOST_PIPE\\[%s\\]"]
    pub host_pipe3: self::host::HOST_PIPE,
    _reserved19: [u8; 0x16],
    #[doc = "0x180..0x18a - HOST_PIPE\\[%s\\]"]
    pub host_pipe4: self::host::HOST_PIPE,
    _reserved20: [u8; 0x16],
    #[doc = "0x1a0..0x1aa - HOST_PIPE\\[%s\\]"]
    pub host_pipe5: self::host::HOST_PIPE,
    _reserved21: [u8; 0x16],
    #[doc = "0x1c0..0x1ca - HOST_PIPE\\[%s\\]"]
    pub host_pipe6: self::host::HOST_PIPE,
    _reserved22: [u8; 0x16],
    #[doc = "0x1e0..0x1ea - HOST_PIPE\\[%s\\]"]
    pub host_pipe7: self::host::HOST_PIPE,
}
#[doc = r"Register block"]
#[doc = "USB is Host"]
pub mod host;
