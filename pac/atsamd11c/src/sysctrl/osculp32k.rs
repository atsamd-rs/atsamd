#[doc = "Register `OSCULP32K` reader"]
pub struct R(crate::R<OSCULP32K_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCULP32K_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCULP32K_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCULP32K_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSCULP32K` writer"]
pub struct W(crate::W<OSCULP32K_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCULP32K_SPEC>;
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
impl From<crate::W<OSCULP32K_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSCULP32K_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALIB` reader - Oscillator Calibration"]
pub struct CALIB_R(crate::FieldReader<u8, u8>);
impl CALIB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CALIB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALIB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALIB` writer - Oscillator Calibration"]
pub struct CALIB_W<'a> {
    w: &'a mut W,
}
impl<'a> CALIB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u8 & 0x1f);
        self.w
    }
}
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub struct WRTLOCK_R(crate::FieldReader<bool, bool>);
impl WRTLOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WRTLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRTLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub struct WRTLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> WRTLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Oscillator Calibration"]
    #[inline(always)]
    pub fn calib(&self) -> CALIB_R {
        CALIB_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WRTLOCK_R {
        WRTLOCK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Oscillator Calibration"]
    #[inline(always)]
    pub fn calib(&mut self) -> CALIB_W {
        CALIB_W { w: self }
    }
    #[doc = "Bit 7 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&mut self) -> WRTLOCK_W {
        WRTLOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osculp32k](index.html) module"]
pub struct OSCULP32K_SPEC;
impl crate::RegisterSpec for OSCULP32K_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [osculp32k::R](R) reader structure"]
impl crate::Readable for OSCULP32K_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osculp32k::W](W) writer structure"]
impl crate::Writable for OSCULP32K_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSCULP32K to value 0x1f"]
impl crate::Resettable for OSCULP32K_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
