#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - Control B"]
    pub ctrlb: CTRLB,
    _reserved2: [u8; 2usize],
    #[doc = "0x08 - NVM Parameter"]
    pub param: PARAM,
    #[doc = "0x0c - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x0e - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x10 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x12 - Status"]
    pub status: STATUS,
    #[doc = "0x14 - Address"]
    pub addr: ADDR,
    #[doc = "0x18 - Lock Section"]
    pub runlock: RUNLOCK,
    #[doc = "0x1c - Page Buffer Load Data x"]
    pub pbldata: [PBLDATA; 2],
    #[doc = "0x24 - ECC Error Status Register"]
    pub eccerr: ECCERR,
    #[doc = "0x28 - Debug Control"]
    pub dbgctrl: DBGCTRL,
    _reserved12: [u8; 1usize],
    #[doc = "0x2a - SmartEEPROM Configuration Register"]
    pub seecfg: SEECFG,
    _reserved13: [u8; 1usize],
    #[doc = "0x2c - SmartEEPROM Status Register"]
    pub seestat: SEESTAT,
}
#[doc = "Control A"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "Control B"]
pub struct CTRLB {
    register: ::vcell::VolatileCell<u16>,
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
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "Interrupt Enable Set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "Interrupt Flag Status and Clear"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u16>,
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
pub struct RUNLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lock Section"]
pub mod runlock;
#[doc = "Page Buffer Load Data x"]
pub struct PBLDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Page Buffer Load Data x"]
pub mod pbldata;
#[doc = "ECC Error Status Register"]
pub struct ECCERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ECC Error Status Register"]
pub mod eccerr;
#[doc = "Debug Control"]
pub struct DBGCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "SmartEEPROM Configuration Register"]
pub struct SEECFG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SmartEEPROM Configuration Register"]
pub mod seecfg;
#[doc = "SmartEEPROM Status Register"]
pub struct SEESTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SmartEEPROM Status Register"]
pub mod seestat;
