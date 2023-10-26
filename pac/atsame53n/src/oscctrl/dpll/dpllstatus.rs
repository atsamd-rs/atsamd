#[doc = "Register `DPLLSTATUS` reader"]
pub type R = crate::R<DPLLSTATUS_SPEC>;
#[doc = "Field `LOCK` reader - DPLL Lock Status"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `CLKRDY` reader - DPLL Clock Ready"]
pub type CLKRDY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DPLL Lock Status"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DPLL Clock Ready"]
    #[inline(always)]
    pub fn clkrdy(&self) -> CLKRDY_R {
        CLKRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "DPLL Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpllstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPLLSTATUS_SPEC;
impl crate::RegisterSpec for DPLLSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpllstatus::R`](R) reader structure"]
impl crate::Readable for DPLLSTATUS_SPEC {}
#[doc = "`reset()` method sets DPLLSTATUS to value 0"]
impl crate::Resettable for DPLLSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
