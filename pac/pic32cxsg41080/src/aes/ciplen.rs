#[doc = "Register `CIPLEN` reader"]
pub type R = crate::R<CiplenSpec>;
#[doc = "Register `CIPLEN` writer"]
pub type W = crate::W<CiplenSpec>;
#[doc = "Field `CIPLEN` reader - Cipher Length"]
pub type CiplenR = crate::FieldReader<u32>;
#[doc = "Field `CIPLEN` writer - Cipher Length"]
pub type CiplenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cipher Length"]
    #[inline(always)]
    pub fn ciplen(&self) -> CiplenR {
        CiplenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cipher Length"]
    #[inline(always)]
    #[must_use]
    pub fn ciplen(&mut self) -> CiplenW<CiplenSpec> {
        CiplenW::new(self, 0)
    }
}
#[doc = "Cipher Length\n\nYou can [`read`](crate::Reg::read) this register and get [`ciplen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ciplen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CiplenSpec;
impl crate::RegisterSpec for CiplenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ciplen::R`](R) reader structure"]
impl crate::Readable for CiplenSpec {}
#[doc = "`write(|w| ..)` method takes [`ciplen::W`](W) writer structure"]
impl crate::Writable for CiplenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CIPLEN to value 0"]
impl crate::Resettable for CiplenSpec {
    const RESET_VALUE: u32 = 0;
}
