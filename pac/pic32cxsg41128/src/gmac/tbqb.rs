#[doc = "Register `TBQB` reader"]
pub type R = crate::R<TbqbSpec>;
#[doc = "Register `TBQB` writer"]
pub type W = crate::W<TbqbSpec>;
#[doc = "Field `ADDR` reader - Transmit Buffer Queue Base Address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Transmit Buffer Queue Base Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Transmit Buffer Queue Base Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Transmit Buffer Queue Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<TbqbSpec> {
        AddrW::new(self, 2)
    }
}
#[doc = "Transmit Buffer Queue Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tbqb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbqb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbqbSpec;
impl crate::RegisterSpec for TbqbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbqb::R`](R) reader structure"]
impl crate::Readable for TbqbSpec {}
#[doc = "`write(|w| ..)` method takes [`tbqb::W`](W) writer structure"]
impl crate::Writable for TbqbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBQB to value 0"]
impl crate::Resettable for TbqbSpec {
    const RESET_VALUE: u32 = 0;
}
