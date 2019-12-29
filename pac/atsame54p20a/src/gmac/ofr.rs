#[doc = "Reader of register OFR"]
pub type R = crate::R<u32, super::OFR>;
#[doc = "Reader of field `OFRX`"]
pub type OFRX_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Oversized Frames Received"]
    #[inline(always)]
    pub fn ofrx(&self) -> OFRX_R {
        OFRX_R::new((self.bits & 0x03ff) as u16)
    }
}
