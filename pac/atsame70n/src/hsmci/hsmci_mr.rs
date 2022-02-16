#[doc = "Register `HSMCI_MR` reader"]
pub struct R(crate::R<HSMCI_MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSMCI_MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSMCI_MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSMCI_MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSMCI_MR` writer"]
pub struct W(crate::W<HSMCI_MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSMCI_MR_SPEC>;
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
impl From<crate::W<HSMCI_MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSMCI_MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKDIV` reader - Clock Divider"]
pub struct CLKDIV_R(crate::FieldReader<u8, u8>);
impl CLKDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKDIV` writer - Clock Divider"]
pub struct CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `PWSDIV` reader - Power Saving Divider"]
pub struct PWSDIV_R(crate::FieldReader<u8, u8>);
impl PWSDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PWSDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWSDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWSDIV` writer - Power Saving Divider"]
pub struct PWSDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PWSDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `RDPROOF` reader - Read Proof Enable"]
pub struct RDPROOF_R(crate::FieldReader<bool, bool>);
impl RDPROOF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RDPROOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDPROOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDPROOF` writer - Read Proof Enable"]
pub struct RDPROOF_W<'a> {
    w: &'a mut W,
}
impl<'a> RDPROOF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `WRPROOF` reader - Write Proof Enable"]
pub struct WRPROOF_R(crate::FieldReader<bool, bool>);
impl WRPROOF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WRPROOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRPROOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRPROOF` writer - Write Proof Enable"]
pub struct WRPROOF_W<'a> {
    w: &'a mut W,
}
impl<'a> WRPROOF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `FBYTE` reader - Force Byte Transfer"]
pub struct FBYTE_R(crate::FieldReader<bool, bool>);
impl FBYTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FBYTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBYTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FBYTE` writer - Force Byte Transfer"]
pub struct FBYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> FBYTE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `PADV` reader - Padding Value"]
pub struct PADV_R(crate::FieldReader<bool, bool>);
impl PADV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PADV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PADV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PADV` writer - Padding Value"]
pub struct PADV_W<'a> {
    w: &'a mut W,
}
impl<'a> PADV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `CLKODD` reader - Clock divider is odd"]
pub struct CLKODD_R(crate::FieldReader<bool, bool>);
impl CLKODD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKODD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKODD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKODD` writer - Clock divider is odd"]
pub struct CLKODD_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKODD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Clock Divider"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Power Saving Divider"]
    #[inline(always)]
    pub fn pwsdiv(&self) -> PWSDIV_R {
        PWSDIV_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Read Proof Enable"]
    #[inline(always)]
    pub fn rdproof(&self) -> RDPROOF_R {
        RDPROOF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Write Proof Enable"]
    #[inline(always)]
    pub fn wrproof(&self) -> WRPROOF_R {
        WRPROOF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Force Byte Transfer"]
    #[inline(always)]
    pub fn fbyte(&self) -> FBYTE_R {
        FBYTE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Padding Value"]
    #[inline(always)]
    pub fn padv(&self) -> PADV_R {
        PADV_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Clock divider is odd"]
    #[inline(always)]
    pub fn clkodd(&self) -> CLKODD_R {
        CLKODD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Divider"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W {
        CLKDIV_W { w: self }
    }
    #[doc = "Bits 8:10 - Power Saving Divider"]
    #[inline(always)]
    pub fn pwsdiv(&mut self) -> PWSDIV_W {
        PWSDIV_W { w: self }
    }
    #[doc = "Bit 11 - Read Proof Enable"]
    #[inline(always)]
    pub fn rdproof(&mut self) -> RDPROOF_W {
        RDPROOF_W { w: self }
    }
    #[doc = "Bit 12 - Write Proof Enable"]
    #[inline(always)]
    pub fn wrproof(&mut self) -> WRPROOF_W {
        WRPROOF_W { w: self }
    }
    #[doc = "Bit 13 - Force Byte Transfer"]
    #[inline(always)]
    pub fn fbyte(&mut self) -> FBYTE_W {
        FBYTE_W { w: self }
    }
    #[doc = "Bit 14 - Padding Value"]
    #[inline(always)]
    pub fn padv(&mut self) -> PADV_W {
        PADV_W { w: self }
    }
    #[doc = "Bit 16 - Clock divider is odd"]
    #[inline(always)]
    pub fn clkodd(&mut self) -> CLKODD_W {
        CLKODD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_mr](index.html) module"]
pub struct HSMCI_MR_SPEC;
impl crate::RegisterSpec for HSMCI_MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsmci_mr::R](R) reader structure"]
impl crate::Readable for HSMCI_MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsmci_mr::W](W) writer structure"]
impl crate::Writable for HSMCI_MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSMCI_MR to value 0"]
impl crate::Resettable for HSMCI_MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
