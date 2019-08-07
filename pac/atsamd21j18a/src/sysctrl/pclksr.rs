#[doc = "Reader of register PCLKSR"]
pub type R = crate::R<u32, super::PCLKSR>;
#[doc = "Reader of field `XOSCRDY`"]
pub type XOSCRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSC32KRDY`"]
pub type XOSC32KRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `OSC32KRDY`"]
pub type OSC32KRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `OSC8MRDY`"]
pub type OSC8MRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `DFLLRDY`"]
pub type DFLLRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `DFLLOOB`"]
pub type DFLLOOB_R = crate::R<bool, bool>;
#[doc = "Reader of field `DFLLLCKF`"]
pub type DFLLLCKF_R = crate::R<bool, bool>;
#[doc = "Reader of field `DFLLLCKC`"]
pub type DFLLLCKC_R = crate::R<bool, bool>;
#[doc = "Reader of field `DFLLRCS`"]
pub type DFLLRCS_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOD33RDY`"]
pub type BOD33RDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOD33DET`"]
pub type BOD33DET_R = crate::R<bool, bool>;
#[doc = "Reader of field `B33SRDY`"]
pub type B33SRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLLLCKR`"]
pub type DPLLLCKR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLLLCKF`"]
pub type DPLLLCKF_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLLLTO`"]
pub type DPLLLTO_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - XOSC Ready"]
    #[inline(always)]
    pub fn xoscrdy(&self) -> XOSCRDY_R {
        XOSCRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - XOSC32K Ready"]
    #[inline(always)]
    pub fn xosc32krdy(&self) -> XOSC32KRDY_R {
        XOSC32KRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OSC32K Ready"]
    #[inline(always)]
    pub fn osc32krdy(&self) -> OSC32KRDY_R {
        OSC32KRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OSC8M Ready"]
    #[inline(always)]
    pub fn osc8mrdy(&self) -> OSC8MRDY_R {
        OSC8MRDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DFLL Ready"]
    #[inline(always)]
    pub fn dfllrdy(&self) -> DFLLRDY_R {
        DFLLRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DFLL Out Of Bounds"]
    #[inline(always)]
    pub fn dflloob(&self) -> DFLLOOB_R {
        DFLLOOB_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DFLL Lock Fine"]
    #[inline(always)]
    pub fn dflllckf(&self) -> DFLLLCKF_R {
        DFLLLCKF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DFLL Lock Coarse"]
    #[inline(always)]
    pub fn dflllckc(&self) -> DFLLLCKC_R {
        DFLLLCKC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DFLL Reference Clock Stopped"]
    #[inline(always)]
    pub fn dfllrcs(&self) -> DFLLRCS_R {
        DFLLRCS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BOD33 Ready"]
    #[inline(always)]
    pub fn bod33rdy(&self) -> BOD33RDY_R {
        BOD33RDY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - BOD33 Detection"]
    #[inline(always)]
    pub fn bod33det(&self) -> BOD33DET_R {
        BOD33DET_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BOD33 Synchronization Ready"]
    #[inline(always)]
    pub fn b33srdy(&self) -> B33SRDY_R {
        B33SRDY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DPLL Lock Rise"]
    #[inline(always)]
    pub fn dplllckr(&self) -> DPLLLCKR_R {
        DPLLLCKR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DPLL Lock Fall"]
    #[inline(always)]
    pub fn dplllckf(&self) -> DPLLLCKF_R {
        DPLLLCKF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DPLL Lock Timeout"]
    #[inline(always)]
    pub fn dplllto(&self) -> DPLLLTO_R {
        DPLLLTO_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
