#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    wpclr: Wpclr,
    wpset: Wpset,
}
impl RegisterBlock {
    #[doc = "0x00 - Write Protection Clear"]
    #[inline(always)]
    pub const fn wpclr(&self) -> &Wpclr {
        &self.wpclr
    }
    #[doc = "0x04 - Write Protection Set"]
    #[inline(always)]
    pub const fn wpset(&self) -> &Wpset {
        &self.wpset
    }
}
#[doc = "WPCLR (rw) register accessor: Write Protection Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`wpclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpclr`]
module"]
#[doc(alias = "WPCLR")]
pub type Wpclr = crate::Reg<wpclr::WpclrSpec>;
#[doc = "Write Protection Clear"]
pub mod wpclr;
#[doc = "WPSET (rw) register accessor: Write Protection Set\n\nYou can [`read`](crate::Reg::read) this register and get [`wpset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpset`]
module"]
#[doc(alias = "WPSET")]
pub type Wpset = crate::Reg<wpset::WpsetSpec>;
#[doc = "Write Protection Set"]
pub mod wpset;
