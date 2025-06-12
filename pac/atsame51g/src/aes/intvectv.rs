#[doc = "Register `INTVECTV[%s]` writer"]
pub type W = crate::W<IntvectvSpec>;
#[doc = "Field `INTVECTV` writer - Initialization Vector Value"]
pub type IntvectvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Initialization Vector Value"]
    #[inline(always)]
    pub fn intvectv(&mut self) -> IntvectvW<IntvectvSpec> {
        IntvectvW::new(self, 0)
    }
}
#[doc = "Initialisation Vector n\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intvectv::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntvectvSpec;
impl crate::RegisterSpec for IntvectvSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intvectv::W`](W) writer structure"]
impl crate::Writable for IntvectvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTVECTV[%s] to value 0"]
impl crate::Resettable for IntvectvSpec {}
