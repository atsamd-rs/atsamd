#[doc = "Register `PMC_SCSR` reader"]
pub struct R(crate::R<PMC_SCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_SCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_SCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_SCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HCLKS` reader - HCLK Status"]
pub struct HCLKS_R(crate::FieldReader<bool, bool>);
impl HCLKS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HCLKS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HCLKS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBCLK` reader - USB FS Clock Status"]
pub struct USBCLK_R(crate::FieldReader<bool, bool>);
impl USBCLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USBCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCK0` reader - Programmable Clock 0 Output Status"]
pub struct PCK0_R(crate::FieldReader<bool, bool>);
impl PCK0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCK0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCK0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCK1` reader - Programmable Clock 1 Output Status"]
pub struct PCK1_R(crate::FieldReader<bool, bool>);
impl PCK1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCK1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCK2` reader - Programmable Clock 2 Output Status"]
pub struct PCK2_R(crate::FieldReader<bool, bool>);
impl PCK2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCK2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCK2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCK3` reader - Programmable Clock 3 Output Status"]
pub struct PCK3_R(crate::FieldReader<bool, bool>);
impl PCK3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCK3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCK3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCK4` reader - Programmable Clock 4 Output Status"]
pub struct PCK4_R(crate::FieldReader<bool, bool>);
impl PCK4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCK4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCK4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCK5` reader - Programmable Clock 5 Output Status"]
pub struct PCK5_R(crate::FieldReader<bool, bool>);
impl PCK5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCK5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCK5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCK6` reader - Programmable Clock 6 Output Status"]
pub struct PCK6_R(crate::FieldReader<bool, bool>);
impl PCK6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCK6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCK6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCK7` reader - Programmable Clock 7 Output Status"]
pub struct PCK7_R(crate::FieldReader<bool, bool>);
impl PCK7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCK7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCK7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - HCLK Status"]
    #[inline(always)]
    pub fn hclks(&self) -> HCLKS_R {
        HCLKS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 5 - USB FS Clock Status"]
    #[inline(always)]
    pub fn usbclk(&self) -> USBCLK_R {
        USBCLK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Programmable Clock 0 Output Status"]
    #[inline(always)]
    pub fn pck0(&self) -> PCK0_R {
        PCK0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Programmable Clock 1 Output Status"]
    #[inline(always)]
    pub fn pck1(&self) -> PCK1_R {
        PCK1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Programmable Clock 2 Output Status"]
    #[inline(always)]
    pub fn pck2(&self) -> PCK2_R {
        PCK2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Programmable Clock 3 Output Status"]
    #[inline(always)]
    pub fn pck3(&self) -> PCK3_R {
        PCK3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Programmable Clock 4 Output Status"]
    #[inline(always)]
    pub fn pck4(&self) -> PCK4_R {
        PCK4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Programmable Clock 5 Output Status"]
    #[inline(always)]
    pub fn pck5(&self) -> PCK5_R {
        PCK5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Programmable Clock 6 Output Status"]
    #[inline(always)]
    pub fn pck6(&self) -> PCK6_R {
        PCK6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Programmable Clock 7 Output Status"]
    #[inline(always)]
    pub fn pck7(&self) -> PCK7_R {
        PCK7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "System Clock Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_scsr](index.html) module"]
pub struct PMC_SCSR_SPEC;
impl crate::RegisterSpec for PMC_SCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_scsr::R](R) reader structure"]
impl crate::Readable for PMC_SCSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PMC_SCSR to value 0"]
impl crate::Resettable for PMC_SCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
