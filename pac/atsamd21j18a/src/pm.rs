#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x01 - Sleep Mode"]
    pub sleep: SLEEP,
    _reserved2: [u8; 6usize],
    #[doc = "0x08 - CPU Clock Select"]
    pub cpusel: CPUSEL,
    #[doc = "0x09 - APBA Clock Select"]
    pub apbasel: APBASEL,
    #[doc = "0x0a - APBB Clock Select"]
    pub apbbsel: APBBSEL,
    #[doc = "0x0b - APBC Clock Select"]
    pub apbcsel: APBCSEL,
    _reserved6: [u8; 8usize],
    #[doc = "0x14 - AHB Mask"]
    pub ahbmask: AHBMASK,
    #[doc = "0x18 - APBA Mask"]
    pub apbamask: APBAMASK,
    #[doc = "0x1c - APBB Mask"]
    pub apbbmask: APBBMASK,
    #[doc = "0x20 - APBC Mask"]
    pub apbcmask: APBCMASK,
    _reserved10: [u8; 16usize],
    #[doc = "0x34 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x35 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x36 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved13: [u8; 1usize],
    #[doc = "0x38 - Reset Cause"]
    pub rcause: RCAUSE,
}
#[doc = "Control"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control"]
pub mod ctrl;
#[doc = "Sleep Mode"]
pub struct SLEEP {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Sleep Mode"]
pub mod sleep;
#[doc = "CPU Clock Select"]
pub struct CPUSEL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CPU Clock Select"]
pub mod cpusel;
#[doc = "APBA Clock Select"]
pub struct APBASEL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "APBA Clock Select"]
pub mod apbasel;
#[doc = "APBB Clock Select"]
pub struct APBBSEL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "APBB Clock Select"]
pub mod apbbsel;
#[doc = "APBC Clock Select"]
pub struct APBCSEL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "APBC Clock Select"]
pub mod apbcsel;
#[doc = "AHB Mask"]
pub struct AHBMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB Mask"]
pub mod ahbmask;
#[doc = "APBA Mask"]
pub struct APBAMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APBA Mask"]
pub mod apbamask;
#[doc = "APBB Mask"]
pub struct APBBMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APBB Mask"]
pub mod apbbmask;
#[doc = "APBC Mask"]
pub struct APBCMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APBC Mask"]
pub mod apbcmask;
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
#[doc = "Reset Cause"]
pub struct RCAUSE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Reset Cause"]
pub mod rcause;
