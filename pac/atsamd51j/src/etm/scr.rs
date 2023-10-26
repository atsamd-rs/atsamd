#[doc = "Register `SCR` reader"]
pub type R = crate::R<SCR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "ETM System Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr::R`](R) reader structure"]
impl crate::Readable for SCR_SPEC {}
#[doc = "`reset()` method sets SCR to value 0x0002_0d09"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0d09;
}
