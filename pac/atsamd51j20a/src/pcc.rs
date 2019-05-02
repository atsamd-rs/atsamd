#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Mode Register"]
    pub mr: MR,
    #[doc = "0x04 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x08 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x0c - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x10 - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x14 - Reception Holding Register"]
    pub rhr: RHR,
    _reserved6: [u8; 200usize],
    #[doc = "0xe0 - Write Protection Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe4 - Write Protection Status Register"]
    pub wpsr: WPSR,
}
#[doc = "Mode Register"]
pub struct MR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod mr;
#[doc = "Interrupt Enable Register"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interrupt Disable Register"]
pub struct IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "Interrupt Mask Register"]
pub struct IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "Interrupt Status Register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "Reception Holding Register"]
pub struct RHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reception Holding Register"]
pub mod rhr;
#[doc = "Write Protection Mode Register"]
pub struct WPMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
#[doc = "Write Protection Status Register"]
pub struct WPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Status Register"]
pub mod wpsr;
