#[doc = "Register `SQRNUM` reader"]
pub struct R(crate::R<SQRNUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SQRNUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SQRNUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SQRNUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SQRNUM` writer"]
pub struct W(crate::W<SQRNUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SQRNUM_SPEC>;
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
impl From<crate::W<SQRNUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SQRNUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SQRNUM` reader - Square Root Input"]
pub struct SQRNUM_R(crate::FieldReader<u32, u32>);
impl SQRNUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SQRNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQRNUM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQRNUM` writer - Square Root Input"]
pub struct SQRNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SQRNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Square Root Input"]
    #[inline(always)]
    pub fn sqrnum(&self) -> SQRNUM_R {
        SQRNUM_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Square Root Input"]
    #[inline(always)]
    pub fn sqrnum(&mut self) -> SQRNUM_W {
        SQRNUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Square Root Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sqrnum](index.html) module"]
pub struct SQRNUM_SPEC;
impl crate::RegisterSpec for SQRNUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sqrnum::R](R) reader structure"]
impl crate::Readable for SQRNUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sqrnum::W](W) writer structure"]
impl crate::Writable for SQRNUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SQRNUM to value 0"]
impl crate::Resettable for SQRNUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
