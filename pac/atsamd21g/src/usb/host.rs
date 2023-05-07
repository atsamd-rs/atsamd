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
    #[doc = "0x100 - HOST End Point Configuration"]
    pub pcfg0: PCFG,
    _reserved16: [u8; 0x02],
    #[doc = "0x103 - HOST Bus Access Period of Pipe"]
    pub binterval0: BINTERVAL,
    #[doc = "0x104 - HOST End Point Pipe Status Clear"]
    pub pstatusclr0: PSTATUSCLR,
    #[doc = "0x105 - HOST End Point Pipe Status Set"]
    pub pstatusset0: PSTATUSSET,
    #[doc = "0x106 - HOST End Point Pipe Status"]
    pub pstatus0: PSTATUS,
    #[doc = "0x107 - HOST Pipe Interrupt Flag"]
    pub pintflag0: PINTFLAG,
    #[doc = "0x108 - HOST Pipe Interrupt Flag Clear"]
    pub pintenclr0: PINTENCLR,
    #[doc = "0x109 - HOST Pipe Interrupt Flag Set"]
    pub pintenset0: PINTENSET,
    _reserved23: [u8; 0x16],
    #[doc = "0x120 - HOST End Point Configuration"]
    pub pcfg1: PCFG,
    _reserved24: [u8; 0x02],
    #[doc = "0x123 - HOST Bus Access Period of Pipe"]
    pub binterval1: BINTERVAL,
    #[doc = "0x124 - HOST End Point Pipe Status Clear"]
    pub pstatusclr1: PSTATUSCLR,
    #[doc = "0x125 - HOST End Point Pipe Status Set"]
    pub pstatusset1: PSTATUSSET,
    #[doc = "0x126 - HOST End Point Pipe Status"]
    pub pstatus1: PSTATUS,
    #[doc = "0x127 - HOST Pipe Interrupt Flag"]
    pub pintflag1: PINTFLAG,
    #[doc = "0x128 - HOST Pipe Interrupt Flag Clear"]
    pub pintenclr1: PINTENCLR,
    #[doc = "0x129 - HOST Pipe Interrupt Flag Set"]
    pub pintenset1: PINTENSET,
    _reserved31: [u8; 0x16],
    #[doc = "0x140 - HOST End Point Configuration"]
    pub pcfg2: PCFG,
    _reserved32: [u8; 0x02],
    #[doc = "0x143 - HOST Bus Access Period of Pipe"]
    pub binterval2: BINTERVAL,
    #[doc = "0x144 - HOST End Point Pipe Status Clear"]
    pub pstatusclr2: PSTATUSCLR,
    #[doc = "0x145 - HOST End Point Pipe Status Set"]
    pub pstatusset2: PSTATUSSET,
    #[doc = "0x146 - HOST End Point Pipe Status"]
    pub pstatus2: PSTATUS,
    #[doc = "0x147 - HOST Pipe Interrupt Flag"]
    pub pintflag2: PINTFLAG,
    #[doc = "0x148 - HOST Pipe Interrupt Flag Clear"]
    pub pintenclr2: PINTENCLR,
    #[doc = "0x149 - HOST Pipe Interrupt Flag Set"]
    pub pintenset2: PINTENSET,
    _reserved39: [u8; 0x16],
    #[doc = "0x160 - HOST End Point Configuration"]
    pub pcfg3: PCFG,
    _reserved40: [u8; 0x02],
    #[doc = "0x163 - HOST Bus Access Period of Pipe"]
    pub binterval3: BINTERVAL,
    #[doc = "0x164 - HOST End Point Pipe Status Clear"]
    pub pstatusclr3: PSTATUSCLR,
    #[doc = "0x165 - HOST End Point Pipe Status Set"]
    pub pstatusset3: PSTATUSSET,
    #[doc = "0x166 - HOST End Point Pipe Status"]
    pub pstatus3: PSTATUS,
    #[doc = "0x167 - HOST Pipe Interrupt Flag"]
    pub pintflag3: PINTFLAG,
    #[doc = "0x168 - HOST Pipe Interrupt Flag Clear"]
    pub pintenclr3: PINTENCLR,
    #[doc = "0x169 - HOST Pipe Interrupt Flag Set"]
    pub pintenset3: PINTENSET,
    _reserved47: [u8; 0x16],
    #[doc = "0x180 - HOST End Point Configuration"]
    pub pcfg4: PCFG,
    _reserved48: [u8; 0x02],
    #[doc = "0x183 - HOST Bus Access Period of Pipe"]
    pub binterval4: BINTERVAL,
    #[doc = "0x184 - HOST End Point Pipe Status Clear"]
    pub pstatusclr4: PSTATUSCLR,
    #[doc = "0x185 - HOST End Point Pipe Status Set"]
    pub pstatusset4: PSTATUSSET,
    #[doc = "0x186 - HOST End Point Pipe Status"]
    pub pstatus4: PSTATUS,
    #[doc = "0x187 - HOST Pipe Interrupt Flag"]
    pub pintflag4: PINTFLAG,
    #[doc = "0x188 - HOST Pipe Interrupt Flag Clear"]
    pub pintenclr4: PINTENCLR,
    #[doc = "0x189 - HOST Pipe Interrupt Flag Set"]
    pub pintenset4: PINTENSET,
    _reserved55: [u8; 0x16],
    #[doc = "0x1a0 - HOST End Point Configuration"]
    pub pcfg5: PCFG,
    _reserved56: [u8; 0x02],
    #[doc = "0x1a3 - HOST Bus Access Period of Pipe"]
    pub binterval5: BINTERVAL,
    #[doc = "0x1a4 - HOST End Point Pipe Status Clear"]
    pub pstatusclr5: PSTATUSCLR,
    #[doc = "0x1a5 - HOST End Point Pipe Status Set"]
    pub pstatusset5: PSTATUSSET,
    #[doc = "0x1a6 - HOST End Point Pipe Status"]
    pub pstatus5: PSTATUS,
    #[doc = "0x1a7 - HOST Pipe Interrupt Flag"]
    pub pintflag5: PINTFLAG,
    #[doc = "0x1a8 - HOST Pipe Interrupt Flag Clear"]
    pub pintenclr5: PINTENCLR,
    #[doc = "0x1a9 - HOST Pipe Interrupt Flag Set"]
    pub pintenset5: PINTENSET,
    _reserved63: [u8; 0x16],
    #[doc = "0x1c0 - HOST End Point Configuration"]
    pub pcfg6: PCFG,
    _reserved64: [u8; 0x02],
    #[doc = "0x1c3 - HOST Bus Access Period of Pipe"]
    pub binterval6: BINTERVAL,
    #[doc = "0x1c4 - HOST End Point Pipe Status Clear"]
    pub pstatusclr6: PSTATUSCLR,
    #[doc = "0x1c5 - HOST End Point Pipe Status Set"]
    pub pstatusset6: PSTATUSSET,
    #[doc = "0x1c6 - HOST End Point Pipe Status"]
    pub pstatus6: PSTATUS,
    #[doc = "0x1c7 - HOST Pipe Interrupt Flag"]
    pub pintflag6: PINTFLAG,
    #[doc = "0x1c8 - HOST Pipe Interrupt Flag Clear"]
    pub pintenclr6: PINTENCLR,
    #[doc = "0x1c9 - HOST Pipe Interrupt Flag Set"]
    pub pintenset6: PINTENSET,
    _reserved71: [u8; 0x16],
    #[doc = "0x1e0 - HOST End Point Configuration"]
    pub pcfg7: PCFG,
    _reserved72: [u8; 0x02],
    #[doc = "0x1e3 - HOST Bus Access Period of Pipe"]
    pub binterval7: BINTERVAL,
    #[doc = "0x1e4 - HOST End Point Pipe Status Clear"]
    pub pstatusclr7: PSTATUSCLR,
    #[doc = "0x1e5 - HOST End Point Pipe Status Set"]
    pub pstatusset7: PSTATUSSET,
    #[doc = "0x1e6 - HOST End Point Pipe Status"]
    pub pstatus7: PSTATUS,
    #[doc = "0x1e7 - HOST Pipe Interrupt Flag"]
    pub pintflag7: PINTFLAG,
    #[doc = "0x1e8 - HOST Pipe Interrupt Flag Clear"]
    pub pintenclr7: PINTENCLR,
    #[doc = "0x1e9 - HOST Pipe Interrupt Flag Set"]
    pub pintenset7: PINTENSET,
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
#[doc = "PCFG (rw) register accessor: an alias for `Reg<PCFG_SPEC>`"]
pub type PCFG = crate::Reg<pcfg::PCFG_SPEC>;
#[doc = "HOST End Point Configuration"]
pub mod pcfg;
#[doc = "BINTERVAL (rw) register accessor: an alias for `Reg<BINTERVAL_SPEC>`"]
pub type BINTERVAL = crate::Reg<binterval::BINTERVAL_SPEC>;
#[doc = "HOST Bus Access Period of Pipe"]
pub mod binterval;
#[doc = "PSTATUSCLR (w) register accessor: an alias for `Reg<PSTATUSCLR_SPEC>`"]
pub type PSTATUSCLR = crate::Reg<pstatusclr::PSTATUSCLR_SPEC>;
#[doc = "HOST End Point Pipe Status Clear"]
pub mod pstatusclr;
#[doc = "PSTATUSSET (w) register accessor: an alias for `Reg<PSTATUSSET_SPEC>`"]
pub type PSTATUSSET = crate::Reg<pstatusset::PSTATUSSET_SPEC>;
#[doc = "HOST End Point Pipe Status Set"]
pub mod pstatusset;
#[doc = "PSTATUS (r) register accessor: an alias for `Reg<PSTATUS_SPEC>`"]
pub type PSTATUS = crate::Reg<pstatus::PSTATUS_SPEC>;
#[doc = "HOST End Point Pipe Status"]
pub mod pstatus;
#[doc = "PINTFLAG (rw) register accessor: an alias for `Reg<PINTFLAG_SPEC>`"]
pub type PINTFLAG = crate::Reg<pintflag::PINTFLAG_SPEC>;
#[doc = "HOST Pipe Interrupt Flag"]
pub mod pintflag;
#[doc = "PINTENCLR (rw) register accessor: an alias for `Reg<PINTENCLR_SPEC>`"]
pub type PINTENCLR = crate::Reg<pintenclr::PINTENCLR_SPEC>;
#[doc = "HOST Pipe Interrupt Flag Clear"]
pub mod pintenclr;
#[doc = "PINTENSET (rw) register accessor: an alias for `Reg<PINTENSET_SPEC>`"]
pub type PINTENSET = crate::Reg<pintenset::PINTENSET_SPEC>;
#[doc = "HOST Pipe Interrupt Flag Set"]
pub mod pintenset;
