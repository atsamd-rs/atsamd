#[doc = "Register `OUTSET%s` reader"]
pub type R = crate::R<OutsetSpec>;
#[doc = "Register `OUTSET%s` writer"]
pub type W = crate::W<OutsetSpec>;
#[doc = "Field `OUTSET` reader - Port Data Output Value Set"]
pub type OutsetR = crate::FieldReader<u32>;
#[doc = "Field `OUTSET` writer - Port Data Output Value Set"]
pub type OutsetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Port Data Output Value Set"]
    #[inline(always)]
    pub fn outset(&self) -> OutsetR {
        OutsetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Output Value Set"]
    #[inline(always)]
    #[must_use]
    pub fn outset(&mut self) -> OutsetW<OutsetSpec> {
        OutsetW::new(self, 0)
    }
}
#[doc = "Data Output Value Set\n\nYou can [`read`](crate::Reg::read) this register and get [`outset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutsetSpec;
impl crate::RegisterSpec for OutsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outset::R`](R) reader structure"]
impl crate::Readable for OutsetSpec {}
#[doc = "`write(|w| ..)` method takes [`outset::W`](W) writer structure"]
impl crate::Writable for OutsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUTSET%s to value 0"]
impl crate::Resettable for OutsetSpec {
    const RESET_VALUE: u32 = 0;
}
