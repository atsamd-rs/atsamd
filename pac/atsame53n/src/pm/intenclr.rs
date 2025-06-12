#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `SLEEPRDY` reader - Sleep Mode Entry Ready Enable"]
pub type SleeprdyR = crate::BitReader;
#[doc = "Field `SLEEPRDY` writer - Sleep Mode Entry Ready Enable"]
pub type SleeprdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Sleep Mode Entry Ready Enable"]
    #[inline(always)]
    pub fn sleeprdy(&self) -> SleeprdyR {
        SleeprdyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sleep Mode Entry Ready Enable"]
    #[inline(always)]
    pub fn sleeprdy(&mut self) -> SleeprdyW<IntenclrSpec> {
        SleeprdyW::new(self, 0)
    }
}
#[doc = "Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}
