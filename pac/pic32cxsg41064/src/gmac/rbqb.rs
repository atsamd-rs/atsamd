#[doc = "Register `RBQB` reader"]
pub type R = crate::R<RbqbSpec>;
#[doc = "Register `RBQB` writer"]
pub type W = crate::W<RbqbSpec>;
#[doc = "Field `ADDR` reader - Receive Buffer Queue Base Address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Receive Buffer Queue Base Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Receive Buffer Queue Base Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Receive Buffer Queue Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<RbqbSpec> {
        AddrW::new(self, 2)
    }
}
#[doc = "Receive Buffer Queue Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`rbqb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbqb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbqbSpec;
impl crate::RegisterSpec for RbqbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbqb::R`](R) reader structure"]
impl crate::Readable for RbqbSpec {}
#[doc = "`write(|w| ..)` method takes [`rbqb::W`](W) writer structure"]
impl crate::Writable for RbqbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RBQB to value 0"]
impl crate::Resettable for RbqbSpec {
    const RESET_VALUE: u32 = 0;
}
