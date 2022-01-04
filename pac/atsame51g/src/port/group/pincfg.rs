#[doc = "Register `PINCFG[%s]` reader"]
pub struct R(crate::R<PINCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINCFG[%s]` writer"]
pub struct W(crate::W<PINCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINCFG_SPEC>;
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
impl From<crate::W<PINCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMUXEN` reader - Peripheral Multiplexer Enable"]
pub struct PMUXEN_R(crate::FieldReader<bool, bool>);
impl PMUXEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PMUXEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMUXEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMUXEN` writer - Peripheral Multiplexer Enable"]
pub struct PMUXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PMUXEN_W<'a> {
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
#[doc = "Field `INEN` reader - Input Enable"]
pub struct INEN_R(crate::FieldReader<bool, bool>);
impl INEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INEN` writer - Input Enable"]
pub struct INEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INEN_W<'a> {
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
#[doc = "Field `PULLEN` reader - Pull Enable"]
pub struct PULLEN_R(crate::FieldReader<bool, bool>);
impl PULLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PULLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULLEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULLEN` writer - Pull Enable"]
pub struct PULLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PULLEN_W<'a> {
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
#[doc = "Field `DRVSTR` reader - Output Driver Strength Selection"]
pub struct DRVSTR_R(crate::FieldReader<bool, bool>);
impl DRVSTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DRVSTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRVSTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRVSTR` writer - Output Driver Strength Selection"]
pub struct DRVSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> DRVSTR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Peripheral Multiplexer Enable"]
    #[inline(always)]
    pub fn pmuxen(&self) -> PMUXEN_R {
        PMUXEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Input Enable"]
    #[inline(always)]
    pub fn inen(&self) -> INEN_R {
        INEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pull Enable"]
    #[inline(always)]
    pub fn pullen(&self) -> PULLEN_R {
        PULLEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Output Driver Strength Selection"]
    #[inline(always)]
    pub fn drvstr(&self) -> DRVSTR_R {
        DRVSTR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral Multiplexer Enable"]
    #[inline(always)]
    pub fn pmuxen(&mut self) -> PMUXEN_W {
        PMUXEN_W { w: self }
    }
    #[doc = "Bit 1 - Input Enable"]
    #[inline(always)]
    pub fn inen(&mut self) -> INEN_W {
        INEN_W { w: self }
    }
    #[doc = "Bit 2 - Pull Enable"]
    #[inline(always)]
    pub fn pullen(&mut self) -> PULLEN_W {
        PULLEN_W { w: self }
    }
    #[doc = "Bit 6 - Output Driver Strength Selection"]
    #[inline(always)]
    pub fn drvstr(&mut self) -> DRVSTR_W {
        DRVSTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pincfg](index.html) module"]
pub struct PINCFG_SPEC;
impl crate::RegisterSpec for PINCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pincfg::R](R) reader structure"]
impl crate::Readable for PINCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pincfg::W](W) writer structure"]
impl crate::Writable for PINCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PINCFG[%s]
to value 0"]
impl crate::Resettable for PINCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
