#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1usize],
    #[doc = "0x01 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x02 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x03 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x04 - HS Clock Division"]
    pub hsdiv: HSDIV,
    #[doc = "0x05 - CPU Clock Division"]
    pub cpudiv: CPUDIV,
    _reserved5: [u8; 10usize],
    #[doc = "0x10 - AHB Mask"]
    pub ahbmask: AHBMASK,
    #[doc = "0x14 - APBA Mask"]
    pub apbamask: APBAMASK,
    #[doc = "0x18 - APBB Mask"]
    pub apbbmask: APBBMASK,
    #[doc = "0x1c - APBC Mask"]
    pub apbcmask: APBCMASK,
    #[doc = "0x20 - APBD Mask"]
    pub apbdmask: APBDMASK,
}
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
#[doc = "HS Clock Division"]
pub struct HSDIV {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "HS Clock Division"]
pub mod hsdiv;
#[doc = "CPU Clock Division"]
pub struct CPUDIV {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CPU Clock Division"]
pub mod cpudiv;
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
#[doc = "APBD Mask"]
pub struct APBDMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APBD Mask"]
pub mod apbdmask;
