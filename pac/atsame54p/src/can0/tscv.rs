#[doc = "Reader of register TSCV"]
pub type R = crate::R<u32, super::TSCV>;
#[doc = "Reader of field `TSC`"]
pub type TSC_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timestamp Counter"]
    #[inline(always)]
    pub fn tsc(&self) -> TSC_R {
        TSC_R::new((self.bits & 0xffff) as u16)
    }
}
