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
#[doc = "Field `EN32K` reader - Enable Out 32k"]
pub type EN32K_R = crate::BitReader<bool>;
#[doc = "Field `EN32K` writer - Enable Out 32k"]
pub type EN32K_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCULP32K_SPEC, bool, O>;
#[doc = "Field `EN1K` reader - Enable Out 1k"]
pub type EN1K_R = crate::BitReader<bool>;
#[doc = "Field `EN1K` writer - Enable Out 1k"]
pub type EN1K_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCULP32K_SPEC, bool, O>;
#[doc = "Field `CALIB` reader - Oscillator Calibration"]
pub type CALIB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALIB` writer - Oscillator Calibration"]
pub type CALIB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSCULP32K_SPEC, u8, u8, 5, O>;
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub type WRTLOCK_R = crate::BitReader<bool>;
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub type WRTLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCULP32K_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Enable Out 32k"]
    #[inline(always)]
    pub fn en32k(&self) -> EN32K_R {
        EN32K_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Out 1k"]
    #[inline(always)]
    pub fn en1k(&self) -> EN1K_R {
        EN1K_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Oscillator Calibration"]
    #[inline(always)]
    pub fn calib(&self) -> CALIB_R {
        CALIB_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WRTLOCK_R {
        WRTLOCK_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable Out 32k"]
    #[inline(always)]
    #[must_use]
    pub fn en32k(&mut self) -> EN32K_W<1> {
        EN32K_W::new(self)
    }
    #[doc = "Bit 2 - Enable Out 1k"]
    #[inline(always)]
    #[must_use]
    pub fn en1k(&mut self) -> EN1K_W<2> {
        EN1K_W::new(self)
    }
    #[doc = "Bits 8:12 - Oscillator Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn calib(&mut self) -> CALIB_W<8> {
        CALIB_W::new(self)
    }
    #[doc = "Bit 15 - Write Lock"]
    #[inline(always)]
    #[must_use]
    pub fn wrtlock(&mut self) -> WRTLOCK_W<15> {
        WRTLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osculp32k](index.html) module"]
pub struct OSCULP32K_SPEC;
impl crate::RegisterSpec for OSCULP32K_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osculp32k::R](R) reader structure"]
impl crate::Readable for OSCULP32K_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osculp32k::W](W) writer structure"]
impl crate::Writable for OSCULP32K_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSCULP32K to value 0"]
impl crate::Resettable for OSCULP32K_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
