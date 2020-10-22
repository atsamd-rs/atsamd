#[doc = "Reader of register MFR"]
pub type R = crate::R<u32, super::MFR>;
#[doc = "Reader of field `MFRX`"]
pub type MFRX_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Multicast Frames Received without Error"]
    #[inline(always)]
    pub fn mfrx(&self) -> MFRX_R {
        MFRX_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
