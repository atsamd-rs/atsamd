#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x02 - Event Control"]
    pub evctrl: EVCTRL,
    _reserved3: [u8; 1usize],
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x05 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x06 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x07 - Status"]
    pub status: STATUS,
    #[doc = "0x08 - Synchronization Busy"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x0c - DAC n Control"]
    pub dacctrl: [DACCTRL; 2],
    #[doc = "0x10 - DAC n Data"]
    pub data: [DATA; 2],
    #[doc = "0x14 - DAC n Data Buffer"]
    pub databuf: [DATABUF; 2],
    #[doc = "0x18 - Debug Control"]
    pub dbgctrl: DBGCTRL,
    _reserved12: [u8; 3usize],
    #[doc = "0x1c - Filter Result"]
    pub result: [RESULT; 2],
}
#[doc = "Control A"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "Control B"]
pub struct CTRLB {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "Event Control"]
pub struct EVCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Event Control"]
pub mod evctrl;
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
#[doc = "Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Status"]
pub mod status;
#[doc = "Synchronization Busy"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "DAC n Control"]
pub struct DACCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "DAC n Control"]
pub mod dacctrl;
#[doc = "DAC n Data"]
pub struct DATA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "DAC n Data"]
pub mod data;
#[doc = "DAC n Data Buffer"]
pub struct DATABUF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "DAC n Data Buffer"]
pub mod databuf;
#[doc = "Debug Control"]
pub struct DBGCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "Filter Result"]
pub struct RESULT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Filter Result"]
pub mod result;
