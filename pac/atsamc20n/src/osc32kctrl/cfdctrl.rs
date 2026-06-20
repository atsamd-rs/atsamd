#[doc = "Register `CFDCTRL` reader"]
pub struct R(crate::R<CFDCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDCTRL` writer"]
pub struct W(crate::W<CFDCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDCTRL_SPEC>;
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
impl From<crate::W<CFDCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFDEN` reader - Clock Failure Detector Enable"]
pub struct CFDEN_R(crate::FieldReader<bool, bool>);
impl CFDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFDEN` writer - Clock Failure Detector Enable"]
pub struct CFDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CFDEN_W<'a> {
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
#[doc = "Field `SWBACK` reader - Clock Switch Back"]
pub struct SWBACK_R(crate::FieldReader<bool, bool>);
impl SWBACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWBACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWBACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWBACK` writer - Clock Switch Back"]
pub struct SWBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> SWBACK_W<'a> {
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
#[doc = "Field `CFDPRESC` reader - Clock Failure Detector Prescaler"]
pub struct CFDPRESC_R(crate::FieldReader<bool, bool>);
impl CFDPRESC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFDPRESC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFDPRESC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFDPRESC` writer - Clock Failure Detector Prescaler"]
pub struct CFDPRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> CFDPRESC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CFDEN_R {
        CFDEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock Switch Back"]
    #[inline(always)]
    pub fn swback(&self) -> SWBACK_R {
        SWBACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clock Failure Detector Prescaler"]
    #[inline(always)]
    pub fn cfdpresc(&self) -> CFDPRESC_R {
        CFDPRESC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&mut self) -> CFDEN_W {
        CFDEN_W { w: self }
    }
    #[doc = "Bit 1 - Clock Switch Back"]
    #[inline(always)]
    pub fn swback(&mut self) -> SWBACK_W {
        SWBACK_W { w: self }
    }
    #[doc = "Bit 2 - Clock Failure Detector Prescaler"]
    #[inline(always)]
    pub fn cfdpresc(&mut self) -> CFDPRESC_W {
        CFDPRESC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Failure Detector Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdctrl](index.html) module"]
pub struct CFDCTRL_SPEC;
impl crate::RegisterSpec for CFDCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cfdctrl::R](R) reader structure"]
impl crate::Readable for CFDCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdctrl::W](W) writer structure"]
impl crate::Writable for CFDCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFDCTRL to value 0"]
impl crate::Resettable for CFDCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
