#[doc = "Register `INTFLAGAHB` reader"]
pub struct R(crate::R<INTFLAGAHB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAGAHB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAGAHB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAGAHB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAGAHB` writer"]
pub struct W(crate::W<INTFLAGAHB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAGAHB_SPEC>;
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
impl From<crate::W<INTFLAGAHB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAGAHB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_` reader - FLASH"]
pub struct FLASH__R(crate::FieldReader<bool, bool>);
impl FLASH__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLASH__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_` writer - FLASH"]
pub struct FLASH__W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH__W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `HSRAMCM0P_` reader - HSRAMCM0P"]
pub struct HSRAMCM0P__R(crate::FieldReader<bool, bool>);
impl HSRAMCM0P__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSRAMCM0P__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSRAMCM0P__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSRAMCM0P_` writer - HSRAMCM0P"]
pub struct HSRAMCM0P__W<'a> {
    w: &'a mut W,
}
impl<'a> HSRAMCM0P__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `HSRAMDSU_` reader - HSRAMDSU"]
pub struct HSRAMDSU__R(crate::FieldReader<bool, bool>);
impl HSRAMDSU__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSRAMDSU__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSRAMDSU__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSRAMDSU_` writer - HSRAMDSU"]
pub struct HSRAMDSU__W<'a> {
    w: &'a mut W,
}
impl<'a> HSRAMDSU__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `HPB1_` reader - HPB1"]
pub struct HPB1__R(crate::FieldReader<bool, bool>);
impl HPB1__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HPB1__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPB1__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPB1_` writer - HPB1"]
pub struct HPB1__W<'a> {
    w: &'a mut W,
}
impl<'a> HPB1__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `HPB0_` reader - HPB0"]
pub struct HPB0__R(crate::FieldReader<bool, bool>);
impl HPB0__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HPB0__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPB0__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPB0_` writer - HPB0"]
pub struct HPB0__W<'a> {
    w: &'a mut W,
}
impl<'a> HPB0__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `HPB2_` reader - HPB2"]
pub struct HPB2__R(crate::FieldReader<bool, bool>);
impl HPB2__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HPB2__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPB2__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPB2_` writer - HPB2"]
pub struct HPB2__W<'a> {
    w: &'a mut W,
}
impl<'a> HPB2__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `LPRAMDMAC_` reader - LPRAMDMAC"]
pub struct LPRAMDMAC__R(crate::FieldReader<bool, bool>);
impl LPRAMDMAC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPRAMDMAC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPRAMDMAC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPRAMDMAC_` writer - LPRAMDMAC"]
pub struct LPRAMDMAC__W<'a> {
    w: &'a mut W,
}
impl<'a> LPRAMDMAC__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `DIVAS_` reader - DIVAS"]
pub struct DIVAS__R(crate::FieldReader<bool, bool>);
impl DIVAS__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIVAS__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVAS__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVAS_` writer - DIVAS"]
pub struct DIVAS__W<'a> {
    w: &'a mut W,
}
impl<'a> DIVAS__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - FLASH"]
    #[inline(always)]
    pub fn flash_(&self) -> FLASH__R {
        FLASH__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HSRAMCM0P"]
    #[inline(always)]
    pub fn hsramcm0p_(&self) -> HSRAMCM0P__R {
        HSRAMCM0P__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSRAMDSU"]
    #[inline(always)]
    pub fn hsramdsu_(&self) -> HSRAMDSU__R {
        HSRAMDSU__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HPB1"]
    #[inline(always)]
    pub fn hpb1_(&self) -> HPB1__R {
        HPB1__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - HPB0"]
    #[inline(always)]
    pub fn hpb0_(&self) -> HPB0__R {
        HPB0__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HPB2"]
    #[inline(always)]
    pub fn hpb2_(&self) -> HPB2__R {
        HPB2__R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LPRAMDMAC"]
    #[inline(always)]
    pub fn lpramdmac_(&self) -> LPRAMDMAC__R {
        LPRAMDMAC__R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DIVAS"]
    #[inline(always)]
    pub fn divas_(&self) -> DIVAS__R {
        DIVAS__R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLASH"]
    #[inline(always)]
    pub fn flash_(&mut self) -> FLASH__W {
        FLASH__W { w: self }
    }
    #[doc = "Bit 1 - HSRAMCM0P"]
    #[inline(always)]
    pub fn hsramcm0p_(&mut self) -> HSRAMCM0P__W {
        HSRAMCM0P__W { w: self }
    }
    #[doc = "Bit 2 - HSRAMDSU"]
    #[inline(always)]
    pub fn hsramdsu_(&mut self) -> HSRAMDSU__W {
        HSRAMDSU__W { w: self }
    }
    #[doc = "Bit 3 - HPB1"]
    #[inline(always)]
    pub fn hpb1_(&mut self) -> HPB1__W {
        HPB1__W { w: self }
    }
    #[doc = "Bit 4 - HPB0"]
    #[inline(always)]
    pub fn hpb0_(&mut self) -> HPB0__W {
        HPB0__W { w: self }
    }
    #[doc = "Bit 5 - HPB2"]
    #[inline(always)]
    pub fn hpb2_(&mut self) -> HPB2__W {
        HPB2__W { w: self }
    }
    #[doc = "Bit 6 - LPRAMDMAC"]
    #[inline(always)]
    pub fn lpramdmac_(&mut self) -> LPRAMDMAC__W {
        LPRAMDMAC__W { w: self }
    }
    #[doc = "Bit 7 - DIVAS"]
    #[inline(always)]
    pub fn divas_(&mut self) -> DIVAS__W {
        DIVAS__W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bridge interrupt flag status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflagahb](index.html) module"]
pub struct INTFLAGAHB_SPEC;
impl crate::RegisterSpec for INTFLAGAHB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intflagahb::R](R) reader structure"]
impl crate::Readable for INTFLAGAHB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflagahb::W](W) writer structure"]
impl crate::Writable for INTFLAGAHB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFLAGAHB to value 0"]
impl crate::Resettable for INTFLAGAHB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
