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
    #[doc = "0x08 - Data"]
    pub data: DATA,
    _reserved8: [u8; 2usize],
    #[doc = "0x0c - Data Buffer"]
    pub databuf: DATABUF,
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
#[doc = "Data"]
pub struct DATA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Data"]
pub mod data;
#[doc = "Data Buffer"]
pub struct DATABUF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Data Buffer"]
pub mod databuf;
