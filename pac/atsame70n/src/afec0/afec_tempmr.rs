#[doc = "Register `AFEC_TEMPMR` reader"]
pub struct R(crate::R<AFEC_TEMPMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFEC_TEMPMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFEC_TEMPMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFEC_TEMPMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFEC_TEMPMR` writer"]
pub struct W(crate::W<AFEC_TEMPMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFEC_TEMPMR_SPEC>;
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
impl From<crate::W<AFEC_TEMPMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFEC_TEMPMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCT` reader - Temperature Sensor RTC Trigger Mode"]
pub struct RTCT_R(crate::FieldReader<bool, bool>);
impl RTCT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTCT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCT` writer - Temperature Sensor RTC Trigger Mode"]
pub struct RTCT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCT_W<'a> {
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
#[doc = "Temperature Comparison Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TEMPCMPMOD_A {
    #[doc = "0: Generates an event when the converted data is lower than the low threshold of the window."]
    LOW = 0,
    #[doc = "1: Generates an event when the converted data is higher than the high threshold of the window."]
    HIGH = 1,
    #[doc = "2: Generates an event when the converted data is in the comparison window."]
    IN = 2,
    #[doc = "3: Generates an event when the converted data is out of the comparison window."]
    OUT = 3,
}
impl From<TEMPCMPMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: TEMPCMPMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TEMPCMPMOD` reader - Temperature Comparison Mode"]
pub struct TEMPCMPMOD_R(crate::FieldReader<u8, TEMPCMPMOD_A>);
impl TEMPCMPMOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TEMPCMPMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEMPCMPMOD_A {
        match self.bits {
            0 => TEMPCMPMOD_A::LOW,
            1 => TEMPCMPMOD_A::HIGH,
            2 => TEMPCMPMOD_A::IN,
            3 => TEMPCMPMOD_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == TEMPCMPMOD_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == TEMPCMPMOD_A::HIGH
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        **self == TEMPCMPMOD_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        **self == TEMPCMPMOD_A::OUT
    }
}
impl core::ops::Deref for TEMPCMPMOD_R {
    type Target = crate::FieldReader<u8, TEMPCMPMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEMPCMPMOD` writer - Temperature Comparison Mode"]
pub struct TEMPCMPMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TEMPCMPMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEMPCMPMOD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(TEMPCMPMOD_A::LOW)
    }
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(TEMPCMPMOD_A::HIGH)
    }
    #[doc = "Generates an event when the converted data is in the comparison window."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(TEMPCMPMOD_A::IN)
    }
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(TEMPCMPMOD_A::OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Temperature Sensor RTC Trigger Mode"]
    #[inline(always)]
    pub fn rtct(&self) -> RTCT_R {
        RTCT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Temperature Comparison Mode"]
    #[inline(always)]
    pub fn tempcmpmod(&self) -> TEMPCMPMOD_R {
        TEMPCMPMOD_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Temperature Sensor RTC Trigger Mode"]
    #[inline(always)]
    pub fn rtct(&mut self) -> RTCT_W {
        RTCT_W { w: self }
    }
    #[doc = "Bits 4:5 - Temperature Comparison Mode"]
    #[inline(always)]
    pub fn tempcmpmod(&mut self) -> TEMPCMPMOD_W {
        TEMPCMPMOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Temperature Sensor Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_tempmr](index.html) module"]
pub struct AFEC_TEMPMR_SPEC;
impl crate::RegisterSpec for AFEC_TEMPMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afec_tempmr::R](R) reader structure"]
impl crate::Readable for AFEC_TEMPMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afec_tempmr::W](W) writer structure"]
impl crate::Writable for AFEC_TEMPMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFEC_TEMPMR to value 0"]
impl crate::Resettable for AFEC_TEMPMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
