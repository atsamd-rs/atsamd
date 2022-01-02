#[doc = "Register `TLPITI` reader"]
pub struct R(crate::R<TLPITI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TLPITI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TLPITI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TLPITI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TLPITI` reader - Increment once over 16 ahb clock when LPI indication bit 20 is set in tx mode"]
pub struct TLPITI_R(crate::FieldReader<u32, u32>);
impl TLPITI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TLPITI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TLPITI_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:23 - Increment once over 16 ahb clock when LPI indication bit 20 is set in tx mode"]
    #[inline(always)]
    pub fn tlpiti(&self) -> TLPITI_R {
        TLPITI_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
#[doc = "Receive LPI Time Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tlpiti](index.html) module"]
pub struct TLPITI_SPEC;
impl crate::RegisterSpec for TLPITI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tlpiti::R](R) reader structure"]
impl crate::Readable for TLPITI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TLPITI to value 0"]
impl crate::Resettable for TLPITI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
