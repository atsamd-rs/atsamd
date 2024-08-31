#[doc = "Register `CALIB` reader"]
pub type R = crate::R<CalibSpec>;
#[doc = "Register `CALIB` writer"]
pub type W = crate::W<CalibSpec>;
#[doc = "Field `BIASCOMP` reader - Bias Comparator Scaling"]
pub type BiascompR = crate::FieldReader;
#[doc = "Field `BIASCOMP` writer - Bias Comparator Scaling"]
pub type BiascompW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIASR2R` reader - Bias R2R Ampli scaling"]
pub type Biasr2rR = crate::FieldReader;
#[doc = "Field `BIASR2R` writer - Bias R2R Ampli scaling"]
pub type Biasr2rW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIASREFBUF` reader - Bias Reference Buffer Scaling"]
pub type BiasrefbufR = crate::FieldReader;
#[doc = "Field `BIASREFBUF` writer - Bias Reference Buffer Scaling"]
pub type BiasrefbufW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Bias Comparator Scaling"]
    #[inline(always)]
    pub fn biascomp(&self) -> BiascompR {
        BiascompR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Bias R2R Ampli scaling"]
    #[inline(always)]
    pub fn biasr2r(&self) -> Biasr2rR {
        Biasr2rR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Bias Reference Buffer Scaling"]
    #[inline(always)]
    pub fn biasrefbuf(&self) -> BiasrefbufR {
        BiasrefbufR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Bias Comparator Scaling"]
    #[inline(always)]
    #[must_use]
    pub fn biascomp(&mut self) -> BiascompW<CalibSpec> {
        BiascompW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Bias R2R Ampli scaling"]
    #[inline(always)]
    #[must_use]
    pub fn biasr2r(&mut self) -> Biasr2rW<CalibSpec> {
        Biasr2rW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Bias Reference Buffer Scaling"]
    #[inline(always)]
    #[must_use]
    pub fn biasrefbuf(&mut self) -> BiasrefbufW<CalibSpec> {
        BiasrefbufW::new(self, 8)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CALIB to value 0"]
impl crate::Resettable for CalibSpec {
    const RESET_VALUE: u16 = 0;
}
