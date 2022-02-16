#[doc = "Register `XDMAC_GIM` reader"]
pub struct R(crate::R<XDMAC_GIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XDMAC_GIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XDMAC_GIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XDMAC_GIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IM0` reader - XDMAC Channel 0 Interrupt Mask Bit"]
pub struct IM0_R(crate::FieldReader<bool, bool>);
impl IM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM1` reader - XDMAC Channel 1 Interrupt Mask Bit"]
pub struct IM1_R(crate::FieldReader<bool, bool>);
impl IM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM2` reader - XDMAC Channel 2 Interrupt Mask Bit"]
pub struct IM2_R(crate::FieldReader<bool, bool>);
impl IM2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM3` reader - XDMAC Channel 3 Interrupt Mask Bit"]
pub struct IM3_R(crate::FieldReader<bool, bool>);
impl IM3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IM3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM4` reader - XDMAC Channel 4 Interrupt Mask Bit"]
pub struct IM4_R(crate::FieldReader<bool, bool>);
impl IM4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IM4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM5` reader - XDMAC Channel 5 Interrupt Mask Bit"]
pub struct IM5_R(crate::FieldReader<bool, bool>);
impl IM5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IM5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM6` reader - XDMAC Channel 6 Interrupt Mask Bit"]
pub struct IM6_R(crate::FieldReader<bool, bool>);
impl IM6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IM6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM7` reader - XDMAC Channel 7 Interrupt Mask Bit"]
pub struct IM7_R(crate::FieldReader<bool, bool>);
impl IM7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IM7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM8` reader - XDMAC Channel 8 Interrupt Mask Bit"]
pub struct IM8_R(crate::FieldReader<bool, bool>);
impl IM8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IM8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM9` reader - XDMAC Channel 9 Interrupt Mask Bit"]
pub struct IM9_R(crate::FieldReader<bool, bool>);
impl IM9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IM9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM10` reader - XDMAC Channel 10 Interrupt Mask Bit"]
pub struct IM10_R(crate::FieldReader<bool, bool>);
impl IM10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IM10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM11` reader - XDMAC Channel 11 Interrupt Mask Bit"]
pub struct IM11_R(crate::FieldReader<bool, bool>);
impl IM11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IM11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM12` reader - XDMAC Channel 12 Interrupt Mask Bit"]
pub struct IM12_R(crate::FieldReader<bool, bool>);
impl IM12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IM12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM13` reader - XDMAC Channel 13 Interrupt Mask Bit"]
pub struct IM13_R(crate::FieldReader<bool, bool>);
impl IM13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IM13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM14` reader - XDMAC Channel 14 Interrupt Mask Bit"]
pub struct IM14_R(crate::FieldReader<bool, bool>);
impl IM14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IM14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM15` reader - XDMAC Channel 15 Interrupt Mask Bit"]
pub struct IM15_R(crate::FieldReader<bool, bool>);
impl IM15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IM15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM16` reader - XDMAC Channel 16 Interrupt Mask Bit"]
pub struct IM16_R(crate::FieldReader<bool, bool>);
impl IM16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IM16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM17` reader - XDMAC Channel 17 Interrupt Mask Bit"]
pub struct IM17_R(crate::FieldReader<bool, bool>);
impl IM17_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IM17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM18` reader - XDMAC Channel 18 Interrupt Mask Bit"]
pub struct IM18_R(crate::FieldReader<bool, bool>);
impl IM18_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IM18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM19` reader - XDMAC Channel 19 Interrupt Mask Bit"]
pub struct IM19_R(crate::FieldReader<bool, bool>);
impl IM19_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IM19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM20` reader - XDMAC Channel 20 Interrupt Mask Bit"]
pub struct IM20_R(crate::FieldReader<bool, bool>);
impl IM20_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IM20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM21` reader - XDMAC Channel 21 Interrupt Mask Bit"]
pub struct IM21_R(crate::FieldReader<bool, bool>);
impl IM21_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IM21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM22` reader - XDMAC Channel 22 Interrupt Mask Bit"]
pub struct IM22_R(crate::FieldReader<bool, bool>);
impl IM22_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IM22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM23` reader - XDMAC Channel 23 Interrupt Mask Bit"]
pub struct IM23_R(crate::FieldReader<bool, bool>);
impl IM23_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IM23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - XDMAC Channel 0 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im0(&self) -> IM0_R {
        IM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im1(&self) -> IM1_R {
        IM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im2(&self) -> IM2_R {
        IM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im3(&self) -> IM3_R {
        IM3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im4(&self) -> IM4_R {
        IM4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im5(&self) -> IM5_R {
        IM5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im6(&self) -> IM6_R {
        IM6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im7(&self) -> IM7_R {
        IM7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im8(&self) -> IM8_R {
        IM8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im9(&self) -> IM9_R {
        IM9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im10(&self) -> IM10_R {
        IM10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im11(&self) -> IM11_R {
        IM11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im12(&self) -> IM12_R {
        IM12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im13(&self) -> IM13_R {
        IM13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im14(&self) -> IM14_R {
        IM14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im15(&self) -> IM15_R {
        IM15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im16(&self) -> IM16_R {
        IM16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im17(&self) -> IM17_R {
        IM17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im18(&self) -> IM18_R {
        IM18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im19(&self) -> IM19_R {
        IM19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im20(&self) -> IM20_R {
        IM20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im21(&self) -> IM21_R {
        IM21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im22(&self) -> IM22_R {
        IM22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im23(&self) -> IM23_R {
        IM23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
#[doc = "Global Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_gim](index.html) module"]
pub struct XDMAC_GIM_SPEC;
impl crate::RegisterSpec for XDMAC_GIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xdmac_gim::R](R) reader structure"]
impl crate::Readable for XDMAC_GIM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets XDMAC_GIM to value 0"]
impl crate::Resettable for XDMAC_GIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
