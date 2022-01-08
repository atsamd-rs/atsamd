#[doc = "Register `BKIN` reader"]
pub struct R(crate::R<BKIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BKIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BKIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BKIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BKIN0` reader - Backup Input 0"]
pub struct BKIN0_R(crate::FieldReader<bool, bool>);
impl BKIN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BKIN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKIN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKIN1` reader - Backup Input 1"]
pub struct BKIN1_R(crate::FieldReader<bool, bool>);
impl BKIN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BKIN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKIN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Backup Input 0"]
    #[inline(always)]
    pub fn bkin0(&self) -> BKIN0_R {
        BKIN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Backup Input 1"]
    #[inline(always)]
    pub fn bkin1(&self) -> BKIN1_R {
        BKIN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "Backup Input Control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkin](index.html) module"]
pub struct BKIN_SPEC;
impl crate::RegisterSpec for BKIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bkin::R](R) reader structure"]
impl crate::Readable for BKIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BKIN to value 0"]
impl crate::Resettable for BKIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
