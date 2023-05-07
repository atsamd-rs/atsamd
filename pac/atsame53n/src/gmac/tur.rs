#[doc = "Register `TUR` reader"]
pub struct R(crate::R<TUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXUNR` reader - Transmit Underruns"]
pub type TXUNR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Transmit Underruns"]
    #[inline(always)]
    pub fn txunr(&self) -> TXUNR_R {
        TXUNR_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Transmit Underruns Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tur](index.html) module"]
pub struct TUR_SPEC;
impl crate::RegisterSpec for TUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tur::R](R) reader structure"]
impl crate::Readable for TUR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TUR to value 0"]
impl crate::Resettable for TUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
