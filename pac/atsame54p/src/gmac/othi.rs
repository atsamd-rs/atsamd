#[doc = "Reader of register OTHI"]
pub type R = crate::R<u32, super::OTHI>;
#[doc = "Reader of field `TXO`"]
pub type TXO_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmitted Octets"]
    #[inline(always)]
    pub fn txo(&self) -> TXO_R {
        TXO_R::new((self.bits & 0xffff) as u16)
    }
}
