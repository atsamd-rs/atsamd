#[doc = "Register `UTMI_OHCIICR` reader"]
pub struct R(crate::R<UTMI_OHCIICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UTMI_OHCIICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UTMI_OHCIICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UTMI_OHCIICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UTMI_OHCIICR` writer"]
pub struct W(crate::W<UTMI_OHCIICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UTMI_OHCIICR_SPEC>;
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
impl From<crate::W<UTMI_OHCIICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UTMI_OHCIICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RES0` reader - USB PORTx Reset"]
pub struct RES0_R(crate::FieldReader<bool, bool>);
impl RES0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RES0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RES0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RES0` writer - USB PORTx Reset"]
pub struct RES0_W<'a> {
    w: &'a mut W,
}
impl<'a> RES0_W<'a> {
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
#[doc = "Field `ARIE` reader - OHCI Asynchronous Resume Interrupt Enable"]
pub struct ARIE_R(crate::FieldReader<bool, bool>);
impl ARIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARIE` writer - OHCI Asynchronous Resume Interrupt Enable"]
pub struct ARIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `APPSTART` reader - "]
pub struct APPSTART_R(crate::FieldReader<bool, bool>);
impl APPSTART_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APPSTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APPSTART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APPSTART` writer - "]
pub struct APPSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> APPSTART_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `UDPPUDIS` reader - USB Device Pull-up Disable"]
pub struct UDPPUDIS_R(crate::FieldReader<bool, bool>);
impl UDPPUDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UDPPUDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UDPPUDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UDPPUDIS` writer - USB Device Pull-up Disable"]
pub struct UDPPUDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> UDPPUDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USB PORTx Reset"]
    #[inline(always)]
    pub fn res0(&self) -> RES0_R {
        RES0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - OHCI Asynchronous Resume Interrupt Enable"]
    #[inline(always)]
    pub fn arie(&self) -> ARIE_R {
        ARIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn appstart(&self) -> APPSTART_R {
        APPSTART_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 23 - USB Device Pull-up Disable"]
    #[inline(always)]
    pub fn udppudis(&self) -> UDPPUDIS_R {
        UDPPUDIS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB PORTx Reset"]
    #[inline(always)]
    pub fn res0(&mut self) -> RES0_W {
        RES0_W { w: self }
    }
    #[doc = "Bit 4 - OHCI Asynchronous Resume Interrupt Enable"]
    #[inline(always)]
    pub fn arie(&mut self) -> ARIE_W {
        ARIE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn appstart(&mut self) -> APPSTART_W {
        APPSTART_W { w: self }
    }
    #[doc = "Bit 23 - USB Device Pull-up Disable"]
    #[inline(always)]
    pub fn udppudis(&mut self) -> UDPPUDIS_W {
        UDPPUDIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OHCI Interrupt Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [utmi_ohciicr](index.html) module"]
pub struct UTMI_OHCIICR_SPEC;
impl crate::RegisterSpec for UTMI_OHCIICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [utmi_ohciicr::R](R) reader structure"]
impl crate::Readable for UTMI_OHCIICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [utmi_ohciicr::W](W) writer structure"]
impl crate::Writable for UTMI_OHCIICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UTMI_OHCIICR to value 0"]
impl crate::Resettable for UTMI_OHCIICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
