#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_host: [u8; 0x0111],
}
impl RegisterBlock {
    #[doc = "0x00..0x111 - USB is Host"]
    #[inline(always)]
    pub fn host(&self) -> &HOST {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const HOST) }
    }
    #[doc = "0x00..0x111 - USB is Device"]
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
    #[doc = "0x100 - DEVICE End Point Configuration"]
    pub epcfg0: crate::Reg<self::device::epcfg::EPCFG_SPEC>,
    _reserved15: [u8; 0x03],
    #[doc = "0x104 - DEVICE End Point Pipe Status Clear"]
    pub epstatusclr0: crate::Reg<self::device::epstatusclr::EPSTATUSCLR_SPEC>,
    #[doc = "0x105 - DEVICE End Point Pipe Status Set"]
    pub epstatusset0: crate::Reg<self::device::epstatusset::EPSTATUSSET_SPEC>,
    #[doc = "0x106 - DEVICE End Point Pipe Status"]
    pub epstatus0: crate::Reg<self::device::epstatus::EPSTATUS_SPEC>,
    #[doc = "0x107 - DEVICE End Point Interrupt Flag"]
    pub epintflag0: crate::Reg<self::device::epintflag::EPINTFLAG_SPEC>,
    #[doc = "0x108 - DEVICE End Point Interrupt Clear Flag"]
    pub epintenclr0: crate::Reg<self::device::epintenclr::EPINTENCLR_SPEC>,
    #[doc = "0x109 - DEVICE End Point Interrupt Set Flag"]
    pub epintenset0: crate::Reg<self::device::epintenset::EPINTENSET_SPEC>,
    _reserved21: [u8; 0x16],
    #[doc = "0x120 - DEVICE End Point Configuration"]
    pub epcfg1: crate::Reg<self::device::epcfg::EPCFG_SPEC>,
    _reserved22: [u8; 0x03],
    #[doc = "0x124 - DEVICE End Point Pipe Status Clear"]
    pub epstatusclr1: crate::Reg<self::device::epstatusclr::EPSTATUSCLR_SPEC>,
    #[doc = "0x125 - DEVICE End Point Pipe Status Set"]
    pub epstatusset1: crate::Reg<self::device::epstatusset::EPSTATUSSET_SPEC>,
    #[doc = "0x126 - DEVICE End Point Pipe Status"]
    pub epstatus1: crate::Reg<self::device::epstatus::EPSTATUS_SPEC>,
    #[doc = "0x127 - DEVICE End Point Interrupt Flag"]
    pub epintflag1: crate::Reg<self::device::epintflag::EPINTFLAG_SPEC>,
    #[doc = "0x128 - DEVICE End Point Interrupt Clear Flag"]
    pub epintenclr1: crate::Reg<self::device::epintenclr::EPINTENCLR_SPEC>,
    #[doc = "0x129 - DEVICE End Point Interrupt Set Flag"]
    pub epintenset1: crate::Reg<self::device::epintenset::EPINTENSET_SPEC>,
    _reserved28: [u8; 0x16],
    #[doc = "0x140 - DEVICE End Point Configuration"]
    pub epcfg2: crate::Reg<self::device::epcfg::EPCFG_SPEC>,
    _reserved29: [u8; 0x03],
    #[doc = "0x144 - DEVICE End Point Pipe Status Clear"]
    pub epstatusclr2: crate::Reg<self::device::epstatusclr::EPSTATUSCLR_SPEC>,
    #[doc = "0x145 - DEVICE End Point Pipe Status Set"]
    pub epstatusset2: crate::Reg<self::device::epstatusset::EPSTATUSSET_SPEC>,
    #[doc = "0x146 - DEVICE End Point Pipe Status"]
    pub epstatus2: crate::Reg<self::device::epstatus::EPSTATUS_SPEC>,
    #[doc = "0x147 - DEVICE End Point Interrupt Flag"]
    pub epintflag2: crate::Reg<self::device::epintflag::EPINTFLAG_SPEC>,
    #[doc = "0x148 - DEVICE End Point Interrupt Clear Flag"]
    pub epintenclr2: crate::Reg<self::device::epintenclr::EPINTENCLR_SPEC>,
    #[doc = "0x149 - DEVICE End Point Interrupt Set Flag"]
    pub epintenset2: crate::Reg<self::device::epintenset::EPINTENSET_SPEC>,
    _reserved35: [u8; 0x16],
    #[doc = "0x160 - DEVICE End Point Configuration"]
    pub epcfg3: crate::Reg<self::device::epcfg::EPCFG_SPEC>,
    _reserved36: [u8; 0x03],
    #[doc = "0x164 - DEVICE End Point Pipe Status Clear"]
    pub epstatusclr3: crate::Reg<self::device::epstatusclr::EPSTATUSCLR_SPEC>,
    #[doc = "0x165 - DEVICE End Point Pipe Status Set"]
    pub epstatusset3: crate::Reg<self::device::epstatusset::EPSTATUSSET_SPEC>,
    #[doc = "0x166 - DEVICE End Point Pipe Status"]
    pub epstatus3: crate::Reg<self::device::epstatus::EPSTATUS_SPEC>,
    #[doc = "0x167 - DEVICE End Point Interrupt Flag"]
    pub epintflag3: crate::Reg<self::device::epintflag::EPINTFLAG_SPEC>,
    #[doc = "0x168 - DEVICE End Point Interrupt Clear Flag"]
    pub epintenclr3: crate::Reg<self::device::epintenclr::EPINTENCLR_SPEC>,
    #[doc = "0x169 - DEVICE End Point Interrupt Set Flag"]
    pub epintenset3: crate::Reg<self::device::epintenset::EPINTENSET_SPEC>,
    _reserved42: [u8; 0x16],
    #[doc = "0x180 - DEVICE End Point Configuration"]
    pub epcfg4: crate::Reg<self::device::epcfg::EPCFG_SPEC>,
    _reserved43: [u8; 0x03],
    #[doc = "0x184 - DEVICE End Point Pipe Status Clear"]
    pub epstatusclr4: crate::Reg<self::device::epstatusclr::EPSTATUSCLR_SPEC>,
    #[doc = "0x185 - DEVICE End Point Pipe Status Set"]
    pub epstatusset4: crate::Reg<self::device::epstatusset::EPSTATUSSET_SPEC>,
    #[doc = "0x186 - DEVICE End Point Pipe Status"]
    pub epstatus4: crate::Reg<self::device::epstatus::EPSTATUS_SPEC>,
    #[doc = "0x187 - DEVICE End Point Interrupt Flag"]
    pub epintflag4: crate::Reg<self::device::epintflag::EPINTFLAG_SPEC>,
    #[doc = "0x188 - DEVICE End Point Interrupt Clear Flag"]
    pub epintenclr4: crate::Reg<self::device::epintenclr::EPINTENCLR_SPEC>,
    #[doc = "0x189 - DEVICE End Point Interrupt Set Flag"]
    pub epintenset4: crate::Reg<self::device::epintenset::EPINTENSET_SPEC>,
    _reserved49: [u8; 0x16],
    #[doc = "0x1a0 - DEVICE End Point Configuration"]
    pub epcfg5: crate::Reg<self::device::epcfg::EPCFG_SPEC>,
    _reserved50: [u8; 0x03],
    #[doc = "0x1a4 - DEVICE End Point Pipe Status Clear"]
    pub epstatusclr5: crate::Reg<self::device::epstatusclr::EPSTATUSCLR_SPEC>,
    #[doc = "0x1a5 - DEVICE End Point Pipe Status Set"]
    pub epstatusset5: crate::Reg<self::device::epstatusset::EPSTATUSSET_SPEC>,
    #[doc = "0x1a6 - DEVICE End Point Pipe Status"]
    pub epstatus5: crate::Reg<self::device::epstatus::EPSTATUS_SPEC>,
    #[doc = "0x1a7 - DEVICE End Point Interrupt Flag"]
    pub epintflag5: crate::Reg<self::device::epintflag::EPINTFLAG_SPEC>,
    #[doc = "0x1a8 - DEVICE End Point Interrupt Clear Flag"]
    pub epintenclr5: crate::Reg<self::device::epintenclr::EPINTENCLR_SPEC>,
    #[doc = "0x1a9 - DEVICE End Point Interrupt Set Flag"]
    pub epintenset5: crate::Reg<self::device::epintenset::EPINTENSET_SPEC>,
    _reserved56: [u8; 0x16],
    #[doc = "0x1c0 - DEVICE End Point Configuration"]
    pub epcfg6: crate::Reg<self::device::epcfg::EPCFG_SPEC>,
    _reserved57: [u8; 0x03],
    #[doc = "0x1c4 - DEVICE End Point Pipe Status Clear"]
    pub epstatusclr6: crate::Reg<self::device::epstatusclr::EPSTATUSCLR_SPEC>,
    #[doc = "0x1c5 - DEVICE End Point Pipe Status Set"]
    pub epstatusset6: crate::Reg<self::device::epstatusset::EPSTATUSSET_SPEC>,
    #[doc = "0x1c6 - DEVICE End Point Pipe Status"]
    pub epstatus6: crate::Reg<self::device::epstatus::EPSTATUS_SPEC>,
    #[doc = "0x1c7 - DEVICE End Point Interrupt Flag"]
    pub epintflag6: crate::Reg<self::device::epintflag::EPINTFLAG_SPEC>,
    #[doc = "0x1c8 - DEVICE End Point Interrupt Clear Flag"]
    pub epintenclr6: crate::Reg<self::device::epintenclr::EPINTENCLR_SPEC>,
    #[doc = "0x1c9 - DEVICE End Point Interrupt Set Flag"]
    pub epintenset6: crate::Reg<self::device::epintenset::EPINTENSET_SPEC>,
    _reserved63: [u8; 0x16],
    #[doc = "0x1e0 - DEVICE End Point Configuration"]
    pub epcfg7: crate::Reg<self::device::epcfg::EPCFG_SPEC>,
    _reserved64: [u8; 0x03],
    #[doc = "0x1e4 - DEVICE End Point Pipe Status Clear"]
    pub epstatusclr7: crate::Reg<self::device::epstatusclr::EPSTATUSCLR_SPEC>,
    #[doc = "0x1e5 - DEVICE End Point Pipe Status Set"]
    pub epstatusset7: crate::Reg<self::device::epstatusset::EPSTATUSSET_SPEC>,
    #[doc = "0x1e6 - DEVICE End Point Pipe Status"]
    pub epstatus7: crate::Reg<self::device::epstatus::EPSTATUS_SPEC>,
    #[doc = "0x1e7 - DEVICE End Point Interrupt Flag"]
    pub epintflag7: crate::Reg<self::device::epintflag::EPINTFLAG_SPEC>,
    #[doc = "0x1e8 - DEVICE End Point Interrupt Clear Flag"]
    pub epintenclr7: crate::Reg<self::device::epintenclr::EPINTENCLR_SPEC>,
    #[doc = "0x1e9 - DEVICE End Point Interrupt Set Flag"]
    pub epintenset7: crate::Reg<self::device::epintenset::EPINTENSET_SPEC>,
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
    #[doc = "0x100 - HOST End Point Configuration"]
    pub pcfg0: crate::Reg<self::host::pcfg::PCFG_SPEC>,
    _reserved16: [u8; 0x02],
    #[doc = "0x103 - HOST Bus Access Period of Pipe"]
    pub binterval0: crate::Reg<self::host::binterval::BINTERVAL_SPEC>,
    #[doc = "0x104 - HOST End Point Pipe Status Clear"]
    pub pstatusclr0: crate::Reg<self::host::pstatusclr::PSTATUSCLR_SPEC>,
    #[doc = "0x105 - HOST End Point Pipe Status Set"]
    pub pstatusset0: crate::Reg<self::host::pstatusset::PSTATUSSET_SPEC>,
    #[doc = "0x106 - HOST End Point Pipe Status"]
    pub pstatus0: crate::Reg<self::host::pstatus::PSTATUS_SPEC>,
    #[doc = "0x107 - HOST Pipe Interrupt Flag"]
    pub pintflag0: crate::Reg<self::host::pintflag::PINTFLAG_SPEC>,
    #[doc = "0x108 - HOST Pipe Interrupt Flag Clear"]
    pub pintenclr0: crate::Reg<self::host::pintenclr::PINTENCLR_SPEC>,
    #[doc = "0x109 - HOST Pipe Interrupt Flag Set"]
    pub pintenset0: crate::Reg<self::host::pintenset::PINTENSET_SPEC>,
    _reserved23: [u8; 0x16],
    #[doc = "0x120 - HOST End Point Configuration"]
    pub pcfg1: crate::Reg<self::host::pcfg::PCFG_SPEC>,
    _reserved24: [u8; 0x02],
    #[doc = "0x123 - HOST Bus Access Period of Pipe"]
    pub binterval1: crate::Reg<self::host::binterval::BINTERVAL_SPEC>,
    #[doc = "0x124 - HOST End Point Pipe Status Clear"]
    pub pstatusclr1: crate::Reg<self::host::pstatusclr::PSTATUSCLR_SPEC>,
    #[doc = "0x125 - HOST End Point Pipe Status Set"]
    pub pstatusset1: crate::Reg<self::host::pstatusset::PSTATUSSET_SPEC>,
    #[doc = "0x126 - HOST End Point Pipe Status"]
    pub pstatus1: crate::Reg<self::host::pstatus::PSTATUS_SPEC>,
    #[doc = "0x127 - HOST Pipe Interrupt Flag"]
    pub pintflag1: crate::Reg<self::host::pintflag::PINTFLAG_SPEC>,
    #[doc = "0x128 - HOST Pipe Interrupt Flag Clear"]
    pub pintenclr1: crate::Reg<self::host::pintenclr::PINTENCLR_SPEC>,
    #[doc = "0x129 - HOST Pipe Interrupt Flag Set"]
    pub pintenset1: crate::Reg<self::host::pintenset::PINTENSET_SPEC>,
    _reserved31: [u8; 0x16],
    #[doc = "0x140 - HOST End Point Configuration"]
    pub pcfg2: crate::Reg<self::host::pcfg::PCFG_SPEC>,
    _reserved32: [u8; 0x02],
    #[doc = "0x143 - HOST Bus Access Period of Pipe"]
    pub binterval2: crate::Reg<self::host::binterval::BINTERVAL_SPEC>,
    #[doc = "0x144 - HOST End Point Pipe Status Clear"]
    pub pstatusclr2: crate::Reg<self::host::pstatusclr::PSTATUSCLR_SPEC>,
    #[doc = "0x145 - HOST End Point Pipe Status Set"]
    pub pstatusset2: crate::Reg<self::host::pstatusset::PSTATUSSET_SPEC>,
    #[doc = "0x146 - HOST End Point Pipe Status"]
    pub pstatus2: crate::Reg<self::host::pstatus::PSTATUS_SPEC>,
    #[doc = "0x147 - HOST Pipe Interrupt Flag"]
    pub pintflag2: crate::Reg<self::host::pintflag::PINTFLAG_SPEC>,
    #[doc = "0x148 - HOST Pipe Interrupt Flag Clear"]
    pub pintenclr2: crate::Reg<self::host::pintenclr::PINTENCLR_SPEC>,
    #[doc = "0x149 - HOST Pipe Interrupt Flag Set"]
    pub pintenset2: crate::Reg<self::host::pintenset::PINTENSET_SPEC>,
    _reserved39: [u8; 0x16],
    #[doc = "0x160 - HOST End Point Configuration"]
    pub pcfg3: crate::Reg<self::host::pcfg::PCFG_SPEC>,
    _reserved40: [u8; 0x02],
    #[doc = "0x163 - HOST Bus Access Period of Pipe"]
    pub binterval3: crate::Reg<self::host::binterval::BINTERVAL_SPEC>,
    #[doc = "0x164 - HOST End Point Pipe Status Clear"]
    pub pstatusclr3: crate::Reg<self::host::pstatusclr::PSTATUSCLR_SPEC>,
    #[doc = "0x165 - HOST End Point Pipe Status Set"]
    pub pstatusset3: crate::Reg<self::host::pstatusset::PSTATUSSET_SPEC>,
    #[doc = "0x166 - HOST End Point Pipe Status"]
    pub pstatus3: crate::Reg<self::host::pstatus::PSTATUS_SPEC>,
    #[doc = "0x167 - HOST Pipe Interrupt Flag"]
    pub pintflag3: crate::Reg<self::host::pintflag::PINTFLAG_SPEC>,
    #[doc = "0x168 - HOST Pipe Interrupt Flag Clear"]
    pub pintenclr3: crate::Reg<self::host::pintenclr::PINTENCLR_SPEC>,
    #[doc = "0x169 - HOST Pipe Interrupt Flag Set"]
    pub pintenset3: crate::Reg<self::host::pintenset::PINTENSET_SPEC>,
    _reserved47: [u8; 0x16],
    #[doc = "0x180 - HOST End Point Configuration"]
    pub pcfg4: crate::Reg<self::host::pcfg::PCFG_SPEC>,
    _reserved48: [u8; 0x02],
    #[doc = "0x183 - HOST Bus Access Period of Pipe"]
    pub binterval4: crate::Reg<self::host::binterval::BINTERVAL_SPEC>,
    #[doc = "0x184 - HOST End Point Pipe Status Clear"]
    pub pstatusclr4: crate::Reg<self::host::pstatusclr::PSTATUSCLR_SPEC>,
    #[doc = "0x185 - HOST End Point Pipe Status Set"]
    pub pstatusset4: crate::Reg<self::host::pstatusset::PSTATUSSET_SPEC>,
    #[doc = "0x186 - HOST End Point Pipe Status"]
    pub pstatus4: crate::Reg<self::host::pstatus::PSTATUS_SPEC>,
    #[doc = "0x187 - HOST Pipe Interrupt Flag"]
    pub pintflag4: crate::Reg<self::host::pintflag::PINTFLAG_SPEC>,
    #[doc = "0x188 - HOST Pipe Interrupt Flag Clear"]
    pub pintenclr4: crate::Reg<self::host::pintenclr::PINTENCLR_SPEC>,
    #[doc = "0x189 - HOST Pipe Interrupt Flag Set"]
    pub pintenset4: crate::Reg<self::host::pintenset::PINTENSET_SPEC>,
    _reserved55: [u8; 0x16],
    #[doc = "0x1a0 - HOST End Point Configuration"]
    pub pcfg5: crate::Reg<self::host::pcfg::PCFG_SPEC>,
    _reserved56: [u8; 0x02],
    #[doc = "0x1a3 - HOST Bus Access Period of Pipe"]
    pub binterval5: crate::Reg<self::host::binterval::BINTERVAL_SPEC>,
    #[doc = "0x1a4 - HOST End Point Pipe Status Clear"]
    pub pstatusclr5: crate::Reg<self::host::pstatusclr::PSTATUSCLR_SPEC>,
    #[doc = "0x1a5 - HOST End Point Pipe Status Set"]
    pub pstatusset5: crate::Reg<self::host::pstatusset::PSTATUSSET_SPEC>,
    #[doc = "0x1a6 - HOST End Point Pipe Status"]
    pub pstatus5: crate::Reg<self::host::pstatus::PSTATUS_SPEC>,
    #[doc = "0x1a7 - HOST Pipe Interrupt Flag"]
    pub pintflag5: crate::Reg<self::host::pintflag::PINTFLAG_SPEC>,
    #[doc = "0x1a8 - HOST Pipe Interrupt Flag Clear"]
    pub pintenclr5: crate::Reg<self::host::pintenclr::PINTENCLR_SPEC>,
    #[doc = "0x1a9 - HOST Pipe Interrupt Flag Set"]
    pub pintenset5: crate::Reg<self::host::pintenset::PINTENSET_SPEC>,
    _reserved63: [u8; 0x16],
    #[doc = "0x1c0 - HOST End Point Configuration"]
    pub pcfg6: crate::Reg<self::host::pcfg::PCFG_SPEC>,
    _reserved64: [u8; 0x02],
    #[doc = "0x1c3 - HOST Bus Access Period of Pipe"]
    pub binterval6: crate::Reg<self::host::binterval::BINTERVAL_SPEC>,
    #[doc = "0x1c4 - HOST End Point Pipe Status Clear"]
    pub pstatusclr6: crate::Reg<self::host::pstatusclr::PSTATUSCLR_SPEC>,
    #[doc = "0x1c5 - HOST End Point Pipe Status Set"]
    pub pstatusset6: crate::Reg<self::host::pstatusset::PSTATUSSET_SPEC>,
    #[doc = "0x1c6 - HOST End Point Pipe Status"]
    pub pstatus6: crate::Reg<self::host::pstatus::PSTATUS_SPEC>,
    #[doc = "0x1c7 - HOST Pipe Interrupt Flag"]
    pub pintflag6: crate::Reg<self::host::pintflag::PINTFLAG_SPEC>,
    #[doc = "0x1c8 - HOST Pipe Interrupt Flag Clear"]
    pub pintenclr6: crate::Reg<self::host::pintenclr::PINTENCLR_SPEC>,
    #[doc = "0x1c9 - HOST Pipe Interrupt Flag Set"]
    pub pintenset6: crate::Reg<self::host::pintenset::PINTENSET_SPEC>,
    _reserved71: [u8; 0x16],
    #[doc = "0x1e0 - HOST End Point Configuration"]
    pub pcfg7: crate::Reg<self::host::pcfg::PCFG_SPEC>,
    _reserved72: [u8; 0x02],
    #[doc = "0x1e3 - HOST Bus Access Period of Pipe"]
    pub binterval7: crate::Reg<self::host::binterval::BINTERVAL_SPEC>,
    #[doc = "0x1e4 - HOST End Point Pipe Status Clear"]
    pub pstatusclr7: crate::Reg<self::host::pstatusclr::PSTATUSCLR_SPEC>,
    #[doc = "0x1e5 - HOST End Point Pipe Status Set"]
    pub pstatusset7: crate::Reg<self::host::pstatusset::PSTATUSSET_SPEC>,
    #[doc = "0x1e6 - HOST End Point Pipe Status"]
    pub pstatus7: crate::Reg<self::host::pstatus::PSTATUS_SPEC>,
    #[doc = "0x1e7 - HOST Pipe Interrupt Flag"]
    pub pintflag7: crate::Reg<self::host::pintflag::PINTFLAG_SPEC>,
    #[doc = "0x1e8 - HOST Pipe Interrupt Flag Clear"]
    pub pintenclr7: crate::Reg<self::host::pintenclr::PINTENCLR_SPEC>,
    #[doc = "0x1e9 - HOST Pipe Interrupt Flag Set"]
    pub pintenset7: crate::Reg<self::host::pintenset::PINTENSET_SPEC>,
}
#[doc = r"Register block"]
#[doc = "USB is Host"]
pub mod host;
