#[doc = "CTRLA register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "SYNCBUSY register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "QOSCTRL register accessor: an alias for `Reg<QOSCTRL_SPEC>`"]
pub type QOSCTRL = crate::Reg<qosctrl::QOSCTRL_SPEC>;
#[doc = "USB Quality Of Service"]
pub mod qosctrl;
#[doc = "CTRLB register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "DEVICE Control B"]
pub mod ctrlb;
#[doc = "DADD register accessor: an alias for `Reg<DADD_SPEC>`"]
pub type DADD = crate::Reg<dadd::DADD_SPEC>;
#[doc = "DEVICE Device Address"]
pub mod dadd;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "DEVICE Status"]
pub mod status;
#[doc = "FSMSTATUS register accessor: an alias for `Reg<FSMSTATUS_SPEC>`"]
pub type FSMSTATUS = crate::Reg<fsmstatus::FSMSTATUS_SPEC>;
#[doc = "Finite State Machine Status"]
pub mod fsmstatus;
#[doc = "FNUM register accessor: an alias for `Reg<FNUM_SPEC>`"]
pub type FNUM = crate::Reg<fnum::FNUM_SPEC>;
#[doc = "DEVICE Device Frame Number"]
pub mod fnum;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "DEVICE Device Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "DEVICE Device Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "DEVICE Device Interrupt Flag"]
pub mod intflag;
#[doc = "EPINTSMRY register accessor: an alias for `Reg<EPINTSMRY_SPEC>`"]
pub type EPINTSMRY = crate::Reg<epintsmry::EPINTSMRY_SPEC>;
#[doc = "DEVICE End Point Interrupt Summary"]
pub mod epintsmry;
#[doc = "DESCADD register accessor: an alias for `Reg<DESCADD_SPEC>`"]
pub type DESCADD = crate::Reg<descadd::DESCADD_SPEC>;
#[doc = "Descriptor Address"]
pub mod descadd;
#[doc = "PADCAL register accessor: an alias for `Reg<PADCAL_SPEC>`"]
pub type PADCAL = crate::Reg<padcal::PADCAL_SPEC>;
#[doc = "USB PAD Calibration"]
pub mod padcal;
#[doc = r"Register block"]
#[repr(C)]
pub struct DEVICE_ENDPOINT {
    #[doc = "0x00 - DEVICE_ENDPOINT End Point Configuration"]
    pub epcfg: crate::Reg<self::device_endpoint::epcfg::EPCFG_SPEC>,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - DEVICE_ENDPOINT End Point Pipe Status Clear"]
    pub epstatusclr: crate::Reg<self::device_endpoint::epstatusclr::EPSTATUSCLR_SPEC>,
    #[doc = "0x05 - DEVICE_ENDPOINT End Point Pipe Status Set"]
    pub epstatusset: crate::Reg<self::device_endpoint::epstatusset::EPSTATUSSET_SPEC>,
    #[doc = "0x06 - DEVICE_ENDPOINT End Point Pipe Status"]
    pub epstatus: crate::Reg<self::device_endpoint::epstatus::EPSTATUS_SPEC>,
    #[doc = "0x07 - DEVICE_ENDPOINT End Point Interrupt Flag"]
    pub epintflag: crate::Reg<self::device_endpoint::epintflag::EPINTFLAG_SPEC>,
    #[doc = "0x08 - DEVICE_ENDPOINT End Point Interrupt Clear Flag"]
    pub epintenclr: crate::Reg<self::device_endpoint::epintenclr::EPINTENCLR_SPEC>,
    #[doc = "0x09 - DEVICE_ENDPOINT End Point Interrupt Set Flag"]
    pub epintenset: crate::Reg<self::device_endpoint::epintenset::EPINTENSET_SPEC>,
}
#[doc = r"Register block"]
#[doc = "DEVICE_ENDPOINT\\[%s\\]"]
pub mod device_endpoint;
