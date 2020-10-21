#[doc = "Reader of register RLPITI"]
pub type R = crate::R<u32, super::RLPITI>;
#[doc = "Reader of field `RLPITI`"]
pub type RLPITI_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Increment once over 16 ahb clock when LPI indication bit 20 is set in rx mode"]
    #[inline(always)]
    pub fn rlpiti(&self) -> RLPITI_R {
        RLPITI_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
