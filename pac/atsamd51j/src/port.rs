#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x60 - GROUP\\[%s\\]"]
    pub group0: GROUP,
    _reserved1: [u8; 0x20],
    #[doc = "0x80..0xe0 - GROUP\\[%s\\]"]
    pub group1: GROUP,
}
#[doc = "GROUP\\[%s\\]"]
pub use self::group::GROUP;
#[doc = r"Cluster"]
#[doc = "GROUP\\[%s\\]"]
pub mod group;
