#[doc = "Register `CID1` reader"]
pub struct R(crate::R<CID1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CID1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CID1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CID1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PREAMBLE` reader - Preamble"]
pub struct PREAMBLE_R(crate::FieldReader<u8, u8>);
impl PREAMBLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PREAMBLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREAMBLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCLASS` reader - Component Class"]
pub struct CCLASS_R(crate::FieldReader<u8, u8>);
impl CCLASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CCLASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCLASS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Preamble"]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Component Class"]
    #[inline(always)]
    pub fn cclass(&self) -> CCLASS_R {
        CCLASS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Component Identification 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid1](index.html) module"]
pub struct CID1_SPEC;
impl crate::RegisterSpec for CID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cid1::R](R) reader structure"]
impl crate::Readable for CID1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CID1 to value 0x10"]
impl crate::Resettable for CID1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
