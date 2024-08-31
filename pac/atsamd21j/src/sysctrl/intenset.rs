#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Field `XOSCRDY` reader - XOSC Ready Interrupt Enable"]
pub type XoscrdyR = crate::BitReader;
#[doc = "Field `XOSCRDY` writer - XOSC Ready Interrupt Enable"]
pub type XoscrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSC32KRDY` reader - XOSC32K Ready Interrupt Enable"]
pub type Xosc32krdyR = crate::BitReader;
#[doc = "Field `XOSC32KRDY` writer - XOSC32K Ready Interrupt Enable"]
pub type Xosc32krdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSC32KRDY` reader - OSC32K Ready Interrupt Enable"]
pub type Osc32krdyR = crate::BitReader;
#[doc = "Field `OSC32KRDY` writer - OSC32K Ready Interrupt Enable"]
pub type Osc32krdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSC8MRDY` reader - OSC8M Ready Interrupt Enable"]
pub type Osc8mrdyR = crate::BitReader;
#[doc = "Field `OSC8MRDY` writer - OSC8M Ready Interrupt Enable"]
pub type Osc8mrdyW<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `BOD33RDY` reader - BOD33 Ready Interrupt Enable"]
pub type Bod33rdyR = crate::BitReader;
#[doc = "Field `BOD33RDY` writer - BOD33 Ready Interrupt Enable"]
pub type Bod33rdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOD33DET` reader - BOD33 Detection Interrupt Enable"]
pub type Bod33detR = crate::BitReader;
#[doc = "Field `BOD33DET` writer - BOD33 Detection Interrupt Enable"]
pub type Bod33detW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B33SRDY` reader - BOD33 Synchronization Ready Interrupt Enable"]
pub type B33srdyR = crate::BitReader;
#[doc = "Field `B33SRDY` writer - BOD33 Synchronization Ready Interrupt Enable"]
pub type B33srdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLLLCKR` reader - DPLL Lock Rise Interrupt Enable"]
pub type DplllckrR = crate::BitReader;
#[doc = "Field `DPLLLCKR` writer - DPLL Lock Rise Interrupt Enable"]
pub type DplllckrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLLLCKF` reader - DPLL Lock Fall Interrupt Enable"]
pub type DplllckfR = crate::BitReader;
#[doc = "Field `DPLLLCKF` writer - DPLL Lock Fall Interrupt Enable"]
pub type DplllckfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLLLTO` reader - DPLL Lock Timeout Interrupt Enable"]
pub type DpllltoR = crate::BitReader;
#[doc = "Field `DPLLLTO` writer - DPLL Lock Timeout Interrupt Enable"]
pub type DpllltoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - XOSC Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xoscrdy(&self) -> XoscrdyR {
        XoscrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XOSC32K Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xosc32krdy(&self) -> Xosc32krdyR {
        Xosc32krdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OSC32K Ready Interrupt Enable"]
    #[inline(always)]
    pub fn osc32krdy(&self) -> Osc32krdyR {
        Osc32krdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OSC8M Ready Interrupt Enable"]
    #[inline(always)]
    pub fn osc8mrdy(&self) -> Osc8mrdyR {
        Osc8mrdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DFLL Ready Interrupt Enable"]
    #[inline(always)]
    pub fn dfllrdy(&self) -> DfllrdyR {
        DfllrdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DFLL Out Of Bounds Interrupt Enable"]
    #[inline(always)]
    pub fn dflloob(&self) -> DflloobR {
        DflloobR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DFLL Lock Fine Interrupt Enable"]
    #[inline(always)]
    pub fn dflllckf(&self) -> DflllckfR {
        DflllckfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DFLL Lock Coarse Interrupt Enable"]
    #[inline(always)]
    pub fn dflllckc(&self) -> DflllckcR {
        DflllckcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DFLL Reference Clock Stopped Interrupt Enable"]
    #[inline(always)]
    pub fn dfllrcs(&self) -> DfllrcsR {
        DfllrcsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BOD33 Ready Interrupt Enable"]
    #[inline(always)]
    pub fn bod33rdy(&self) -> Bod33rdyR {
        Bod33rdyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BOD33 Detection Interrupt Enable"]
    #[inline(always)]
    pub fn bod33det(&self) -> Bod33detR {
        Bod33detR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BOD33 Synchronization Ready Interrupt Enable"]
    #[inline(always)]
    pub fn b33srdy(&self) -> B33srdyR {
        B33srdyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - DPLL Lock Rise Interrupt Enable"]
    #[inline(always)]
    pub fn dplllckr(&self) -> DplllckrR {
        DplllckrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DPLL Lock Fall Interrupt Enable"]
    #[inline(always)]
    pub fn dplllckf(&self) -> DplllckfR {
        DplllckfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DPLL Lock Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn dplllto(&self) -> DpllltoR {
        DpllltoR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XOSC Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xoscrdy(&mut self) -> XoscrdyW<IntensetSpec> {
        XoscrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - XOSC32K Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xosc32krdy(&mut self) -> Xosc32krdyW<IntensetSpec> {
        Xosc32krdyW::new(self, 1)
    }
    #[doc = "Bit 2 - OSC32K Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn osc32krdy(&mut self) -> Osc32krdyW<IntensetSpec> {
        Osc32krdyW::new(self, 2)
    }
    #[doc = "Bit 3 - OSC8M Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn osc8mrdy(&mut self) -> Osc8mrdyW<IntensetSpec> {
        Osc8mrdyW::new(self, 3)
    }
    #[doc = "Bit 4 - DFLL Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfllrdy(&mut self) -> DfllrdyW<IntensetSpec> {
        DfllrdyW::new(self, 4)
    }
    #[doc = "Bit 5 - DFLL Out Of Bounds Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dflloob(&mut self) -> DflloobW<IntensetSpec> {
        DflloobW::new(self, 5)
    }
    #[doc = "Bit 6 - DFLL Lock Fine Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dflllckf(&mut self) -> DflllckfW<IntensetSpec> {
        DflllckfW::new(self, 6)
    }
    #[doc = "Bit 7 - DFLL Lock Coarse Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dflllckc(&mut self) -> DflllckcW<IntensetSpec> {
        DflllckcW::new(self, 7)
    }
    #[doc = "Bit 8 - DFLL Reference Clock Stopped Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfllrcs(&mut self) -> DfllrcsW<IntensetSpec> {
        DfllrcsW::new(self, 8)
    }
    #[doc = "Bit 9 - BOD33 Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bod33rdy(&mut self) -> Bod33rdyW<IntensetSpec> {
        Bod33rdyW::new(self, 9)
    }
    #[doc = "Bit 10 - BOD33 Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bod33det(&mut self) -> Bod33detW<IntensetSpec> {
        Bod33detW::new(self, 10)
    }
    #[doc = "Bit 11 - BOD33 Synchronization Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn b33srdy(&mut self) -> B33srdyW<IntensetSpec> {
        B33srdyW::new(self, 11)
    }
    #[doc = "Bit 15 - DPLL Lock Rise Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dplllckr(&mut self) -> DplllckrW<IntensetSpec> {
        DplllckrW::new(self, 15)
    }
    #[doc = "Bit 16 - DPLL Lock Fall Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dplllckf(&mut self) -> DplllckfW<IntensetSpec> {
        DplllckfW::new(self, 16)
    }
    #[doc = "Bit 17 - DPLL Lock Timeout Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dplllto(&mut self) -> DpllltoW<IntensetSpec> {
        DpllltoW::new(self, 17)
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
