#[doc = "Reader of register MCF"]
pub type R = crate::R<u32, super::MCF>;
#[doc = "Reader of field `MCOL`"]
pub type MCOL_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:17 - Multiple Collision"]
    #[inline(always)]
    pub fn mcol(&self) -> MCOL_R {
        MCOL_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
