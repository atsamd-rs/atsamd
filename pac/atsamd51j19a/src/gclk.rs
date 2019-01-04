#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrla: CTRLA,
    _reserved1: [u8; 3usize],
    #[doc = "0x04 - Synchronization Busy"]
    pub syncbusy: SYNCBUSY,
    _reserved2: [u8; 24usize],
    #[doc = "0x20 - Generic Clock Generator Control"]
    pub genctrl: [GENCTRL; 12],
    _reserved3: [u8; 48usize],
    #[doc = "0x80 - Peripheral Clock Control"]
    pub pchctrl: [PCHCTRL; 48],
}
#[doc = "Control"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control"]
pub mod ctrla;
#[doc = "Synchronization Busy"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "Generic Clock Generator Control"]
pub struct GENCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Generic Clock Generator Control"]
pub mod genctrl;
#[doc = "Peripheral Clock Control"]
pub struct PCHCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Clock Control"]
pub mod pchctrl;
