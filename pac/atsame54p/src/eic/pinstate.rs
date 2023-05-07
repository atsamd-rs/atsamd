#[doc = "Register `PINSTATE` reader"]
pub struct R(crate::R<PINSTATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINSTATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINSTATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINSTATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PINSTATE` reader - Pin State"]
pub type PINSTATE_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Pin State"]
    #[inline(always)]
    pub fn pinstate(&self) -> PINSTATE_R {
        PINSTATE_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Pin State\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinstate](index.html) module"]
pub struct PINSTATE_SPEC;
impl crate::RegisterSpec for PINSTATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinstate::R](R) reader structure"]
impl crate::Readable for PINSTATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PINSTATE to value 0"]
impl crate::Resettable for PINSTATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
