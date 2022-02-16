#[doc = "Register `PMC_IMR` reader"]
pub struct R(crate::R<PMC_IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MOSCXTS` reader - Main Crystal Oscillator Status Interrupt Mask"]
pub struct MOSCXTS_R(crate::FieldReader<bool, bool>);
impl MOSCXTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MOSCXTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOSCXTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKA` reader - PLLA Lock Interrupt Mask"]
pub struct LOCKA_R(crate::FieldReader<bool, bool>);
impl LOCKA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCKA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCKA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCKRDY` reader - Master Clock Ready Interrupt Mask"]
pub struct MCKRDY_R(crate::FieldReader<bool, bool>);
impl MCKRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCKRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCKRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKU` reader - UTMI PLL Lock Interrupt Mask"]
pub struct LOCKU_R(crate::FieldReader<bool, bool>);
impl LOCKU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCKU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCKU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCKRDY0` reader - Programmable Clock Ready 0 Interrupt Mask"]
pub struct PCKRDY0_R(crate::FieldReader<bool, bool>);
impl PCKRDY0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCKRDY0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCKRDY0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCKRDY1` reader - Programmable Clock Ready 1 Interrupt Mask"]
pub struct PCKRDY1_R(crate::FieldReader<bool, bool>);
impl PCKRDY1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCKRDY1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCKRDY1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCKRDY2` reader - Programmable Clock Ready 2 Interrupt Mask"]
pub struct PCKRDY2_R(crate::FieldReader<bool, bool>);
impl PCKRDY2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCKRDY2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCKRDY2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCKRDY3` reader - Programmable Clock Ready 3 Interrupt Mask"]
pub struct PCKRDY3_R(crate::FieldReader<bool, bool>);
impl PCKRDY3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCKRDY3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCKRDY3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCKRDY4` reader - Programmable Clock Ready 4 Interrupt Mask"]
pub struct PCKRDY4_R(crate::FieldReader<bool, bool>);
impl PCKRDY4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCKRDY4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCKRDY4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCKRDY5` reader - Programmable Clock Ready 5 Interrupt Mask"]
pub struct PCKRDY5_R(crate::FieldReader<bool, bool>);
impl PCKRDY5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCKRDY5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCKRDY5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCKRDY6` reader - Programmable Clock Ready 6 Interrupt Mask"]
pub struct PCKRDY6_R(crate::FieldReader<bool, bool>);
impl PCKRDY6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCKRDY6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCKRDY6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCKRDY7` reader - Programmable Clock Ready 7 Interrupt Mask"]
pub struct PCKRDY7_R(crate::FieldReader<bool, bool>);
impl PCKRDY7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCKRDY7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCKRDY7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOSCSELS` reader - Main Clock Source Oscillator Selection Status Interrupt Mask"]
pub struct MOSCSELS_R(crate::FieldReader<bool, bool>);
impl MOSCSELS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MOSCSELS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOSCSELS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOSCRCS` reader - Main RC Status Interrupt Mask"]
pub struct MOSCRCS_R(crate::FieldReader<bool, bool>);
impl MOSCRCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MOSCRCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOSCRCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFDEV` reader - Clock Failure Detector Event Interrupt Mask"]
pub struct CFDEV_R(crate::FieldReader<bool, bool>);
impl CFDEV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFDEV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFDEV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XT32KERR` reader - 32.768 kHz Crystal Oscillator Error Interrupt Mask"]
pub struct XT32KERR_R(crate::FieldReader<bool, bool>);
impl XT32KERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XT32KERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XT32KERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Main Crystal Oscillator Status Interrupt Mask"]
    #[inline(always)]
    pub fn moscxts(&self) -> MOSCXTS_R {
        MOSCXTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PLLA Lock Interrupt Mask"]
    #[inline(always)]
    pub fn locka(&self) -> LOCKA_R {
        LOCKA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Master Clock Ready Interrupt Mask"]
    #[inline(always)]
    pub fn mckrdy(&self) -> MCKRDY_R {
        MCKRDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - UTMI PLL Lock Interrupt Mask"]
    #[inline(always)]
    pub fn locku(&self) -> LOCKU_R {
        LOCKU_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Programmable Clock Ready 0 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy0(&self) -> PCKRDY0_R {
        PCKRDY0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Programmable Clock Ready 1 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy1(&self) -> PCKRDY1_R {
        PCKRDY1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Programmable Clock Ready 2 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy2(&self) -> PCKRDY2_R {
        PCKRDY2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Programmable Clock Ready 3 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy3(&self) -> PCKRDY3_R {
        PCKRDY3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Programmable Clock Ready 4 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy4(&self) -> PCKRDY4_R {
        PCKRDY4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Programmable Clock Ready 5 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy5(&self) -> PCKRDY5_R {
        PCKRDY5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Programmable Clock Ready 6 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy6(&self) -> PCKRDY6_R {
        PCKRDY6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Programmable Clock Ready 7 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy7(&self) -> PCKRDY7_R {
        PCKRDY7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Main Clock Source Oscillator Selection Status Interrupt Mask"]
    #[inline(always)]
    pub fn moscsels(&self) -> MOSCSELS_R {
        MOSCSELS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Main RC Status Interrupt Mask"]
    #[inline(always)]
    pub fn moscrcs(&self) -> MOSCRCS_R {
        MOSCRCS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Clock Failure Detector Event Interrupt Mask"]
    #[inline(always)]
    pub fn cfdev(&self) -> CFDEV_R {
        CFDEV_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 32.768 kHz Crystal Oscillator Error Interrupt Mask"]
    #[inline(always)]
    pub fn xt32kerr(&self) -> XT32KERR_R {
        XT32KERR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_imr](index.html) module"]
pub struct PMC_IMR_SPEC;
impl crate::RegisterSpec for PMC_IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_imr::R](R) reader structure"]
impl crate::Readable for PMC_IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PMC_IMR to value 0"]
impl crate::Resettable for PMC_IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
