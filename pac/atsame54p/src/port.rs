#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x60 - GROUP\\[%s\\]"]
    pub group0: GROUP,
    _reserved1: [u8; 0x20],
    #[doc = "0x80..0xe0 - GROUP\\[%s\\]"]
    pub group1: GROUP,
    _reserved2: [u8; 0x20],
    #[doc = "0x100..0x160 - GROUP\\[%s\\]"]
    pub group2: GROUP,
    _reserved3: [u8; 0x20],
    #[doc = "0x180..0x1e0 - GROUP\\[%s\\]"]
    pub group3: GROUP,
}
#[doc = "GROUP\\[%s\\]"]
pub use self::group::GROUP;
#[doc = r"Cluster"]
#[doc = "GROUP\\[%s\\]"]
pub mod group;
