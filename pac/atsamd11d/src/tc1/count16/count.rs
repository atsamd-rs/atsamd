#[doc = "Register `COUNT` reader"]
pub type R = crate::R<CountSpec>;
#[doc = "Register `COUNT` writer"]
pub type W = crate::W<CountSpec>;
#[doc = "Field `COUNT` reader - Count Value"]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Count Value"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Count Value"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Count Value"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<CountSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "COUNT16 Counter Value\n\nYou can [`read`](crate::Reg::read) this register and get [`count::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CountSpec;
impl crate::RegisterSpec for CountSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`count::R`](R) reader structure"]
impl crate::Readable for CountSpec {}
#[doc = "`write(|w| ..)` method takes [`count::W`](W) writer structure"]
impl crate::Writable for CountSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COUNT to value 0"]
impl crate::Resettable for CountSpec {}
