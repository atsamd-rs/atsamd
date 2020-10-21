#[doc = "Reader of register TUR"]
pub type R = crate::R<u32, super::TUR>;
#[doc = "Reader of field `TXUNR`"]
pub type TXUNR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Transmit Underruns"]
    #[inline(always)]
    pub fn txunr(&self) -> TXUNR_R {
        TXUNR_R::new((self.bits & 0x03ff) as u16)
    }
}
