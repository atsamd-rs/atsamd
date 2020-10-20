#[doc = "Reader of register ORHI"]
pub type R = crate::R<u32, super::ORHI>;
#[doc = "Reader of field `RXO`"]
pub type RXO_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Received Octets"]
    #[inline(always)]
    pub fn rxo(&self) -> RXO_R {
        RXO_R::new((self.bits & 0xffff) as u16)
    }
}
