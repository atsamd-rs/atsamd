#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrla: CTRLA,
    _reserved1: [u8; 3usize],
    #[doc = "0x04 - Software Event"]
    pub swevt: SWEVT,
    #[doc = "0x08 - Priority Control"]
    pub prictrl: PRICTRL,
    _reserved3: [u8; 7usize],
    #[doc = "0x10 - Channel Pending Interrupt"]
    pub intpend: INTPEND,
    _reserved4: [u8; 2usize],
    #[doc = "0x14 - Interrupt Status"]
    pub intstatus: INTSTATUS,
    #[doc = "0x18 - Busy Channels"]
    pub busych: BUSYCH,
    #[doc = "0x1c - Ready Users"]
    pub readyusr: READYUSR,
    #[doc = "0x20 - Channel n Control"]
    pub channel0: CHANNEL,
    #[doc = "0x24 - Channel n Interrupt Enable Clear"]
    pub chintenclr0: CHINTENCLR,
    #[doc = "0x25 - Channel n Interrupt Enable Set"]
    pub chintenset0: CHINTENSET,
    #[doc = "0x26 - Channel n Interrupt Flag Status and Clear"]
    pub chintflag0: CHINTFLAG,
    #[doc = "0x27 - Channel n Status"]
    pub chstatus0: CHSTATUS,
    #[doc = "0x28 - Channel n Control"]
    pub channel1: CHANNEL,
    #[doc = "0x2c - Channel n Interrupt Enable Clear"]
    pub chintenclr1: CHINTENCLR,
    #[doc = "0x2d - Channel n Interrupt Enable Set"]
    pub chintenset1: CHINTENSET,
    #[doc = "0x2e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag1: CHINTFLAG,
    #[doc = "0x2f - Channel n Status"]
    pub chstatus1: CHSTATUS,
    #[doc = "0x30 - Channel n Control"]
    pub channel2: CHANNEL,
    #[doc = "0x34 - Channel n Interrupt Enable Clear"]
    pub chintenclr2: CHINTENCLR,
    #[doc = "0x35 - Channel n Interrupt Enable Set"]
    pub chintenset2: CHINTENSET,
    #[doc = "0x36 - Channel n Interrupt Flag Status and Clear"]
    pub chintflag2: CHINTFLAG,
    #[doc = "0x37 - Channel n Status"]
    pub chstatus2: CHSTATUS,
    #[doc = "0x38 - Channel n Control"]
    pub channel3: CHANNEL,
    #[doc = "0x3c - Channel n Interrupt Enable Clear"]
    pub chintenclr3: CHINTENCLR,
    #[doc = "0x3d - Channel n Interrupt Enable Set"]
    pub chintenset3: CHINTENSET,
    #[doc = "0x3e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag3: CHINTFLAG,
    #[doc = "0x3f - Channel n Status"]
    pub chstatus3: CHSTATUS,
    #[doc = "0x40 - Channel n Control"]
    pub channel4: CHANNEL,
    #[doc = "0x44 - Channel n Interrupt Enable Clear"]
    pub chintenclr4: CHINTENCLR,
    #[doc = "0x45 - Channel n Interrupt Enable Set"]
    pub chintenset4: CHINTENSET,
    #[doc = "0x46 - Channel n Interrupt Flag Status and Clear"]
    pub chintflag4: CHINTFLAG,
    #[doc = "0x47 - Channel n Status"]
    pub chstatus4: CHSTATUS,
    #[doc = "0x48 - Channel n Control"]
    pub channel5: CHANNEL,
    #[doc = "0x4c - Channel n Interrupt Enable Clear"]
    pub chintenclr5: CHINTENCLR,
    #[doc = "0x4d - Channel n Interrupt Enable Set"]
    pub chintenset5: CHINTENSET,
    #[doc = "0x4e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag5: CHINTFLAG,
    #[doc = "0x4f - Channel n Status"]
    pub chstatus5: CHSTATUS,
    #[doc = "0x50 - Channel n Control"]
    pub channel6: CHANNEL,
    #[doc = "0x54 - Channel n Interrupt Enable Clear"]
    pub chintenclr6: CHINTENCLR,
    #[doc = "0x55 - Channel n Interrupt Enable Set"]
    pub chintenset6: CHINTENSET,
    #[doc = "0x56 - Channel n Interrupt Flag Status and Clear"]
    pub chintflag6: CHINTFLAG,
    #[doc = "0x57 - Channel n Status"]
    pub chstatus6: CHSTATUS,
    #[doc = "0x58 - Channel n Control"]
    pub channel7: CHANNEL,
    #[doc = "0x5c - Channel n Interrupt Enable Clear"]
    pub chintenclr7: CHINTENCLR,
    #[doc = "0x5d - Channel n Interrupt Enable Set"]
    pub chintenset7: CHINTENSET,
    #[doc = "0x5e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag7: CHINTFLAG,
    #[doc = "0x5f - Channel n Status"]
    pub chstatus7: CHSTATUS,
    #[doc = "0x60 - Channel n Control"]
    pub channel8: CHANNEL,
    #[doc = "0x64 - Channel n Interrupt Enable Clear"]
    pub chintenclr8: CHINTENCLR,
    #[doc = "0x65 - Channel n Interrupt Enable Set"]
    pub chintenset8: CHINTENSET,
    #[doc = "0x66 - Channel n Interrupt Flag Status and Clear"]
    pub chintflag8: CHINTFLAG,
    #[doc = "0x67 - Channel n Status"]
    pub chstatus8: CHSTATUS,
    #[doc = "0x68 - Channel n Control"]
    pub channel9: CHANNEL,
    #[doc = "0x6c - Channel n Interrupt Enable Clear"]
    pub chintenclr9: CHINTENCLR,
    #[doc = "0x6d - Channel n Interrupt Enable Set"]
    pub chintenset9: CHINTENSET,
    #[doc = "0x6e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag9: CHINTFLAG,
    #[doc = "0x6f - Channel n Status"]
    pub chstatus9: CHSTATUS,
    #[doc = "0x70 - Channel n Control"]
    pub channel10: CHANNEL,
    #[doc = "0x74 - Channel n Interrupt Enable Clear"]
    pub chintenclr10: CHINTENCLR,
    #[doc = "0x75 - Channel n Interrupt Enable Set"]
    pub chintenset10: CHINTENSET,
    #[doc = "0x76 - Channel n Interrupt Flag Status and Clear"]
    pub chintflag10: CHINTFLAG,
    #[doc = "0x77 - Channel n Status"]
    pub chstatus10: CHSTATUS,
    #[doc = "0x78 - Channel n Control"]
    pub channel11: CHANNEL,
    #[doc = "0x7c - Channel n Interrupt Enable Clear"]
    pub chintenclr11: CHINTENCLR,
    #[doc = "0x7d - Channel n Interrupt Enable Set"]
    pub chintenset11: CHINTENSET,
    #[doc = "0x7e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag11: CHINTFLAG,
    #[doc = "0x7f - Channel n Status"]
    pub chstatus11: CHSTATUS,
    #[doc = "0x80 - Channel n Control"]
    pub channel12: CHANNEL,
    #[doc = "0x84 - Channel n Interrupt Enable Clear"]
    pub chintenclr12: CHINTENCLR,
    #[doc = "0x85 - Channel n Interrupt Enable Set"]
    pub chintenset12: CHINTENSET,
    #[doc = "0x86 - Channel n Interrupt Flag Status and Clear"]
    pub chintflag12: CHINTFLAG,
    #[doc = "0x87 - Channel n Status"]
    pub chstatus12: CHSTATUS,
    #[doc = "0x88 - Channel n Control"]
    pub channel13: CHANNEL,
    #[doc = "0x8c - Channel n Interrupt Enable Clear"]
    pub chintenclr13: CHINTENCLR,
    #[doc = "0x8d - Channel n Interrupt Enable Set"]
    pub chintenset13: CHINTENSET,
    #[doc = "0x8e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag13: CHINTFLAG,
    #[doc = "0x8f - Channel n Status"]
    pub chstatus13: CHSTATUS,
    #[doc = "0x90 - Channel n Control"]
    pub channel14: CHANNEL,
    #[doc = "0x94 - Channel n Interrupt Enable Clear"]
    pub chintenclr14: CHINTENCLR,
    #[doc = "0x95 - Channel n Interrupt Enable Set"]
    pub chintenset14: CHINTENSET,
    #[doc = "0x96 - Channel n Interrupt Flag Status and Clear"]
    pub chintflag14: CHINTFLAG,
    #[doc = "0x97 - Channel n Status"]
    pub chstatus14: CHSTATUS,
    #[doc = "0x98 - Channel n Control"]
    pub channel15: CHANNEL,
    #[doc = "0x9c - Channel n Interrupt Enable Clear"]
    pub chintenclr15: CHINTENCLR,
    #[doc = "0x9d - Channel n Interrupt Enable Set"]
    pub chintenset15: CHINTENSET,
    #[doc = "0x9e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag15: CHINTFLAG,
    #[doc = "0x9f - Channel n Status"]
    pub chstatus15: CHSTATUS,
    #[doc = "0xa0 - Channel n Control"]
    pub channel16: CHANNEL,
    #[doc = "0xa4 - Channel n Interrupt Enable Clear"]
    pub chintenclr16: CHINTENCLR,
    #[doc = "0xa5 - Channel n Interrupt Enable Set"]
    pub chintenset16: CHINTENSET,
    #[doc = "0xa6 - Channel n Interrupt Flag Status and Clear"]
    pub chintflag16: CHINTFLAG,
    #[doc = "0xa7 - Channel n Status"]
    pub chstatus16: CHSTATUS,
    #[doc = "0xa8 - Channel n Control"]
    pub channel17: CHANNEL,
    #[doc = "0xac - Channel n Interrupt Enable Clear"]
    pub chintenclr17: CHINTENCLR,
    #[doc = "0xad - Channel n Interrupt Enable Set"]
    pub chintenset17: CHINTENSET,
    #[doc = "0xae - Channel n Interrupt Flag Status and Clear"]
    pub chintflag17: CHINTFLAG,
    #[doc = "0xaf - Channel n Status"]
    pub chstatus17: CHSTATUS,
    #[doc = "0xb0 - Channel n Control"]
    pub channel18: CHANNEL,
    #[doc = "0xb4 - Channel n Interrupt Enable Clear"]
    pub chintenclr18: CHINTENCLR,
    #[doc = "0xb5 - Channel n Interrupt Enable Set"]
    pub chintenset18: CHINTENSET,
    #[doc = "0xb6 - Channel n Interrupt Flag Status and Clear"]
    pub chintflag18: CHINTFLAG,
    #[doc = "0xb7 - Channel n Status"]
    pub chstatus18: CHSTATUS,
    #[doc = "0xb8 - Channel n Control"]
    pub channel19: CHANNEL,
    #[doc = "0xbc - Channel n Interrupt Enable Clear"]
    pub chintenclr19: CHINTENCLR,
    #[doc = "0xbd - Channel n Interrupt Enable Set"]
    pub chintenset19: CHINTENSET,
    #[doc = "0xbe - Channel n Interrupt Flag Status and Clear"]
    pub chintflag19: CHINTFLAG,
    #[doc = "0xbf - Channel n Status"]
    pub chstatus19: CHSTATUS,
    #[doc = "0xc0 - Channel n Control"]
    pub channel20: CHANNEL,
    #[doc = "0xc4 - Channel n Interrupt Enable Clear"]
    pub chintenclr20: CHINTENCLR,
    #[doc = "0xc5 - Channel n Interrupt Enable Set"]
    pub chintenset20: CHINTENSET,
    #[doc = "0xc6 - Channel n Interrupt Flag Status and Clear"]
    pub chintflag20: CHINTFLAG,
    #[doc = "0xc7 - Channel n Status"]
    pub chstatus20: CHSTATUS,
    #[doc = "0xc8 - Channel n Control"]
    pub channel21: CHANNEL,
    #[doc = "0xcc - Channel n Interrupt Enable Clear"]
    pub chintenclr21: CHINTENCLR,
    #[doc = "0xcd - Channel n Interrupt Enable Set"]
    pub chintenset21: CHINTENSET,
    #[doc = "0xce - Channel n Interrupt Flag Status and Clear"]
    pub chintflag21: CHINTFLAG,
    #[doc = "0xcf - Channel n Status"]
    pub chstatus21: CHSTATUS,
    #[doc = "0xd0 - Channel n Control"]
    pub channel22: CHANNEL,
    #[doc = "0xd4 - Channel n Interrupt Enable Clear"]
    pub chintenclr22: CHINTENCLR,
    #[doc = "0xd5 - Channel n Interrupt Enable Set"]
    pub chintenset22: CHINTENSET,
    #[doc = "0xd6 - Channel n Interrupt Flag Status and Clear"]
    pub chintflag22: CHINTFLAG,
    #[doc = "0xd7 - Channel n Status"]
    pub chstatus22: CHSTATUS,
    #[doc = "0xd8 - Channel n Control"]
    pub channel23: CHANNEL,
    #[doc = "0xdc - Channel n Interrupt Enable Clear"]
    pub chintenclr23: CHINTENCLR,
    #[doc = "0xdd - Channel n Interrupt Enable Set"]
    pub chintenset23: CHINTENSET,
    #[doc = "0xde - Channel n Interrupt Flag Status and Clear"]
    pub chintflag23: CHINTFLAG,
    #[doc = "0xdf - Channel n Status"]
    pub chstatus23: CHSTATUS,
    #[doc = "0xe0 - Channel n Control"]
    pub channel24: CHANNEL,
    #[doc = "0xe4 - Channel n Interrupt Enable Clear"]
    pub chintenclr24: CHINTENCLR,
    #[doc = "0xe5 - Channel n Interrupt Enable Set"]
    pub chintenset24: CHINTENSET,
    #[doc = "0xe6 - Channel n Interrupt Flag Status and Clear"]
    pub chintflag24: CHINTFLAG,
    #[doc = "0xe7 - Channel n Status"]
    pub chstatus24: CHSTATUS,
    #[doc = "0xe8 - Channel n Control"]
    pub channel25: CHANNEL,
    #[doc = "0xec - Channel n Interrupt Enable Clear"]
    pub chintenclr25: CHINTENCLR,
    #[doc = "0xed - Channel n Interrupt Enable Set"]
    pub chintenset25: CHINTENSET,
    #[doc = "0xee - Channel n Interrupt Flag Status and Clear"]
    pub chintflag25: CHINTFLAG,
    #[doc = "0xef - Channel n Status"]
    pub chstatus25: CHSTATUS,
    #[doc = "0xf0 - Channel n Control"]
    pub channel26: CHANNEL,
    #[doc = "0xf4 - Channel n Interrupt Enable Clear"]
    pub chintenclr26: CHINTENCLR,
    #[doc = "0xf5 - Channel n Interrupt Enable Set"]
    pub chintenset26: CHINTENSET,
    #[doc = "0xf6 - Channel n Interrupt Flag Status and Clear"]
    pub chintflag26: CHINTFLAG,
    #[doc = "0xf7 - Channel n Status"]
    pub chstatus26: CHSTATUS,
    #[doc = "0xf8 - Channel n Control"]
    pub channel27: CHANNEL,
    #[doc = "0xfc - Channel n Interrupt Enable Clear"]
    pub chintenclr27: CHINTENCLR,
    #[doc = "0xfd - Channel n Interrupt Enable Set"]
    pub chintenset27: CHINTENSET,
    #[doc = "0xfe - Channel n Interrupt Flag Status and Clear"]
    pub chintflag27: CHINTFLAG,
    #[doc = "0xff - Channel n Status"]
    pub chstatus27: CHSTATUS,
    #[doc = "0x100 - Channel n Control"]
    pub channel28: CHANNEL,
    #[doc = "0x104 - Channel n Interrupt Enable Clear"]
    pub chintenclr28: CHINTENCLR,
    #[doc = "0x105 - Channel n Interrupt Enable Set"]
    pub chintenset28: CHINTENSET,
    #[doc = "0x106 - Channel n Interrupt Flag Status and Clear"]
    pub chintflag28: CHINTFLAG,
    #[doc = "0x107 - Channel n Status"]
    pub chstatus28: CHSTATUS,
    #[doc = "0x108 - Channel n Control"]
    pub channel29: CHANNEL,
    #[doc = "0x10c - Channel n Interrupt Enable Clear"]
    pub chintenclr29: CHINTENCLR,
    #[doc = "0x10d - Channel n Interrupt Enable Set"]
    pub chintenset29: CHINTENSET,
    #[doc = "0x10e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag29: CHINTFLAG,
    #[doc = "0x10f - Channel n Status"]
    pub chstatus29: CHSTATUS,
    #[doc = "0x110 - Channel n Control"]
    pub channel30: CHANNEL,
    #[doc = "0x114 - Channel n Interrupt Enable Clear"]
    pub chintenclr30: CHINTENCLR,
    #[doc = "0x115 - Channel n Interrupt Enable Set"]
    pub chintenset30: CHINTENSET,
    #[doc = "0x116 - Channel n Interrupt Flag Status and Clear"]
    pub chintflag30: CHINTFLAG,
    #[doc = "0x117 - Channel n Status"]
    pub chstatus30: CHSTATUS,
    #[doc = "0x118 - Channel n Control"]
    pub channel31: CHANNEL,
    #[doc = "0x11c - Channel n Interrupt Enable Clear"]
    pub chintenclr31: CHINTENCLR,
    #[doc = "0x11d - Channel n Interrupt Enable Set"]
    pub chintenset31: CHINTENSET,
    #[doc = "0x11e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag31: CHINTFLAG,
    #[doc = "0x11f - Channel n Status"]
    pub chstatus31: CHSTATUS,
    #[doc = "0x120 - User Multiplexer n"]
    pub user: [USER; 67],
}
#[doc = "Control"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control"]
pub mod ctrla;
#[doc = "Software Event"]
pub struct SWEVT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Event"]
pub mod swevt;
#[doc = "Priority Control"]
pub struct PRICTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Priority Control"]
pub mod prictrl;
#[doc = "Channel Pending Interrupt"]
pub struct INTPEND {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Channel Pending Interrupt"]
pub mod intpend;
#[doc = "Interrupt Status"]
pub struct INTSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status"]
pub mod intstatus;
#[doc = "Busy Channels"]
pub struct BUSYCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Busy Channels"]
pub mod busych;
#[doc = "Ready Users"]
pub struct READYUSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ready Users"]
pub mod readyusr;
#[doc = "Channel n Control"]
pub struct CHANNEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel n Control"]
pub mod channel;
#[doc = "Channel n Interrupt Enable Clear"]
pub struct CHINTENCLR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Interrupt Enable Clear"]
pub mod chintenclr;
#[doc = "Channel n Interrupt Enable Set"]
pub struct CHINTENSET {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Interrupt Enable Set"]
pub mod chintenset;
#[doc = "Channel n Interrupt Flag Status and Clear"]
pub struct CHINTFLAG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Interrupt Flag Status and Clear"]
pub mod chintflag;
#[doc = "Channel n Status"]
pub struct CHSTATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Status"]
pub mod chstatus;
#[doc = "User Multiplexer n"]
pub struct USER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "User Multiplexer n"]
pub mod user;
