#[doc = "Register `CCCR` reader"]
pub struct R(crate::R<CCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCCR` writer"]
pub struct W(crate::W<CCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCCR_SPEC>;
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
impl From<crate::W<CCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INIT` reader - Initialization"]
pub struct INIT_R(crate::FieldReader<bool, bool>);
impl INIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INIT` writer - Initialization"]
pub struct INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_W<'a> {
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
#[doc = "Field `CCE` reader - Configuration Change Enable"]
pub struct CCE_R(crate::FieldReader<bool, bool>);
impl CCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCE` writer - Configuration Change Enable"]
pub struct CCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCE_W<'a> {
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
#[doc = "Field `ASM` reader - ASM Restricted Operation Mode"]
pub struct ASM_R(crate::FieldReader<bool, bool>);
impl ASM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ASM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASM` writer - ASM Restricted Operation Mode"]
pub struct ASM_W<'a> {
    w: &'a mut W,
}
impl<'a> ASM_W<'a> {
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
#[doc = "Field `CSA` reader - Clock Stop Acknowledge"]
pub struct CSA_R(crate::FieldReader<bool, bool>);
impl CSA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSA` writer - Clock Stop Acknowledge"]
pub struct CSA_W<'a> {
    w: &'a mut W,
}
impl<'a> CSA_W<'a> {
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
#[doc = "Field `CSR` reader - Clock Stop Request"]
pub struct CSR_R(crate::FieldReader<bool, bool>);
impl CSR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSR` writer - Clock Stop Request"]
pub struct CSR_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR_W<'a> {
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
#[doc = "Field `MON` reader - Bus Monitoring Mode"]
pub struct MON_R(crate::FieldReader<bool, bool>);
impl MON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MON` writer - Bus Monitoring Mode"]
pub struct MON_W<'a> {
    w: &'a mut W,
}
impl<'a> MON_W<'a> {
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
#[doc = "Field `DAR` reader - Disable Automatic Retransmission"]
pub struct DAR_R(crate::FieldReader<bool, bool>);
impl DAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAR` writer - Disable Automatic Retransmission"]
pub struct DAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DAR_W<'a> {
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
#[doc = "Field `TEST` reader - Test Mode Enable"]
pub struct TEST_R(crate::FieldReader<bool, bool>);
impl TEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEST` writer - Test Mode Enable"]
pub struct TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_W<'a> {
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
#[doc = "Field `FDOE` reader - FD Operation Enable"]
pub struct FDOE_R(crate::FieldReader<bool, bool>);
impl FDOE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FDOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDOE` writer - FD Operation Enable"]
pub struct FDOE_W<'a> {
    w: &'a mut W,
}
impl<'a> FDOE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `BRSE` reader - Bit Rate Switch Enable"]
pub struct BRSE_R(crate::FieldReader<bool, bool>);
impl BRSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BRSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRSE` writer - Bit Rate Switch Enable"]
pub struct BRSE_W<'a> {
    w: &'a mut W,
}
impl<'a> BRSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `PXHD` reader - Protocol Exception Handling Disable"]
pub struct PXHD_R(crate::FieldReader<bool, bool>);
impl PXHD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PXHD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PXHD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PXHD` writer - Protocol Exception Handling Disable"]
pub struct PXHD_W<'a> {
    w: &'a mut W,
}
impl<'a> PXHD_W<'a> {
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
#[doc = "Field `EFBI` reader - Edge Filtering during Bus Integration"]
pub struct EFBI_R(crate::FieldReader<bool, bool>);
impl EFBI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EFBI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFBI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFBI` writer - Edge Filtering during Bus Integration"]
pub struct EFBI_W<'a> {
    w: &'a mut W,
}
impl<'a> EFBI_W<'a> {
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
#[doc = "Field `TXP` reader - Transmit Pause"]
pub struct TXP_R(crate::FieldReader<bool, bool>);
impl TXP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXP` writer - Transmit Pause"]
pub struct TXP_W<'a> {
    w: &'a mut W,
}
impl<'a> TXP_W<'a> {
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
#[doc = "Field `NISO` reader - Non ISO Operation"]
pub struct NISO_R(crate::FieldReader<bool, bool>);
impl NISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NISO` writer - Non ISO Operation"]
pub struct NISO_W<'a> {
    w: &'a mut W,
}
impl<'a> NISO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Initialization"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Configuration Change Enable"]
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ASM Restricted Operation Mode"]
    #[inline(always)]
    pub fn asm(&self) -> ASM_R {
        ASM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clock Stop Acknowledge"]
    #[inline(always)]
    pub fn csa(&self) -> CSA_R {
        CSA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clock Stop Request"]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Bus Monitoring Mode"]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Disable Automatic Retransmission"]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Test Mode Enable"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FD Operation Enable"]
    #[inline(always)]
    pub fn fdoe(&self) -> FDOE_R {
        FDOE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Bit Rate Switch Enable"]
    #[inline(always)]
    pub fn brse(&self) -> BRSE_R {
        BRSE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Protocol Exception Handling Disable"]
    #[inline(always)]
    pub fn pxhd(&self) -> PXHD_R {
        PXHD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Edge Filtering during Bus Integration"]
    #[inline(always)]
    pub fn efbi(&self) -> EFBI_R {
        EFBI_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Transmit Pause"]
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Non ISO Operation"]
    #[inline(always)]
    pub fn niso(&self) -> NISO_R {
        NISO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initialization"]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W {
        INIT_W { w: self }
    }
    #[doc = "Bit 1 - Configuration Change Enable"]
    #[inline(always)]
    pub fn cce(&mut self) -> CCE_W {
        CCE_W { w: self }
    }
    #[doc = "Bit 2 - ASM Restricted Operation Mode"]
    #[inline(always)]
    pub fn asm(&mut self) -> ASM_W {
        ASM_W { w: self }
    }
    #[doc = "Bit 3 - Clock Stop Acknowledge"]
    #[inline(always)]
    pub fn csa(&mut self) -> CSA_W {
        CSA_W { w: self }
    }
    #[doc = "Bit 4 - Clock Stop Request"]
    #[inline(always)]
    pub fn csr(&mut self) -> CSR_W {
        CSR_W { w: self }
    }
    #[doc = "Bit 5 - Bus Monitoring Mode"]
    #[inline(always)]
    pub fn mon(&mut self) -> MON_W {
        MON_W { w: self }
    }
    #[doc = "Bit 6 - Disable Automatic Retransmission"]
    #[inline(always)]
    pub fn dar(&mut self) -> DAR_W {
        DAR_W { w: self }
    }
    #[doc = "Bit 7 - Test Mode Enable"]
    #[inline(always)]
    pub fn test(&mut self) -> TEST_W {
        TEST_W { w: self }
    }
    #[doc = "Bit 8 - FD Operation Enable"]
    #[inline(always)]
    pub fn fdoe(&mut self) -> FDOE_W {
        FDOE_W { w: self }
    }
    #[doc = "Bit 9 - Bit Rate Switch Enable"]
    #[inline(always)]
    pub fn brse(&mut self) -> BRSE_W {
        BRSE_W { w: self }
    }
    #[doc = "Bit 12 - Protocol Exception Handling Disable"]
    #[inline(always)]
    pub fn pxhd(&mut self) -> PXHD_W {
        PXHD_W { w: self }
    }
    #[doc = "Bit 13 - Edge Filtering during Bus Integration"]
    #[inline(always)]
    pub fn efbi(&mut self) -> EFBI_W {
        EFBI_W { w: self }
    }
    #[doc = "Bit 14 - Transmit Pause"]
    #[inline(always)]
    pub fn txp(&mut self) -> TXP_W {
        TXP_W { w: self }
    }
    #[doc = "Bit 15 - Non ISO Operation"]
    #[inline(always)]
    pub fn niso(&mut self) -> NISO_W {
        NISO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CC Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cccr](index.html) module"]
pub struct CCCR_SPEC;
impl crate::RegisterSpec for CCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cccr::R](R) reader structure"]
impl crate::Readable for CCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cccr::W](W) writer structure"]
impl crate::Writable for CCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCCR to value 0x01"]
impl crate::Resettable for CCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
