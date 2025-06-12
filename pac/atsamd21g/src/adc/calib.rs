#[doc = "Register `CALIB` reader"]
pub type R = crate::R<CalibSpec>;
#[doc = "Register `CALIB` writer"]
pub type W = crate::W<CalibSpec>;
#[doc = "Field `LINEARITY_CAL` reader - Linearity Calibration Value"]
pub type LinearityCalR = crate::FieldReader;
#[doc = "Field `LINEARITY_CAL` writer - Linearity Calibration Value"]
pub type LinearityCalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIAS_CAL` reader - Bias Calibration Value"]
pub type BiasCalR = crate::FieldReader;
#[doc = "Field `BIAS_CAL` writer - Bias Calibration Value"]
pub type BiasCalW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - Linearity Calibration Value"]
    #[inline(always)]
    pub fn linearity_cal(&self) -> LinearityCalR {
        LinearityCalR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Bias Calibration Value"]
    #[inline(always)]
    pub fn bias_cal(&self) -> BiasCalR {
        BiasCalR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Linearity Calibration Value"]
    #[inline(always)]
    pub fn linearity_cal(&mut self) -> LinearityCalW<CalibSpec> {
        LinearityCalW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Bias Calibration Value"]
    #[inline(always)]
    pub fn bias_cal(&mut self) -> BiasCalW<CalibSpec> {
        BiasCalW::new(self, 8)
    }
}
#[doc = "Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`calib::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calib::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalibSpec;
impl crate::RegisterSpec for CalibSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`calib::R`](R) reader structure"]
impl crate::Readable for CalibSpec {}
#[doc = "`write(|w| ..)` method takes [`calib::W`](W) writer structure"]
impl crate::Writable for CalibSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CALIB to value 0"]
impl crate::Resettable for CalibSpec {}
