#[doc = "Reader of register TBFR255"]
pub type R = crate::R<u32, super::TBFR255>;
#[doc = "Reader of field `NFRX`"]
pub type NFRX_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 128 to 255 Byte Frames Received without Error"]
    #[inline(always)]
    pub fn nfrx(&self) -> NFRX_R {
        NFRX_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
