#[doc = "Register `PADCAL` reader"]
pub type R = crate::R<PadcalSpec>;
#[doc = "Register `PADCAL` writer"]
pub type W = crate::W<PadcalSpec>;
#[doc = "Field `TRANSP` reader - USB Pad Transp calibration"]
pub type TranspR = crate::FieldReader;
#[doc = "Field `TRANSP` writer - USB Pad Transp calibration"]
pub type TranspW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRANSN` reader - USB Pad Transn calibration"]
pub type TransnR = crate::FieldReader;
#[doc = "Field `TRANSN` writer - USB Pad Transn calibration"]
pub type TransnW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRIM` reader - USB Pad Trim calibration"]
pub type TrimR = crate::FieldReader;
#[doc = "Field `TRIM` writer - USB Pad Trim calibration"]
pub type TrimW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:4 - USB Pad Transp calibration"]
    #[inline(always)]
    pub fn transp(&self) -> TranspR {
        TranspR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - USB Pad Transn calibration"]
    #[inline(always)]
    pub fn transn(&self) -> TransnR {
        TransnR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 12:14 - USB Pad Trim calibration"]
    #[inline(always)]
    pub fn trim(&self) -> TrimR {
        TrimR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - USB Pad Transp calibration"]
    #[inline(always)]
    #[must_use]
    pub fn transp(&mut self) -> TranspW<PadcalSpec> {
        TranspW::new(self, 0)
    }
    #[doc = "Bits 6:10 - USB Pad Transn calibration"]
    #[inline(always)]
    #[must_use]
    pub fn transn(&mut self) -> TransnW<PadcalSpec> {
        TransnW::new(self, 6)
    }
    #[doc = "Bits 12:14 - USB Pad Trim calibration"]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TrimW<PadcalSpec> {
        TrimW::new(self, 12)
    }
}
#[doc = "USB PAD Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`padcal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padcal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcalSpec;
impl crate::RegisterSpec for PadcalSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`padcal::R`](R) reader structure"]
impl crate::Readable for PadcalSpec {}
#[doc = "`write(|w| ..)` method takes [`padcal::W`](W) writer structure"]
impl crate::Writable for PadcalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PADCAL to value 0"]
impl crate::Resettable for PadcalSpec {
    const RESET_VALUE: u16 = 0;
}
