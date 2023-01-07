#[doc = "Register `STATUSD` reader"]
pub struct R(crate::R<STATUSD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUSD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUSD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUSD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SERCOM6_` reader - SERCOM6 APB Protect Enable"]
pub struct SERCOM6__R(crate::FieldReader<bool, bool>);
impl SERCOM6__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SERCOM6__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERCOM6__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERCOM7_` reader - SERCOM7 APB Protect Enable"]
pub struct SERCOM7__R(crate::FieldReader<bool, bool>);
impl SERCOM7__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SERCOM7__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERCOM7__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC5_` reader - TC5 APB Protect Enable"]
pub struct TC5__R(crate::FieldReader<bool, bool>);
impl TC5__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC5__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC5__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC6_` reader - TC6 APB Protect Enable"]
pub struct TC6__R(crate::FieldReader<bool, bool>);
impl TC6__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC6__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC6__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC7_` reader - TC7 APB Protect Enable"]
pub struct TC7__R(crate::FieldReader<bool, bool>);
impl TC7__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC7__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC7__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - SERCOM6 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom6_(&self) -> SERCOM6__R {
        SERCOM6__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SERCOM7 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom7_(&self) -> SERCOM7__R {
        SERCOM7__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TC5 APB Protect Enable"]
    #[inline(always)]
    pub fn tc5_(&self) -> TC5__R {
        TC5__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TC6 APB Protect Enable"]
    #[inline(always)]
    pub fn tc6_(&self) -> TC6__R {
        TC6__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TC7 APB Protect Enable"]
    #[inline(always)]
    pub fn tc7_(&self) -> TC7__R {
        TC7__R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
#[doc = "Peripheral write protection status - Bridge D\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusd](index.html) module"]
pub struct STATUSD_SPEC;
impl crate::RegisterSpec for STATUSD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statusd::R](R) reader structure"]
impl crate::Readable for STATUSD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUSD to value 0"]
impl crate::Resettable for STATUSD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
