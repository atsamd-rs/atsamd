#[doc = "Register `ANACTRL` reader"]
pub struct R(crate::R<ANACTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANACTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANACTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANACTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANACTRL` writer"]
pub struct W(crate::W<ANACTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANACTRL_SPEC>;
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
impl From<crate::W<ANACTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANACTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTRSDADC` reader - SDADC Control"]
pub struct CTRSDADC_R(crate::FieldReader<u8, u8>);
impl CTRSDADC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CTRSDADC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRSDADC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRSDADC` writer - SDADC Control"]
pub struct CTRSDADC_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRSDADC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u8 & 0x3f);
        self.w
    }
}
#[doc = "Field `ONCHOP` reader - Chopper"]
pub struct ONCHOP_R(crate::FieldReader<bool, bool>);
impl ONCHOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ONCHOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ONCHOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONCHOP` writer - Chopper"]
pub struct ONCHOP_W<'a> {
    w: &'a mut W,
}
impl<'a> ONCHOP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `BUFTEST` reader - BUFTEST"]
pub struct BUFTEST_R(crate::FieldReader<bool, bool>);
impl BUFTEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUFTEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUFTEST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUFTEST` writer - BUFTEST"]
pub struct BUFTEST_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFTEST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - SDADC Control"]
    #[inline(always)]
    pub fn ctrsdadc(&self) -> CTRSDADC_R {
        CTRSDADC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Chopper"]
    #[inline(always)]
    pub fn onchop(&self) -> ONCHOP_R {
        ONCHOP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - BUFTEST"]
    #[inline(always)]
    pub fn buftest(&self) -> BUFTEST_R {
        BUFTEST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - SDADC Control"]
    #[inline(always)]
    pub fn ctrsdadc(&mut self) -> CTRSDADC_W {
        CTRSDADC_W { w: self }
    }
    #[doc = "Bit 6 - Chopper"]
    #[inline(always)]
    pub fn onchop(&mut self) -> ONCHOP_W {
        ONCHOP_W { w: self }
    }
    #[doc = "Bit 7 - BUFTEST"]
    #[inline(always)]
    pub fn buftest(&mut self) -> BUFTEST_W {
        BUFTEST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [anactrl](index.html) module"]
pub struct ANACTRL_SPEC;
impl crate::RegisterSpec for ANACTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [anactrl::R](R) reader structure"]
impl crate::Readable for ANACTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [anactrl::W](W) writer structure"]
impl crate::Writable for ANACTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ANACTRL to value 0"]
impl crate::Resettable for ANACTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
