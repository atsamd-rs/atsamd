#[doc = "Register `US_IF` reader"]
pub struct R(crate::R<US_IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_IF` writer"]
pub struct W(crate::W<US_IF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_IF_SPEC>;
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
impl From<crate::W<US_IF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_IF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRDA_FILTER` reader - IrDA Filter"]
pub struct IRDA_FILTER_R(crate::FieldReader<u8, u8>);
impl IRDA_FILTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IRDA_FILTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRDA_FILTER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRDA_FILTER` writer - IrDA Filter"]
pub struct IRDA_FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> IRDA_FILTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - IrDA Filter"]
    #[inline(always)]
    pub fn irda_filter(&self) -> IRDA_FILTER_R {
        IRDA_FILTER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IrDA Filter"]
    #[inline(always)]
    pub fn irda_filter(&mut self) -> IRDA_FILTER_W {
        IRDA_FILTER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IrDA Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_if](index.html) module"]
pub struct US_IF_SPEC;
impl crate::RegisterSpec for US_IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_if::R](R) reader structure"]
impl crate::Readable for US_IF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_if::W](W) writer structure"]
impl crate::Writable for US_IF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets US_IF to value 0"]
impl crate::Resettable for US_IF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
