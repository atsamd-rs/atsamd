#[doc = "Register `SCL` reader"]
pub type R = crate::R<SclSpec>;
#[doc = "Register `SCL` writer"]
pub type W = crate::W<SclSpec>;
#[doc = "Field `SEC` reader - 1588 Timer Second comparison value"]
pub type SecR = crate::FieldReader<u32>;
#[doc = "Field `SEC` writer - 1588 Timer Second comparison value"]
pub type SecW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1588 Timer Second comparison value"]
    #[inline(always)]
    pub fn sec(&self) -> SecR {
        SecR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1588 Timer Second comparison value"]
    #[inline(always)]
    pub fn sec(&mut self) -> SecW<SclSpec> {
        SecW::new(self, 0)
    }
}
#[doc = "Tsu timer second comparison Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SclSpec;
impl crate::RegisterSpec for SclSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl::R`](R) reader structure"]
impl crate::Readable for SclSpec {}
#[doc = "`write(|w| ..)` method takes [`scl::W`](W) writer structure"]
impl crate::Writable for SclSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCL to value 0"]
impl crate::Resettable for SclSpec {}
