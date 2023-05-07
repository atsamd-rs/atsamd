#[doc = "Register `RLPITI` reader"]
pub struct R(crate::R<RLPITI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RLPITI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RLPITI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RLPITI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RLPITI` reader - Increment once over 16 ahb clock when LPI indication bit 20 is set in rx mode"]
pub type RLPITI_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Increment once over 16 ahb clock when LPI indication bit 20 is set in rx mode"]
    #[inline(always)]
    pub fn rlpiti(&self) -> RLPITI_R {
        RLPITI_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Receive LPI Time Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlpiti](index.html) module"]
pub struct RLPITI_SPEC;
impl crate::RegisterSpec for RLPITI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rlpiti::R](R) reader structure"]
impl crate::Readable for RLPITI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RLPITI to value 0"]
impl crate::Resettable for RLPITI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
