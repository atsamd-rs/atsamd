#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x08 - NVM Parameter"]
    pub param: PARAM,
    #[doc = "0x0c - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    _reserved4: [u8; 3usize],
    #[doc = "0x10 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    _reserved5: [u8; 3usize],
    #[doc = "0x14 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved6: [u8; 3usize],
    #[doc = "0x18 - Status"]
    pub status: STATUS,
    _reserved7: [u8; 2usize],
    #[doc = "0x1c - Address"]
    pub addr: ADDR,
    #[doc = "0x20 - Lock Section"]
    pub lock: LOCK,
}
#[doc = "Control A"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "Control B"]
pub struct CTRLB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "NVM Parameter"]
pub struct PARAM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NVM Parameter"]
pub mod param;
#[doc = "Interrupt Enable Clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "Interrupt Enable Set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "Interrupt Flag Status and Clear"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Status"]
pub mod status;
#[doc = "Address"]
pub struct ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Address"]
pub mod addr;
#[doc = "Lock Section"]
pub struct LOCK {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Lock Section"]
pub mod lock;
