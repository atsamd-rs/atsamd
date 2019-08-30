#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_host: [u8; 273usize],
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
    #[doc = "0x100 - DEVICE End Point Configuration"]
    pub epcfg0: self::device::EPCFG,
    _reserved15: [u8; 3usize],
    #[doc = "0x104 - DEVICE End Point Pipe Status Clear"]
    pub epstatusclr0: self::device::EPSTATUSCLR,
    #[doc = "0x105 - DEVICE End Point Pipe Status Set"]
    pub epstatusset0: self::device::EPSTATUSSET,
    #[doc = "0x106 - DEVICE End Point Pipe Status"]
    pub epstatus0: self::device::EPSTATUS,
    #[doc = "0x107 - DEVICE End Point Interrupt Flag"]
    pub epintflag0: self::device::EPINTFLAG,
    #[doc = "0x108 - DEVICE End Point Interrupt Clear Flag"]
    pub epintenclr0: self::device::EPINTENCLR,
    #[doc = "0x109 - DEVICE End Point Interrupt Set Flag"]
    pub epintenset0: self::device::EPINTENSET,
    _reserved21: [u8; 22usize],
    #[doc = "0x120 - DEVICE End Point Configuration"]
    pub epcfg1: self::device::EPCFG,
    _reserved22: [u8; 3usize],
    #[doc = "0x124 - DEVICE End Point Pipe Status Clear"]
    pub epstatusclr1: self::device::EPSTATUSCLR,
    #[doc = "0x125 - DEVICE End Point Pipe Status Set"]
    pub epstatusset1: self::device::EPSTATUSSET,
    #[doc = "0x126 - DEVICE End Point Pipe Status"]
    pub epstatus1: self::device::EPSTATUS,
    #[doc = "0x127 - DEVICE End Point Interrupt Flag"]
    pub epintflag1: self::device::EPINTFLAG,
    #[doc = "0x128 - DEVICE End Point Interrupt Clear Flag"]
    pub epintenclr1: self::device::EPINTENCLR,
    #[doc = "0x129 - DEVICE End Point Interrupt Set Flag"]
    pub epintenset1: self::device::EPINTENSET,
    _reserved28: [u8; 22usize],
    #[doc = "0x140 - DEVICE End Point Configuration"]
    pub epcfg2: self::device::EPCFG,
    _reserved29: [u8; 3usize],
    #[doc = "0x144 - DEVICE End Point Pipe Status Clear"]
    pub epstatusclr2: self::device::EPSTATUSCLR,
    #[doc = "0x145 - DEVICE End Point Pipe Status Set"]
    pub epstatusset2: self::device::EPSTATUSSET,
    #[doc = "0x146 - DEVICE End Point Pipe Status"]
    pub epstatus2: self::device::EPSTATUS,
    #[doc = "0x147 - DEVICE End Point Interrupt Flag"]
    pub epintflag2: self::device::EPINTFLAG,
    #[doc = "0x148 - DEVICE End Point Interrupt Clear Flag"]
    pub epintenclr2: self::device::EPINTENCLR,
    #[doc = "0x149 - DEVICE End Point Interrupt Set Flag"]
    pub epintenset2: self::device::EPINTENSET,
    _reserved35: [u8; 22usize],
    #[doc = "0x160 - DEVICE End Point Configuration"]
    pub epcfg3: self::device::EPCFG,
    _reserved36: [u8; 3usize],
    #[doc = "0x164 - DEVICE End Point Pipe Status Clear"]
    pub epstatusclr3: self::device::EPSTATUSCLR,
    #[doc = "0x165 - DEVICE End Point Pipe Status Set"]
    pub epstatusset3: self::device::EPSTATUSSET,
    #[doc = "0x166 - DEVICE End Point Pipe Status"]
    pub epstatus3: self::device::EPSTATUS,
    #[doc = "0x167 - DEVICE End Point Interrupt Flag"]
    pub epintflag3: self::device::EPINTFLAG,
    #[doc = "0x168 - DEVICE End Point Interrupt Clear Flag"]
    pub epintenclr3: self::device::EPINTENCLR,
    #[doc = "0x169 - DEVICE End Point Interrupt Set Flag"]
    pub epintenset3: self::device::EPINTENSET,
    _reserved42: [u8; 22usize],
    #[doc = "0x180 - DEVICE End Point Configuration"]
    pub epcfg4: self::device::EPCFG,
    _reserved43: [u8; 3usize],
    #[doc = "0x184 - DEVICE End Point Pipe Status Clear"]
    pub epstatusclr4: self::device::EPSTATUSCLR,
    #[doc = "0x185 - DEVICE End Point Pipe Status Set"]
    pub epstatusset4: self::device::EPSTATUSSET,
    #[doc = "0x186 - DEVICE End Point Pipe Status"]
    pub epstatus4: self::device::EPSTATUS,
    #[doc = "0x187 - DEVICE End Point Interrupt Flag"]
    pub epintflag4: self::device::EPINTFLAG,
    #[doc = "0x188 - DEVICE End Point Interrupt Clear Flag"]
    pub epintenclr4: self::device::EPINTENCLR,
    #[doc = "0x189 - DEVICE End Point Interrupt Set Flag"]
    pub epintenset4: self::device::EPINTENSET,
    _reserved49: [u8; 22usize],
    #[doc = "0x1a0 - DEVICE End Point Configuration"]
    pub epcfg5: self::device::EPCFG,
    _reserved50: [u8; 3usize],
    #[doc = "0x1a4 - DEVICE End Point Pipe Status Clear"]
    pub epstatusclr5: self::device::EPSTATUSCLR,
    #[doc = "0x1a5 - DEVICE End Point Pipe Status Set"]
    pub epstatusset5: self::device::EPSTATUSSET,
    #[doc = "0x1a6 - DEVICE End Point Pipe Status"]
    pub epstatus5: self::device::EPSTATUS,
    #[doc = "0x1a7 - DEVICE End Point Interrupt Flag"]
    pub epintflag5: self::device::EPINTFLAG,
    #[doc = "0x1a8 - DEVICE End Point Interrupt Clear Flag"]
    pub epintenclr5: self::device::EPINTENCLR,
    #[doc = "0x1a9 - DEVICE End Point Interrupt Set Flag"]
    pub epintenset5: self::device::EPINTENSET,
    _reserved56: [u8; 22usize],
    #[doc = "0x1c0 - DEVICE End Point Configuration"]
    pub epcfg6: self::device::EPCFG,
    _reserved57: [u8; 3usize],
    #[doc = "0x1c4 - DEVICE End Point Pipe Status Clear"]
    pub epstatusclr6: self::device::EPSTATUSCLR,
    #[doc = "0x1c5 - DEVICE End Point Pipe Status Set"]
    pub epstatusset6: self::device::EPSTATUSSET,
    #[doc = "0x1c6 - DEVICE End Point Pipe Status"]
    pub epstatus6: self::device::EPSTATUS,
    #[doc = "0x1c7 - DEVICE End Point Interrupt Flag"]
    pub epintflag6: self::device::EPINTFLAG,
    #[doc = "0x1c8 - DEVICE End Point Interrupt Clear Flag"]
    pub epintenclr6: self::device::EPINTENCLR,
    #[doc = "0x1c9 - DEVICE End Point Interrupt Set Flag"]
    pub epintenset6: self::device::EPINTENSET,
    _reserved63: [u8; 22usize],
    #[doc = "0x1e0 - DEVICE End Point Configuration"]
    pub epcfg7: self::device::EPCFG,
    _reserved64: [u8; 3usize],
    #[doc = "0x1e4 - DEVICE End Point Pipe Status Clear"]
    pub epstatusclr7: self::device::EPSTATUSCLR,
    #[doc = "0x1e5 - DEVICE End Point Pipe Status Set"]
    pub epstatusset7: self::device::EPSTATUSSET,
    #[doc = "0x1e6 - DEVICE End Point Pipe Status"]
    pub epstatus7: self::device::EPSTATUS,
    #[doc = "0x1e7 - DEVICE End Point Interrupt Flag"]
    pub epintflag7: self::device::EPINTFLAG,
    #[doc = "0x1e8 - DEVICE End Point Interrupt Clear Flag"]
    pub epintenclr7: self::device::EPINTENCLR,
    #[doc = "0x1e9 - DEVICE End Point Interrupt Set Flag"]
    pub epintenset7: self::device::EPINTENSET,
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
    #[doc = "0x100 - HOST End Point Configuration"]
    pub pcfg0: self::host::PCFG,
    _reserved16: [u8; 2usize],
    #[doc = "0x103 - HOST Bus Access Period of Pipe"]
    pub binterval0: self::host::BINTERVAL,
    #[doc = "0x104 - HOST End Point Pipe Status Clear"]
    pub pstatusclr0: self::host::PSTATUSCLR,
    #[doc = "0x105 - HOST End Point Pipe Status Set"]
    pub pstatusset0: self::host::PSTATUSSET,
    #[doc = "0x106 - HOST End Point Pipe Status"]
    pub pstatus0: self::host::PSTATUS,
    #[doc = "0x107 - HOST Pipe Interrupt Flag"]
    pub pintflag0: self::host::PINTFLAG,
    #[doc = "0x108 - HOST Pipe Interrupt Flag Clear"]
    pub pintenclr0: self::host::PINTENCLR,
    #[doc = "0x109 - HOST Pipe Interrupt Flag Set"]
    pub pintenset0: self::host::PINTENSET,
    _reserved23: [u8; 22usize],
    #[doc = "0x120 - HOST End Point Configuration"]
    pub pcfg1: self::host::PCFG,
    _reserved24: [u8; 2usize],
    #[doc = "0x123 - HOST Bus Access Period of Pipe"]
    pub binterval1: self::host::BINTERVAL,
    #[doc = "0x124 - HOST End Point Pipe Status Clear"]
    pub pstatusclr1: self::host::PSTATUSCLR,
    #[doc = "0x125 - HOST End Point Pipe Status Set"]
    pub pstatusset1: self::host::PSTATUSSET,
    #[doc = "0x126 - HOST End Point Pipe Status"]
    pub pstatus1: self::host::PSTATUS,
    #[doc = "0x127 - HOST Pipe Interrupt Flag"]
    pub pintflag1: self::host::PINTFLAG,
    #[doc = "0x128 - HOST Pipe Interrupt Flag Clear"]
    pub pintenclr1: self::host::PINTENCLR,
    #[doc = "0x129 - HOST Pipe Interrupt Flag Set"]
    pub pintenset1: self::host::PINTENSET,
    _reserved31: [u8; 22usize],
    #[doc = "0x140 - HOST End Point Configuration"]
    pub pcfg2: self::host::PCFG,
    _reserved32: [u8; 2usize],
    #[doc = "0x143 - HOST Bus Access Period of Pipe"]
    pub binterval2: self::host::BINTERVAL,
    #[doc = "0x144 - HOST End Point Pipe Status Clear"]
    pub pstatusclr2: self::host::PSTATUSCLR,
    #[doc = "0x145 - HOST End Point Pipe Status Set"]
    pub pstatusset2: self::host::PSTATUSSET,
    #[doc = "0x146 - HOST End Point Pipe Status"]
    pub pstatus2: self::host::PSTATUS,
    #[doc = "0x147 - HOST Pipe Interrupt Flag"]
    pub pintflag2: self::host::PINTFLAG,
    #[doc = "0x148 - HOST Pipe Interrupt Flag Clear"]
    pub pintenclr2: self::host::PINTENCLR,
    #[doc = "0x149 - HOST Pipe Interrupt Flag Set"]
    pub pintenset2: self::host::PINTENSET,
    _reserved39: [u8; 22usize],
    #[doc = "0x160 - HOST End Point Configuration"]
    pub pcfg3: self::host::PCFG,
    _reserved40: [u8; 2usize],
    #[doc = "0x163 - HOST Bus Access Period of Pipe"]
    pub binterval3: self::host::BINTERVAL,
    #[doc = "0x164 - HOST End Point Pipe Status Clear"]
    pub pstatusclr3: self::host::PSTATUSCLR,
    #[doc = "0x165 - HOST End Point Pipe Status Set"]
    pub pstatusset3: self::host::PSTATUSSET,
    #[doc = "0x166 - HOST End Point Pipe Status"]
    pub pstatus3: self::host::PSTATUS,
    #[doc = "0x167 - HOST Pipe Interrupt Flag"]
    pub pintflag3: self::host::PINTFLAG,
    #[doc = "0x168 - HOST Pipe Interrupt Flag Clear"]
    pub pintenclr3: self::host::PINTENCLR,
    #[doc = "0x169 - HOST Pipe Interrupt Flag Set"]
    pub pintenset3: self::host::PINTENSET,
    _reserved47: [u8; 22usize],
    #[doc = "0x180 - HOST End Point Configuration"]
    pub pcfg4: self::host::PCFG,
    _reserved48: [u8; 2usize],
    #[doc = "0x183 - HOST Bus Access Period of Pipe"]
    pub binterval4: self::host::BINTERVAL,
    #[doc = "0x184 - HOST End Point Pipe Status Clear"]
    pub pstatusclr4: self::host::PSTATUSCLR,
    #[doc = "0x185 - HOST End Point Pipe Status Set"]
    pub pstatusset4: self::host::PSTATUSSET,
    #[doc = "0x186 - HOST End Point Pipe Status"]
    pub pstatus4: self::host::PSTATUS,
    #[doc = "0x187 - HOST Pipe Interrupt Flag"]
    pub pintflag4: self::host::PINTFLAG,
    #[doc = "0x188 - HOST Pipe Interrupt Flag Clear"]
    pub pintenclr4: self::host::PINTENCLR,
    #[doc = "0x189 - HOST Pipe Interrupt Flag Set"]
    pub pintenset4: self::host::PINTENSET,
    _reserved55: [u8; 22usize],
    #[doc = "0x1a0 - HOST End Point Configuration"]
    pub pcfg5: self::host::PCFG,
    _reserved56: [u8; 2usize],
    #[doc = "0x1a3 - HOST Bus Access Period of Pipe"]
    pub binterval5: self::host::BINTERVAL,
    #[doc = "0x1a4 - HOST End Point Pipe Status Clear"]
    pub pstatusclr5: self::host::PSTATUSCLR,
    #[doc = "0x1a5 - HOST End Point Pipe Status Set"]
    pub pstatusset5: self::host::PSTATUSSET,
    #[doc = "0x1a6 - HOST End Point Pipe Status"]
    pub pstatus5: self::host::PSTATUS,
    #[doc = "0x1a7 - HOST Pipe Interrupt Flag"]
    pub pintflag5: self::host::PINTFLAG,
    #[doc = "0x1a8 - HOST Pipe Interrupt Flag Clear"]
    pub pintenclr5: self::host::PINTENCLR,
    #[doc = "0x1a9 - HOST Pipe Interrupt Flag Set"]
    pub pintenset5: self::host::PINTENSET,
    _reserved63: [u8; 22usize],
    #[doc = "0x1c0 - HOST End Point Configuration"]
    pub pcfg6: self::host::PCFG,
    _reserved64: [u8; 2usize],
    #[doc = "0x1c3 - HOST Bus Access Period of Pipe"]
    pub binterval6: self::host::BINTERVAL,
    #[doc = "0x1c4 - HOST End Point Pipe Status Clear"]
    pub pstatusclr6: self::host::PSTATUSCLR,
    #[doc = "0x1c5 - HOST End Point Pipe Status Set"]
    pub pstatusset6: self::host::PSTATUSSET,
    #[doc = "0x1c6 - HOST End Point Pipe Status"]
    pub pstatus6: self::host::PSTATUS,
    #[doc = "0x1c7 - HOST Pipe Interrupt Flag"]
    pub pintflag6: self::host::PINTFLAG,
    #[doc = "0x1c8 - HOST Pipe Interrupt Flag Clear"]
    pub pintenclr6: self::host::PINTENCLR,
    #[doc = "0x1c9 - HOST Pipe Interrupt Flag Set"]
    pub pintenset6: self::host::PINTENSET,
    _reserved71: [u8; 22usize],
    #[doc = "0x1e0 - HOST End Point Configuration"]
    pub pcfg7: self::host::PCFG,
    _reserved72: [u8; 2usize],
    #[doc = "0x1e3 - HOST Bus Access Period of Pipe"]
    pub binterval7: self::host::BINTERVAL,
    #[doc = "0x1e4 - HOST End Point Pipe Status Clear"]
    pub pstatusclr7: self::host::PSTATUSCLR,
    #[doc = "0x1e5 - HOST End Point Pipe Status Set"]
    pub pstatusset7: self::host::PSTATUSSET,
    #[doc = "0x1e6 - HOST End Point Pipe Status"]
    pub pstatus7: self::host::PSTATUS,
    #[doc = "0x1e7 - HOST Pipe Interrupt Flag"]
    pub pintflag7: self::host::PINTFLAG,
    #[doc = "0x1e8 - HOST Pipe Interrupt Flag Clear"]
    pub pintenclr7: self::host::PINTENCLR,
    #[doc = "0x1e9 - HOST Pipe Interrupt Flag Set"]
    pub pintenset7: self::host::PINTENSET,
}
#[doc = r"Register block"]
#[doc = "USB is Host"]
pub mod host;
