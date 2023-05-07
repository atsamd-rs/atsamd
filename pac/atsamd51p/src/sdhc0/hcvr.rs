#[doc = "Register `HCVR` reader"]
pub struct R(crate::R<HCVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SVER` reader - Spec Version"]
pub type SVER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VVER` reader - Vendor Version"]
pub type VVER_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Spec Version"]
    #[inline(always)]
    pub fn sver(&self) -> SVER_R {
        SVER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Vendor Version"]
    #[inline(always)]
    pub fn vver(&self) -> VVER_R {
        VVER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Host Controller Version\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcvr](index.html) module"]
pub struct HCVR_SPEC;
impl crate::RegisterSpec for HCVR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [hcvr::R](R) reader structure"]
impl crate::Readable for HCVR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HCVR to value 0x1802"]
impl crate::Resettable for HCVR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1802;
}
