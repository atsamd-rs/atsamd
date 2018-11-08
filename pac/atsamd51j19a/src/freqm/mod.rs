#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A Register"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Control B Register"]
    pub ctrlb: CTRLB,
    #[doc = "0x02 - Config A register"]
    pub cfga: CFGA,
    _reserved3: [u8; 4usize],
    #[doc = "0x08 - Interrupt Enable Clear Register"]
    pub intenclr: INTENCLR,
    #[doc = "0x09 - Interrupt Enable Set Register"]
    pub intenset: INTENSET,
    #[doc = "0x0a - Interrupt Flag Register"]
    pub intflag: INTFLAG,
    #[doc = "0x0b - Status Register"]
    pub status: STATUS,
    #[doc = "0x0c - Synchronization Busy Register"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x10 - Count Value Register"]
    pub value: VALUE,
}
#[doc = "Control A Register"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control A Register"]
pub mod ctrla;
#[doc = "Control B Register"]
pub struct CTRLB {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control B Register"]
pub mod ctrlb;
#[doc = "Config A register"]
pub struct CFGA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Config A register"]
pub mod cfga;
#[doc = "Interrupt Enable Clear Register"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt Enable Clear Register"]
pub mod intenclr;
#[doc = "Interrupt Enable Set Register"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt Enable Set Register"]
pub mod intenset;
#[doc = "Interrupt Flag Register"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt Flag Register"]
pub mod intflag;
#[doc = "Status Register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Status Register"]
pub mod status;
#[doc = "Synchronization Busy Register"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "Count Value Register"]
pub struct VALUE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Count Value Register"]
pub mod value;
