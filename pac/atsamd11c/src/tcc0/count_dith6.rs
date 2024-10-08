#[doc = "Register `COUNT_DITH6` reader"]
pub type R = crate::R<CountDith6Spec>;
#[doc = "Register `COUNT_DITH6` writer"]
pub type W = crate::W<CountDith6Spec>;
#[doc = "Field `COUNT` reader - Counter Value"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Counter Value"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 6:23 - Counter Value"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits >> 6) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 6:23 - Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> CountW<CountDith6Spec> {
        CountW::new(self, 6)
    }
}
#[doc = "Count\n\nYou can [`read`](crate::Reg::read) this register and get [`count_dith6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count_dith6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CountDith6Spec;
impl crate::RegisterSpec for CountDith6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`count_dith6::R`](R) reader structure"]
impl crate::Readable for CountDith6Spec {}
#[doc = "`write(|w| ..)` method takes [`count_dith6::W`](W) writer structure"]
impl crate::Writable for CountDith6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COUNT_DITH6 to value 0"]
impl crate::Resettable for CountDith6Spec {
    const RESET_VALUE: u32 = 0;
}
