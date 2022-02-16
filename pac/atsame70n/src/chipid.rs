#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Chip ID Register"]
    pub chipid_cidr: crate::Reg<chipid_cidr::CHIPID_CIDR_SPEC>,
    #[doc = "0x04 - Chip ID Extension Register"]
    pub chipid_exid: crate::Reg<chipid_exid::CHIPID_EXID_SPEC>,
}
#[doc = "CHIPID_CIDR register accessor: an alias for `Reg<CHIPID_CIDR_SPEC>`"]
pub type CHIPID_CIDR = crate::Reg<chipid_cidr::CHIPID_CIDR_SPEC>;
#[doc = "Chip ID Register"]
pub mod chipid_cidr;
#[doc = "CHIPID_EXID register accessor: an alias for `Reg<CHIPID_EXID_SPEC>`"]
pub type CHIPID_EXID = crate::Reg<chipid_exid::CHIPID_EXID_SPEC>;
#[doc = "Chip ID Extension Register"]
pub mod chipid_exid;
