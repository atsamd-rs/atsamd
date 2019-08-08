#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cache Type Register"]
    pub type_: TYPE,
    #[doc = "0x04 - Cache Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x08 - Cache Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x0c - Cache Status Register"]
    pub sr: SR,
    #[doc = "0x10 - Cache Lock per Way Register"]
    pub lckway: LCKWAY,
    _reserved5: [u8; 12usize],
    #[doc = "0x20 - Cache Maintenance Register 0"]
    pub maint0: MAINT0,
    #[doc = "0x24 - Cache Maintenance Register 1"]
    pub maint1: MAINT1,
    #[doc = "0x28 - Cache Monitor Configuration Register"]
    pub mcfg: MCFG,
    #[doc = "0x2c - Cache Monitor Enable Register"]
    pub men: MEN,
    #[doc = "0x30 - Cache Monitor Control Register"]
    pub mctrl: MCTRL,
    #[doc = "0x34 - Cache Monitor Status Register"]
    pub msr: MSR,
}
#[doc = "Cache Type Register"]
pub struct TYPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Type Register"]
pub mod type_;
#[doc = "Cache Configuration Register"]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Configuration Register"]
pub mod cfg;
#[doc = "Cache Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Control Register"]
pub mod ctrl;
#[doc = "Cache Status Register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Status Register"]
pub mod sr;
#[doc = "Cache Lock per Way Register"]
pub struct LCKWAY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Lock per Way Register"]
pub mod lckway;
#[doc = "Cache Maintenance Register 0"]
pub struct MAINT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Maintenance Register 0"]
pub mod maint0;
#[doc = "Cache Maintenance Register 1"]
pub struct MAINT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Maintenance Register 1"]
pub mod maint1;
#[doc = "Cache Monitor Configuration Register"]
pub struct MCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Monitor Configuration Register"]
pub mod mcfg;
#[doc = "Cache Monitor Enable Register"]
pub struct MEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Monitor Enable Register"]
pub mod men;
#[doc = "Cache Monitor Control Register"]
pub struct MCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Monitor Control Register"]
pub mod mctrl;
#[doc = "Cache Monitor Status Register"]
pub struct MSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Monitor Status Register"]
pub mod msr;
