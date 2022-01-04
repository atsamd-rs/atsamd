#[doc = "Register `BINTERVAL` reader"]
pub struct R(crate::R<BINTERVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BINTERVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BINTERVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BINTERVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BINTERVAL` writer"]
pub struct W(crate::W<BINTERVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BINTERVAL_SPEC>;
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
impl From<crate::W<BINTERVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BINTERVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BITINTERVAL` reader - Bit Interval"]
pub struct BITINTERVAL_R(crate::FieldReader<u8, u8>);
impl BITINTERVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BITINTERVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BITINTERVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BITINTERVAL` writer - Bit Interval"]
pub struct BITINTERVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> BITINTERVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value as u8;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Bit Interval"]
    #[inline(always)]
    pub fn bitinterval(&self) -> BITINTERVAL_R {
        BITINTERVAL_R::new(self.bits as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bit Interval"]
    #[inline(always)]
    pub fn bitinterval(&mut self) -> BITINTERVAL_W {
        BITINTERVAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HOST_PIPE Bus Access Period of Pipe\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [binterval](index.html) module"]
pub struct BINTERVAL_SPEC;
impl crate::RegisterSpec for BINTERVAL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [binterval::R](R) reader structure"]
impl crate::Readable for BINTERVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [binterval::W](W) writer structure"]
impl crate::Writable for BINTERVAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BINTERVAL to value 0"]
impl crate::Resettable for BINTERVAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
