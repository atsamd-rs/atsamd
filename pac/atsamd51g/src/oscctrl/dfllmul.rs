#[doc = "Register `DFLLMUL` reader"]
pub type R = crate::R<DfllmulSpec>;
#[doc = "Register `DFLLMUL` writer"]
pub type W = crate::W<DfllmulSpec>;
#[doc = "Field `MUL` reader - DFLL Multiply Factor"]
pub type MulR = crate::FieldReader<u16>;
#[doc = "Field `MUL` writer - DFLL Multiply Factor"]
pub type MulW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FSTEP` reader - Fine Maximum Step"]
pub type FstepR = crate::FieldReader;
#[doc = "Field `FSTEP` writer - Fine Maximum Step"]
pub type FstepW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CSTEP` reader - Coarse Maximum Step"]
pub type CstepR = crate::FieldReader;
#[doc = "Field `CSTEP` writer - Coarse Maximum Step"]
pub type CstepW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:15 - DFLL Multiply Factor"]
    #[inline(always)]
    pub fn mul(&self) -> MulR {
        MulR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Fine Maximum Step"]
    #[inline(always)]
    pub fn fstep(&self) -> FstepR {
        FstepR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 26:31 - Coarse Maximum Step"]
    #[inline(always)]
    pub fn cstep(&self) -> CstepR {
        CstepR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - DFLL Multiply Factor"]
    #[inline(always)]
    pub fn mul(&mut self) -> MulW<DfllmulSpec> {
        MulW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Fine Maximum Step"]
    #[inline(always)]
    pub fn fstep(&mut self) -> FstepW<DfllmulSpec> {
        FstepW::new(self, 16)
    }
    #[doc = "Bits 26:31 - Coarse Maximum Step"]
    #[inline(always)]
    pub fn cstep(&mut self) -> CstepW<DfllmulSpec> {
        CstepW::new(self, 26)
    }
}
#[doc = "DFLL48M Multiplier\n\nYou can [`read`](crate::Reg::read) this register and get [`dfllmul::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfllmul::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfllmulSpec;
impl crate::RegisterSpec for DfllmulSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfllmul::R`](R) reader structure"]
impl crate::Readable for DfllmulSpec {}
#[doc = "`write(|w| ..)` method takes [`dfllmul::W`](W) writer structure"]
impl crate::Writable for DfllmulSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DFLLMUL to value 0"]
impl crate::Resettable for DfllmulSpec {}
