#[doc = "Reader of register RESS"]
pub type R = crate::R<u16, super::RESS>;
#[doc = "Reader of field `RESS`"]
pub type RESS_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Last ADC conversion result"]
    #[inline(always)]
    pub fn ress(&self) -> RESS_R {
        RESS_R::new((self.bits & 0xffff) as u16)
    }
}
