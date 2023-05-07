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
#[doc = "Field `DW` reader - Data Width"]
pub type DW_R = crate::BitReader<DWSELECT_A>;
#[doc = "Data Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DWSELECT_A {
    #[doc = "0: 1-bit mode"]
    _1BIT = 0,
    #[doc = "1: 4-bit mode"]
    _4BIT = 1,
}
impl From<DWSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DWSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DWSELECT_A {
        match self.bits {
            false => DWSELECT_A::_1BIT,
            true => DWSELECT_A::_4BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_1BIT`"]
    #[inline(always)]
    pub fn is_1bit(&self) -> bool {
        *self == DWSELECT_A::_1BIT
    }
    #[doc = "Checks if the value of the field is `_4BIT`"]
    #[inline(always)]
    pub fn is_4bit(&self) -> bool {
        *self == DWSELECT_A::_4BIT
    }
}
#[doc = "Field `DW` writer - Data Width"]
pub type DW_W<'a, const O: u8> = crate::BitWriter<'a, u8, HC1R_EMMC_MODE_SPEC, DWSELECT_A, O>;
impl<'a, const O: u8> DW_W<'a, O> {
    #[doc = "1-bit mode"]
    #[inline(always)]
    pub fn _1bit(self) -> &'a mut W {
        self.variant(DWSELECT_A::_1BIT)
    }
    #[doc = "4-bit mode"]
    #[inline(always)]
    pub fn _4bit(self) -> &'a mut W {
        self.variant(DWSELECT_A::_4BIT)
    }
}
#[doc = "Field `HSEN` reader - High Speed Enable"]
pub type HSEN_R = crate::BitReader<HSENSELECT_A>;
#[doc = "High Speed Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSENSELECT_A {
    #[doc = "0: Normal Speed mode"]
    NORMAL = 0,
    #[doc = "1: High Speed mode"]
    HIGH = 1,
}
impl From<HSENSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: HSENSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl HSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSENSELECT_A {
        match self.bits {
            false => HSENSELECT_A::NORMAL,
            true => HSENSELECT_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == HSENSELECT_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == HSENSELECT_A::HIGH
    }
}
#[doc = "Field `HSEN` writer - High Speed Enable"]
pub type HSEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, HC1R_EMMC_MODE_SPEC, HSENSELECT_A, O>;
impl<'a, const O: u8> HSEN_W<'a, O> {
    #[doc = "Normal Speed mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(HSENSELECT_A::NORMAL)
    }
    #[doc = "High Speed mode"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(HSENSELECT_A::HIGH)
    }
}
#[doc = "Field `DMASEL` reader - DMA Select"]
pub type DMASEL_R = crate::FieldReader<u8, DMASELSELECT_A>;
#[doc = "DMA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMASELSELECT_A {
    #[doc = "0: SDMA is selected"]
    SDMA = 0,
    #[doc = "2: 32-bit Address ADMA2 is selected"]
    _32BIT = 2,
}
impl From<DMASELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DMASELSELECT_A) -> Self {
        variant as _
    }
}
impl DMASEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DMASELSELECT_A> {
        match self.bits {
            0 => Some(DMASELSELECT_A::SDMA),
            2 => Some(DMASELSELECT_A::_32BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SDMA`"]
    #[inline(always)]
    pub fn is_sdma(&self) -> bool {
        *self == DMASELSELECT_A::SDMA
    }
    #[doc = "Checks if the value of the field is `_32BIT`"]
    #[inline(always)]
    pub fn is_32bit(&self) -> bool {
        *self == DMASELSELECT_A::_32BIT
    }
}
#[doc = "Field `DMASEL` writer - DMA Select"]
pub type DMASEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, HC1R_EMMC_MODE_SPEC, u8, DMASELSELECT_A, 2, O>;
impl<'a, const O: u8> DMASEL_W<'a, O> {
    #[doc = "SDMA is selected"]
    #[inline(always)]
    pub fn sdma(self) -> &'a mut W {
        self.variant(DMASELSELECT_A::SDMA)
    }
    #[doc = "32-bit Address ADMA2 is selected"]
    #[inline(always)]
    pub fn _32bit(self) -> &'a mut W {
        self.variant(DMASELSELECT_A::_32BIT)
    }
}
impl R {
    #[doc = "Bit 1 - Data Width"]
    #[inline(always)]
    pub fn dw(&self) -> DW_R {
        DW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - High Speed Enable"]
    #[inline(always)]
    pub fn hsen(&self) -> HSEN_R {
        HSEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - DMA Select"]
    #[inline(always)]
    pub fn dmasel(&self) -> DMASEL_R {
        DMASEL_R::new((self.bits >> 3) & 3)
    }
}
impl W {
    #[doc = "Bit 1 - Data Width"]
    #[inline(always)]
    #[must_use]
    pub fn dw(&mut self) -> DW_W<1> {
        DW_W::new(self)
    }
    #[doc = "Bit 2 - High Speed Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsen(&mut self) -> HSEN_W<2> {
        HSEN_W::new(self)
    }
    #[doc = "Bits 3:4 - DMA Select"]
    #[inline(always)]
    #[must_use]
    pub fn dmasel(&mut self) -> DMASEL_W<3> {
        DMASEL_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HC1R_EMMC_MODE to value 0"]
impl crate::Resettable for HC1R_EMMC_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
