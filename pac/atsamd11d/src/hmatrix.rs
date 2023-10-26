#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x80],
    #[doc = "0x80 - Priority A for Slave"]
    pub pras0: PRAS,
    #[doc = "0x84 - Priority B for Slave"]
    pub prbs0: PRBS,
    #[doc = "0x88 - Priority A for Slave"]
    pub pras1: PRAS,
    #[doc = "0x8c - Priority B for Slave"]
    pub prbs1: PRBS,
    #[doc = "0x90 - Priority A for Slave"]
    pub pras2: PRAS,
    #[doc = "0x94 - Priority B for Slave"]
    pub prbs2: PRBS,
    #[doc = "0x98 - Priority A for Slave"]
    pub pras3: PRAS,
    #[doc = "0x9c - Priority B for Slave"]
    pub prbs3: PRBS,
    #[doc = "0xa0 - Priority A for Slave"]
    pub pras4: PRAS,
    #[doc = "0xa4 - Priority B for Slave"]
    pub prbs4: PRBS,
    #[doc = "0xa8 - Priority A for Slave"]
    pub pras5: PRAS,
    #[doc = "0xac - Priority B for Slave"]
    pub prbs5: PRBS,
    #[doc = "0xb0 - Priority A for Slave"]
    pub pras6: PRAS,
    #[doc = "0xb4 - Priority B for Slave"]
    pub prbs6: PRBS,
    #[doc = "0xb8 - Priority A for Slave"]
    pub pras7: PRAS,
    #[doc = "0xbc - Priority B for Slave"]
    pub prbs7: PRBS,
    #[doc = "0xc0 - Priority A for Slave"]
    pub pras8: PRAS,
    #[doc = "0xc4 - Priority B for Slave"]
    pub prbs8: PRBS,
    #[doc = "0xc8 - Priority A for Slave"]
    pub pras9: PRAS,
    #[doc = "0xcc - Priority B for Slave"]
    pub prbs9: PRBS,
    #[doc = "0xd0 - Priority A for Slave"]
    pub pras10: PRAS,
    #[doc = "0xd4 - Priority B for Slave"]
    pub prbs10: PRBS,
    #[doc = "0xd8 - Priority A for Slave"]
    pub pras11: PRAS,
    #[doc = "0xdc - Priority B for Slave"]
    pub prbs11: PRBS,
    #[doc = "0xe0 - Priority A for Slave"]
    pub pras12: PRAS,
    #[doc = "0xe4 - Priority B for Slave"]
    pub prbs12: PRBS,
    #[doc = "0xe8 - Priority A for Slave"]
    pub pras13: PRAS,
    #[doc = "0xec - Priority B for Slave"]
    pub prbs13: PRBS,
    #[doc = "0xf0 - Priority A for Slave"]
    pub pras14: PRAS,
    #[doc = "0xf4 - Priority B for Slave"]
    pub prbs14: PRBS,
    #[doc = "0xf8 - Priority A for Slave"]
    pub pras15: PRAS,
    #[doc = "0xfc - Priority B for Slave"]
    pub prbs15: PRBS,
    _reserved32: [u8; 0x10],
    #[doc = "0x110..0x150 - Special Function"]
    pub sfr: [SFR; 16],
}
#[doc = "PRAS (rw) register accessor: Priority A for Slave\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pras::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pras::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pras`]
module"]
pub type PRAS = crate::Reg<pras::PRAS_SPEC>;
#[doc = "Priority A for Slave"]
pub mod pras;
#[doc = "PRBS (rw) register accessor: Priority B for Slave\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prbs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prbs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prbs`]
module"]
pub type PRBS = crate::Reg<prbs::PRBS_SPEC>;
#[doc = "Priority B for Slave"]
pub mod prbs;
#[doc = "SFR (rw) register accessor: Special Function\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr`]
module"]
pub type SFR = crate::Reg<sfr::SFR_SPEC>;
#[doc = "Special Function"]
pub mod sfr;
