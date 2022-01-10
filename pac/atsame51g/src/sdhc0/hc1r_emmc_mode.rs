#[doc = "Register `HC1R_EMMC_MODE` reader"]
pub struct R(crate::R<HC1R_EMMC_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HC1R_EMMC_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HC1R_EMMC_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HC1R_EMMC_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HC1R_EMMC_MODE` writer"]
pub struct W(crate::W<HC1R_EMMC_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HC1R_EMMC_MODE_SPEC>;
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
impl From<crate::W<HC1R_EMMC_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HC1R_EMMC_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Data Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DW_A {
    #[doc = "0: 1-bit mode"]
    _1BIT = 0,
    #[doc = "1: 4-bit mode"]
    _4BIT = 1,
}
impl From<DW_A> for bool {
    #[inline(always)]
    fn from(variant: DW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DW` reader - Data Width"]
pub struct DW_R(crate::FieldReader<bool, DW_A>);
impl DW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DW_A {
        match self.bits {
            false => DW_A::_1BIT,
            true => DW_A::_4BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_1BIT`"]
    #[inline(always)]
    pub fn is_1bit(&self) -> bool {
        **self == DW_A::_1BIT
    }
    #[doc = "Checks if the value of the field is `_4BIT`"]
    #[inline(always)]
    pub fn is_4bit(&self) -> bool {
        **self == DW_A::_4BIT
    }
}
impl core::ops::Deref for DW_R {
    type Target = crate::FieldReader<bool, DW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DW` writer - Data Width"]
pub struct DW_W<'a> {
    w: &'a mut W,
}
impl<'a> DW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1-bit mode"]
    #[inline(always)]
    pub fn _1bit(self) -> &'a mut W {
        self.variant(DW_A::_1BIT)
    }
    #[doc = "4-bit mode"]
    #[inline(always)]
    pub fn _4bit(self) -> &'a mut W {
        self.variant(DW_A::_4BIT)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "High Speed Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSEN_A {
    #[doc = "0: Normal Speed mode"]
    NORMAL = 0,
    #[doc = "1: High Speed mode"]
    HIGH = 1,
}
impl From<HSEN_A> for bool {
    #[inline(always)]
    fn from(variant: HSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSEN` reader - High Speed Enable"]
pub struct HSEN_R(crate::FieldReader<bool, HSEN_A>);
impl HSEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSEN_A {
        match self.bits {
            false => HSEN_A::NORMAL,
            true => HSEN_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == HSEN_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == HSEN_A::HIGH
    }
}
impl core::ops::Deref for HSEN_R {
    type Target = crate::FieldReader<bool, HSEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSEN` writer - High Speed Enable"]
pub struct HSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal Speed mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(HSEN_A::NORMAL)
    }
    #[doc = "High Speed mode"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(HSEN_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "DMA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMASEL_A {
    #[doc = "0: SDMA is selected"]
    SDMA = 0,
    #[doc = "2: 32-bit Address ADMA2 is selected"]
    _32BIT = 2,
}
impl From<DMASEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DMASEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DMASEL` reader - DMA Select"]
pub struct DMASEL_R(crate::FieldReader<u8, DMASEL_A>);
impl DMASEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMASEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DMASEL_A> {
        match self.bits {
            0 => Some(DMASEL_A::SDMA),
            2 => Some(DMASEL_A::_32BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SDMA`"]
    #[inline(always)]
    pub fn is_sdma(&self) -> bool {
        **self == DMASEL_A::SDMA
    }
    #[doc = "Checks if the value of the field is `_32BIT`"]
    #[inline(always)]
    pub fn is_32bit(&self) -> bool {
        **self == DMASEL_A::_32BIT
    }
}
impl core::ops::Deref for DMASEL_R {
    type Target = crate::FieldReader<u8, DMASEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMASEL` writer - DMA Select"]
pub struct DMASEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMASEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SDMA is selected"]
    #[inline(always)]
    pub fn sdma(self) -> &'a mut W {
        self.variant(DMASEL_A::SDMA)
    }
    #[doc = "32-bit Address ADMA2 is selected"]
    #[inline(always)]
    pub fn _32bit(self) -> &'a mut W {
        self.variant(DMASEL_A::_32BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u8 & 0x03) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Data Width"]
    #[inline(always)]
    pub fn dw(&self) -> DW_R {
        DW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - High Speed Enable"]
    #[inline(always)]
    pub fn hsen(&self) -> HSEN_R {
        HSEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - DMA Select"]
    #[inline(always)]
    pub fn dmasel(&self) -> DMASEL_R {
        DMASEL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Data Width"]
    #[inline(always)]
    pub fn dw(&mut self) -> DW_W {
        DW_W { w: self }
    }
    #[doc = "Bit 2 - High Speed Enable"]
    #[inline(always)]
    pub fn hsen(&mut self) -> HSEN_W {
        HSEN_W { w: self }
    }
    #[doc = "Bits 3:4 - DMA Select"]
    #[inline(always)]
    pub fn dmasel(&mut self) -> DMASEL_W {
        DMASEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc1r_emmc_mode](index.html) module"]
pub struct HC1R_EMMC_MODE_SPEC;
impl crate::RegisterSpec for HC1R_EMMC_MODE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [hc1r_emmc_mode::R](R) reader structure"]
impl crate::Readable for HC1R_EMMC_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hc1r_emmc_mode::W](W) writer structure"]
impl crate::Writable for HC1R_EMMC_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HC1R_EMMC_MODE to value 0"]
impl crate::Resettable for HC1R_EMMC_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
