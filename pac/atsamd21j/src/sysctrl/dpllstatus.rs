#[doc = "Register `DPLLSTATUS` reader"]
pub type R = crate::R<DpllstatusSpec>;
#[doc = "Field `LOCK` reader - DPLL Lock Status"]
pub type LockR = crate::BitReader;
#[doc = "Field `CLKRDY` reader - Output Clock Ready"]
pub type ClkrdyR = crate::BitReader;
#[doc = "Field `ENABLE` reader - DPLL Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `DIV` reader - Divider Enable"]
pub type DivR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DPLL Lock Status"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output Clock Ready"]
    #[inline(always)]
    pub fn clkrdy(&self) -> ClkrdyR {
        ClkrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DPLL Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Divider Enable"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "DPLL Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dpllstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpllstatusSpec;
impl crate::RegisterSpec for DpllstatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dpllstatus::R`](R) reader structure"]
impl crate::Readable for DpllstatusSpec {}
#[doc = "`reset()` method sets DPLLSTATUS to value 0"]
impl crate::Resettable for DpllstatusSpec {
    const RESET_VALUE: u8 = 0;
}
