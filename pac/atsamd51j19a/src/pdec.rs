#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x04 - Control B Clear"]
    pub ctrlbclr: CTRLBCLR,
    #[doc = "0x05 - Control B Set"]
    pub ctrlbset: CTRLBSET,
    #[doc = "0x06 - Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x08 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x09 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x0a - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved7: [u8; 1usize],
    #[doc = "0x0c - Status"]
    pub status: STATUS,
    _reserved8: [u8; 1usize],
    #[doc = "0x0f - Debug Control"]
    pub dbgctrl: DBGCTRL,
    #[doc = "0x10 - Synchronization Status"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x14 - Prescaler Value"]
    pub presc: PRESC,
    #[doc = "0x15 - Filter Value"]
    pub filter: FILTER,
    _reserved12: [u8; 2usize],
    #[doc = "0x18 - Prescaler Buffer Value"]
    pub prescbuf: PRESCBUF,
    #[doc = "0x19 - Filter Buffer Value"]
    pub filterbuf: FILTERBUF,
    _reserved14: [u8; 2usize],
    #[doc = "0x1c - Counter Value"]
    pub count: COUNT,
    #[doc = "0x20 - Channel n Compare Value"]
    pub cc: [CC; 2],
    _reserved16: [u8; 8usize],
    #[doc = "0x30 - Channel Compare Buffer Value"]
    pub ccbuf: [CCBUF; 2],
}
#[doc = "Control A"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "Control B Clear"]
pub struct CTRLBCLR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control B Clear"]
pub mod ctrlbclr;
#[doc = "Control B Set"]
pub struct CTRLBSET {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control B Set"]
pub mod ctrlbset;
#[doc = "Event Control"]
pub struct EVCTRL {
    register: ::vcell::VolatileCell<u16>,
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
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Status"]
pub mod status;
#[doc = "Debug Control"]
pub struct DBGCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "Synchronization Status"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronization Status"]
pub mod syncbusy;
#[doc = "Prescaler Value"]
pub struct PRESC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Prescaler Value"]
pub mod presc;
#[doc = "Filter Value"]
pub struct FILTER {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Filter Value"]
pub mod filter;
#[doc = "Prescaler Buffer Value"]
pub struct PRESCBUF {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Prescaler Buffer Value"]
pub mod prescbuf;
#[doc = "Filter Buffer Value"]
pub struct FILTERBUF {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Filter Buffer Value"]
pub mod filterbuf;
#[doc = "Counter Value"]
pub struct COUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter Value"]
pub mod count;
#[doc = "Channel n Compare Value"]
pub struct CC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel n Compare Value"]
pub mod cc;
#[doc = "Channel Compare Buffer Value"]
pub struct CCBUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Compare Buffer Value"]
pub mod ccbuf;
