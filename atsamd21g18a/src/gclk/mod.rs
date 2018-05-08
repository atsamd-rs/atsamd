#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x01 - Status"]
    pub status: STATUS,
    #[doc = "0x02 - Generic Clock Control"]
    pub clkctrl: CLKCTRL,
    #[doc = "0x04 - Generic Clock Generator Control"]
    pub genctrl: GENCTRL,
    #[doc = "0x08 - Generic Clock Generator Division"]
    pub gendiv: GENDIV,
}
#[doc = "Control"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control"]
pub mod ctrl;
#[doc = "Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Status"]
pub mod status;
#[doc = "Generic Clock Control"]
pub struct CLKCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Generic Clock Control"]
pub mod clkctrl;
#[doc = "Generic Clock Generator Control"]
pub struct GENCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Generic Clock Generator Control"]
pub mod genctrl;
#[doc = "Generic Clock Generator Division"]
pub struct GENDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Generic Clock Generator Division"]
pub mod gendiv;
