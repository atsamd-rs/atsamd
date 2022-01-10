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
#[doc = "PCFG register accessor: an alias for `Reg<PCFG_SPEC>`"]
pub type PCFG = crate::Reg<pcfg::PCFG_SPEC>;
#[doc = "HOST End Point Configuration"]
pub mod pcfg;
#[doc = "BINTERVAL register accessor: an alias for `Reg<BINTERVAL_SPEC>`"]
pub type BINTERVAL = crate::Reg<binterval::BINTERVAL_SPEC>;
#[doc = "HOST Bus Access Period of Pipe"]
pub mod binterval;
#[doc = "PSTATUSCLR register accessor: an alias for `Reg<PSTATUSCLR_SPEC>`"]
pub type PSTATUSCLR = crate::Reg<pstatusclr::PSTATUSCLR_SPEC>;
#[doc = "HOST End Point Pipe Status Clear"]
pub mod pstatusclr;
#[doc = "PSTATUSSET register accessor: an alias for `Reg<PSTATUSSET_SPEC>`"]
pub type PSTATUSSET = crate::Reg<pstatusset::PSTATUSSET_SPEC>;
#[doc = "HOST End Point Pipe Status Set"]
pub mod pstatusset;
#[doc = "PSTATUS register accessor: an alias for `Reg<PSTATUS_SPEC>`"]
pub type PSTATUS = crate::Reg<pstatus::PSTATUS_SPEC>;
#[doc = "HOST End Point Pipe Status"]
pub mod pstatus;
#[doc = "PINTFLAG register accessor: an alias for `Reg<PINTFLAG_SPEC>`"]
pub type PINTFLAG = crate::Reg<pintflag::PINTFLAG_SPEC>;
#[doc = "HOST Pipe Interrupt Flag"]
pub mod pintflag;
#[doc = "PINTENCLR register accessor: an alias for `Reg<PINTENCLR_SPEC>`"]
pub type PINTENCLR = crate::Reg<pintenclr::PINTENCLR_SPEC>;
#[doc = "HOST Pipe Interrupt Flag Clear"]
pub mod pintenclr;
#[doc = "PINTENSET register accessor: an alias for `Reg<PINTENSET_SPEC>`"]
pub type PINTENSET = crate::Reg<pintenset::PINTENSET_SPEC>;
#[doc = "HOST Pipe Interrupt Flag Set"]
pub mod pintenset;
