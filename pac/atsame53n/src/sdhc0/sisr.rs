#[doc = "Reader of register SISR"]
pub type R = crate::R<u16, super::SISR>;
#[doc = "Reader of field `INTSSL`"]
pub type INTSSL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Interrupt Signal for Each Slot"]
    #[inline(always)]
    pub fn intssl(&self) -> INTSSL_R {
        INTSSL_R::new((self.bits & 0x01) != 0)
    }
}
