#[doc = "Reader of register TLPITI"]
pub type R = crate::R<u32, super::TLPITI>;
#[doc = "Reader of field `TLPITI`"]
pub type TLPITI_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Increment once over 16 ahb clock when LPI indication bit 20 is set in tx mode"]
    #[inline(always)]
    pub fn tlpiti(&self) -> TLPITI_R {
        TLPITI_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
