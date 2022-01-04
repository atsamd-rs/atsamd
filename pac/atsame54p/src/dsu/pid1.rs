#[doc = "Register `PID1` reader"]
pub struct R(crate::R<PID1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PID1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PID1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PID1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PARTNBH` reader - Part Number High"]
pub struct PARTNBH_R(crate::FieldReader<u8, u8>);
impl PARTNBH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PARTNBH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARTNBH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JEPIDCL` reader - Low part of the JEP-106 Identity Code"]
pub struct JEPIDCL_R(crate::FieldReader<u8, u8>);
impl JEPIDCL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        JEPIDCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JEPIDCL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Part Number High"]
    #[inline(always)]
    pub fn partnbh(&self) -> PARTNBH_R {
        PARTNBH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Low part of the JEP-106 Identity Code"]
    #[inline(always)]
    pub fn jepidcl(&self) -> JEPIDCL_R {
        JEPIDCL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral Identification 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid1](index.html) module"]
pub struct PID1_SPEC;
impl crate::RegisterSpec for PID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pid1::R](R) reader structure"]
impl crate::Readable for PID1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PID1 to value 0xfc"]
impl crate::Resettable for PID1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfc
    }
}
