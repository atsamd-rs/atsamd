#[doc = "Register `DFLLVAL` reader"]
pub struct R(crate::R<DFLLVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFLLVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFLLVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFLLVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFLLVAL` writer"]
pub struct W(crate::W<DFLLVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFLLVAL_SPEC>;
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
impl From<crate::W<DFLLVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFLLVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FINE` reader - Fine Value"]
pub struct FINE_R(crate::FieldReader<u16, u16>);
impl FINE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FINE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FINE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FINE` writer - Fine Value"]
pub struct FINE_W<'a> {
    w: &'a mut W,
}
impl<'a> FINE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `COARSE` reader - Coarse Value"]
pub struct COARSE_R(crate::FieldReader<u8, u8>);
impl COARSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COARSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COARSE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COARSE` writer - Coarse Value"]
pub struct COARSE_W<'a> {
    w: &'a mut W,
}
impl<'a> COARSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | ((value as u32 & 0x3f) << 10);
        self.w
    }
}
#[doc = "Field `DIFF` reader - Multiplication Ratio Difference"]
pub struct DIFF_R(crate::FieldReader<u16, u16>);
impl DIFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DIFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFF_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:9 - Fine Value"]
    #[inline(always)]
    pub fn fine(&self) -> FINE_R {
        FINE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - Coarse Value"]
    #[inline(always)]
    pub fn coarse(&self) -> COARSE_R {
        COARSE_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - Multiplication Ratio Difference"]
    #[inline(always)]
    pub fn diff(&self) -> DIFF_R {
        DIFF_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Fine Value"]
    #[inline(always)]
    pub fn fine(&mut self) -> FINE_W {
        FINE_W { w: self }
    }
    #[doc = "Bits 10:15 - Coarse Value"]
    #[inline(always)]
    pub fn coarse(&mut self) -> COARSE_W {
        COARSE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFLL48M Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllval](index.html) module"]
pub struct DFLLVAL_SPEC;
impl crate::RegisterSpec for DFLLVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfllval::R](R) reader structure"]
impl crate::Readable for DFLLVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfllval::W](W) writer structure"]
impl crate::Writable for DFLLVAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFLLVAL to value 0"]
impl crate::Resettable for DFLLVAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
