#[doc = "Register `DESCADD` reader"]
pub type R = crate::R<DescaddSpec>;
#[doc = "Register `DESCADD` writer"]
pub type W = crate::W<DescaddSpec>;
#[doc = "Field `DESCADD` reader - Descriptor Address Value"]
pub type DescaddR = crate::FieldReader<u32>;
#[doc = "Field `DESCADD` writer - Descriptor Address Value"]
pub type DescaddW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Descriptor Address Value"]
    #[inline(always)]
    pub fn descadd(&self) -> DescaddR {
        DescaddR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Descriptor Address Value"]
    #[inline(always)]
    #[must_use]
    pub fn descadd(&mut self) -> DescaddW<DescaddSpec> {
        DescaddW::new(self, 0)
    }
}
#[doc = "Descriptor Address\n\nYou can [`read`](crate::Reg::read) this register and get [`descadd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`descadd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DescaddSpec;
impl crate::RegisterSpec for DescaddSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`descadd::R`](R) reader structure"]
impl crate::Readable for DescaddSpec {}
#[doc = "`write(|w| ..)` method takes [`descadd::W`](W) writer structure"]
impl crate::Writable for DescaddSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DESCADD to value 0"]
impl crate::Resettable for DescaddSpec {
    const RESET_VALUE: u32 = 0;
}
