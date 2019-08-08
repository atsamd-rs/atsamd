#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Non-Maskable Interrupt Control"]
    pub nmictrl: NMICTRL,
    #[doc = "0x02 - Non-Maskable Interrupt Flag Status and Clear"]
    pub nmiflag: NMIFLAG,
    #[doc = "0x04 - Synchronization Busy"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x08 - Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x0c - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x10 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x14 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x18 - External Interrupt Asynchronous Mode"]
    pub asynch: ASYNCH,
    #[doc = "0x1c - External Interrupt Sense Configuration"]
    pub config: [CONFIG; 2],
    _reserved10: [u8; 12usize],
    #[doc = "0x30 - Debouncer Enable"]
    pub debouncen: DEBOUNCEN,
    #[doc = "0x34 - Debouncer Prescaler"]
    pub dprescaler: DPRESCALER,
    #[doc = "0x38 - Pin State"]
    pub pinstate: PINSTATE,
}
#[doc = "Control A"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "Non-Maskable Interrupt Control"]
pub struct NMICTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Non-Maskable Interrupt Control"]
pub mod nmictrl;
#[doc = "Non-Maskable Interrupt Flag Status and Clear"]
pub struct NMIFLAG {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Non-Maskable Interrupt Flag Status and Clear"]
pub mod nmiflag;
#[doc = "Synchronization Busy"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronization Busy"]
pub mod syncbusy;
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
#[doc = "External Interrupt Asynchronous Mode"]
pub struct ASYNCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External Interrupt Asynchronous Mode"]
pub mod asynch;
#[doc = "External Interrupt Sense Configuration"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External Interrupt Sense Configuration"]
pub mod config;
#[doc = "Debouncer Enable"]
pub struct DEBOUNCEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debouncer Enable"]
pub mod debouncen;
#[doc = "Debouncer Prescaler"]
pub struct DPRESCALER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debouncer Prescaler"]
pub mod dprescaler;
#[doc = "Pin State"]
pub struct PINSTATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin State"]
pub mod pinstate;
