#[doc = "Register `SPI_MR` reader"]
pub struct R(crate::R<SPI_MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MR` writer"]
pub struct W(crate::W<SPI_MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MR_SPEC>;
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
impl From<crate::W<SPI_MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Master/Slave Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTR_A {
    #[doc = "1: Master"]
    MASTER = 1,
    #[doc = "0: Slave"]
    SLAVE = 0,
}
impl From<MSTR_A> for bool {
    #[inline(always)]
    fn from(variant: MSTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTR` reader - Master/Slave Mode"]
pub struct MSTR_R(crate::FieldReader<bool, MSTR_A>);
impl MSTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MSTR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTR_A {
        match self.bits {
            true => MSTR_A::MASTER,
            false => MSTR_A::SLAVE,
        }
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        **self == MSTR_A::MASTER
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        **self == MSTR_A::SLAVE
    }
}
impl core::ops::Deref for MSTR_R {
    type Target = crate::FieldReader<bool, MSTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTR` writer - Master/Slave Mode"]
pub struct MSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Master"]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(MSTR_A::MASTER)
    }
    #[doc = "Slave"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(MSTR_A::SLAVE)
    }
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
#[doc = "Field `PS` reader - Peripheral Select"]
pub struct PS_R(crate::FieldReader<bool, bool>);
impl PS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PS` writer - Peripheral Select"]
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
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
#[doc = "Field `PCSDEC` reader - Chip Select Decode"]
pub struct PCSDEC_R(crate::FieldReader<bool, bool>);
impl PCSDEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCSDEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSDEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSDEC` writer - Chip Select Decode"]
pub struct PCSDEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSDEC_W<'a> {
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
#[doc = "Field `MODFDIS` reader - Mode Fault Detection"]
pub struct MODFDIS_R(crate::FieldReader<bool, bool>);
impl MODFDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MODFDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODFDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODFDIS` writer - Mode Fault Detection"]
pub struct MODFDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MODFDIS_W<'a> {
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
#[doc = "Field `WDRBT` reader - Wait Data Read Before Transfer"]
pub struct WDRBT_R(crate::FieldReader<bool, bool>);
impl WDRBT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDRBT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDRBT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDRBT` writer - Wait Data Read Before Transfer"]
pub struct WDRBT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDRBT_W<'a> {
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
#[doc = "Field `LLB` reader - Local Loopback Enable"]
pub struct LLB_R(crate::FieldReader<bool, bool>);
impl LLB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LLB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LLB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LLB` writer - Local Loopback Enable"]
pub struct LLB_W<'a> {
    w: &'a mut W,
}
impl<'a> LLB_W<'a> {
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
#[doc = "Peripheral Chip Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCS_A {
    #[doc = "14: NPCS0 as Chip Select"]
    NPCS0 = 14,
    #[doc = "13: NPCS1 as Chip Select"]
    NPCS1 = 13,
    #[doc = "11: NPCS2 as Chip Select"]
    NPCS2 = 11,
    #[doc = "7: NPCS3 as Chip Select"]
    NPCS3 = 7,
}
impl From<PCS_A> for u8 {
    #[inline(always)]
    fn from(variant: PCS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PCS` reader - Peripheral Chip Select"]
pub struct PCS_R(crate::FieldReader<u8, PCS_A>);
impl PCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PCS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PCS_A> {
        match self.bits {
            14 => Some(PCS_A::NPCS0),
            13 => Some(PCS_A::NPCS1),
            11 => Some(PCS_A::NPCS2),
            7 => Some(PCS_A::NPCS3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NPCS0`"]
    #[inline(always)]
    pub fn is_npcs0(&self) -> bool {
        **self == PCS_A::NPCS0
    }
    #[doc = "Checks if the value of the field is `NPCS1`"]
    #[inline(always)]
    pub fn is_npcs1(&self) -> bool {
        **self == PCS_A::NPCS1
    }
    #[doc = "Checks if the value of the field is `NPCS2`"]
    #[inline(always)]
    pub fn is_npcs2(&self) -> bool {
        **self == PCS_A::NPCS2
    }
    #[doc = "Checks if the value of the field is `NPCS3`"]
    #[inline(always)]
    pub fn is_npcs3(&self) -> bool {
        **self == PCS_A::NPCS3
    }
}
impl core::ops::Deref for PCS_R {
    type Target = crate::FieldReader<u8, PCS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCS` writer - Peripheral Chip Select"]
pub struct PCS_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "NPCS0 as Chip Select"]
    #[inline(always)]
    pub fn npcs0(self) -> &'a mut W {
        self.variant(PCS_A::NPCS0)
    }
    #[doc = "NPCS1 as Chip Select"]
    #[inline(always)]
    pub fn npcs1(self) -> &'a mut W {
        self.variant(PCS_A::NPCS1)
    }
    #[doc = "NPCS2 as Chip Select"]
    #[inline(always)]
    pub fn npcs2(self) -> &'a mut W {
        self.variant(PCS_A::NPCS2)
    }
    #[doc = "NPCS3 as Chip Select"]
    #[inline(always)]
    pub fn npcs3(self) -> &'a mut W {
        self.variant(PCS_A::NPCS3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `DLYBCS` reader - Delay Between Chip Selects"]
pub struct DLYBCS_R(crate::FieldReader<u8, u8>);
impl DLYBCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DLYBCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLYBCS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYBCS` writer - Delay Between Chip Selects"]
pub struct DLYBCS_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYBCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Master/Slave Mode"]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Peripheral Select"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Chip Select Decode"]
    #[inline(always)]
    pub fn pcsdec(&self) -> PCSDEC_R {
        PCSDEC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Mode Fault Detection"]
    #[inline(always)]
    pub fn modfdis(&self) -> MODFDIS_R {
        MODFDIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Wait Data Read Before Transfer"]
    #[inline(always)]
    pub fn wdrbt(&self) -> WDRBT_R {
        WDRBT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Local Loopback Enable"]
    #[inline(always)]
    pub fn llb(&self) -> LLB_R {
        LLB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline(always)]
    pub fn pcs(&self) -> PCS_R {
        PCS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Delay Between Chip Selects"]
    #[inline(always)]
    pub fn dlybcs(&self) -> DLYBCS_R {
        DLYBCS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Master/Slave Mode"]
    #[inline(always)]
    pub fn mstr(&mut self) -> MSTR_W {
        MSTR_W { w: self }
    }
    #[doc = "Bit 1 - Peripheral Select"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    #[doc = "Bit 2 - Chip Select Decode"]
    #[inline(always)]
    pub fn pcsdec(&mut self) -> PCSDEC_W {
        PCSDEC_W { w: self }
    }
    #[doc = "Bit 4 - Mode Fault Detection"]
    #[inline(always)]
    pub fn modfdis(&mut self) -> MODFDIS_W {
        MODFDIS_W { w: self }
    }
    #[doc = "Bit 5 - Wait Data Read Before Transfer"]
    #[inline(always)]
    pub fn wdrbt(&mut self) -> WDRBT_W {
        WDRBT_W { w: self }
    }
    #[doc = "Bit 7 - Local Loopback Enable"]
    #[inline(always)]
    pub fn llb(&mut self) -> LLB_W {
        LLB_W { w: self }
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline(always)]
    pub fn pcs(&mut self) -> PCS_W {
        PCS_W { w: self }
    }
    #[doc = "Bits 24:31 - Delay Between Chip Selects"]
    #[inline(always)]
    pub fn dlybcs(&mut self) -> DLYBCS_W {
        DLYBCS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mr](index.html) module"]
pub struct SPI_MR_SPEC;
impl crate::RegisterSpec for SPI_MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mr::R](R) reader structure"]
impl crate::Readable for SPI_MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mr::W](W) writer structure"]
impl crate::Writable for SPI_MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_MR to value 0"]
impl crate::Resettable for SPI_MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
