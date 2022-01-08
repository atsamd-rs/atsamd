#[doc = "Register `FFCR` reader"]
pub struct R(crate::R<FFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FFCR` writer"]
pub struct W(crate::W<FFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFCR_SPEC>;
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
impl From<crate::W<FFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EnFCont` reader - "]
pub struct ENFCONT_R(crate::FieldReader<bool, bool>);
impl ENFCONT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENFCONT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENFCONT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EnFCont` writer - "]
pub struct ENFCONT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENFCONT_W<'a> {
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
#[doc = "Field `TrigIn` reader - "]
pub struct TRIGIN_R(crate::FieldReader<bool, bool>);
impl TRIGIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIGIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TrigIn` writer - "]
pub struct TRIGIN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGIN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn en_fcont(&self) -> ENFCONT_R {
        ENFCONT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn trig_in(&self) -> TRIGIN_R {
        TRIGIN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn en_fcont(&mut self) -> ENFCONT_W {
        ENFCONT_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn trig_in(&mut self) -> TRIGIN_W {
        TRIGIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Formatter and Flush Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffcr](index.html) module"]
pub struct FFCR_SPEC;
impl crate::RegisterSpec for FFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffcr::R](R) reader structure"]
impl crate::Readable for FFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ffcr::W](W) writer structure"]
impl crate::Writable for FFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FFCR to value 0"]
impl crate::Resettable for FFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
