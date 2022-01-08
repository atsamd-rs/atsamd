#[doc = "Register `CTRLC` reader"]
pub struct R(crate::R<CTRLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLC` writer"]
pub struct W(crate::W<CTRLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLC_SPEC>;
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
impl From<crate::W<CTRLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INVEN0` reader - Output Waveform 0 Invert Enable"]
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
#[doc = "Field `INVEN0` writer - Output Waveform 0 Invert Enable"]
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
#[doc = "Field `INVEN1` reader - Output Waveform 1 Invert Enable"]
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
#[doc = "Field `INVEN1` writer - Output Waveform 1 Invert Enable"]
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
#[doc = "Field `CPTEN0` reader - Capture Channel 0 Enable"]
pub struct CPTEN0_R(crate::FieldReader<bool, bool>);
impl CPTEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPTEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPTEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPTEN0` writer - Capture Channel 0 Enable"]
pub struct CPTEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CPTEN0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CPTEN1` reader - Capture Channel 1 Enable"]
pub struct CPTEN1_R(crate::FieldReader<bool, bool>);
impl CPTEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPTEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPTEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPTEN1` writer - Capture Channel 1 Enable"]
pub struct CPTEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CPTEN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Output Waveform 0 Invert Enable"]
    #[inline(always)]
    pub fn inven0(&self) -> INVEN0_R {
        INVEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Output Waveform 1 Invert Enable"]
    #[inline(always)]
    pub fn inven1(&self) -> INVEN1_R {
        INVEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Capture Channel 0 Enable"]
    #[inline(always)]
    pub fn cpten0(&self) -> CPTEN0_R {
        CPTEN0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Capture Channel 1 Enable"]
    #[inline(always)]
    pub fn cpten1(&self) -> CPTEN1_R {
        CPTEN1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Waveform 0 Invert Enable"]
    #[inline(always)]
    pub fn inven0(&mut self) -> INVEN0_W {
        INVEN0_W { w: self }
    }
    #[doc = "Bit 1 - Output Waveform 1 Invert Enable"]
    #[inline(always)]
    pub fn inven1(&mut self) -> INVEN1_W {
        INVEN1_W { w: self }
    }
    #[doc = "Bit 4 - Capture Channel 0 Enable"]
    #[inline(always)]
    pub fn cpten0(&mut self) -> CPTEN0_W {
        CPTEN0_W { w: self }
    }
    #[doc = "Bit 5 - Capture Channel 1 Enable"]
    #[inline(always)]
    pub fn cpten1(&mut self) -> CPTEN1_W {
        CPTEN1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlc](index.html) module"]
pub struct CTRLC_SPEC;
impl crate::RegisterSpec for CTRLC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrlc::R](R) reader structure"]
impl crate::Readable for CTRLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlc::W](W) writer structure"]
impl crate::Writable for CTRLC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CTRLC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
