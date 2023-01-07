#[doc = "Register `DRVCTRL` reader"]
pub struct R(crate::R<DRVCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DRVCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DRVCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DRVCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DRVCTRL` writer"]
pub struct W(crate::W<DRVCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DRVCTRL_SPEC>;
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
impl From<crate::W<DRVCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DRVCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INVEN0` reader - Output Waveform Invert Enable 0"]
pub struct INVEN0_R(crate::FieldReader<bool, bool>);
impl INVEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVEN0` writer - Output Waveform Invert Enable 0"]
pub struct INVEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEN0_W<'a> {
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
#[doc = "Field `INVEN1` reader - Output Waveform Invert Enable 1"]
pub struct INVEN1_R(crate::FieldReader<bool, bool>);
impl INVEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVEN1` writer - Output Waveform Invert Enable 1"]
pub struct INVEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEN1_W<'a> {
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
    #[doc = "Bit 0 - Output Waveform Invert Enable 0"]
    #[inline(always)]
    pub fn inven0(&self) -> INVEN0_R {
        INVEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Output Waveform Invert Enable 1"]
    #[inline(always)]
    pub fn inven1(&self) -> INVEN1_R {
        INVEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Waveform Invert Enable 0"]
    #[inline(always)]
    pub fn inven0(&mut self) -> INVEN0_W {
        INVEN0_W { w: self }
    }
    #[doc = "Bit 1 - Output Waveform Invert Enable 1"]
    #[inline(always)]
    pub fn inven1(&mut self) -> INVEN1_W {
        INVEN1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drvctrl](index.html) module"]
pub struct DRVCTRL_SPEC;
impl crate::RegisterSpec for DRVCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [drvctrl::R](R) reader structure"]
impl crate::Readable for DRVCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [drvctrl::W](W) writer structure"]
impl crate::Writable for DRVCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DRVCTRL to value 0"]
impl crate::Resettable for DRVCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
