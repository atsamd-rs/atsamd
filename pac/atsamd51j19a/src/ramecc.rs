#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x01 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x02 - Interrupt Flag"]
    pub intflag: INTFLAG,
    #[doc = "0x03 - Status"]
    pub status: STATUS,
    #[doc = "0x04 - Error Address"]
    pub erraddr: ERRADDR,
    _reserved5: [u8; 7usize],
    #[doc = "0x0f - Debug Control"]
    pub dbgctrl: DBGCTRL,
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
#[doc = "Interrupt Flag"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt Flag"]
pub mod intflag;
#[doc = "Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Status"]
pub mod status;
#[doc = "Error Address"]
pub struct ERRADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Address"]
pub mod erraddr;
#[doc = "Debug Control"]
pub struct DBGCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Debug Control"]
pub mod dbgctrl;
