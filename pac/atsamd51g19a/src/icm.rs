#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration"]
    pub cfg: CFG,
    #[doc = "0x04 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x08 - Status"]
    pub sr: SR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Interrupt Enable"]
    pub ier: IER,
    #[doc = "0x14 - Interrupt Disable"]
    pub idr: IDR,
    #[doc = "0x18 - Interrupt Mask"]
    pub imr: IMR,
    #[doc = "0x1c - Interrupt Status"]
    pub isr: ISR,
    #[doc = "0x20 - Undefined Access Status"]
    pub uasr: UASR,
    _reserved8: [u8; 12usize],
    #[doc = "0x30 - Region Descriptor Area Start Address"]
    pub dscr: DSCR,
    #[doc = "0x34 - Region Hash Area Start Address"]
    pub hash: HASH,
    #[doc = "0x38 - User Initial Hash Value n"]
    pub uihval: [UIHVAL; 8],
}
#[doc = "Configuration"]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration"]
pub mod cfg;
#[doc = "Control"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control"]
pub mod ctrl;
#[doc = "Status"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status"]
pub mod sr;
#[doc = "Interrupt Enable"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable"]
pub mod ier;
#[doc = "Interrupt Disable"]
pub struct IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable"]
pub mod idr;
#[doc = "Interrupt Mask"]
pub struct IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask"]
pub mod imr;
#[doc = "Interrupt Status"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status"]
pub mod isr;
#[doc = "Undefined Access Status"]
pub struct UASR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Undefined Access Status"]
pub mod uasr;
#[doc = "Region Descriptor Area Start Address"]
pub struct DSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Region Descriptor Area Start Address"]
pub mod dscr;
#[doc = "Region Hash Area Start Address"]
pub struct HASH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Region Hash Area Start Address"]
pub mod hash;
#[doc = "User Initial Hash Value n"]
pub struct UIHVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "User Initial Hash Value n"]
pub mod uihval;
