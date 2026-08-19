#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x80],
    prs: [Prs; 16],
}
impl RegisterBlock {
    #[doc = "0x80..0x100 - PRS\\[%s\\]"]
    #[inline(always)]
    pub const fn prs(&self, n: usize) -> &Prs {
        &self.prs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x100 - PRS\\[%s\\]"]
    #[inline(always)]
    pub fn prs_iter(&self) -> impl Iterator<Item = &Prs> {
        self.prs.iter()
    }
}
#[doc = "PRS\\[%s\\]"]
pub use self::prs::Prs;
#[doc = r"Cluster"]
#[doc = "PRS\\[%s\\]"]
pub mod prs;
