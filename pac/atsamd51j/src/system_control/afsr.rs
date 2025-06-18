#[doc = "Register `AFSR` reader"]
pub type R = crate::R<AfsrSpec>;
#[doc = "Register `AFSR` writer"]
pub type W = crate::W<AfsrSpec>;
#[doc = "Field `IMPDEF` reader - AUXFAULT input signals"]
pub type ImpdefR = crate::FieldReader<u32>;
#[doc = "Field `IMPDEF` writer - AUXFAULT input signals"]
pub type ImpdefW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AUXFAULT input signals"]
    #[inline(always)]
    pub fn impdef(&self) -> ImpdefR {
        ImpdefR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AUXFAULT input signals"]
    #[inline(always)]
    pub fn impdef(&mut self) -> ImpdefW<AfsrSpec> {
        ImpdefW::new(self, 0)
    }
}
#[doc = "Auxiliary Fault Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`afsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfsrSpec;
impl crate::RegisterSpec for AfsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afsr::R`](R) reader structure"]
impl crate::Readable for AfsrSpec {}
#[doc = "`write(|w| ..)` method takes [`afsr::W`](W) writer structure"]
impl crate::Writable for AfsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AFSR to value 0"]
impl crate::Resettable for AfsrSpec {}
