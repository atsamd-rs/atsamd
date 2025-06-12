#[doc = "Register `SAB` reader"]
pub type R = crate::R<SabSpec>;
#[doc = "Register `SAB` writer"]
pub type W = crate::W<SabSpec>;
#[doc = "Field `ADDR` reader - Specific Address 1"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Specific Address 1"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specific Address 1"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specific Address 1"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<SabSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Specific Address Bottom \\[31:0\\] Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sab::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sab::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SabSpec;
impl crate::RegisterSpec for SabSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sab::R`](R) reader structure"]
impl crate::Readable for SabSpec {}
#[doc = "`write(|w| ..)` method takes [`sab::W`](W) writer structure"]
impl crate::Writable for SabSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAB to value 0"]
impl crate::Resettable for SabSpec {}
