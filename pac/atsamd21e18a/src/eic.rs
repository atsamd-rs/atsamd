#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x01 - Status"]
    pub status: STATUS,
    #[doc = "0x02 - Non-Maskable Interrupt Control"]
    pub nmictrl: NMICTRL,
    #[doc = "0x03 - Non-Maskable Interrupt Flag Status and Clear"]
    pub nmiflag: NMIFLAG,
    #[doc = "0x04 - Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x08 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x0c - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x10 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x14 - Wake-Up Enable"]
    pub wakeup: WAKEUP,
    #[doc = "0x18 - Configuration n"]
    pub config: [CONFIG; 2],
}
#[doc = "Control"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control"]
pub mod ctrl;
#[doc = "Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Status"]
pub mod status;
#[doc = "Non-Maskable Interrupt Control"]
pub struct NMICTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Non-Maskable Interrupt Control"]
pub mod nmictrl;
#[doc = "Non-Maskable Interrupt Flag Status and Clear"]
pub struct NMIFLAG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Non-Maskable Interrupt Flag Status and Clear"]
pub mod nmiflag;
#[doc = "Event Control"]
pub struct EVCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Control"]
pub mod evctrl;
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
#[doc = "Wake-Up Enable"]
pub struct WAKEUP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake-Up Enable"]
pub mod wakeup;
#[doc = "Configuration n"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration n"]
pub mod config;
