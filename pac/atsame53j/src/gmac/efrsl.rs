#[doc = "Reader of register EFRSL"]
pub type R = crate::R<u32, super::EFRSL>;
#[doc = "Reader of field `RUD`"]
pub type RUD_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RUD_R {
        RUD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
