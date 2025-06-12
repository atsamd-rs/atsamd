#[doc = "Register `ACTLR` reader"]
pub type R = crate::R<ActlrSpec>;
#[doc = "Register `ACTLR` writer"]
pub type W = crate::W<ActlrSpec>;
#[doc = "Field `DISMCYCINT` reader - Disable interruption of LDM/STM instructions"]
pub type DismcycintR = crate::BitReader;
#[doc = "Field `DISMCYCINT` writer - Disable interruption of LDM/STM instructions"]
pub type DismcycintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISDEFWBUF` reader - Disable wruite buffer use during default memory map accesses"]
pub type DisdefwbufR = crate::BitReader;
#[doc = "Field `DISDEFWBUF` writer - Disable wruite buffer use during default memory map accesses"]
pub type DisdefwbufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISFOLD` reader - Disable IT folding"]
pub type DisfoldR = crate::BitReader;
#[doc = "Field `DISFOLD` writer - Disable IT folding"]
pub type DisfoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISFPCA` reader - Disable automatic update of CONTROL.FPCA"]
pub type DisfpcaR = crate::BitReader;
#[doc = "Field `DISFPCA` writer - Disable automatic update of CONTROL.FPCA"]
pub type DisfpcaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISOOFP` reader - Disable out-of-order FP instructions"]
pub type DisoofpR = crate::BitReader;
#[doc = "Field `DISOOFP` writer - Disable out-of-order FP instructions"]
pub type DisoofpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Disable interruption of LDM/STM instructions"]
    #[inline(always)]
    pub fn dismcycint(&self) -> DismcycintR {
        DismcycintR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable wruite buffer use during default memory map accesses"]
    #[inline(always)]
    pub fn disdefwbuf(&self) -> DisdefwbufR {
        DisdefwbufR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable IT folding"]
    #[inline(always)]
    pub fn disfold(&self) -> DisfoldR {
        DisfoldR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Disable automatic update of CONTROL.FPCA"]
    #[inline(always)]
    pub fn disfpca(&self) -> DisfpcaR {
        DisfpcaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable out-of-order FP instructions"]
    #[inline(always)]
    pub fn disoofp(&self) -> DisoofpR {
        DisoofpR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable interruption of LDM/STM instructions"]
    #[inline(always)]
    pub fn dismcycint(&mut self) -> DismcycintW<ActlrSpec> {
        DismcycintW::new(self, 0)
    }
    #[doc = "Bit 1 - Disable wruite buffer use during default memory map accesses"]
    #[inline(always)]
    pub fn disdefwbuf(&mut self) -> DisdefwbufW<ActlrSpec> {
        DisdefwbufW::new(self, 1)
    }
    #[doc = "Bit 2 - Disable IT folding"]
    #[inline(always)]
    pub fn disfold(&mut self) -> DisfoldW<ActlrSpec> {
        DisfoldW::new(self, 2)
    }
    #[doc = "Bit 8 - Disable automatic update of CONTROL.FPCA"]
    #[inline(always)]
    pub fn disfpca(&mut self) -> DisfpcaW<ActlrSpec> {
        DisfpcaW::new(self, 8)
    }
    #[doc = "Bit 9 - Disable out-of-order FP instructions"]
    #[inline(always)]
    pub fn disoofp(&mut self) -> DisoofpW<ActlrSpec> {
        DisoofpW::new(self, 9)
    }
}
#[doc = "Auxiliary Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`actlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`actlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActlrSpec;
impl crate::RegisterSpec for ActlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`actlr::R`](R) reader structure"]
impl crate::Readable for ActlrSpec {}
#[doc = "`write(|w| ..)` method takes [`actlr::W`](W) writer structure"]
impl crate::Writable for ActlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACTLR to value 0"]
impl crate::Resettable for ActlrSpec {}
