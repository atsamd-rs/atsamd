#[doc = "Register `CALIB` reader"]
pub type R = crate::R<CalibSpec>;
#[doc = "Register `CALIB` writer"]
pub type W = crate::W<CalibSpec>;
#[doc = "Field `BIAS0` reader - COMP0/1 Bias Scaling"]
pub type Bias0R = crate::FieldReader;
#[doc = "Field `BIAS0` writer - COMP0/1 Bias Scaling"]
pub type Bias0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - COMP0/1 Bias Scaling"]
    #[inline(always)]
    pub fn bias0(&self) -> Bias0R {
        Bias0R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - COMP0/1 Bias Scaling"]
    #[inline(always)]
    pub fn bias0(&mut self) -> Bias0W<CalibSpec> {
        Bias0W::new(self, 0)
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
#[doc = "`reset()` method sets CALIB to value 0x0101"]
impl crate::Resettable for CalibSpec {
    const RESET_VALUE: u16 = 0x0101;
}
