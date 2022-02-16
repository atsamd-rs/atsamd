#[doc = "Register `PMC_PCSR0` reader"]
pub struct R(crate::R<PMC_PCSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_PCSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_PCSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_PCSR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PID7` reader - Peripheral Clock 7 Status"]
pub struct PID7_R(crate::FieldReader<bool, bool>);
impl PID7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID8` reader - Peripheral Clock 8 Status"]
pub struct PID8_R(crate::FieldReader<bool, bool>);
impl PID8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID10` reader - Peripheral Clock 10 Status"]
pub struct PID10_R(crate::FieldReader<bool, bool>);
impl PID10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID11` reader - Peripheral Clock 11 Status"]
pub struct PID11_R(crate::FieldReader<bool, bool>);
impl PID11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID13` reader - Peripheral Clock 13 Status"]
pub struct PID13_R(crate::FieldReader<bool, bool>);
impl PID13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID14` reader - Peripheral Clock 14 Status"]
pub struct PID14_R(crate::FieldReader<bool, bool>);
impl PID14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID15` reader - Peripheral Clock 15 Status"]
pub struct PID15_R(crate::FieldReader<bool, bool>);
impl PID15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID16` reader - Peripheral Clock 16 Status"]
pub struct PID16_R(crate::FieldReader<bool, bool>);
impl PID16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID18` reader - Peripheral Clock 18 Status"]
pub struct PID18_R(crate::FieldReader<bool, bool>);
impl PID18_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID19` reader - Peripheral Clock 19 Status"]
pub struct PID19_R(crate::FieldReader<bool, bool>);
impl PID19_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID20` reader - Peripheral Clock 20 Status"]
pub struct PID20_R(crate::FieldReader<bool, bool>);
impl PID20_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID21` reader - Peripheral Clock 21 Status"]
pub struct PID21_R(crate::FieldReader<bool, bool>);
impl PID21_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID22` reader - Peripheral Clock 22 Status"]
pub struct PID22_R(crate::FieldReader<bool, bool>);
impl PID22_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID23` reader - Peripheral Clock 23 Status"]
pub struct PID23_R(crate::FieldReader<bool, bool>);
impl PID23_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID24` reader - Peripheral Clock 24 Status"]
pub struct PID24_R(crate::FieldReader<bool, bool>);
impl PID24_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID25` reader - Peripheral Clock 25 Status"]
pub struct PID25_R(crate::FieldReader<bool, bool>);
impl PID25_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID26` reader - Peripheral Clock 26 Status"]
pub struct PID26_R(crate::FieldReader<bool, bool>);
impl PID26_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID27` reader - Peripheral Clock 27 Status"]
pub struct PID27_R(crate::FieldReader<bool, bool>);
impl PID27_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID28` reader - Peripheral Clock 28 Status"]
pub struct PID28_R(crate::FieldReader<bool, bool>);
impl PID28_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID29` reader - Peripheral Clock 29 Status"]
pub struct PID29_R(crate::FieldReader<bool, bool>);
impl PID29_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID30` reader - Peripheral Clock 30 Status"]
pub struct PID30_R(crate::FieldReader<bool, bool>);
impl PID30_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID31` reader - Peripheral Clock 31 Status"]
pub struct PID31_R(crate::FieldReader<bool, bool>);
impl PID31_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 7 - Peripheral Clock 7 Status"]
    #[inline(always)]
    pub fn pid7(&self) -> PID7_R {
        PID7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Peripheral Clock 8 Status"]
    #[inline(always)]
    pub fn pid8(&self) -> PID8_R {
        PID8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Peripheral Clock 10 Status"]
    #[inline(always)]
    pub fn pid10(&self) -> PID10_R {
        PID10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Peripheral Clock 11 Status"]
    #[inline(always)]
    pub fn pid11(&self) -> PID11_R {
        PID11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Peripheral Clock 13 Status"]
    #[inline(always)]
    pub fn pid13(&self) -> PID13_R {
        PID13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Peripheral Clock 14 Status"]
    #[inline(always)]
    pub fn pid14(&self) -> PID14_R {
        PID14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Peripheral Clock 15 Status"]
    #[inline(always)]
    pub fn pid15(&self) -> PID15_R {
        PID15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Peripheral Clock 16 Status"]
    #[inline(always)]
    pub fn pid16(&self) -> PID16_R {
        PID16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Peripheral Clock 18 Status"]
    #[inline(always)]
    pub fn pid18(&self) -> PID18_R {
        PID18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Peripheral Clock 19 Status"]
    #[inline(always)]
    pub fn pid19(&self) -> PID19_R {
        PID19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Peripheral Clock 20 Status"]
    #[inline(always)]
    pub fn pid20(&self) -> PID20_R {
        PID20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Peripheral Clock 21 Status"]
    #[inline(always)]
    pub fn pid21(&self) -> PID21_R {
        PID21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Peripheral Clock 22 Status"]
    #[inline(always)]
    pub fn pid22(&self) -> PID22_R {
        PID22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Peripheral Clock 23 Status"]
    #[inline(always)]
    pub fn pid23(&self) -> PID23_R {
        PID23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Peripheral Clock 24 Status"]
    #[inline(always)]
    pub fn pid24(&self) -> PID24_R {
        PID24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Peripheral Clock 25 Status"]
    #[inline(always)]
    pub fn pid25(&self) -> PID25_R {
        PID25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Peripheral Clock 26 Status"]
    #[inline(always)]
    pub fn pid26(&self) -> PID26_R {
        PID26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Peripheral Clock 27 Status"]
    #[inline(always)]
    pub fn pid27(&self) -> PID27_R {
        PID27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Peripheral Clock 28 Status"]
    #[inline(always)]
    pub fn pid28(&self) -> PID28_R {
        PID28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Peripheral Clock 29 Status"]
    #[inline(always)]
    pub fn pid29(&self) -> PID29_R {
        PID29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Peripheral Clock 30 Status"]
    #[inline(always)]
    pub fn pid30(&self) -> PID30_R {
        PID30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Peripheral Clock 31 Status"]
    #[inline(always)]
    pub fn pid31(&self) -> PID31_R {
        PID31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Peripheral Clock Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_pcsr0](index.html) module"]
pub struct PMC_PCSR0_SPEC;
impl crate::RegisterSpec for PMC_PCSR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_pcsr0::R](R) reader structure"]
impl crate::Readable for PMC_PCSR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PMC_PCSR0 to value 0"]
impl crate::Resettable for PMC_PCSR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
