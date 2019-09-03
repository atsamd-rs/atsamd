#[doc = "Reader of register MEMTYPE"]
pub type R = crate::R<u32, super::MEMTYPE>;
#[doc = "Reader of field `SMEMP`"]
pub type SMEMP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - System Memory Present"]
    #[inline(always)]
    pub fn smemp(&self) -> SMEMP_R {
        SMEMP_R::new((self.bits & 0x01) != 0)
    }
}
