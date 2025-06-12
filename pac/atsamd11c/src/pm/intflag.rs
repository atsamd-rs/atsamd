#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `CKRDY` reader - Clock Ready"]
pub type CkrdyR = crate::BitReader;
#[doc = "Field `CKRDY` writer - Clock Ready"]
pub type CkrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFD` reader - Clock Failure Detector"]
pub type CfdR = crate::BitReader;
#[doc = "Field `CFD` writer - Clock Failure Detector"]
pub type CfdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock Ready"]
    #[inline(always)]
    pub fn ckrdy(&self) -> CkrdyR {
        CkrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Failure Detector"]
    #[inline(always)]
    pub fn cfd(&self) -> CfdR {
        CfdR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Ready"]
    #[inline(always)]
    pub fn ckrdy(&mut self) -> CkrdyW<IntflagSpec> {
        CkrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock Failure Detector"]
    #[inline(always)]
    pub fn cfd(&mut self) -> CfdW<IntflagSpec> {
        CfdW::new(self, 1)
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}
