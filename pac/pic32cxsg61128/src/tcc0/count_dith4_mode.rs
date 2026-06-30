#[doc = "Register `COUNT_DITH4_MODE` reader"]
pub type R = crate::R<CountDith4ModeSpec>;
#[doc = "Register `COUNT_DITH4_MODE` writer"]
pub type W = crate::W<CountDith4ModeSpec>;
#[doc = "Field `COUNT` reader - Counter Value"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Counter Value"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 4:23 - Counter Value"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits >> 4) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 4:23 - Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> CountW<CountDith4ModeSpec> {
        CountW::new(self, 4)
    }
}
#[doc = "Count\n\nYou can [`read`](crate::Reg::read) this register and get [`count_dith4_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count_dith4_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CountDith4ModeSpec;
impl crate::RegisterSpec for CountDith4ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`count_dith4_mode::R`](R) reader structure"]
impl crate::Readable for CountDith4ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`count_dith4_mode::W`](W) writer structure"]
impl crate::Writable for CountDith4ModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COUNT_DITH4_MODE to value 0"]
impl crate::Resettable for CountDith4ModeSpec {
    const RESET_VALUE: u32 = 0;
}
