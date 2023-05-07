#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ENABLE` reader - ICM Controller Enable Register"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `RAWRMDIS` reader - RAW Region Monitoring Disabled Status"]
pub type RAWRMDIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RMDIS` reader - Region Monitoring Disabled Status"]
pub type RMDIS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - ICM Controller Enable Register"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - RAW Region Monitoring Disabled Status"]
    #[inline(always)]
    pub fn rawrmdis(&self) -> RAWRMDIS_R {
        RAWRMDIS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Region Monitoring Disabled Status"]
    #[inline(always)]
    pub fn rmdis(&self) -> RMDIS_R {
        RMDIS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
