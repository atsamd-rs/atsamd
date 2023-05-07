#[doc = "Register `HSDIV` reader"]
pub struct R(crate::R<HSDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIV` reader - CPU Clock Division Factor"]
pub type DIV_R = crate::FieldReader<u8, DIVSELECT_A>;
#[doc = "CPU Clock Division Factor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVSELECT_A {
    #[doc = "1: Divide by 1"]
    DIV1 = 1,
}
impl From<DIVSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVSELECT_A) -> Self {
        variant as _
    }
}
impl DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIVSELECT_A> {
        match self.bits {
            1 => Some(DIVSELECT_A::DIV1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == DIVSELECT_A::DIV1
    }
}
impl R {
    #[doc = "Bits 0:7 - CPU Clock Division Factor"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(self.bits)
    }
}
#[doc = "HS Clock Division\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsdiv](index.html) module"]
pub struct HSDIV_SPEC;
impl crate::RegisterSpec for HSDIV_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [hsdiv::R](R) reader structure"]
impl crate::Readable for HSDIV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HSDIV to value 0x01"]
impl crate::Resettable for HSDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
