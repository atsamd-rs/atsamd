#[doc = "Register `OSCULP32K` reader"]
pub type R = crate::R<Osculp32kSpec>;
#[doc = "Register `OSCULP32K` writer"]
pub type W = crate::W<Osculp32kSpec>;
#[doc = "Field `EN32K` reader - Enable Out 32k"]
pub type En32kR = crate::BitReader;
#[doc = "Field `EN32K` writer - Enable Out 32k"]
pub type En32kW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN1K` reader - Enable Out 1k"]
pub type En1kR = crate::BitReader;
#[doc = "Field `EN1K` writer - Enable Out 1k"]
pub type En1kW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALIB` reader - Oscillator Calibration"]
pub type CalibR = crate::FieldReader;
#[doc = "Field `CALIB` writer - Oscillator Calibration"]
pub type CalibW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub type WrtlockR = crate::BitReader;
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub type WrtlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Enable Out 32k"]
    #[inline(always)]
    pub fn en32k(&self) -> En32kR {
        En32kR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Out 1k"]
    #[inline(always)]
    pub fn en1k(&self) -> En1kR {
        En1kR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Oscillator Calibration"]
    #[inline(always)]
    pub fn calib(&self) -> CalibR {
        CalibR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WrtlockR {
        WrtlockR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable Out 32k"]
    #[inline(always)]
    #[must_use]
    pub fn en32k(&mut self) -> En32kW<Osculp32kSpec> {
        En32kW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Out 1k"]
    #[inline(always)]
    #[must_use]
    pub fn en1k(&mut self) -> En1kW<Osculp32kSpec> {
        En1kW::new(self, 2)
    }
    #[doc = "Bits 8:13 - Oscillator Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn calib(&mut self) -> CalibW<Osculp32kSpec> {
        CalibW::new(self, 8)
    }
    #[doc = "Bit 15 - Write Lock"]
    #[inline(always)]
    #[must_use]
    pub fn wrtlock(&mut self) -> WrtlockW<Osculp32kSpec> {
        WrtlockW::new(self, 15)
    }
}
#[doc = "32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`osculp32k::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osculp32k::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Osculp32kSpec;
impl crate::RegisterSpec for Osculp32kSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osculp32k::R`](R) reader structure"]
impl crate::Readable for Osculp32kSpec {}
#[doc = "`write(|w| ..)` method takes [`osculp32k::W`](W) writer structure"]
impl crate::Writable for Osculp32kSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSCULP32K to value 0"]
impl crate::Resettable for Osculp32kSpec {
    const RESET_VALUE: u32 = 0;
}
