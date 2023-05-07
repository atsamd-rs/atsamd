#[doc = r"Register block"]
#[repr(C)]
pub struct DPLL {
    #[doc = "0x00 - DPLL Control A"]
    pub dpllctrla: DPLLCTRLA,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - DPLL Ratio Control"]
    pub dpllratio: DPLLRATIO,
    #[doc = "0x08 - DPLL Control B"]
    pub dpllctrlb: DPLLCTRLB,
    #[doc = "0x0c - DPLL Synchronization Busy"]
    pub dpllsyncbusy: DPLLSYNCBUSY,
    #[doc = "0x10 - DPLL Status"]
    pub dpllstatus: DPLLSTATUS,
}
#[doc = "DPLLCTRLA (rw) register accessor: an alias for `Reg<DPLLCTRLA_SPEC>`"]
pub type DPLLCTRLA = crate::Reg<dpllctrla::DPLLCTRLA_SPEC>;
#[doc = "DPLL Control A"]
pub mod dpllctrla;
#[doc = "DPLLRATIO (rw) register accessor: an alias for `Reg<DPLLRATIO_SPEC>`"]
pub type DPLLRATIO = crate::Reg<dpllratio::DPLLRATIO_SPEC>;
#[doc = "DPLL Ratio Control"]
pub mod dpllratio;
#[doc = "DPLLCTRLB (rw) register accessor: an alias for `Reg<DPLLCTRLB_SPEC>`"]
pub type DPLLCTRLB = crate::Reg<dpllctrlb::DPLLCTRLB_SPEC>;
#[doc = "DPLL Control B"]
pub mod dpllctrlb;
#[doc = "DPLLSYNCBUSY (r) register accessor: an alias for `Reg<DPLLSYNCBUSY_SPEC>`"]
pub type DPLLSYNCBUSY = crate::Reg<dpllsyncbusy::DPLLSYNCBUSY_SPEC>;
#[doc = "DPLL Synchronization Busy"]
pub mod dpllsyncbusy;
#[doc = "DPLLSTATUS (r) register accessor: an alias for `Reg<DPLLSTATUS_SPEC>`"]
pub type DPLLSTATUS = crate::Reg<dpllstatus::DPLLSTATUS_SPEC>;
#[doc = "DPLL Status"]
pub mod dpllstatus;
