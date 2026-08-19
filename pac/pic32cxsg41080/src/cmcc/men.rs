#[doc = "Register `MEN` reader"]
pub type R = crate::R<MenSpec>;
#[doc = "Register `MEN` writer"]
pub type W = crate::W<MenSpec>;
#[doc = "Field `MENABLE` reader - Cache Controller Monitor Enable"]
pub type MenableR = crate::BitReader;
#[doc = "Field `MENABLE` writer - Cache Controller Monitor Enable"]
pub type MenableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Cache Controller Monitor Enable"]
    #[inline(always)]
    pub fn menable(&self) -> MenableR {
        MenableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache Controller Monitor Enable"]
    #[inline(always)]
    #[must_use]
    pub fn menable(&mut self) -> MenableW<MenSpec> {
        MenableW::new(self, 0)
    }
}
#[doc = "Cache Monitor Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`men::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`men::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MenSpec;
impl crate::RegisterSpec for MenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`men::R`](R) reader structure"]
impl crate::Readable for MenSpec {}
#[doc = "`write(|w| ..)` method takes [`men::W`](W) writer structure"]
impl crate::Writable for MenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEN to value 0"]
impl crate::Resettable for MenSpec {
    const RESET_VALUE: u32 = 0;
}
