#[doc = "Reader of register DTF"]
pub type R = crate::R<u32, super::DTF>;
#[doc = "Reader of field `DEFT`"]
pub type DEFT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:17 - Deferred Transmission"]
    #[inline(always)]
    pub fn deft(&self) -> DEFT_R {
        DEFT_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
