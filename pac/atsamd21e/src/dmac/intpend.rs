#[doc = "Register `INTPEND` reader"]
pub struct R(crate::R<INTPEND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTPEND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTPEND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTPEND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTPEND` writer"]
pub struct W(crate::W<INTPEND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTPEND_SPEC>;
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
impl From<crate::W<INTPEND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTPEND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID` reader - Channel ID"]
pub struct ID_R(crate::FieldReader<u8, u8>);
impl ID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ID` writer - Channel ID"]
pub struct ID_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u16 & 0x0f);
        self.w
    }
}
#[doc = "Field `TERR` reader - Transfer Error"]
pub struct TERR_R(crate::FieldReader<bool, bool>);
impl TERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TERR` writer - Transfer Error"]
pub struct TERR_W<'a> {
    w: &'a mut W,
}
impl<'a> TERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `TCMPL` reader - Transfer Complete"]
pub struct TCMPL_R(crate::FieldReader<bool, bool>);
impl TCMPL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCMPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCMPL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCMPL` writer - Transfer Complete"]
pub struct TCMPL_W<'a> {
    w: &'a mut W,
}
impl<'a> TCMPL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u16 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `SUSP` reader - Channel Suspend"]
pub struct SUSP_R(crate::FieldReader<bool, bool>);
impl SUSP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSP` writer - Channel Suspend"]
pub struct SUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u16 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `FERR` reader - Fetch Error"]
pub struct FERR_R(crate::FieldReader<bool, bool>);
impl FERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSY` reader - Busy"]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEND` reader - Pending"]
pub struct PEND_R(crate::FieldReader<bool, bool>);
impl PEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Channel ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Transfer Error"]
    #[inline(always)]
    pub fn terr(&self) -> TERR_R {
        TERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transfer Complete"]
    #[inline(always)]
    pub fn tcmpl(&self) -> TCMPL_R {
        TCMPL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel Suspend"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Fetch Error"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Pending"]
    #[inline(always)]
    pub fn pend(&self) -> PEND_R {
        PEND_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel ID"]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W { w: self }
    }
    #[doc = "Bit 8 - Transfer Error"]
    #[inline(always)]
    pub fn terr(&mut self) -> TERR_W {
        TERR_W { w: self }
    }
    #[doc = "Bit 9 - Transfer Complete"]
    #[inline(always)]
    pub fn tcmpl(&mut self) -> TCMPL_W {
        TCMPL_W { w: self }
    }
    #[doc = "Bit 10 - Channel Suspend"]
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W {
        SUSP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Pending\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intpend](index.html) module"]
pub struct INTPEND_SPEC;
impl crate::RegisterSpec for INTPEND_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [intpend::R](R) reader structure"]
impl crate::Readable for INTPEND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intpend::W](W) writer structure"]
impl crate::Writable for INTPEND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTPEND to value 0"]
impl crate::Resettable for INTPEND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
