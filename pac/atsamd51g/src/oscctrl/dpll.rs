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
#[doc = "DPLLCTRLA (rw) register accessor: DPLL Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpllctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpllctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpllctrla`]
module"]
pub type DPLLCTRLA = crate::Reg<dpllctrla::DPLLCTRLA_SPEC>;
#[doc = "DPLL Control A"]
pub mod dpllctrla;
#[doc = "DPLLRATIO (rw) register accessor: DPLL Ratio Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpllratio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpllratio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpllratio`]
module"]
pub type DPLLRATIO = crate::Reg<dpllratio::DPLLRATIO_SPEC>;
#[doc = "DPLL Ratio Control"]
pub mod dpllratio;
#[doc = "DPLLCTRLB (rw) register accessor: DPLL Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpllctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpllctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpllctrlb`]
module"]
pub type DPLLCTRLB = crate::Reg<dpllctrlb::DPLLCTRLB_SPEC>;
#[doc = "DPLL Control B"]
pub mod dpllctrlb;
#[doc = "DPLLSYNCBUSY (r) register accessor: DPLL Synchronization Busy\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpllsyncbusy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpllsyncbusy`]
module"]
pub type DPLLSYNCBUSY = crate::Reg<dpllsyncbusy::DPLLSYNCBUSY_SPEC>;
#[doc = "DPLL Synchronization Busy"]
pub mod dpllsyncbusy;
#[doc = "DPLLSTATUS (r) register accessor: DPLL Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpllstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpllstatus`]
module"]
pub type DPLLSTATUS = crate::Reg<dpllstatus::DPLLSTATUS_SPEC>;
#[doc = "DPLL Status"]
pub mod dpllstatus;
