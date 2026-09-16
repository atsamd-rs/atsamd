#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Field `XOSCRDY0` reader - XOSC 0 Ready Interrupt Enable"]
pub type Xoscrdy0R = crate::BitReader;
#[doc = "Field `XOSCRDY0` writer - XOSC 0 Ready Interrupt Enable"]
pub type Xoscrdy0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSCRDY1` reader - XOSC 1 Ready Interrupt Enable"]
pub type Xoscrdy1R = crate::BitReader;
#[doc = "Field `XOSCRDY1` writer - XOSC 1 Ready Interrupt Enable"]
pub type Xoscrdy1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSCFAIL0` reader - XOSC 0 Clock Failure Detector Interrupt Enable"]
pub type Xoscfail0R = crate::BitReader;
#[doc = "Field `XOSCFAIL0` writer - XOSC 0 Clock Failure Detector Interrupt Enable"]
pub type Xoscfail0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSCFAIL1` reader - XOSC 1 Clock Failure Detector Interrupt Enable"]
pub type Xoscfail1R = crate::BitReader;
#[doc = "Field `XOSCFAIL1` writer - XOSC 1 Clock Failure Detector Interrupt Enable"]
pub type Xoscfail1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFLLRDY` reader - DFLL Ready Interrupt Enable"]
pub type DfllrdyR = crate::BitReader;
#[doc = "Field `DFLLRDY` writer - DFLL Ready Interrupt Enable"]
pub type DfllrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFLLOOB` reader - DFLL Out Of Bounds Interrupt Enable"]
pub type DflloobR = crate::BitReader;
#[doc = "Field `DFLLOOB` writer - DFLL Out Of Bounds Interrupt Enable"]
pub type DflloobW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFLLLCKF` reader - DFLL Lock Fine Interrupt Enable"]
pub type DflllckfR = crate::BitReader;
#[doc = "Field `DFLLLCKF` writer - DFLL Lock Fine Interrupt Enable"]
pub type DflllckfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFLLLCKC` reader - DFLL Lock Coarse Interrupt Enable"]
pub type DflllckcR = crate::BitReader;
#[doc = "Field `DFLLLCKC` writer - DFLL Lock Coarse Interrupt Enable"]
pub type DflllckcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFLLRCS` reader - DFLL Reference Clock Stopped Interrupt Enable"]
pub type DfllrcsR = crate::BitReader;
#[doc = "Field `DFLLRCS` writer - DFLL Reference Clock Stopped Interrupt Enable"]
pub type DfllrcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLL0LCKR` reader - DPLL0 Lock Rise Interrupt Enable"]
pub type Dpll0lckrR = crate::BitReader;
#[doc = "Field `DPLL0LCKR` writer - DPLL0 Lock Rise Interrupt Enable"]
pub type Dpll0lckrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLL0LCKF` reader - DPLL0 Lock Fall Interrupt Enable"]
pub type Dpll0lckfR = crate::BitReader;
#[doc = "Field `DPLL0LCKF` writer - DPLL0 Lock Fall Interrupt Enable"]
pub type Dpll0lckfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLL0LTO` reader - DPLL0 Lock Timeout Interrupt Enable"]
pub type Dpll0ltoR = crate::BitReader;
#[doc = "Field `DPLL0LTO` writer - DPLL0 Lock Timeout Interrupt Enable"]
pub type Dpll0ltoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLL0LDRTO` reader - DPLL0 Loop Divider Ratio Update Complete Interrupt Enable"]
pub type Dpll0ldrtoR = crate::BitReader;
#[doc = "Field `DPLL0LDRTO` writer - DPLL0 Loop Divider Ratio Update Complete Interrupt Enable"]
pub type Dpll0ldrtoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLL1LCKR` reader - DPLL1 Lock Rise Interrupt Enable"]
pub type Dpll1lckrR = crate::BitReader;
#[doc = "Field `DPLL1LCKR` writer - DPLL1 Lock Rise Interrupt Enable"]
pub type Dpll1lckrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLL1LCKF` reader - DPLL1 Lock Fall Interrupt Enable"]
pub type Dpll1lckfR = crate::BitReader;
#[doc = "Field `DPLL1LCKF` writer - DPLL1 Lock Fall Interrupt Enable"]
pub type Dpll1lckfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLL1LTO` reader - DPLL1 Lock Timeout Interrupt Enable"]
pub type Dpll1ltoR = crate::BitReader;
#[doc = "Field `DPLL1LTO` writer - DPLL1 Lock Timeout Interrupt Enable"]
pub type Dpll1ltoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLL1LDRTO` reader - DPLL1 Loop Divider Ratio Update Complete Interrupt Enable"]
pub type Dpll1ldrtoR = crate::BitReader;
#[doc = "Field `DPLL1LDRTO` writer - DPLL1 Loop Divider Ratio Update Complete Interrupt Enable"]
pub type Dpll1ldrtoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - XOSC 0 Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xoscrdy0(&self) -> Xoscrdy0R {
        Xoscrdy0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XOSC 1 Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xoscrdy1(&self) -> Xoscrdy1R {
        Xoscrdy1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - XOSC 0 Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    pub fn xoscfail0(&self) -> Xoscfail0R {
        Xoscfail0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - XOSC 1 Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    pub fn xoscfail1(&self) -> Xoscfail1R {
        Xoscfail1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - DFLL Ready Interrupt Enable"]
    #[inline(always)]
    pub fn dfllrdy(&self) -> DfllrdyR {
        DfllrdyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DFLL Out Of Bounds Interrupt Enable"]
    #[inline(always)]
    pub fn dflloob(&self) -> DflloobR {
        DflloobR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DFLL Lock Fine Interrupt Enable"]
    #[inline(always)]
    pub fn dflllckf(&self) -> DflllckfR {
        DflllckfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DFLL Lock Coarse Interrupt Enable"]
    #[inline(always)]
    pub fn dflllckc(&self) -> DflllckcR {
        DflllckcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DFLL Reference Clock Stopped Interrupt Enable"]
    #[inline(always)]
    pub fn dfllrcs(&self) -> DfllrcsR {
        DfllrcsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - DPLL0 Lock Rise Interrupt Enable"]
    #[inline(always)]
    pub fn dpll0lckr(&self) -> Dpll0lckrR {
        Dpll0lckrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DPLL0 Lock Fall Interrupt Enable"]
    #[inline(always)]
    pub fn dpll0lckf(&self) -> Dpll0lckfR {
        Dpll0lckfR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DPLL0 Lock Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn dpll0lto(&self) -> Dpll0ltoR {
        Dpll0ltoR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DPLL0 Loop Divider Ratio Update Complete Interrupt Enable"]
    #[inline(always)]
    pub fn dpll0ldrto(&self) -> Dpll0ldrtoR {
        Dpll0ldrtoR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - DPLL1 Lock Rise Interrupt Enable"]
    #[inline(always)]
    pub fn dpll1lckr(&self) -> Dpll1lckrR {
        Dpll1lckrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DPLL1 Lock Fall Interrupt Enable"]
    #[inline(always)]
    pub fn dpll1lckf(&self) -> Dpll1lckfR {
        Dpll1lckfR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DPLL1 Lock Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn dpll1lto(&self) -> Dpll1ltoR {
        Dpll1ltoR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DPLL1 Loop Divider Ratio Update Complete Interrupt Enable"]
    #[inline(always)]
    pub fn dpll1ldrto(&self) -> Dpll1ldrtoR {
        Dpll1ldrtoR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XOSC 0 Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xoscrdy0(&mut self) -> Xoscrdy0W<IntensetSpec> {
        Xoscrdy0W::new(self, 0)
    }
    #[doc = "Bit 1 - XOSC 1 Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xoscrdy1(&mut self) -> Xoscrdy1W<IntensetSpec> {
        Xoscrdy1W::new(self, 1)
    }
    #[doc = "Bit 2 - XOSC 0 Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xoscfail0(&mut self) -> Xoscfail0W<IntensetSpec> {
        Xoscfail0W::new(self, 2)
    }
    #[doc = "Bit 3 - XOSC 1 Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xoscfail1(&mut self) -> Xoscfail1W<IntensetSpec> {
        Xoscfail1W::new(self, 3)
    }
    #[doc = "Bit 8 - DFLL Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfllrdy(&mut self) -> DfllrdyW<IntensetSpec> {
        DfllrdyW::new(self, 8)
    }
    #[doc = "Bit 9 - DFLL Out Of Bounds Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dflloob(&mut self) -> DflloobW<IntensetSpec> {
        DflloobW::new(self, 9)
    }
    #[doc = "Bit 10 - DFLL Lock Fine Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dflllckf(&mut self) -> DflllckfW<IntensetSpec> {
        DflllckfW::new(self, 10)
    }
    #[doc = "Bit 11 - DFLL Lock Coarse Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dflllckc(&mut self) -> DflllckcW<IntensetSpec> {
        DflllckcW::new(self, 11)
    }
    #[doc = "Bit 12 - DFLL Reference Clock Stopped Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfllrcs(&mut self) -> DfllrcsW<IntensetSpec> {
        DfllrcsW::new(self, 12)
    }
    #[doc = "Bit 16 - DPLL0 Lock Rise Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dpll0lckr(&mut self) -> Dpll0lckrW<IntensetSpec> {
        Dpll0lckrW::new(self, 16)
    }
    #[doc = "Bit 17 - DPLL0 Lock Fall Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dpll0lckf(&mut self) -> Dpll0lckfW<IntensetSpec> {
        Dpll0lckfW::new(self, 17)
    }
    #[doc = "Bit 18 - DPLL0 Lock Timeout Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dpll0lto(&mut self) -> Dpll0ltoW<IntensetSpec> {
        Dpll0ltoW::new(self, 18)
    }
    #[doc = "Bit 19 - DPLL0 Loop Divider Ratio Update Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dpll0ldrto(&mut self) -> Dpll0ldrtoW<IntensetSpec> {
        Dpll0ldrtoW::new(self, 19)
    }
    #[doc = "Bit 24 - DPLL1 Lock Rise Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dpll1lckr(&mut self) -> Dpll1lckrW<IntensetSpec> {
        Dpll1lckrW::new(self, 24)
    }
    #[doc = "Bit 25 - DPLL1 Lock Fall Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dpll1lckf(&mut self) -> Dpll1lckfW<IntensetSpec> {
        Dpll1lckfW::new(self, 25)
    }
    #[doc = "Bit 26 - DPLL1 Lock Timeout Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dpll1lto(&mut self) -> Dpll1ltoW<IntensetSpec> {
        Dpll1ltoW::new(self, 26)
    }
    #[doc = "Bit 27 - DPLL1 Loop Divider Ratio Update Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dpll1ldrto(&mut self) -> Dpll1ldrtoW<IntensetSpec> {
        Dpll1ldrtoW::new(self, 27)
    }
}
#[doc = "Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {
    const RESET_VALUE: u32 = 0;
}
