#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x20 - General Purpose Backup Register 0"]
    pub sys_gpbr: [crate::Reg<sys_gpbr::SYS_GPBR_SPEC>; 8],
}
#[doc = "SYS_GPBR register accessor: an alias for `Reg<SYS_GPBR_SPEC>`"]
pub type SYS_GPBR = crate::Reg<sys_gpbr::SYS_GPBR_SPEC>;
#[doc = "General Purpose Backup Register 0"]
pub mod sys_gpbr;
