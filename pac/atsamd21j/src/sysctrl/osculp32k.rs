#[doc = "Register `OSCULP32K` reader"]
pub type R = crate::R<Osculp32kSpec>;
#[doc = "Register `OSCULP32K` writer"]
pub type W = crate::W<Osculp32kSpec>;
#[doc = "Field `CALIB` reader - Oscillator Calibration"]
pub type CalibR = crate::FieldReader;
#[doc = "Field `CALIB` writer - Oscillator Calibration"]
pub type CalibW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub type WrtlockR = crate::BitReader;
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub type WrtlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Oscillator Calibration"]
    #[inline(always)]
    pub fn calib(&self) -> CalibR {
        CalibR::new(self.bits & 0x1f)
    }
    #[doc = "Bit 7 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WrtlockR {
        WrtlockR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Oscillator Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn calib(&mut self) -> CalibW<Osculp32kSpec> {
        CalibW::new(self, 0)
    }
    #[doc = "Bit 7 - Write Lock"]
    #[inline(always)]
    #[must_use]
    pub fn wrtlock(&mut self) -> WrtlockW<Osculp32kSpec> {
        WrtlockW::new(self, 7)
    }
}
#[doc = "32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`osculp32k::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osculp32k::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Osculp32kSpec;
impl crate::RegisterSpec for Osculp32kSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`osculp32k::R`](R) reader structure"]
impl crate::Readable for Osculp32kSpec {}
#[doc = "`write(|w| ..)` method takes [`osculp32k::W`](W) writer structure"]
impl crate::Writable for Osculp32kSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OSCULP32K to value 0x1f"]
impl crate::Resettable for Osculp32kSpec {
    const RESET_VALUE: u8 = 0x1f;
}
