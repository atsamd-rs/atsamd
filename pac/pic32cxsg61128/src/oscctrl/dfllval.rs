#[doc = "Register `DFLLVAL` reader"]
pub type R = crate::R<DfllvalSpec>;
#[doc = "Register `DFLLVAL` writer"]
pub type W = crate::W<DfllvalSpec>;
#[doc = "Field `FINE` reader - Fine Value"]
pub type FineR = crate::FieldReader;
#[doc = "Field `FINE` writer - Fine Value"]
pub type FineW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COARSE` reader - Coarse Value"]
pub type CoarseR = crate::FieldReader;
#[doc = "Field `COARSE` writer - Coarse Value"]
pub type CoarseW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DIFF` reader - Multiplication Ratio Difference"]
pub type DiffR = crate::FieldReader<u16>;
#[doc = "Field `DIFF` writer - Multiplication Ratio Difference"]
pub type DiffW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - Fine Value"]
    #[inline(always)]
    pub fn fine(&self) -> FineR {
        FineR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 10:15 - Coarse Value"]
    #[inline(always)]
    pub fn coarse(&self) -> CoarseR {
        CoarseR::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - Multiplication Ratio Difference"]
    #[inline(always)]
    pub fn diff(&self) -> DiffR {
        DiffR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fine Value"]
    #[inline(always)]
    #[must_use]
    pub fn fine(&mut self) -> FineW<DfllvalSpec> {
        FineW::new(self, 0)
    }
    #[doc = "Bits 10:15 - Coarse Value"]
    #[inline(always)]
    #[must_use]
    pub fn coarse(&mut self) -> CoarseW<DfllvalSpec> {
        CoarseW::new(self, 10)
    }
    #[doc = "Bits 16:31 - Multiplication Ratio Difference"]
    #[inline(always)]
    #[must_use]
    pub fn diff(&mut self) -> DiffW<DfllvalSpec> {
        DiffW::new(self, 16)
    }
}
#[doc = "DFLL48M Value\n\nYou can [`read`](crate::Reg::read) this register and get [`dfllval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfllval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfllvalSpec;
impl crate::RegisterSpec for DfllvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfllval::R`](R) reader structure"]
impl crate::Readable for DfllvalSpec {}
#[doc = "`write(|w| ..)` method takes [`dfllval::W`](W) writer structure"]
impl crate::Writable for DfllvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFLLVAL to value 0"]
impl crate::Resettable for DfllvalSpec {
    const RESET_VALUE: u32 = 0;
}
