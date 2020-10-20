#[doc = "Reader of register TIMESTAMP"]
pub type R = crate::R<u32, super::TIMESTAMP>;
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Count Timestamp Value"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
