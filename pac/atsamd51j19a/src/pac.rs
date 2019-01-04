#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Write control"]
    pub wrctrl: WRCTRL,
    #[doc = "0x04 - Event control"]
    pub evctrl: EVCTRL,
    _reserved2: [u8; 3usize],
    #[doc = "0x08 - Interrupt enable clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x09 - Interrupt enable set"]
    pub intenset: INTENSET,
    _reserved4: [u8; 6usize],
    #[doc = "0x10 - Bridge interrupt flag status"]
    pub intflagahb: INTFLAGAHB,
    #[doc = "0x14 - Peripheral interrupt flag status - Bridge A"]
    pub intflaga: INTFLAGA,
    #[doc = "0x18 - Peripheral interrupt flag status - Bridge B"]
    pub intflagb: INTFLAGB,
    #[doc = "0x1c - Peripheral interrupt flag status - Bridge C"]
    pub intflagc: INTFLAGC,
    #[doc = "0x20 - Peripheral interrupt flag status - Bridge D"]
    pub intflagd: INTFLAGD,
    _reserved9: [u8; 16usize],
    #[doc = "0x34 - Peripheral write protection status - Bridge A"]
    pub statusa: STATUSA,
    #[doc = "0x38 - Peripheral write protection status - Bridge B"]
    pub statusb: STATUSB,
    #[doc = "0x3c - Peripheral write protection status - Bridge C"]
    pub statusc: STATUSC,
    #[doc = "0x40 - Peripheral write protection status - Bridge D"]
    pub statusd: STATUSD,
}
#[doc = "Write control"]
pub struct WRCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write control"]
pub mod wrctrl;
#[doc = "Event control"]
pub struct EVCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Event control"]
pub mod evctrl;
#[doc = "Interrupt enable clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt enable clear"]
pub mod intenclr;
#[doc = "Interrupt enable set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt enable set"]
pub mod intenset;
#[doc = "Bridge interrupt flag status"]
pub struct INTFLAGAHB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bridge interrupt flag status"]
pub mod intflagahb;
#[doc = "Peripheral interrupt flag status - Bridge A"]
pub struct INTFLAGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral interrupt flag status - Bridge A"]
pub mod intflaga;
#[doc = "Peripheral interrupt flag status - Bridge B"]
pub struct INTFLAGB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral interrupt flag status - Bridge B"]
pub mod intflagb;
#[doc = "Peripheral interrupt flag status - Bridge C"]
pub struct INTFLAGC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral interrupt flag status - Bridge C"]
pub mod intflagc;
#[doc = "Peripheral interrupt flag status - Bridge D"]
pub struct INTFLAGD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral interrupt flag status - Bridge D"]
pub mod intflagd;
#[doc = "Peripheral write protection status - Bridge A"]
pub struct STATUSA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral write protection status - Bridge A"]
pub mod statusa;
#[doc = "Peripheral write protection status - Bridge B"]
pub struct STATUSB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral write protection status - Bridge B"]
pub mod statusb;
#[doc = "Peripheral write protection status - Bridge C"]
pub struct STATUSC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral write protection status - Bridge C"]
pub mod statusc;
#[doc = "Peripheral write protection status - Bridge D"]
pub struct STATUSD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral write protection status - Bridge D"]
pub mod statusd;
