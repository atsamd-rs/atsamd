#[doc = "Register `CRCDATAIN` reader"]
pub type R = crate::R<CrcdatainSpec>;
#[doc = "Register `CRCDATAIN` writer"]
pub type W = crate::W<CrcdatainSpec>;
#[doc = "Field `CRCDATAIN` reader - CRC Data Input"]
pub type CrcdatainR = crate::FieldReader<u32>;
#[doc = "Field `CRCDATAIN` writer - CRC Data Input"]
pub type CrcdatainW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC Data Input"]
    #[inline(always)]
    pub fn crcdatain(&self) -> CrcdatainR {
        CrcdatainR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Data Input"]
    #[inline(always)]
    #[must_use]
    pub fn crcdatain(&mut self) -> CrcdatainW<CrcdatainSpec> {
        CrcdatainW::new(self, 0)
    }
}
#[doc = "CRC Data Input\n\nYou can [`read`](crate::Reg::read) this register and get [`crcdatain::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdatain::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcdatainSpec;
impl crate::RegisterSpec for CrcdatainSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcdatain::R`](R) reader structure"]
impl crate::Readable for CrcdatainSpec {}
#[doc = "`write(|w| ..)` method takes [`crcdatain::W`](W) writer structure"]
impl crate::Writable for CrcdatainSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRCDATAIN to value 0"]
impl crate::Resettable for CrcdatainSpec {
    const RESET_VALUE: u32 = 0;
}
