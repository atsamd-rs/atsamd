#[doc = "Register `COUNT_DITH5` reader"]
pub type R = crate::R<CountDith5Spec>;
#[doc = "Register `COUNT_DITH5` writer"]
pub type W = crate::W<CountDith5Spec>;
#[doc = "Field `COUNT` reader - Counter Value"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Counter Value"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 5:23 - Counter Value"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits >> 5) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 5:23 - Counter Value"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<CountDith5Spec> {
        CountW::new(self, 5)
    }
}
#[doc = "Count\n\nYou can [`read`](crate::Reg::read) this register and get [`count_dith5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count_dith5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CountDith5Spec;
impl crate::RegisterSpec for CountDith5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`count_dith5::R`](R) reader structure"]
impl crate::Readable for CountDith5Spec {}
#[doc = "`write(|w| ..)` method takes [`count_dith5::W`](W) writer structure"]
impl crate::Writable for CountDith5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COUNT_DITH5 to value 0"]
impl crate::Resettable for CountDith5Spec {}
