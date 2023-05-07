#[doc = "Register `FNUM` reader"]
pub struct R(crate::R<FNUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FNUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FNUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FNUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FNUM` writer"]
pub struct W(crate::W<FNUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FNUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FNUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FNUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MFNUM` reader - Micro Frame Number"]
pub type MFNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MFNUM` writer - Micro Frame Number"]
pub type MFNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FNUM_SPEC, u8, u8, 3, O>;
#[doc = "Field `FNUM` reader - Frame Number"]
pub type FNUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FNUM` writer - Frame Number"]
pub type FNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FNUM_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:2 - Micro Frame Number"]
    #[inline(always)]
    pub fn mfnum(&self) -> MFNUM_R {
        MFNUM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    pub fn fnum(&self) -> FNUM_R {
        FNUM_R::new((self.bits >> 3) & 0x07ff)
    }
}
impl W {
    #[doc = "Bits 0:2 - Micro Frame Number"]
    #[inline(always)]
    #[must_use]
    pub fn mfnum(&mut self) -> MFNUM_W<0> {
        MFNUM_W::new(self)
    }
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    #[must_use]
    pub fn fnum(&mut self) -> FNUM_W<3> {
        FNUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HOST Host Frame Number\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fnum](index.html) module"]
pub struct FNUM_SPEC;
impl crate::RegisterSpec for FNUM_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fnum::R](R) reader structure"]
impl crate::Readable for FNUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fnum::W](W) writer structure"]
impl crate::Writable for FNUM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FNUM to value 0"]
impl crate::Resettable for FNUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
