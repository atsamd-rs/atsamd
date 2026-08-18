#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRST` reader - Software Reset"]
pub struct SWRST_R(crate::FieldReader<bool, bool>);
impl SWRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRST` writer - Software Reset"]
pub struct SWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Field `DMAENABLE` reader - DMA Enable"]
pub struct DMAENABLE_R(crate::FieldReader<bool, bool>);
impl DMAENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAENABLE` writer - DMA Enable"]
pub struct DMAENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CRCENABLE` reader - CRC Enable"]
pub struct CRCENABLE_R(crate::FieldReader<bool, bool>);
impl CRCENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRCENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCENABLE` writer - CRC Enable"]
pub struct CRCENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `LVLEN0` reader - Priority Level 0 Enable"]
pub struct LVLEN0_R(crate::FieldReader<bool, bool>);
impl LVLEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LVLEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LVLEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVLEN0` writer - Priority Level 0 Enable"]
pub struct LVLEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> LVLEN0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `LVLEN1` reader - Priority Level 1 Enable"]
pub struct LVLEN1_R(crate::FieldReader<bool, bool>);
impl LVLEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LVLEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LVLEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVLEN1` writer - Priority Level 1 Enable"]
pub struct LVLEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> LVLEN1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u16 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `LVLEN2` reader - Priority Level 2 Enable"]
pub struct LVLEN2_R(crate::FieldReader<bool, bool>);
impl LVLEN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LVLEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LVLEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVLEN2` writer - Priority Level 2 Enable"]
pub struct LVLEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> LVLEN2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u16 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `LVLEN3` reader - Priority Level 3 Enable"]
pub struct LVLEN3_R(crate::FieldReader<bool, bool>);
impl LVLEN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LVLEN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LVLEN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVLEN3` writer - Priority Level 3 Enable"]
pub struct LVLEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> LVLEN3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u16 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA Enable"]
    #[inline(always)]
    pub fn dmaenable(&self) -> DMAENABLE_R {
        DMAENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CRC Enable"]
    #[inline(always)]
    pub fn crcenable(&self) -> CRCENABLE_R {
        CRCENABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Priority Level 0 Enable"]
    #[inline(always)]
    pub fn lvlen0(&self) -> LVLEN0_R {
        LVLEN0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Priority Level 1 Enable"]
    #[inline(always)]
    pub fn lvlen1(&self) -> LVLEN1_R {
        LVLEN1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Priority Level 2 Enable"]
    #[inline(always)]
    pub fn lvlen2(&self) -> LVLEN2_R {
        LVLEN2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Priority Level 3 Enable"]
    #[inline(always)]
    pub fn lvlen3(&self) -> LVLEN3_R {
        LVLEN3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
    #[doc = "Bit 1 - DMA Enable"]
    #[inline(always)]
    pub fn dmaenable(&mut self) -> DMAENABLE_W {
        DMAENABLE_W { w: self }
    }
    #[doc = "Bit 2 - CRC Enable"]
    #[inline(always)]
    pub fn crcenable(&mut self) -> CRCENABLE_W {
        CRCENABLE_W { w: self }
    }
    #[doc = "Bit 8 - Priority Level 0 Enable"]
    #[inline(always)]
    pub fn lvlen0(&mut self) -> LVLEN0_W {
        LVLEN0_W { w: self }
    }
    #[doc = "Bit 9 - Priority Level 1 Enable"]
    #[inline(always)]
    pub fn lvlen1(&mut self) -> LVLEN1_W {
        LVLEN1_W { w: self }
    }
    #[doc = "Bit 10 - Priority Level 2 Enable"]
    #[inline(always)]
    pub fn lvlen2(&mut self) -> LVLEN2_W {
        LVLEN2_W { w: self }
    }
    #[doc = "Bit 11 - Priority Level 3 Enable"]
    #[inline(always)]
    pub fn lvlen3(&mut self) -> LVLEN3_W {
        LVLEN3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
