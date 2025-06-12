#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `CKRDY` reader - Clock Ready Interrupt Enable"]
pub type CkrdyR = crate::BitReader;
#[doc = "Field `CKRDY` writer - Clock Ready Interrupt Enable"]
pub type CkrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFD` reader - Clock Failure Detector Interrupt Enable"]
pub type CfdR = crate::BitReader;
#[doc = "Field `CFD` writer - Clock Failure Detector Interrupt Enable"]
pub type CfdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock Ready Interrupt Enable"]
    #[inline(always)]
    pub fn ckrdy(&self) -> CkrdyR {
        CkrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    pub fn cfd(&self) -> CfdR {
        CfdR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Ready Interrupt Enable"]
    #[inline(always)]
    pub fn ckrdy(&mut self) -> CkrdyW<IntenclrSpec> {
        CkrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    pub fn cfd(&mut self) -> CfdW<IntenclrSpec> {
        CfdW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}
