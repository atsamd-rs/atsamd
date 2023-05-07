#[doc = r"Register block"]
#[repr(C)]
pub struct DEVICE {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - Synchronization Busy"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x03 - USB Quality Of Service"]
    pub qosctrl: QOSCTRL,
    _reserved3: [u8; 0x04],
    #[doc = "0x08 - DEVICE Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x0a - DEVICE Device Address"]
    pub dadd: DADD,
    _reserved5: [u8; 0x01],
    #[doc = "0x0c - DEVICE Status"]
    pub status: STATUS,
    #[doc = "0x0d - Finite State Machine Status"]
    pub fsmstatus: FSMSTATUS,
    _reserved7: [u8; 0x02],
    #[doc = "0x10 - DEVICE Device Frame Number"]
    pub fnum: FNUM,
    _reserved8: [u8; 0x02],
    #[doc = "0x14 - DEVICE Device Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    _reserved9: [u8; 0x02],
    #[doc = "0x18 - DEVICE Device Interrupt Enable Set"]
    pub intenset: INTENSET,
    _reserved10: [u8; 0x02],
    #[doc = "0x1c - DEVICE Device Interrupt Flag"]
    pub intflag: INTFLAG,
    _reserved11: [u8; 0x02],
    #[doc = "0x20 - DEVICE End Point Interrupt Summary"]
    pub epintsmry: EPINTSMRY,
    _reserved12: [u8; 0x02],
    #[doc = "0x24 - Descriptor Address"]
    pub descadd: DESCADD,
    #[doc = "0x28 - USB PAD Calibration"]
    pub padcal: PADCAL,
    _reserved14: [u8; 0xd6],
    #[doc = "0x100..0x10a - DEVICE_ENDPOINT\\[%s\\]"]
    pub device_endpoint0: DEVICE_ENDPOINT,
    _reserved15: [u8; 0x16],
    #[doc = "0x120..0x12a - DEVICE_ENDPOINT\\[%s\\]"]
    pub device_endpoint1: DEVICE_ENDPOINT,
    _reserved16: [u8; 0x16],
    #[doc = "0x140..0x14a - DEVICE_ENDPOINT\\[%s\\]"]
    pub device_endpoint2: DEVICE_ENDPOINT,
    _reserved17: [u8; 0x16],
    #[doc = "0x160..0x16a - DEVICE_ENDPOINT\\[%s\\]"]
    pub device_endpoint3: DEVICE_ENDPOINT,
    _reserved18: [u8; 0x16],
    #[doc = "0x180..0x18a - DEVICE_ENDPOINT\\[%s\\]"]
    pub device_endpoint4: DEVICE_ENDPOINT,
    _reserved19: [u8; 0x16],
    #[doc = "0x1a0..0x1aa - DEVICE_ENDPOINT\\[%s\\]"]
    pub device_endpoint5: DEVICE_ENDPOINT,
    _reserved20: [u8; 0x16],
    #[doc = "0x1c0..0x1ca - DEVICE_ENDPOINT\\[%s\\]"]
    pub device_endpoint6: DEVICE_ENDPOINT,
    _reserved21: [u8; 0x16],
    #[doc = "0x1e0..0x1ea - DEVICE_ENDPOINT\\[%s\\]"]
    pub device_endpoint7: DEVICE_ENDPOINT,
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "QOSCTRL (rw) register accessor: an alias for `Reg<QOSCTRL_SPEC>`"]
pub type QOSCTRL = crate::Reg<qosctrl::QOSCTRL_SPEC>;
#[doc = "USB Quality Of Service"]
pub mod qosctrl;
#[doc = "CTRLB (rw) register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "DEVICE Control B"]
pub mod ctrlb;
#[doc = "DADD (rw) register accessor: an alias for `Reg<DADD_SPEC>`"]
pub type DADD = crate::Reg<dadd::DADD_SPEC>;
#[doc = "DEVICE Device Address"]
pub mod dadd;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "DEVICE Status"]
pub mod status;
#[doc = "FSMSTATUS (r) register accessor: an alias for `Reg<FSMSTATUS_SPEC>`"]
pub type FSMSTATUS = crate::Reg<fsmstatus::FSMSTATUS_SPEC>;
#[doc = "Finite State Machine Status"]
pub mod fsmstatus;
#[doc = "FNUM (r) register accessor: an alias for `Reg<FNUM_SPEC>`"]
pub type FNUM = crate::Reg<fnum::FNUM_SPEC>;
#[doc = "DEVICE Device Frame Number"]
pub mod fnum;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "DEVICE Device Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "DEVICE Device Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "DEVICE Device Interrupt Flag"]
pub mod intflag;
#[doc = "EPINTSMRY (r) register accessor: an alias for `Reg<EPINTSMRY_SPEC>`"]
pub type EPINTSMRY = crate::Reg<epintsmry::EPINTSMRY_SPEC>;
#[doc = "DEVICE End Point Interrupt Summary"]
pub mod epintsmry;
#[doc = "DESCADD (rw) register accessor: an alias for `Reg<DESCADD_SPEC>`"]
pub type DESCADD = crate::Reg<descadd::DESCADD_SPEC>;
#[doc = "Descriptor Address"]
pub mod descadd;
#[doc = "PADCAL (rw) register accessor: an alias for `Reg<PADCAL_SPEC>`"]
pub type PADCAL = crate::Reg<padcal::PADCAL_SPEC>;
#[doc = "USB PAD Calibration"]
pub mod padcal;
#[doc = "DEVICE_ENDPOINT\\[%s\\]"]
pub use self::device_endpoint::DEVICE_ENDPOINT;
#[doc = r"Cluster"]
#[doc = "DEVICE_ENDPOINT\\[%s\\]"]
pub mod device_endpoint;
