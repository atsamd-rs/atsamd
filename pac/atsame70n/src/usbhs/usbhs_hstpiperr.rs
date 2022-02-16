#[doc = "Register `USBHS_HSTPIPERR[%s]` reader"]
pub struct R(crate::R<USBHS_HSTPIPERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBHS_HSTPIPERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBHS_HSTPIPERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBHS_HSTPIPERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBHS_HSTPIPERR[%s]` writer"]
pub struct W(crate::W<USBHS_HSTPIPERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_HSTPIPERR_SPEC>;
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
impl From<crate::W<USBHS_HSTPIPERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_HSTPIPERR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATATGL` reader - Data Toggle Error"]
pub struct DATATGL_R(crate::FieldReader<bool, bool>);
impl DATATGL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATATGL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATATGL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATATGL` writer - Data Toggle Error"]
pub struct DATATGL_W<'a> {
    w: &'a mut W,
}
impl<'a> DATATGL_W<'a> {
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
#[doc = "Field `DATAPID` reader - Data PID Error"]
pub struct DATAPID_R(crate::FieldReader<bool, bool>);
impl DATAPID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATAPID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAPID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAPID` writer - Data PID Error"]
pub struct DATAPID_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAPID_W<'a> {
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
#[doc = "Field `PID` reader - Data PID Error"]
pub struct PID_R(crate::FieldReader<bool, bool>);
impl PID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID` writer - Data PID Error"]
pub struct PID_W<'a> {
    w: &'a mut W,
}
impl<'a> PID_W<'a> {
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
#[doc = "Field `TIMEOUT` reader - Time-Out Error"]
pub struct TIMEOUT_R(crate::FieldReader<bool, bool>);
impl TIMEOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMEOUT` writer - Time-Out Error"]
pub struct TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_W<'a> {
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
#[doc = "Field `CRC16` reader - CRC16 Error"]
pub struct CRC16_R(crate::FieldReader<bool, bool>);
impl CRC16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRC16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC16` writer - CRC16 Error"]
pub struct CRC16_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC16_W<'a> {
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
#[doc = "Field `COUNTER` reader - Error Counter"]
pub struct COUNTER_R(crate::FieldReader<u8, u8>);
impl COUNTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COUNTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNTER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNTER` writer - Error Counter"]
pub struct COUNTER_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Data Toggle Error"]
    #[inline(always)]
    pub fn datatgl(&self) -> DATATGL_R {
        DATATGL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data PID Error"]
    #[inline(always)]
    pub fn datapid(&self) -> DATAPID_R {
        DATAPID_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Data PID Error"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Time-Out Error"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRC16 Error"]
    #[inline(always)]
    pub fn crc16(&self) -> CRC16_R {
        CRC16_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Error Counter"]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new(((self.bits >> 5) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Data Toggle Error"]
    #[inline(always)]
    pub fn datatgl(&mut self) -> DATATGL_W {
        DATATGL_W { w: self }
    }
    #[doc = "Bit 1 - Data PID Error"]
    #[inline(always)]
    pub fn datapid(&mut self) -> DATAPID_W {
        DATAPID_W { w: self }
    }
    #[doc = "Bit 2 - Data PID Error"]
    #[inline(always)]
    pub fn pid(&mut self) -> PID_W {
        PID_W { w: self }
    }
    #[doc = "Bit 3 - Time-Out Error"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W { w: self }
    }
    #[doc = "Bit 4 - CRC16 Error"]
    #[inline(always)]
    pub fn crc16(&mut self) -> CRC16_W {
        CRC16_W { w: self }
    }
    #[doc = "Bits 5:6 - Error Counter"]
    #[inline(always)]
    pub fn counter(&mut self) -> COUNTER_W {
        COUNTER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Pipe Error Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpiperr](index.html) module"]
pub struct USBHS_HSTPIPERR_SPEC;
impl crate::RegisterSpec for USBHS_HSTPIPERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbhs_hstpiperr::R](R) reader structure"]
impl crate::Readable for USBHS_HSTPIPERR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbhs_hstpiperr::W](W) writer structure"]
impl crate::Writable for USBHS_HSTPIPERR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_HSTPIPERR[%s]
to value 0"]
impl crate::Resettable for USBHS_HSTPIPERR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
