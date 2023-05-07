#[doc = "Register `DPLLSTATUS` reader"]
pub struct R(crate::R<DPLLSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPLLSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPLLSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPLLSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LOCK` reader - DPLL Lock Status"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `CLKRDY` reader - Output Clock Ready"]
pub type CLKRDY_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` reader - DPLL Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `DIV` reader - Divider Enable"]
pub type DIV_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - DPLL Lock Status"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output Clock Ready"]
    #[inline(always)]
    pub fn clkrdy(&self) -> CLKRDY_R {
        CLKRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DPLL Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Divider Enable"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "DPLL Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllstatus](index.html) module"]
pub struct DPLLSTATUS_SPEC;
impl crate::RegisterSpec for DPLLSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dpllstatus::R](R) reader structure"]
impl crate::Readable for DPLLSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DPLLSTATUS to value 0"]
impl crate::Resettable for DPLLSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
