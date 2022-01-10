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
#[doc = "HOST Control B"]
pub mod ctrlb;
#[doc = "HSOFC register accessor: an alias for `Reg<HSOFC_SPEC>`"]
pub type HSOFC = crate::Reg<hsofc::HSOFC_SPEC>;
#[doc = "HOST Host Start Of Frame Control"]
pub mod hsofc;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "HOST Status"]
pub mod status;
#[doc = "FSMSTATUS register accessor: an alias for `Reg<FSMSTATUS_SPEC>`"]
pub type FSMSTATUS = crate::Reg<fsmstatus::FSMSTATUS_SPEC>;
#[doc = "Finite State Machine Status"]
pub mod fsmstatus;
#[doc = "FNUM register accessor: an alias for `Reg<FNUM_SPEC>`"]
pub type FNUM = crate::Reg<fnum::FNUM_SPEC>;
#[doc = "HOST Host Frame Number"]
pub mod fnum;
#[doc = "FLENHIGH register accessor: an alias for `Reg<FLENHIGH_SPEC>`"]
pub type FLENHIGH = crate::Reg<flenhigh::FLENHIGH_SPEC>;
#[doc = "HOST Host Frame Length"]
pub mod flenhigh;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "HOST Host Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "HOST Host Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "HOST Host Interrupt Flag"]
pub mod intflag;
#[doc = "PINTSMRY register accessor: an alias for `Reg<PINTSMRY_SPEC>`"]
pub type PINTSMRY = crate::Reg<pintsmry::PINTSMRY_SPEC>;
#[doc = "HOST Pipe Interrupt Summary"]
pub mod pintsmry;
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
pub struct HOST_PIPE {
    #[doc = "0x00 - HOST_PIPE End Point Configuration"]
    pub pcfg: crate::Reg<self::host_pipe::pcfg::PCFG_SPEC>,
    _reserved1: [u8; 0x02],
    #[doc = "0x03 - HOST_PIPE Bus Access Period of Pipe"]
    pub binterval: crate::Reg<self::host_pipe::binterval::BINTERVAL_SPEC>,
    #[doc = "0x04 - HOST_PIPE End Point Pipe Status Clear"]
    pub pstatusclr: crate::Reg<self::host_pipe::pstatusclr::PSTATUSCLR_SPEC>,
    #[doc = "0x05 - HOST_PIPE End Point Pipe Status Set"]
    pub pstatusset: crate::Reg<self::host_pipe::pstatusset::PSTATUSSET_SPEC>,
    #[doc = "0x06 - HOST_PIPE End Point Pipe Status"]
    pub pstatus: crate::Reg<self::host_pipe::pstatus::PSTATUS_SPEC>,
    #[doc = "0x07 - HOST_PIPE Pipe Interrupt Flag"]
    pub pintflag: crate::Reg<self::host_pipe::pintflag::PINTFLAG_SPEC>,
    #[doc = "0x08 - HOST_PIPE Pipe Interrupt Flag Clear"]
    pub pintenclr: crate::Reg<self::host_pipe::pintenclr::PINTENCLR_SPEC>,
    #[doc = "0x09 - HOST_PIPE Pipe Interrupt Flag Set"]
    pub pintenset: crate::Reg<self::host_pipe::pintenset::PINTENSET_SPEC>,
}
#[doc = r"Register block"]
#[doc = "HOST_PIPE\\[%s\\]"]
pub mod host_pipe;
