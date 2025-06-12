#[doc = "Register `PCLKSR` reader"]
pub type R = crate::R<PclksrSpec>;
#[doc = "Field `XOSCRDY` reader - XOSC Ready"]
pub type XoscrdyR = crate::BitReader;
#[doc = "Field `XOSC32KRDY` reader - XOSC32K Ready"]
pub type Xosc32krdyR = crate::BitReader;
#[doc = "Field `OSC32KRDY` reader - OSC32K Ready"]
pub type Osc32krdyR = crate::BitReader;
#[doc = "Field `OSC8MRDY` reader - OSC8M Ready"]
pub type Osc8mrdyR = crate::BitReader;
#[doc = "Field `DFLLRDY` reader - DFLL Ready"]
pub type DfllrdyR = crate::BitReader;
#[doc = "Field `DFLLOOB` reader - DFLL Out Of Bounds"]
pub type DflloobR = crate::BitReader;
#[doc = "Field `DFLLLCKF` reader - DFLL Lock Fine"]
pub type DflllckfR = crate::BitReader;
#[doc = "Field `DFLLLCKC` reader - DFLL Lock Coarse"]
pub type DflllckcR = crate::BitReader;
#[doc = "Field `DFLLRCS` reader - DFLL Reference Clock Stopped"]
pub type DfllrcsR = crate::BitReader;
#[doc = "Field `BOD33RDY` reader - BOD33 Ready"]
pub type Bod33rdyR = crate::BitReader;
#[doc = "Field `BOD33DET` reader - BOD33 Detection"]
pub type Bod33detR = crate::BitReader;
#[doc = "Field `B33SRDY` reader - BOD33 Synchronization Ready"]
pub type B33srdyR = crate::BitReader;
#[doc = "Field `DPLLLCKR` reader - DPLL Lock Rise"]
pub type DplllckrR = crate::BitReader;
#[doc = "Field `DPLLLCKF` reader - DPLL Lock Fall"]
pub type DplllckfR = crate::BitReader;
#[doc = "Field `DPLLLTO` reader - DPLL Lock Timeout"]
pub type DpllltoR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - XOSC Ready"]
    #[inline(always)]
    pub fn xoscrdy(&self) -> XoscrdyR {
        XoscrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XOSC32K Ready"]
    #[inline(always)]
    pub fn xosc32krdy(&self) -> Xosc32krdyR {
        Xosc32krdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OSC32K Ready"]
    #[inline(always)]
    pub fn osc32krdy(&self) -> Osc32krdyR {
        Osc32krdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OSC8M Ready"]
    #[inline(always)]
    pub fn osc8mrdy(&self) -> Osc8mrdyR {
        Osc8mrdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DFLL Ready"]
    #[inline(always)]
    pub fn dfllrdy(&self) -> DfllrdyR {
        DfllrdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DFLL Out Of Bounds"]
    #[inline(always)]
    pub fn dflloob(&self) -> DflloobR {
        DflloobR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DFLL Lock Fine"]
    #[inline(always)]
    pub fn dflllckf(&self) -> DflllckfR {
        DflllckfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DFLL Lock Coarse"]
    #[inline(always)]
    pub fn dflllckc(&self) -> DflllckcR {
        DflllckcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DFLL Reference Clock Stopped"]
    #[inline(always)]
    pub fn dfllrcs(&self) -> DfllrcsR {
        DfllrcsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BOD33 Ready"]
    #[inline(always)]
    pub fn bod33rdy(&self) -> Bod33rdyR {
        Bod33rdyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BOD33 Detection"]
    #[inline(always)]
    pub fn bod33det(&self) -> Bod33detR {
        Bod33detR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BOD33 Synchronization Ready"]
    #[inline(always)]
    pub fn b33srdy(&self) -> B33srdyR {
        B33srdyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - DPLL Lock Rise"]
    #[inline(always)]
    pub fn dplllckr(&self) -> DplllckrR {
        DplllckrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DPLL Lock Fall"]
    #[inline(always)]
    pub fn dplllckf(&self) -> DplllckfR {
        DplllckfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DPLL Lock Timeout"]
    #[inline(always)]
    pub fn dplllto(&self) -> DpllltoR {
        DpllltoR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Power and Clocks Status\n\nYou can [`read`](crate::Reg::read) this register and get [`pclksr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PclksrSpec;
impl crate::RegisterSpec for PclksrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pclksr::R`](R) reader structure"]
impl crate::Readable for PclksrSpec {}
#[doc = "`reset()` method sets PCLKSR to value 0"]
impl crate::Resettable for PclksrSpec {}
