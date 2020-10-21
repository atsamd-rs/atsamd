#[doc = "Reader of register BCFR"]
pub type R = crate::R<u32, super::BCFR>;
#[doc = "Reader of field `BFRX`"]
pub type BFRX_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Broadcast Frames Received without Error"]
    #[inline(always)]
    pub fn bfrx(&self) -> BFRX_R {
        BFRX_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
