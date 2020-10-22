#[doc = "Reader of register FR"]
pub type R = crate::R<u32, super::FR>;
#[doc = "Reader of field `FRX`"]
pub type FRX_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Frames Received without Error"]
    #[inline(always)]
    pub fn frx(&self) -> FRX_R {
        FRX_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
