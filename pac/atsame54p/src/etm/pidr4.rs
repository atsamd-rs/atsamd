#[doc = "Register `PIDR4` reader"]
pub struct R(crate::R<PIDR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIDR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIDR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "ETM Peripheral Identification Register #4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr4](index.html) module"]
pub struct PIDR4_SPEC;
impl crate::RegisterSpec for PIDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr4::R](R) reader structure"]
impl crate::Readable for PIDR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIDR4 to value 0x04"]
impl crate::Resettable for PIDR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
