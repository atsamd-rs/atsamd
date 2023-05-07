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
#[doc = "SAB (rw) register accessor: an alias for `Reg<SAB_SPEC>`"]
pub type SAB = crate::Reg<sab::SAB_SPEC>;
#[doc = "Specific Address Bottom \\[31:0\\]
Register"]
pub mod sab;
#[doc = "SAT (rw) register accessor: an alias for `Reg<SAT_SPEC>`"]
pub type SAT = crate::Reg<sat::SAT_SPEC>;
#[doc = "Specific Address Top \\[47:32\\]
Register"]
pub mod sat;
