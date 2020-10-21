#[doc = "Reader of register ORLO"]
pub type R = crate::R<u32, super::ORLO>;
#[doc = "Reader of field `RXO`"]
pub type RXO_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Received Octets"]
    #[inline(always)]
    pub fn rxo(&self) -> RXO_R {
        RXO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
