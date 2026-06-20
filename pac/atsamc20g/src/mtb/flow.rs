#[doc = "Register `FLOW` reader"]
pub struct R(crate::R<FLOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLOW` writer"]
pub struct W(crate::W<FLOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLOW_SPEC>;
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
impl From<crate::W<FLOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUTOSTOP` reader - Auto Stop Tracing"]
pub struct AUTOSTOP_R(crate::FieldReader<bool, bool>);
impl AUTOSTOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTOSTOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTOSTOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTOSTOP` writer - Auto Stop Tracing"]
pub struct AUTOSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOSTOP_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `AUTOHALT` reader - Auto Halt Request"]
pub struct AUTOHALT_R(crate::FieldReader<bool, bool>);
impl AUTOHALT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTOHALT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTOHALT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTOHALT` writer - Auto Halt Request"]
pub struct AUTOHALT_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOHALT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `WATERMARK` reader - Watermark value"]
pub struct WATERMARK_R(crate::FieldReader<u32, u32>);
impl WATERMARK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        WATERMARK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WATERMARK_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WATERMARK` writer - Watermark value"]
pub struct WATERMARK_W<'a> {
    w: &'a mut W,
}
impl<'a> WATERMARK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | ((value as u32 & 0x1fff_ffff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Auto Stop Tracing"]
    #[inline(always)]
    pub fn autostop(&self) -> AUTOSTOP_R {
        AUTOSTOP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Auto Halt Request"]
    #[inline(always)]
    pub fn autohalt(&self) -> AUTOHALT_R {
        AUTOHALT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 3:31 - Watermark value"]
    #[inline(always)]
    pub fn watermark(&self) -> WATERMARK_R {
        WATERMARK_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - Auto Stop Tracing"]
    #[inline(always)]
    pub fn autostop(&mut self) -> AUTOSTOP_W {
        AUTOSTOP_W { w: self }
    }
    #[doc = "Bit 1 - Auto Halt Request"]
    #[inline(always)]
    pub fn autohalt(&mut self) -> AUTOHALT_W {
        AUTOHALT_W { w: self }
    }
    #[doc = "Bits 3:31 - Watermark value"]
    #[inline(always)]
    pub fn watermark(&mut self) -> WATERMARK_W {
        WATERMARK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MTB Flow\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flow](index.html) module"]
pub struct FLOW_SPEC;
impl crate::RegisterSpec for FLOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flow::R](R) reader structure"]
impl crate::Readable for FLOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flow::W](W) writer structure"]
impl crate::Writable for FLOW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLOW to value 0"]
impl crate::Resettable for FLOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
