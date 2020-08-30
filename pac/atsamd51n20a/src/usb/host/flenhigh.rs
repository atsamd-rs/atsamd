#[doc = "Reader of register FLENHIGH"]
pub type R = crate::R<u8, super::FLENHIGH>;
#[doc = "Reader of field `FLENHIGH`"]
pub type FLENHIGH_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Frame Length"]
    #[inline(always)]
    pub fn flenhigh(&self) -> FLENHIGH_R {
        FLENHIGH_R::new((self.bits & 0xff) as u8)
    }
}
