#[doc = "Register `PMUX[%s]` reader"]
pub struct R(crate::R<PMUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMUX[%s]` writer"]
pub struct W(crate::W<PMUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMUX_SPEC>;
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
impl From<crate::W<PMUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMUXE` reader - Peripheral Multiplexing for Even-Numbered Pin"]
pub struct PMUXE_R(crate::FieldReader<u8, u8>);
impl PMUXE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PMUXE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMUXE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMUXE` writer - Peripheral Multiplexing for Even-Numbered Pin"]
pub struct PMUXE_W<'a> {
    w: &'a mut W,
}
impl<'a> PMUXE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
        self.w
    }
}
#[doc = "Field `PMUXO` reader - Peripheral Multiplexing for Odd-Numbered Pin"]
pub struct PMUXO_R(crate::FieldReader<u8, u8>);
impl PMUXO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PMUXO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMUXO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMUXO` writer - Peripheral Multiplexing for Odd-Numbered Pin"]
pub struct PMUXO_W<'a> {
    w: &'a mut W,
}
impl<'a> PMUXO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u8 & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Peripheral Multiplexing for Even-Numbered Pin"]
    #[inline(always)]
    pub fn pmuxe(&self) -> PMUXE_R {
        PMUXE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Peripheral Multiplexing for Odd-Numbered Pin"]
    #[inline(always)]
    pub fn pmuxo(&self) -> PMUXO_R {
        PMUXO_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Peripheral Multiplexing for Even-Numbered Pin"]
    #[inline(always)]
    pub fn pmuxe(&mut self) -> PMUXE_W {
        PMUXE_W { w: self }
    }
    #[doc = "Bits 4:7 - Peripheral Multiplexing for Odd-Numbered Pin"]
    #[inline(always)]
    pub fn pmuxo(&mut self) -> PMUXO_W {
        PMUXO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Multiplexing\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmux](index.html) module"]
pub struct PMUX_SPEC;
impl crate::RegisterSpec for PMUX_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pmux::R](R) reader structure"]
impl crate::Readable for PMUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmux::W](W) writer structure"]
impl crate::Writable for PMUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMUX[%s]
to value 0"]
impl crate::Resettable for PMUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
