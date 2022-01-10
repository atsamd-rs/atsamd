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
pub struct LOCK_R(crate::FieldReader<bool, bool>);
impl LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKRDY` reader - DPLL Clock Ready"]
pub struct CLKRDY_R(crate::FieldReader<bool, bool>);
impl CLKRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "DPLL Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllstatus](index.html) module"]
pub struct DPLLSTATUS_SPEC;
impl crate::RegisterSpec for DPLLSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dpllstatus::R](R) reader structure"]
impl crate::Readable for DPLLSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DPLLSTATUS to value 0"]
impl crate::Resettable for DPLLSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
