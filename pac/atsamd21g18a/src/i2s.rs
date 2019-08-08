#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    _reserved1: [u8; 3usize],
    #[doc = "0x04 - Clock Unit n Control"]
    pub clkctrl: [CLKCTRL; 2],
    #[doc = "0x0c - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    _reserved3: [u8; 2usize],
    #[doc = "0x10 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    _reserved4: [u8; 2usize],
    #[doc = "0x14 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved5: [u8; 2usize],
    #[doc = "0x18 - Synchronization Status"]
    pub syncbusy: SYNCBUSY,
    _reserved6: [u8; 6usize],
    #[doc = "0x20 - Serializer n Control"]
    pub serctrl: [SERCTRL; 2],
    _reserved7: [u8; 8usize],
    #[doc = "0x30 - Data n"]
    pub data: [DATA; 2],
}
#[doc = "Control A"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "Clock Unit n Control"]
pub struct CLKCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Unit n Control"]
pub mod clkctrl;
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
#[doc = "Synchronization Status"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Synchronization Status"]
pub mod syncbusy;
#[doc = "Serializer n Control"]
pub struct SERCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Serializer n Control"]
pub mod serctrl;
#[doc = "Data n"]
pub struct DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data n"]
pub mod data;
