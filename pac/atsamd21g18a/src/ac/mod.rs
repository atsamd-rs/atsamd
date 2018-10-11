#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x02 - Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x05 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x06 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved6: [u8; 1usize],
    #[doc = "0x08 - Status A"]
    pub statusa: STATUSA,
    #[doc = "0x09 - Status B"]
    pub statusb: STATUSB,
    #[doc = "0x0a - Status C"]
    pub statusc: STATUSC,
    _reserved9: [u8; 1usize],
    #[doc = "0x0c - Window Control"]
    pub winctrl: WINCTRL,
    _reserved10: [u8; 3usize],
    #[doc = "0x10 - Comparator Control n"]
    pub compctrl: [COMPCTRL; 2],
    _reserved11: [u8; 8usize],
    #[doc = "0x20 - Scaler n"]
    pub scaler: [SCALER; 2],
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
#[doc = "Status A"]
pub struct STATUSA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Status A"]
pub mod statusa;
#[doc = "Status B"]
pub struct STATUSB {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Status B"]
pub mod statusb;
#[doc = "Status C"]
pub struct STATUSC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Status C"]
pub mod statusc;
#[doc = "Window Control"]
pub struct WINCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Window Control"]
pub mod winctrl;
#[doc = "Comparator Control n"]
pub struct COMPCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Comparator Control n"]
pub mod compctrl;
#[doc = "Scaler n"]
pub struct SCALER {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Scaler n"]
pub mod scaler;
