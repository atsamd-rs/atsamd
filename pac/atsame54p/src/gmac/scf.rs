#[doc = "Reader of register SCF"]
pub type R = crate::R<u32, super::SCF>;
#[doc = "Reader of field `SCOL`"]
pub type SCOL_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:17 - Single Collision"]
    #[inline(always)]
    pub fn scol(&self) -> SCOL_R {
        SCOL_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
