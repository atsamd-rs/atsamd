#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `XOSC32KRDY` reader - XOSC32K Ready"]
pub type Xosc32krdyR = crate::BitReader;
#[doc = "Field `XOSC32KRDY` writer - XOSC32K Ready"]
pub type Xosc32krdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSC32KFAIL` reader - XOSC32K Clock Failure Detector"]
pub type Xosc32kfailR = crate::BitReader;
#[doc = "Field `XOSC32KFAIL` writer - XOSC32K Clock Failure Detector"]
pub type Xosc32kfailW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - XOSC32K Ready"]
    #[inline(always)]
    pub fn xosc32krdy(&self) -> Xosc32krdyR {
        Xosc32krdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - XOSC32K Clock Failure Detector"]
    #[inline(always)]
    pub fn xosc32kfail(&self) -> Xosc32kfailR {
        Xosc32kfailR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XOSC32K Ready"]
    #[inline(always)]
    pub fn xosc32krdy(&mut self) -> Xosc32krdyW<IntflagSpec> {
        Xosc32krdyW::new(self, 0)
    }
    #[doc = "Bit 2 - XOSC32K Clock Failure Detector"]
    #[inline(always)]
    pub fn xosc32kfail(&mut self) -> Xosc32kfailW<IntflagSpec> {
        Xosc32kfailW::new(self, 2)
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}
