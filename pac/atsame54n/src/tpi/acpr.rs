#[doc = "Register `ACPR` reader"]
pub type R = crate::R<AcprSpec>;
#[doc = "Register `ACPR` writer"]
pub type W = crate::W<AcprSpec>;
#[doc = "Field `PRESCALER` reader - "]
pub type PrescalerR = crate::FieldReader<u16>;
#[doc = "Field `PRESCALER` writer - "]
pub type PrescalerW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn prescaler(&self) -> PrescalerR {
        PrescalerR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PrescalerW<AcprSpec> {
        PrescalerW::new(self, 0)
    }
}
#[doc = "Asynchronous Clock Prescaler Register\n\nYou can [`read`](crate::Reg::read) this register and get [`acpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcprSpec;
impl crate::RegisterSpec for AcprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acpr::R`](R) reader structure"]
impl crate::Readable for AcprSpec {}
#[doc = "`write(|w| ..)` method takes [`acpr::W`](W) writer structure"]
impl crate::Writable for AcprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACPR to value 0"]
impl crate::Resettable for AcprSpec {
    const RESET_VALUE: u32 = 0;
}
