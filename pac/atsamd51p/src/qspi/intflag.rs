#[doc = "Register `INTFLAG` reader"]
pub struct R(crate::R<INTFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAG` writer"]
pub struct W(crate::W<INTFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAG_SPEC>;
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
impl From<crate::W<INTFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXC` reader - Receive Data Register Full"]
pub struct RXC_R(crate::FieldReader<bool, bool>);
impl RXC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXC` writer - Receive Data Register Full"]
pub struct RXC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXC_W<'a> {
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
#[doc = "Field `DRE` reader - Transmit Data Register Empty"]
pub struct DRE_R(crate::FieldReader<bool, bool>);
impl DRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRE` writer - Transmit Data Register Empty"]
pub struct DRE_W<'a> {
    w: &'a mut W,
}
impl<'a> DRE_W<'a> {
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
#[doc = "Field `TXC` reader - Transmission Complete"]
pub struct TXC_R(crate::FieldReader<bool, bool>);
impl TXC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXC` writer - Transmission Complete"]
pub struct TXC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `ERROR` reader - Overrun Error"]
pub struct ERROR_R(crate::FieldReader<bool, bool>);
impl ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROR` writer - Overrun Error"]
pub struct ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CSRISE` reader - Chip Select Rise"]
pub struct CSRISE_R(crate::FieldReader<bool, bool>);
impl CSRISE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSRISE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSRISE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSRISE` writer - Chip Select Rise"]
pub struct CSRISE_W<'a> {
    w: &'a mut W,
}
impl<'a> CSRISE_W<'a> {
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
#[doc = "Field `INSTREND` reader - Instruction End"]
pub struct INSTREND_R(crate::FieldReader<bool, bool>);
impl INSTREND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INSTREND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INSTREND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INSTREND` writer - Instruction End"]
pub struct INSTREND_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTREND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Receive Data Register Full"]
    #[inline(always)]
    pub fn rxc(&self) -> RXC_R {
        RXC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Data Register Empty"]
    #[inline(always)]
    pub fn dre(&self) -> DRE_R {
        DRE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmission Complete"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Overrun Error"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Chip Select Rise"]
    #[inline(always)]
    pub fn csrise(&self) -> CSRISE_R {
        CSRISE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Instruction End"]
    #[inline(always)]
    pub fn instrend(&self) -> INSTREND_R {
        INSTREND_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Data Register Full"]
    #[inline(always)]
    pub fn rxc(&mut self) -> RXC_W {
        RXC_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Data Register Empty"]
    #[inline(always)]
    pub fn dre(&mut self) -> DRE_W {
        DRE_W { w: self }
    }
    #[doc = "Bit 2 - Transmission Complete"]
    #[inline(always)]
    pub fn txc(&mut self) -> TXC_W {
        TXC_W { w: self }
    }
    #[doc = "Bit 3 - Overrun Error"]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W {
        ERROR_W { w: self }
    }
    #[doc = "Bit 8 - Chip Select Rise"]
    #[inline(always)]
    pub fn csrise(&mut self) -> CSRISE_W {
        CSRISE_W { w: self }
    }
    #[doc = "Bit 10 - Instruction End"]
    #[inline(always)]
    pub fn instrend(&mut self) -> INSTREND_W {
        INSTREND_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflag](index.html) module"]
pub struct INTFLAG_SPEC;
impl crate::RegisterSpec for INTFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intflag::R](R) reader structure"]
impl crate::Readable for INTFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflag::W](W) writer structure"]
impl crate::Writable for INTFLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for INTFLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
