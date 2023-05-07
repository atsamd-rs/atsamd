#[doc = "Register `RLPITR` reader"]
pub struct R(crate::R<RLPITR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RLPITR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RLPITR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RLPITR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RLPITR` reader - Count number of times transition from rx normal idle to low power idle"]
pub type RLPITR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Count number of times transition from rx normal idle to low power idle"]
    #[inline(always)]
    pub fn rlpitr(&self) -> RLPITR_R {
        RLPITR_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receive LPI transition Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlpitr](index.html) module"]
pub struct RLPITR_SPEC;
impl crate::RegisterSpec for RLPITR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rlpitr::R](R) reader structure"]
impl crate::Readable for RLPITR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RLPITR to value 0"]
impl crate::Resettable for RLPITR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
