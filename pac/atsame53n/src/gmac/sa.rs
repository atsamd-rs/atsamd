#[doc = r"Register block"]
#[repr(C)]
pub struct SA {
    #[doc = "0x00 - Specific Address Bottom \\[31:0\\]
Register"]
    pub sab: SAB,
    #[doc = "0x04 - Specific Address Top \\[47:32\\]
Register"]
    pub sat: SAT,
}
#[doc = "SAB (rw) register accessor: Specific Address Bottom \\[31:0\\]
Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sab::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sab::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sab`]
module"]
pub type SAB = crate::Reg<sab::SAB_SPEC>;
#[doc = "Specific Address Bottom \\[31:0\\]
Register"]
pub mod sab;
#[doc = "SAT (rw) register accessor: Specific Address Top \\[47:32\\]
Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sat`]
module"]
pub type SAT = crate::Reg<sat::SAT_SPEC>;
#[doc = "Specific Address Top \\[47:32\\]
Register"]
pub mod sat;
