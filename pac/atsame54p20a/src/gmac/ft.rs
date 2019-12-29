#[doc = "Reader of register FT"]
pub type R = crate::R<u32, super::FT>;
#[doc = "Reader of field `FTX`"]
pub type FTX_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Frames Transmitted without Error"]
    #[inline(always)]
    pub fn ftx(&self) -> FTX_R {
        FTX_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
