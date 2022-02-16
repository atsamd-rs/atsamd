#[doc = "Register `RTT_MR` reader"]
pub struct R(crate::R<RTT_MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTT_MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTT_MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTT_MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTT_MR` writer"]
pub struct W(crate::W<RTT_MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTT_MR_SPEC>;
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
impl From<crate::W<RTT_MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTT_MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTPRES` reader - Real-time Timer Prescaler Value"]
pub struct RTPRES_R(crate::FieldReader<u16, u16>);
impl RTPRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RTPRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTPRES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTPRES` writer - Real-time Timer Prescaler Value"]
pub struct RTPRES_W<'a> {
    w: &'a mut W,
}
impl<'a> RTPRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `ALMIEN` reader - Alarm Interrupt Enable"]
pub struct ALMIEN_R(crate::FieldReader<bool, bool>);
impl ALMIEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALMIEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALMIEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALMIEN` writer - Alarm Interrupt Enable"]
pub struct ALMIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALMIEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `RTTINCIEN` reader - Real-time Timer Increment Interrupt Enable"]
pub struct RTTINCIEN_R(crate::FieldReader<bool, bool>);
impl RTTINCIEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTTINCIEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTTINCIEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTTINCIEN` writer - Real-time Timer Increment Interrupt Enable"]
pub struct RTTINCIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTTINCIEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `RTTRST` reader - Real-time Timer Restart"]
pub struct RTTRST_R(crate::FieldReader<bool, bool>);
impl RTTRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTTRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTTRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTTRST` writer - Real-time Timer Restart"]
pub struct RTTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RTTRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `RTTDIS` reader - Real-time Timer Disable"]
pub struct RTTDIS_R(crate::FieldReader<bool, bool>);
impl RTTDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTTDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTTDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTTDIS` writer - Real-time Timer Disable"]
pub struct RTTDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTTDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `RTC1HZ` reader - Real-Time Clock 1Hz Clock Selection"]
pub struct RTC1HZ_R(crate::FieldReader<bool, bool>);
impl RTC1HZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC1HZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC1HZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC1HZ` writer - Real-Time Clock 1Hz Clock Selection"]
pub struct RTC1HZ_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC1HZ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Real-time Timer Prescaler Value"]
    #[inline(always)]
    pub fn rtpres(&self) -> RTPRES_R {
        RTPRES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Alarm Interrupt Enable"]
    #[inline(always)]
    pub fn almien(&self) -> ALMIEN_R {
        ALMIEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Real-time Timer Increment Interrupt Enable"]
    #[inline(always)]
    pub fn rttincien(&self) -> RTTINCIEN_R {
        RTTINCIEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Real-time Timer Restart"]
    #[inline(always)]
    pub fn rttrst(&self) -> RTTRST_R {
        RTTRST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Real-time Timer Disable"]
    #[inline(always)]
    pub fn rttdis(&self) -> RTTDIS_R {
        RTTDIS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Real-Time Clock 1Hz Clock Selection"]
    #[inline(always)]
    pub fn rtc1hz(&self) -> RTC1HZ_R {
        RTC1HZ_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Real-time Timer Prescaler Value"]
    #[inline(always)]
    pub fn rtpres(&mut self) -> RTPRES_W {
        RTPRES_W { w: self }
    }
    #[doc = "Bit 16 - Alarm Interrupt Enable"]
    #[inline(always)]
    pub fn almien(&mut self) -> ALMIEN_W {
        ALMIEN_W { w: self }
    }
    #[doc = "Bit 17 - Real-time Timer Increment Interrupt Enable"]
    #[inline(always)]
    pub fn rttincien(&mut self) -> RTTINCIEN_W {
        RTTINCIEN_W { w: self }
    }
    #[doc = "Bit 18 - Real-time Timer Restart"]
    #[inline(always)]
    pub fn rttrst(&mut self) -> RTTRST_W {
        RTTRST_W { w: self }
    }
    #[doc = "Bit 20 - Real-time Timer Disable"]
    #[inline(always)]
    pub fn rttdis(&mut self) -> RTTDIS_W {
        RTTDIS_W { w: self }
    }
    #[doc = "Bit 24 - Real-Time Clock 1Hz Clock Selection"]
    #[inline(always)]
    pub fn rtc1hz(&mut self) -> RTC1HZ_W {
        RTC1HZ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtt_mr](index.html) module"]
pub struct RTT_MR_SPEC;
impl crate::RegisterSpec for RTT_MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtt_mr::R](R) reader structure"]
impl crate::Readable for RTT_MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtt_mr::W](W) writer structure"]
impl crate::Writable for RTT_MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTT_MR to value 0"]
impl crate::Resettable for RTT_MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
