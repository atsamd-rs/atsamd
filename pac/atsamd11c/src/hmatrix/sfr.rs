#[doc = "Register `SFR%s` reader"]
pub type R = crate::R<SfrSpec>;
#[doc = "Register `SFR%s` writer"]
pub type W = crate::W<SfrSpec>;
#[doc = "Field `SFR` reader - Special Function Register"]
pub type SfrR = crate::FieldReader<u32>;
#[doc = "Field `SFR` writer - Special Function Register"]
pub type SfrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Special Function Register"]
    #[inline(always)]
    pub fn sfr(&self) -> SfrR {
        SfrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Special Function Register"]
    #[inline(always)]
    pub fn sfr(&mut self) -> SfrW<SfrSpec> {
        SfrW::new(self, 0)
    }
}
#[doc = "Special Function\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSpec;
impl crate::RegisterSpec for SfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr::R`](R) reader structure"]
impl crate::Readable for SfrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr::W`](W) writer structure"]
impl crate::Writable for SfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR%s to value 0"]
impl crate::Resettable for SfrSpec {}
