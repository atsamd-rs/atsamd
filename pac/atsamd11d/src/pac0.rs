#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Write Protection Clear"]
    pub wpclr: WPCLR,
    #[doc = "0x04 - Write Protection Set"]
    pub wpset: WPSET,
}
#[doc = "WPCLR (rw) register accessor: Write Protection Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpclr`]
module"]
pub type WPCLR = crate::Reg<wpclr::WPCLR_SPEC>;
#[doc = "Write Protection Clear"]
pub mod wpclr;
#[doc = "WPSET (rw) register accessor: Write Protection Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpset`]
module"]
pub type WPSET = crate::Reg<wpset::WPSET_SPEC>;
#[doc = "Write Protection Set"]
pub mod wpset;
