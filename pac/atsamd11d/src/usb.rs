#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x111 - USB is Device"]
    pub device: DEVICE,
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
