#[doc = "Register `US_LINIR` reader"]
pub struct R(crate::R<US_LINIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_LINIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_LINIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_LINIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_LINIR` writer"]
pub struct W(crate::W<US_LINIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_LINIR_SPEC>;
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
impl From<crate::W<US_LINIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_LINIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDCHR` reader - Identifier Character"]
pub struct IDCHR_R(crate::FieldReader<u8, u8>);
impl IDCHR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IDCHR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDCHR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDCHR` writer - Identifier Character"]
pub struct IDCHR_W<'a> {
    w: &'a mut W,
}
impl<'a> IDCHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Identifier Character"]
    #[inline(always)]
    pub fn idchr(&self) -> IDCHR_R {
        IDCHR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Identifier Character"]
    #[inline(always)]
    pub fn idchr(&mut self) -> IDCHR_W {
        IDCHR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LIN Identifier Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_linir](index.html) module"]
pub struct US_LINIR_SPEC;
impl crate::RegisterSpec for US_LINIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_linir::R](R) reader structure"]
impl crate::Readable for US_LINIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_linir::W](W) writer structure"]
impl crate::Writable for US_LINIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets US_LINIR to value 0"]
impl crate::Resettable for US_LINIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
