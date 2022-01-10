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
pub struct XOSCRDY_R(crate::FieldReader<bool, bool>);
impl XOSCRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XOSCRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSCRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSC32KRDY` reader - XOSC32K Ready"]
pub struct XOSC32KRDY_R(crate::FieldReader<bool, bool>);
impl XOSC32KRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XOSC32KRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSC32KRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSC32KRDY` reader - OSC32K Ready"]
pub struct OSC32KRDY_R(crate::FieldReader<bool, bool>);
impl OSC32KRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSC32KRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC32KRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSC8MRDY` reader - OSC8M Ready"]
pub struct OSC8MRDY_R(crate::FieldReader<bool, bool>);
impl OSC8MRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSC8MRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC8MRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFLLRDY` reader - DFLL Ready"]
pub struct DFLLRDY_R(crate::FieldReader<bool, bool>);
impl DFLLRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFLLRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFLLRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFLLOOB` reader - DFLL Out Of Bounds"]
pub struct DFLLOOB_R(crate::FieldReader<bool, bool>);
impl DFLLOOB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFLLOOB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFLLOOB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFLLLCKF` reader - DFLL Lock Fine"]
pub struct DFLLLCKF_R(crate::FieldReader<bool, bool>);
impl DFLLLCKF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFLLLCKF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFLLLCKF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFLLLCKC` reader - DFLL Lock Coarse"]
pub struct DFLLLCKC_R(crate::FieldReader<bool, bool>);
impl DFLLLCKC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFLLLCKC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFLLLCKC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFLLRCS` reader - DFLL Reference Clock Stopped"]
pub struct DFLLRCS_R(crate::FieldReader<bool, bool>);
impl DFLLRCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFLLRCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFLLRCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOD33RDY` reader - BOD33 Ready"]
pub struct BOD33RDY_R(crate::FieldReader<bool, bool>);
impl BOD33RDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BOD33RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOD33RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOD33DET` reader - BOD33 Detection"]
pub struct BOD33DET_R(crate::FieldReader<bool, bool>);
impl BOD33DET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BOD33DET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOD33DET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B33SRDY` reader - BOD33 Synchronization Ready"]
pub struct B33SRDY_R(crate::FieldReader<bool, bool>);
impl B33SRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        B33SRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B33SRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLLLCKR` reader - DPLL Lock Rise"]
pub struct DPLLLCKR_R(crate::FieldReader<bool, bool>);
impl DPLLLCKR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLLLCKR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLLLCKR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLLLCKF` reader - DPLL Lock Fall"]
pub struct DPLLLCKF_R(crate::FieldReader<bool, bool>);
impl DPLLLCKF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLLLCKF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLLLCKF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLLLTO` reader - DPLL Lock Timeout"]
pub struct DPLLLTO_R(crate::FieldReader<bool, bool>);
impl DPLLLTO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLLLTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLLLTO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
