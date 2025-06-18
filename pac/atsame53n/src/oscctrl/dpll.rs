#[repr(C)]
#[doc = "DPLL\\[%s\\]"]
#[doc(alias = "DPLL")]
pub struct Dpll {
    dpllctrla: Dpllctrla,
    _reserved1: [u8; 0x03],
    dpllratio: Dpllratio,
    dpllctrlb: Dpllctrlb,
    dpllsyncbusy: Dpllsyncbusy,
    dpllstatus: Dpllstatus,
}
impl Dpll {
    #[doc = "0x00 - DPLL Control A"]
    #[inline(always)]
    pub const fn dpllctrla(&self) -> &Dpllctrla {
        &self.dpllctrla
    }
    #[doc = "0x04 - DPLL Ratio Control"]
    #[inline(always)]
    pub const fn dpllratio(&self) -> &Dpllratio {
        &self.dpllratio
    }
    #[doc = "0x08 - DPLL Control B"]
    #[inline(always)]
    pub const fn dpllctrlb(&self) -> &Dpllctrlb {
        &self.dpllctrlb
    }
    #[doc = "0x0c - DPLL Synchronization Busy"]
    #[inline(always)]
    pub const fn dpllsyncbusy(&self) -> &Dpllsyncbusy {
        &self.dpllsyncbusy
    }
    #[doc = "0x10 - DPLL Status"]
    #[inline(always)]
    pub const fn dpllstatus(&self) -> &Dpllstatus {
        &self.dpllstatus
    }
}
#[doc = "DPLLCTRLA (rw) register accessor: DPLL Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`dpllctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpllctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpllctrla`] module"]
#[doc(alias = "DPLLCTRLA")]
pub type Dpllctrla = crate::Reg<dpllctrla::DpllctrlaSpec>;
#[doc = "DPLL Control A"]
pub mod dpllctrla;
#[doc = "DPLLRATIO (rw) register accessor: DPLL Ratio Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dpllratio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpllratio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpllratio`] module"]
#[doc(alias = "DPLLRATIO")]
pub type Dpllratio = crate::Reg<dpllratio::DpllratioSpec>;
#[doc = "DPLL Ratio Control"]
pub mod dpllratio;
#[doc = "DPLLCTRLB (rw) register accessor: DPLL Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`dpllctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpllctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpllctrlb`] module"]
#[doc(alias = "DPLLCTRLB")]
pub type Dpllctrlb = crate::Reg<dpllctrlb::DpllctrlbSpec>;
#[doc = "DPLL Control B"]
pub mod dpllctrlb;
#[doc = "DPLLSYNCBUSY (r) register accessor: DPLL Synchronization Busy\n\nYou can [`read`](crate::Reg::read) this register and get [`dpllsyncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpllsyncbusy`] module"]
#[doc(alias = "DPLLSYNCBUSY")]
pub type Dpllsyncbusy = crate::Reg<dpllsyncbusy::DpllsyncbusySpec>;
#[doc = "DPLL Synchronization Busy"]
pub mod dpllsyncbusy;
#[doc = "DPLLSTATUS (r) register accessor: DPLL Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dpllstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpllstatus`] module"]
#[doc(alias = "DPLLSTATUS")]
pub type Dpllstatus = crate::Reg<dpllstatus::DpllstatusSpec>;
#[doc = "DPLL Status"]
pub mod dpllstatus;
