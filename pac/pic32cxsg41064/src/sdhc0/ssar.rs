#[doc = "Register `SSAR` reader"]
pub type R = crate::R<SsarSpec>;
#[doc = "Register `SSAR` writer"]
pub type W = crate::W<SsarSpec>;
#[doc = "Field `ADDR` reader - SDMA System Address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - SDMA System Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SDMA System Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SDMA System Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<SsarSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "SDMA System Address / Argument 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ssar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsarSpec;
impl crate::RegisterSpec for SsarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssar::R`](R) reader structure"]
impl crate::Readable for SsarSpec {}
#[doc = "`write(|w| ..)` method takes [`ssar::W`](W) writer structure"]
impl crate::Writable for SsarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSAR to value 0"]
impl crate::Resettable for SsarSpec {
    const RESET_VALUE: u32 = 0;
}
