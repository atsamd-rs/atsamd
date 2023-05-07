#[doc = "Register `PCLKSR` reader"]
pub struct R(crate::R<PCLKSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCLKSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCLKSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCLKSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `XOSCRDY` reader - XOSC Ready"]
pub type XOSCRDY_R = crate::BitReader<bool>;
#[doc = "Field `XOSC32KRDY` reader - XOSC32K Ready"]
pub type XOSC32KRDY_R = crate::BitReader<bool>;
#[doc = "Field `OSC32KRDY` reader - OSC32K Ready"]
pub type OSC32KRDY_R = crate::BitReader<bool>;
#[doc = "Field `OSC8MRDY` reader - OSC8M Ready"]
pub type OSC8MRDY_R = crate::BitReader<bool>;
#[doc = "Field `DFLLRDY` reader - DFLL Ready"]
pub type DFLLRDY_R = crate::BitReader<bool>;
#[doc = "Field `DFLLOOB` reader - DFLL Out Of Bounds"]
pub type DFLLOOB_R = crate::BitReader<bool>;
#[doc = "Field `DFLLLCKF` reader - DFLL Lock Fine"]
pub type DFLLLCKF_R = crate::BitReader<bool>;
#[doc = "Field `DFLLLCKC` reader - DFLL Lock Coarse"]
pub type DFLLLCKC_R = crate::BitReader<bool>;
#[doc = "Field `DFLLRCS` reader - DFLL Reference Clock Stopped"]
pub type DFLLRCS_R = crate::BitReader<bool>;
#[doc = "Field `BOD33RDY` reader - BOD33 Ready"]
pub type BOD33RDY_R = crate::BitReader<bool>;
#[doc = "Field `BOD33DET` reader - BOD33 Detection"]
pub type BOD33DET_R = crate::BitReader<bool>;
#[doc = "Field `B33SRDY` reader - BOD33 Synchronization Ready"]
pub type B33SRDY_R = crate::BitReader<bool>;
#[doc = "Field `DPLLLCKR` reader - DPLL Lock Rise"]
pub type DPLLLCKR_R = crate::BitReader<bool>;
#[doc = "Field `DPLLLCKF` reader - DPLL Lock Fall"]
pub type DPLLLCKF_R = crate::BitReader<bool>;
#[doc = "Field `DPLLLTO` reader - DPLL Lock Timeout"]
pub type DPLLLTO_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - XOSC Ready"]
    #[inline(always)]
    pub fn xoscrdy(&self) -> XOSCRDY_R {
        XOSCRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XOSC32K Ready"]
    #[inline(always)]
    pub fn xosc32krdy(&self) -> XOSC32KRDY_R {
        XOSC32KRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OSC32K Ready"]
    #[inline(always)]
    pub fn osc32krdy(&self) -> OSC32KRDY_R {
        OSC32KRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OSC8M Ready"]
    #[inline(always)]
    pub fn osc8mrdy(&self) -> OSC8MRDY_R {
        OSC8MRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DFLL Ready"]
    #[inline(always)]
    pub fn dfllrdy(&self) -> DFLLRDY_R {
        DFLLRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DFLL Out Of Bounds"]
    #[inline(always)]
    pub fn dflloob(&self) -> DFLLOOB_R {
        DFLLOOB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DFLL Lock Fine"]
    #[inline(always)]
    pub fn dflllckf(&self) -> DFLLLCKF_R {
        DFLLLCKF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DFLL Lock Coarse"]
    #[inline(always)]
    pub fn dflllckc(&self) -> DFLLLCKC_R {
        DFLLLCKC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DFLL Reference Clock Stopped"]
    #[inline(always)]
    pub fn dfllrcs(&self) -> DFLLRCS_R {
        DFLLRCS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BOD33 Ready"]
    #[inline(always)]
    pub fn bod33rdy(&self) -> BOD33RDY_R {
        BOD33RDY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BOD33 Detection"]
    #[inline(always)]
    pub fn bod33det(&self) -> BOD33DET_R {
        BOD33DET_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BOD33 Synchronization Ready"]
    #[inline(always)]
    pub fn b33srdy(&self) -> B33SRDY_R {
        B33SRDY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - DPLL Lock Rise"]
    #[inline(always)]
    pub fn dplllckr(&self) -> DPLLLCKR_R {
        DPLLLCKR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DPLL Lock Fall"]
    #[inline(always)]
    pub fn dplllckf(&self) -> DPLLLCKF_R {
        DPLLLCKF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DPLL Lock Timeout"]
    #[inline(always)]
    pub fn dplllto(&self) -> DPLLLTO_R {
        DPLLLTO_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Power and Clocks Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pclksr](index.html) module"]
pub struct PCLKSR_SPEC;
impl crate::RegisterSpec for PCLKSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pclksr::R](R) reader structure"]
impl crate::Readable for PCLKSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PCLKSR to value 0"]
impl crate::Resettable for PCLKSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
