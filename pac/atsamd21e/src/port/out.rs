#[doc = "Register `OUT%s` reader"]
pub type R = crate::R<OutSpec>;
#[doc = "Register `OUT%s` writer"]
pub type W = crate::W<OutSpec>;
#[doc = "Field `OUT` reader - Port Data Output Value"]
pub type OutR = crate::FieldReader<u32>;
#[doc = "Field `OUT` writer - Port Data Output Value"]
pub type OutW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Port Data Output Value"]
    #[inline(always)]
    pub fn out(&self) -> OutR {
        OutR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn out(&mut self) -> OutW<OutSpec> {
        OutW::new(self, 0)
    }
}
#[doc = "Data Output Value\n\nYou can [`read`](crate::Reg::read) this register and get [`out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutSpec;
impl crate::RegisterSpec for OutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out::R`](R) reader structure"]
impl crate::Readable for OutSpec {}
#[doc = "`write(|w| ..)` method takes [`out::W`](W) writer structure"]
impl crate::Writable for OutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT%s to value 0"]
impl crate::Resettable for OutSpec {
    const RESET_VALUE: u32 = 0;
}
