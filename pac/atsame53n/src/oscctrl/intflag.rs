#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `XOSCRDY0` reader - XOSC 0 Ready"]
pub type Xoscrdy0R = crate::BitReader;
#[doc = "Field `XOSCRDY0` writer - XOSC 0 Ready"]
pub type Xoscrdy0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSCRDY1` reader - XOSC 1 Ready"]
pub type Xoscrdy1R = crate::BitReader;
#[doc = "Field `XOSCRDY1` writer - XOSC 1 Ready"]
pub type Xoscrdy1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSCFAIL0` reader - XOSC 0 Clock Failure Detector"]
pub type Xoscfail0R = crate::BitReader;
#[doc = "Field `XOSCFAIL0` writer - XOSC 0 Clock Failure Detector"]
pub type Xoscfail0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSCFAIL1` reader - XOSC 1 Clock Failure Detector"]
pub type Xoscfail1R = crate::BitReader;
#[doc = "Field `XOSCFAIL1` writer - XOSC 1 Clock Failure Detector"]
pub type Xoscfail1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFLLRDY` reader - DFLL Ready"]
pub type DfllrdyR = crate::BitReader;
#[doc = "Field `DFLLRDY` writer - DFLL Ready"]
pub type DfllrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFLLOOB` reader - DFLL Out Of Bounds"]
pub type DflloobR = crate::BitReader;
#[doc = "Field `DFLLOOB` writer - DFLL Out Of Bounds"]
pub type DflloobW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFLLLCKF` reader - DFLL Lock Fine"]
pub type DflllckfR = crate::BitReader;
#[doc = "Field `DFLLLCKF` writer - DFLL Lock Fine"]
pub type DflllckfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFLLLCKC` reader - DFLL Lock Coarse"]
pub type DflllckcR = crate::BitReader;
#[doc = "Field `DFLLLCKC` writer - DFLL Lock Coarse"]
pub type DflllckcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFLLRCS` reader - DFLL Reference Clock Stopped"]
pub type DfllrcsR = crate::BitReader;
#[doc = "Field `DFLLRCS` writer - DFLL Reference Clock Stopped"]
pub type DfllrcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLL0LCKR` reader - DPLL0 Lock Rise"]
pub type Dpll0lckrR = crate::BitReader;
#[doc = "Field `DPLL0LCKR` writer - DPLL0 Lock Rise"]
pub type Dpll0lckrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLL0LCKF` reader - DPLL0 Lock Fall"]
pub type Dpll0lckfR = crate::BitReader;
#[doc = "Field `DPLL0LCKF` writer - DPLL0 Lock Fall"]
pub type Dpll0lckfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLL0LTO` reader - DPLL0 Lock Timeout"]
pub type Dpll0ltoR = crate::BitReader;
#[doc = "Field `DPLL0LTO` writer - DPLL0 Lock Timeout"]
pub type Dpll0ltoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLL0LDRTO` reader - DPLL0 Loop Divider Ratio Update Complete"]
pub type Dpll0ldrtoR = crate::BitReader;
#[doc = "Field `DPLL0LDRTO` writer - DPLL0 Loop Divider Ratio Update Complete"]
pub type Dpll0ldrtoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLL1LCKR` reader - DPLL1 Lock Rise"]
pub type Dpll1lckrR = crate::BitReader;
#[doc = "Field `DPLL1LCKR` writer - DPLL1 Lock Rise"]
pub type Dpll1lckrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLL1LCKF` reader - DPLL1 Lock Fall"]
pub type Dpll1lckfR = crate::BitReader;
#[doc = "Field `DPLL1LCKF` writer - DPLL1 Lock Fall"]
pub type Dpll1lckfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLL1LTO` reader - DPLL1 Lock Timeout"]
pub type Dpll1ltoR = crate::BitReader;
#[doc = "Field `DPLL1LTO` writer - DPLL1 Lock Timeout"]
pub type Dpll1ltoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLL1LDRTO` reader - DPLL1 Loop Divider Ratio Update Complete"]
pub type Dpll1ldrtoR = crate::BitReader;
#[doc = "Field `DPLL1LDRTO` writer - DPLL1 Loop Divider Ratio Update Complete"]
pub type Dpll1ldrtoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - XOSC 0 Ready"]
    #[inline(always)]
    pub fn xoscrdy0(&self) -> Xoscrdy0R {
        Xoscrdy0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XOSC 1 Ready"]
    #[inline(always)]
    pub fn xoscrdy1(&self) -> Xoscrdy1R {
        Xoscrdy1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - XOSC 0 Clock Failure Detector"]
    #[inline(always)]
    pub fn xoscfail0(&self) -> Xoscfail0R {
        Xoscfail0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - XOSC 1 Clock Failure Detector"]
    #[inline(always)]
    pub fn xoscfail1(&self) -> Xoscfail1R {
        Xoscfail1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - DFLL Ready"]
    #[inline(always)]
    pub fn dfllrdy(&self) -> DfllrdyR {
        DfllrdyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DFLL Out Of Bounds"]
    #[inline(always)]
    pub fn dflloob(&self) -> DflloobR {
        DflloobR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DFLL Lock Fine"]
    #[inline(always)]
    pub fn dflllckf(&self) -> DflllckfR {
        DflllckfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DFLL Lock Coarse"]
    #[inline(always)]
    pub fn dflllckc(&self) -> DflllckcR {
        DflllckcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DFLL Reference Clock Stopped"]
    #[inline(always)]
    pub fn dfllrcs(&self) -> DfllrcsR {
        DfllrcsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - DPLL0 Lock Rise"]
    #[inline(always)]
    pub fn dpll0lckr(&self) -> Dpll0lckrR {
        Dpll0lckrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DPLL0 Lock Fall"]
    #[inline(always)]
    pub fn dpll0lckf(&self) -> Dpll0lckfR {
        Dpll0lckfR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DPLL0 Lock Timeout"]
    #[inline(always)]
    pub fn dpll0lto(&self) -> Dpll0ltoR {
        Dpll0ltoR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DPLL0 Loop Divider Ratio Update Complete"]
    #[inline(always)]
    pub fn dpll0ldrto(&self) -> Dpll0ldrtoR {
        Dpll0ldrtoR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - DPLL1 Lock Rise"]
    #[inline(always)]
    pub fn dpll1lckr(&self) -> Dpll1lckrR {
        Dpll1lckrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DPLL1 Lock Fall"]
    #[inline(always)]
    pub fn dpll1lckf(&self) -> Dpll1lckfR {
        Dpll1lckfR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DPLL1 Lock Timeout"]
    #[inline(always)]
    pub fn dpll1lto(&self) -> Dpll1ltoR {
        Dpll1ltoR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DPLL1 Loop Divider Ratio Update Complete"]
    #[inline(always)]
    pub fn dpll1ldrto(&self) -> Dpll1ldrtoR {
        Dpll1ldrtoR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XOSC 0 Ready"]
    #[inline(always)]
    pub fn xoscrdy0(&mut self) -> Xoscrdy0W<IntflagSpec> {
        Xoscrdy0W::new(self, 0)
    }
    #[doc = "Bit 1 - XOSC 1 Ready"]
    #[inline(always)]
    pub fn xoscrdy1(&mut self) -> Xoscrdy1W<IntflagSpec> {
        Xoscrdy1W::new(self, 1)
    }
    #[doc = "Bit 2 - XOSC 0 Clock Failure Detector"]
    #[inline(always)]
    pub fn xoscfail0(&mut self) -> Xoscfail0W<IntflagSpec> {
        Xoscfail0W::new(self, 2)
    }
    #[doc = "Bit 3 - XOSC 1 Clock Failure Detector"]
    #[inline(always)]
    pub fn xoscfail1(&mut self) -> Xoscfail1W<IntflagSpec> {
        Xoscfail1W::new(self, 3)
    }
    #[doc = "Bit 8 - DFLL Ready"]
    #[inline(always)]
    pub fn dfllrdy(&mut self) -> DfllrdyW<IntflagSpec> {
        DfllrdyW::new(self, 8)
    }
    #[doc = "Bit 9 - DFLL Out Of Bounds"]
    #[inline(always)]
    pub fn dflloob(&mut self) -> DflloobW<IntflagSpec> {
        DflloobW::new(self, 9)
    }
    #[doc = "Bit 10 - DFLL Lock Fine"]
    #[inline(always)]
    pub fn dflllckf(&mut self) -> DflllckfW<IntflagSpec> {
        DflllckfW::new(self, 10)
    }
    #[doc = "Bit 11 - DFLL Lock Coarse"]
    #[inline(always)]
    pub fn dflllckc(&mut self) -> DflllckcW<IntflagSpec> {
        DflllckcW::new(self, 11)
    }
    #[doc = "Bit 12 - DFLL Reference Clock Stopped"]
    #[inline(always)]
    pub fn dfllrcs(&mut self) -> DfllrcsW<IntflagSpec> {
        DfllrcsW::new(self, 12)
    }
    #[doc = "Bit 16 - DPLL0 Lock Rise"]
    #[inline(always)]
    pub fn dpll0lckr(&mut self) -> Dpll0lckrW<IntflagSpec> {
        Dpll0lckrW::new(self, 16)
    }
    #[doc = "Bit 17 - DPLL0 Lock Fall"]
    #[inline(always)]
    pub fn dpll0lckf(&mut self) -> Dpll0lckfW<IntflagSpec> {
        Dpll0lckfW::new(self, 17)
    }
    #[doc = "Bit 18 - DPLL0 Lock Timeout"]
    #[inline(always)]
    pub fn dpll0lto(&mut self) -> Dpll0ltoW<IntflagSpec> {
        Dpll0ltoW::new(self, 18)
    }
    #[doc = "Bit 19 - DPLL0 Loop Divider Ratio Update Complete"]
    #[inline(always)]
    pub fn dpll0ldrto(&mut self) -> Dpll0ldrtoW<IntflagSpec> {
        Dpll0ldrtoW::new(self, 19)
    }
    #[doc = "Bit 24 - DPLL1 Lock Rise"]
    #[inline(always)]
    pub fn dpll1lckr(&mut self) -> Dpll1lckrW<IntflagSpec> {
        Dpll1lckrW::new(self, 24)
    }
    #[doc = "Bit 25 - DPLL1 Lock Fall"]
    #[inline(always)]
    pub fn dpll1lckf(&mut self) -> Dpll1lckfW<IntflagSpec> {
        Dpll1lckfW::new(self, 25)
    }
    #[doc = "Bit 26 - DPLL1 Lock Timeout"]
    #[inline(always)]
    pub fn dpll1lto(&mut self) -> Dpll1ltoW<IntflagSpec> {
        Dpll1ltoW::new(self, 26)
    }
    #[doc = "Bit 27 - DPLL1 Loop Divider Ratio Update Complete"]
    #[inline(always)]
    pub fn dpll1ldrto(&mut self) -> Dpll1ldrtoW<IntflagSpec> {
        Dpll1ldrtoW::new(self, 27)
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
