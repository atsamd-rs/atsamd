#[doc = "Reader of register PFT"]
pub type R = crate::R<u32, super::PFT>;
#[doc = "Reader of field `PFTX`"]
pub type PFTX_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Pause Frames Transmitted Register"]
    #[inline(always)]
    pub fn pftx(&self) -> PFTX_R {
        PFTX_R::new((self.bits & 0xffff) as u16)
    }
}
