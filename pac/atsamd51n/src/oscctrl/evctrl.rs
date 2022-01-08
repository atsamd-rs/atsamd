#[doc = "Register `EVCTRL` reader"]
pub struct R(crate::R<EVCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVCTRL` writer"]
pub struct W(crate::W<EVCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVCTRL_SPEC>;
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
impl From<crate::W<EVCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFDEO0` reader - Clock 0 Failure Detector Event Output Enable"]
pub struct CFDEO0_R(crate::FieldReader<bool, bool>);
impl CFDEO0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFDEO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFDEO0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFDEO0` writer - Clock 0 Failure Detector Event Output Enable"]
pub struct CFDEO0_W<'a> {
    w: &'a mut W,
}
impl<'a> CFDEO0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `CFDEO1` reader - Clock 1 Failure Detector Event Output Enable"]
pub struct CFDEO1_R(crate::FieldReader<bool, bool>);
impl CFDEO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFDEO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFDEO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFDEO1` writer - Clock 1 Failure Detector Event Output Enable"]
pub struct CFDEO1_W<'a> {
    w: &'a mut W,
}
impl<'a> CFDEO1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clock 0 Failure Detector Event Output Enable"]
    #[inline(always)]
    pub fn cfdeo0(&self) -> CFDEO0_R {
        CFDEO0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock 1 Failure Detector Event Output Enable"]
    #[inline(always)]
    pub fn cfdeo1(&self) -> CFDEO1_R {
        CFDEO1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock 0 Failure Detector Event Output Enable"]
    #[inline(always)]
    pub fn cfdeo0(&mut self) -> CFDEO0_W {
        CFDEO0_W { w: self }
    }
    #[doc = "Bit 1 - Clock 1 Failure Detector Event Output Enable"]
    #[inline(always)]
    pub fn cfdeo1(&mut self) -> CFDEO1_W {
        CFDEO1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evctrl](index.html) module"]
pub struct EVCTRL_SPEC;
impl crate::RegisterSpec for EVCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [evctrl::R](R) reader structure"]
impl crate::Readable for EVCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evctrl::W](W) writer structure"]
impl crate::Writable for EVCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EVCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
