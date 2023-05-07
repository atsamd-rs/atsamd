#[doc = "Register `DID` reader"]
pub struct R(crate::R<DID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DEVSEL` reader - Device Select"]
pub type DEVSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REVISION` reader - Revision"]
pub type REVISION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIE` reader - Die Identification"]
pub type DIE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SERIES` reader - Product Series"]
pub type SERIES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FAMILY` reader - Product Family"]
pub type FAMILY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PROCESSOR` reader - Processor"]
pub type PROCESSOR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Device Select"]
    #[inline(always)]
    pub fn devsel(&self) -> DEVSEL_R {
        DEVSEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Revision"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Die Identification"]
    #[inline(always)]
    pub fn die(&self) -> DIE_R {
        DIE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Product Series"]
    #[inline(always)]
    pub fn series(&self) -> SERIES_R {
        SERIES_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 23:27 - Product Family"]
    #[inline(always)]
    pub fn family(&self) -> FAMILY_R {
        FAMILY_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:31 - Processor"]
    #[inline(always)]
    pub fn processor(&self) -> PROCESSOR_R {
        PROCESSOR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Device Identification\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [did](index.html) module"]
pub struct DID_SPEC;
impl crate::RegisterSpec for DID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [did::R](R) reader structure"]
impl crate::Readable for DID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DID to value 0x1001_0305"]
impl crate::Resettable for DID_SPEC {
    const RESET_VALUE: Self::Ux = 0x1001_0305;
}
