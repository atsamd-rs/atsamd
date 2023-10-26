#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x80],
    #[doc = "0x80..0x100 - PRS\\[%s\\]"]
    pub prs: [PRS; 16],
}
#[doc = "PRS\\[%s\\]"]
pub use self::prs::PRS;
#[doc = r"Cluster"]
#[doc = "PRS\\[%s\\]"]
pub mod prs;
