#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Configuration"]
    pub config: CONFIG,
    #[doc = "0x02 - Early Warning Interrupt Control"]
    pub ewctrl: EWCTRL,
    _reserved3: [u8; 1usize],
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x05 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x06 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved6: [u8; 1usize],
    #[doc = "0x08 - Synchronization Busy"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x0c - Clear"]
    pub clear: CLEAR,
}
#[doc = "Control"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control"]
pub mod ctrla;
#[doc = "Configuration"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Configuration"]
pub mod config;
#[doc = "Early Warning Interrupt Control"]
pub struct EWCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Early Warning Interrupt Control"]
pub mod ewctrl;
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
#[doc = "Synchronization Busy"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "Clear"]
pub struct CLEAR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clear"]
pub mod clear;
