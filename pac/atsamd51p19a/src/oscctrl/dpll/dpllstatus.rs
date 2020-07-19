#[doc = "Reader of register DPLLSTATUS"]
pub type R = crate::R<u32, super::DPLLSTATUS>;
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLKRDY`"]
pub type CLKRDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - DPLL Lock Status"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DPLL Clock Ready"]
    #[inline(always)]
    pub fn clkrdy(&self) -> CLKRDY_R {
        CLKRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
