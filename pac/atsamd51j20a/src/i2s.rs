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
    #[doc = "0x20 - Tx Serializer Control"]
    pub txctrl: TXCTRL,
    #[doc = "0x24 - Rx Serializer Control"]
    pub rxctrl: RXCTRL,
    _reserved8: [u8; 8usize],
    #[doc = "0x30 - Tx Data"]
    pub txdata: TXDATA,
    #[doc = "0x34 - Rx Data"]
    pub rxdata: RXDATA,
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
#[doc = "Tx Serializer Control"]
pub struct TXCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tx Serializer Control"]
pub mod txctrl;
#[doc = "Rx Serializer Control"]
pub struct RXCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx Serializer Control"]
pub mod rxctrl;
#[doc = "Tx Data"]
pub struct TXDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tx Data"]
pub mod txdata;
#[doc = "Rx Data"]
pub struct RXDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx Data"]
pub mod rxdata;
