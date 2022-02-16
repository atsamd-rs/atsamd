#[doc = "Register `XDMAC_GSWS` reader"]
pub struct R(crate::R<XDMAC_GSWS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XDMAC_GSWS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XDMAC_GSWS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XDMAC_GSWS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SWRS0` reader - XDMAC Channel 0 Software Request Status Bit"]
pub struct SWRS0_R(crate::FieldReader<bool, bool>);
impl SWRS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRS1` reader - XDMAC Channel 1 Software Request Status Bit"]
pub struct SWRS1_R(crate::FieldReader<bool, bool>);
impl SWRS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRS2` reader - XDMAC Channel 2 Software Request Status Bit"]
pub struct SWRS2_R(crate::FieldReader<bool, bool>);
impl SWRS2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRS3` reader - XDMAC Channel 3 Software Request Status Bit"]
pub struct SWRS3_R(crate::FieldReader<bool, bool>);
impl SWRS3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRS3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRS4` reader - XDMAC Channel 4 Software Request Status Bit"]
pub struct SWRS4_R(crate::FieldReader<bool, bool>);
impl SWRS4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRS4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRS4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRS5` reader - XDMAC Channel 5 Software Request Status Bit"]
pub struct SWRS5_R(crate::FieldReader<bool, bool>);
impl SWRS5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRS5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRS5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRS6` reader - XDMAC Channel 6 Software Request Status Bit"]
pub struct SWRS6_R(crate::FieldReader<bool, bool>);
impl SWRS6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRS6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRS6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRS7` reader - XDMAC Channel 7 Software Request Status Bit"]
pub struct SWRS7_R(crate::FieldReader<bool, bool>);
impl SWRS7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRS7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRS7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRS8` reader - XDMAC Channel 8 Software Request Status Bit"]
pub struct SWRS8_R(crate::FieldReader<bool, bool>);
impl SWRS8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRS8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRS8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRS9` reader - XDMAC Channel 9 Software Request Status Bit"]
pub struct SWRS9_R(crate::FieldReader<bool, bool>);
impl SWRS9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRS9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRS9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRS10` reader - XDMAC Channel 10 Software Request Status Bit"]
pub struct SWRS10_R(crate::FieldReader<bool, bool>);
impl SWRS10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRS10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRS10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRS11` reader - XDMAC Channel 11 Software Request Status Bit"]
pub struct SWRS11_R(crate::FieldReader<bool, bool>);
impl SWRS11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRS11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRS11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRS12` reader - XDMAC Channel 12 Software Request Status Bit"]
pub struct SWRS12_R(crate::FieldReader<bool, bool>);
impl SWRS12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRS12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRS12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRS13` reader - XDMAC Channel 13 Software Request Status Bit"]
pub struct SWRS13_R(crate::FieldReader<bool, bool>);
impl SWRS13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRS13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRS13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRS14` reader - XDMAC Channel 14 Software Request Status Bit"]
pub struct SWRS14_R(crate::FieldReader<bool, bool>);
impl SWRS14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRS14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRS14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRS15` reader - XDMAC Channel 15 Software Request Status Bit"]
pub struct SWRS15_R(crate::FieldReader<bool, bool>);
impl SWRS15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRS15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRS15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRS16` reader - XDMAC Channel 16 Software Request Status Bit"]
pub struct SWRS16_R(crate::FieldReader<bool, bool>);
impl SWRS16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRS16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRS16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRS17` reader - XDMAC Channel 17 Software Request Status Bit"]
pub struct SWRS17_R(crate::FieldReader<bool, bool>);
impl SWRS17_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRS17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRS17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRS18` reader - XDMAC Channel 18 Software Request Status Bit"]
pub struct SWRS18_R(crate::FieldReader<bool, bool>);
impl SWRS18_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRS18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRS18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRS19` reader - XDMAC Channel 19 Software Request Status Bit"]
pub struct SWRS19_R(crate::FieldReader<bool, bool>);
impl SWRS19_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRS19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRS19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRS20` reader - XDMAC Channel 20 Software Request Status Bit"]
pub struct SWRS20_R(crate::FieldReader<bool, bool>);
impl SWRS20_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRS20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRS20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRS21` reader - XDMAC Channel 21 Software Request Status Bit"]
pub struct SWRS21_R(crate::FieldReader<bool, bool>);
impl SWRS21_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRS21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRS21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRS22` reader - XDMAC Channel 22 Software Request Status Bit"]
pub struct SWRS22_R(crate::FieldReader<bool, bool>);
impl SWRS22_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRS22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRS22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRS23` reader - XDMAC Channel 23 Software Request Status Bit"]
pub struct SWRS23_R(crate::FieldReader<bool, bool>);
impl SWRS23_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRS23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRS23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - XDMAC Channel 0 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs0(&self) -> SWRS0_R {
        SWRS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs1(&self) -> SWRS1_R {
        SWRS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs2(&self) -> SWRS2_R {
        SWRS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs3(&self) -> SWRS3_R {
        SWRS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs4(&self) -> SWRS4_R {
        SWRS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs5(&self) -> SWRS5_R {
        SWRS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs6(&self) -> SWRS6_R {
        SWRS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs7(&self) -> SWRS7_R {
        SWRS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs8(&self) -> SWRS8_R {
        SWRS8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs9(&self) -> SWRS9_R {
        SWRS9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs10(&self) -> SWRS10_R {
        SWRS10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs11(&self) -> SWRS11_R {
        SWRS11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs12(&self) -> SWRS12_R {
        SWRS12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs13(&self) -> SWRS13_R {
        SWRS13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs14(&self) -> SWRS14_R {
        SWRS14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs15(&self) -> SWRS15_R {
        SWRS15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs16(&self) -> SWRS16_R {
        SWRS16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs17(&self) -> SWRS17_R {
        SWRS17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs18(&self) -> SWRS18_R {
        SWRS18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs19(&self) -> SWRS19_R {
        SWRS19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs20(&self) -> SWRS20_R {
        SWRS20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs21(&self) -> SWRS21_R {
        SWRS21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs22(&self) -> SWRS22_R {
        SWRS22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs23(&self) -> SWRS23_R {
        SWRS23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
#[doc = "Global Channel Software Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_gsws](index.html) module"]
pub struct XDMAC_GSWS_SPEC;
impl crate::RegisterSpec for XDMAC_GSWS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xdmac_gsws::R](R) reader structure"]
impl crate::Readable for XDMAC_GSWS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets XDMAC_GSWS to value 0"]
impl crate::Resettable for XDMAC_GSWS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
