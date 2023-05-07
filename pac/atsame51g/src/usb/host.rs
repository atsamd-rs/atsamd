#[doc = r"Register block"]
#[repr(C)]
pub struct HOST {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - Synchronization Busy"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x03 - USB Quality Of Service"]
    pub qosctrl: QOSCTRL,
    _reserved3: [u8; 0x04],
    #[doc = "0x08 - HOST Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x0a - HOST Host Start Of Frame Control"]
    pub hsofc: HSOFC,
    _reserved5: [u8; 0x01],
    #[doc = "0x0c - HOST Status"]
    pub status: STATUS,
    #[doc = "0x0d - Finite State Machine Status"]
    pub fsmstatus: FSMSTATUS,
    _reserved7: [u8; 0x02],
    #[doc = "0x10 - HOST Host Frame Number"]
    pub fnum: FNUM,
    #[doc = "0x12 - HOST Host Frame Length"]
    pub flenhigh: FLENHIGH,
    _reserved9: [u8; 0x01],
    #[doc = "0x14 - HOST Host Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    _reserved10: [u8; 0x02],
    #[doc = "0x18 - HOST Host Interrupt Enable Set"]
    pub intenset: INTENSET,
    _reserved11: [u8; 0x02],
    #[doc = "0x1c - HOST Host Interrupt Flag"]
    pub intflag: INTFLAG,
    _reserved12: [u8; 0x02],
    #[doc = "0x20 - HOST Pipe Interrupt Summary"]
    pub pintsmry: PINTSMRY,
    _reserved13: [u8; 0x02],
    #[doc = "0x24 - Descriptor Address"]
    pub descadd: DESCADD,
    #[doc = "0x28 - USB PAD Calibration"]
    pub padcal: PADCAL,
    _reserved15: [u8; 0xd6],
    #[doc = "0x100..0x10a - HOST_PIPE\\[%s\\]"]
    pub host_pipe0: HOST_PIPE,
    _reserved16: [u8; 0x16],
    #[doc = "0x120..0x12a - HOST_PIPE\\[%s\\]"]
    pub host_pipe1: HOST_PIPE,
    _reserved17: [u8; 0x16],
    #[doc = "0x140..0x14a - HOST_PIPE\\[%s\\]"]
    pub host_pipe2: HOST_PIPE,
    _reserved18: [u8; 0x16],
    #[doc = "0x160..0x16a - HOST_PIPE\\[%s\\]"]
    pub host_pipe3: HOST_PIPE,
    _reserved19: [u8; 0x16],
    #[doc = "0x180..0x18a - HOST_PIPE\\[%s\\]"]
    pub host_pipe4: HOST_PIPE,
    _reserved20: [u8; 0x16],
    #[doc = "0x1a0..0x1aa - HOST_PIPE\\[%s\\]"]
    pub host_pipe5: HOST_PIPE,
    _reserved21: [u8; 0x16],
    #[doc = "0x1c0..0x1ca - HOST_PIPE\\[%s\\]"]
    pub host_pipe6: HOST_PIPE,
    _reserved22: [u8; 0x16],
    #[doc = "0x1e0..0x1ea - HOST_PIPE\\[%s\\]"]
    pub host_pipe7: HOST_PIPE,
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
#[doc = "HOST Control B"]
pub mod ctrlb;
#[doc = "HSOFC (rw) register accessor: an alias for `Reg<HSOFC_SPEC>`"]
pub type HSOFC = crate::Reg<hsofc::HSOFC_SPEC>;
#[doc = "HOST Host Start Of Frame Control"]
pub mod hsofc;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "HOST Status"]
pub mod status;
#[doc = "FSMSTATUS (r) register accessor: an alias for `Reg<FSMSTATUS_SPEC>`"]
pub type FSMSTATUS = crate::Reg<fsmstatus::FSMSTATUS_SPEC>;
#[doc = "Finite State Machine Status"]
pub mod fsmstatus;
#[doc = "FNUM (rw) register accessor: an alias for `Reg<FNUM_SPEC>`"]
pub type FNUM = crate::Reg<fnum::FNUM_SPEC>;
#[doc = "HOST Host Frame Number"]
pub mod fnum;
#[doc = "FLENHIGH (r) register accessor: an alias for `Reg<FLENHIGH_SPEC>`"]
pub type FLENHIGH = crate::Reg<flenhigh::FLENHIGH_SPEC>;
#[doc = "HOST Host Frame Length"]
pub mod flenhigh;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "HOST Host Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "HOST Host Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "HOST Host Interrupt Flag"]
pub mod intflag;
#[doc = "PINTSMRY (r) register accessor: an alias for `Reg<PINTSMRY_SPEC>`"]
pub type PINTSMRY = crate::Reg<pintsmry::PINTSMRY_SPEC>;
#[doc = "HOST Pipe Interrupt Summary"]
pub mod pintsmry;
#[doc = "DESCADD (rw) register accessor: an alias for `Reg<DESCADD_SPEC>`"]
pub type DESCADD = crate::Reg<descadd::DESCADD_SPEC>;
#[doc = "Descriptor Address"]
pub mod descadd;
#[doc = "PADCAL (rw) register accessor: an alias for `Reg<PADCAL_SPEC>`"]
pub type PADCAL = crate::Reg<padcal::PADCAL_SPEC>;
#[doc = "USB PAD Calibration"]
pub mod padcal;
#[doc = "HOST_PIPE\\[%s\\]"]
pub use self::host_pipe::HOST_PIPE;
#[doc = r"Cluster"]
#[doc = "HOST_PIPE\\[%s\\]"]
pub mod host_pipe;
