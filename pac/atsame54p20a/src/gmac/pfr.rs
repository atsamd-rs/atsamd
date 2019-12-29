#[doc = "Reader of register PFR"]
pub type R = crate::R<u32, super::PFR>;
#[doc = "Reader of field `PFRX`"]
pub type PFRX_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Pause Frames Received Register"]
    #[inline(always)]
    pub fn pfrx(&self) -> PFRX_R {
        PFRX_R::new((self.bits & 0xffff) as u16)
    }
}
