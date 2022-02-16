#[doc = "Register `RTC_IER` writer"]
pub struct W(crate::W<RTC_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_IER_SPEC>;
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
impl From<crate::W<RTC_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACKEN` writer - Acknowledge Update Interrupt Enable"]
pub struct ACKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKEN_W<'a> {
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
#[doc = "Field `ALREN` writer - Alarm Interrupt Enable"]
pub struct ALREN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALREN_W<'a> {
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
#[doc = "Field `SECEN` writer - Second Event Interrupt Enable"]
pub struct SECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SECEN_W<'a> {
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
#[doc = "Field `TIMEN` writer - Time Event Interrupt Enable"]
pub struct TIMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEN_W<'a> {
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
#[doc = "Field `CALEN` writer - Calendar Event Interrupt Enable"]
pub struct CALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CALEN_W<'a> {
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
#[doc = "Field `TDERREN` writer - Time and/or Date Error Interrupt Enable"]
pub struct TDERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> TDERREN_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Acknowledge Update Interrupt Enable"]
    #[inline(always)]
    pub fn acken(&mut self) -> ACKEN_W {
        ACKEN_W { w: self }
    }
    #[doc = "Bit 1 - Alarm Interrupt Enable"]
    #[inline(always)]
    pub fn alren(&mut self) -> ALREN_W {
        ALREN_W { w: self }
    }
    #[doc = "Bit 2 - Second Event Interrupt Enable"]
    #[inline(always)]
    pub fn secen(&mut self) -> SECEN_W {
        SECEN_W { w: self }
    }
    #[doc = "Bit 3 - Time Event Interrupt Enable"]
    #[inline(always)]
    pub fn timen(&mut self) -> TIMEN_W {
        TIMEN_W { w: self }
    }
    #[doc = "Bit 4 - Calendar Event Interrupt Enable"]
    #[inline(always)]
    pub fn calen(&mut self) -> CALEN_W {
        CALEN_W { w: self }
    }
    #[doc = "Bit 5 - Time and/or Date Error Interrupt Enable"]
    #[inline(always)]
    pub fn tderren(&mut self) -> TDERREN_W {
        TDERREN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_ier](index.html) module"]
pub struct RTC_IER_SPEC;
impl crate::RegisterSpec for RTC_IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [rtc_ier::W](W) writer structure"]
impl crate::Writable for RTC_IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_IER to value 0"]
impl crate::Resettable for RTC_IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
