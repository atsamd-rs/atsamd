#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    _reserved1: [u8; 3usize],
    #[doc = "0x04 - Channel"]
    pub channel: CHANNEL,
    #[doc = "0x08 - User Multiplexer"]
    pub user: USER,
    _reserved3: [u8; 2usize],
    #[doc = "0x0c - Channel Status"]
    pub chstatus: CHSTATUS,
    #[doc = "0x10 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x14 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x18 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
}
#[doc = "Control"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control"]
pub mod ctrl;
#[doc = "Channel"]
pub struct CHANNEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel"]
pub mod channel;
#[doc = "User Multiplexer"]
pub struct USER {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "User Multiplexer"]
pub mod user;
#[doc = "Channel Status"]
pub struct CHSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Status"]
pub mod chstatus;
#[doc = "Interrupt Enable Clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "Interrupt Enable Set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "Interrupt Flag Status and Clear"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
