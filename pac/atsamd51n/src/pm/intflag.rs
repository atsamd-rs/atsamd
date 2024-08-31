#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `SLEEPRDY` reader - Sleep Mode Entry Ready"]
pub type SleeprdyR = crate::BitReader;
#[doc = "Field `SLEEPRDY` writer - Sleep Mode Entry Ready"]
pub type SleeprdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Sleep Mode Entry Ready"]
    #[inline(always)]
    pub fn sleeprdy(&self) -> SleeprdyR {
        SleeprdyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sleep Mode Entry Ready"]
    #[inline(always)]
    #[must_use]
    pub fn sleeprdy(&mut self) -> SleeprdyW<IntflagSpec> {
        SleeprdyW::new(self, 0)
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {
    const RESET_VALUE: u8 = 0;
}
