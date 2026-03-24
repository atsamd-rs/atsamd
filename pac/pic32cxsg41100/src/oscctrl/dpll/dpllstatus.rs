#[doc = "Register `DPLLSTATUS` reader"]
pub type R = crate::R<DpllstatusSpec>;
#[doc = "Field `LOCK` reader - DPLL Lock Status"]
pub type LockR = crate::BitReader;
#[doc = "Field `CLKRDY` reader - DPLL Clock Ready"]
pub type ClkrdyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DPLL Lock Status"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DPLL Clock Ready"]
    #[inline(always)]
    pub fn clkrdy(&self) -> ClkrdyR {
        ClkrdyR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "DPLL Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dpllstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpllstatusSpec;
impl crate::RegisterSpec for DpllstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpllstatus::R`](R) reader structure"]
impl crate::Readable for DpllstatusSpec {}
#[doc = "`reset()` method sets DPLLSTATUS to value 0"]
impl crate::Resettable for DpllstatusSpec {
    const RESET_VALUE: u32 = 0;
}
