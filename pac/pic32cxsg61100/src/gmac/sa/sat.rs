#[doc = "Register `SAT` reader"]
pub type R = crate::R<SatSpec>;
#[doc = "Register `SAT` writer"]
pub type W = crate::W<SatSpec>;
#[doc = "Field `ADDR` reader - Specific Address 1"]
pub type AddrR = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - Specific Address 1"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Specific Address 1"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specific Address 1"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<SatSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Specific Address Top \\[47:32\\]
Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SatSpec;
impl crate::RegisterSpec for SatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sat::R`](R) reader structure"]
impl crate::Readable for SatSpec {}
#[doc = "`write(|w| ..)` method takes [`sat::W`](W) writer structure"]
impl crate::Writable for SatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAT to value 0"]
impl crate::Resettable for SatSpec {
    const RESET_VALUE: u32 = 0;
}
