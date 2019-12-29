#[doc = "Reader of register TLPITR"]
pub type R = crate::R<u32, super::TLPITR>;
#[doc = "Reader of field `TLPITR`"]
pub type TLPITR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Count number of times enable LPI tx bit 20 goes from low to high"]
    #[inline(always)]
    pub fn tlpitr(&self) -> TLPITR_R {
        TLPITR_R::new((self.bits & 0xffff) as u16)
    }
}
