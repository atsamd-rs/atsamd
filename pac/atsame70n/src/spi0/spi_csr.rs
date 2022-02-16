#[doc = "Register `SPI_CSR[%s]` reader"]
pub struct R(crate::R<SPI_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_CSR[%s]` writer"]
pub struct W(crate::W<SPI_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CSR_SPEC>;
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
impl From<crate::W<SPI_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    #[doc = "0: Clock is low when inactive (CPOL=0)"]
    IDLE_LOW = 0,
    #[doc = "1: Clock is high when inactive (CPOL=1)"]
    IDLE_HIGH = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub struct CPOL_R(crate::FieldReader<bool, CPOL_A>);
impl CPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::IDLE_LOW,
            true => CPOL_A::IDLE_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_LOW`"]
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        **self == CPOL_A::IDLE_LOW
    }
    #[doc = "Checks if the value of the field is `IDLE_HIGH`"]
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        **self == CPOL_A::IDLE_HIGH
    }
}
impl core::ops::Deref for CPOL_R {
    type Target = crate::FieldReader<bool, CPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock is low when inactive (CPOL=0)"]
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut W {
        self.variant(CPOL_A::IDLE_LOW)
    }
    #[doc = "Clock is high when inactive (CPOL=1)"]
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut W {
        self.variant(CPOL_A::IDLE_HIGH)
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
#[doc = "Clock Phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCPHA_A {
    #[doc = "1: Data is valid on clock leading edge (CPHA=0)"]
    VALID_LEADING_EDGE = 1,
    #[doc = "0: Data is valid on clock trailing edge (CPHA=1)"]
    VALID_TRAILING_EDGE = 0,
}
impl From<NCPHA_A> for bool {
    #[inline(always)]
    fn from(variant: NCPHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCPHA` reader - Clock Phase"]
pub struct NCPHA_R(crate::FieldReader<bool, NCPHA_A>);
impl NCPHA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NCPHA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCPHA_A {
        match self.bits {
            true => NCPHA_A::VALID_LEADING_EDGE,
            false => NCPHA_A::VALID_TRAILING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `VALID_LEADING_EDGE`"]
    #[inline(always)]
    pub fn is_valid_leading_edge(&self) -> bool {
        **self == NCPHA_A::VALID_LEADING_EDGE
    }
    #[doc = "Checks if the value of the field is `VALID_TRAILING_EDGE`"]
    #[inline(always)]
    pub fn is_valid_trailing_edge(&self) -> bool {
        **self == NCPHA_A::VALID_TRAILING_EDGE
    }
}
impl core::ops::Deref for NCPHA_R {
    type Target = crate::FieldReader<bool, NCPHA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NCPHA` writer - Clock Phase"]
pub struct NCPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> NCPHA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NCPHA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data is valid on clock leading edge (CPHA=0)"]
    #[inline(always)]
    pub fn valid_leading_edge(self) -> &'a mut W {
        self.variant(NCPHA_A::VALID_LEADING_EDGE)
    }
    #[doc = "Data is valid on clock trailing edge (CPHA=1)"]
    #[inline(always)]
    pub fn valid_trailing_edge(self) -> &'a mut W {
        self.variant(NCPHA_A::VALID_TRAILING_EDGE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CSNAAT` reader - Chip Select Not Active After Transfer (Ignored if CSAAT = 1)"]
pub struct CSNAAT_R(crate::FieldReader<bool, bool>);
impl CSNAAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSNAAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSNAAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSNAAT` writer - Chip Select Not Active After Transfer (Ignored if CSAAT = 1)"]
pub struct CSNAAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CSNAAT_W<'a> {
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
#[doc = "Field `CSAAT` reader - Chip Select Active After Transfer"]
pub struct CSAAT_R(crate::FieldReader<bool, bool>);
impl CSAAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSAAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSAAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSAAT` writer - Chip Select Active After Transfer"]
pub struct CSAAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CSAAT_W<'a> {
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
#[doc = "Bits Per Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BITS_A {
    #[doc = "0: 8 bits for transfer"]
    _8_BIT = 0,
    #[doc = "1: 9 bits for transfer"]
    _9_BIT = 1,
    #[doc = "2: 10 bits for transfer"]
    _10_BIT = 2,
    #[doc = "3: 11 bits for transfer"]
    _11_BIT = 3,
    #[doc = "4: 12 bits for transfer"]
    _12_BIT = 4,
    #[doc = "5: 13 bits for transfer"]
    _13_BIT = 5,
    #[doc = "6: 14 bits for transfer"]
    _14_BIT = 6,
    #[doc = "7: 15 bits for transfer"]
    _15_BIT = 7,
    #[doc = "8: 16 bits for transfer"]
    _16_BIT = 8,
}
impl From<BITS_A> for u8 {
    #[inline(always)]
    fn from(variant: BITS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BITS` reader - Bits Per Transfer"]
pub struct BITS_R(crate::FieldReader<u8, BITS_A>);
impl BITS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BITS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BITS_A> {
        match self.bits {
            0 => Some(BITS_A::_8_BIT),
            1 => Some(BITS_A::_9_BIT),
            2 => Some(BITS_A::_10_BIT),
            3 => Some(BITS_A::_11_BIT),
            4 => Some(BITS_A::_12_BIT),
            5 => Some(BITS_A::_13_BIT),
            6 => Some(BITS_A::_14_BIT),
            7 => Some(BITS_A::_15_BIT),
            8 => Some(BITS_A::_16_BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        **self == BITS_A::_8_BIT
    }
    #[doc = "Checks if the value of the field is `_9_BIT`"]
    #[inline(always)]
    pub fn is_9_bit(&self) -> bool {
        **self == BITS_A::_9_BIT
    }
    #[doc = "Checks if the value of the field is `_10_BIT`"]
    #[inline(always)]
    pub fn is_10_bit(&self) -> bool {
        **self == BITS_A::_10_BIT
    }
    #[doc = "Checks if the value of the field is `_11_BIT`"]
    #[inline(always)]
    pub fn is_11_bit(&self) -> bool {
        **self == BITS_A::_11_BIT
    }
    #[doc = "Checks if the value of the field is `_12_BIT`"]
    #[inline(always)]
    pub fn is_12_bit(&self) -> bool {
        **self == BITS_A::_12_BIT
    }
    #[doc = "Checks if the value of the field is `_13_BIT`"]
    #[inline(always)]
    pub fn is_13_bit(&self) -> bool {
        **self == BITS_A::_13_BIT
    }
    #[doc = "Checks if the value of the field is `_14_BIT`"]
    #[inline(always)]
    pub fn is_14_bit(&self) -> bool {
        **self == BITS_A::_14_BIT
    }
    #[doc = "Checks if the value of the field is `_15_BIT`"]
    #[inline(always)]
    pub fn is_15_bit(&self) -> bool {
        **self == BITS_A::_15_BIT
    }
    #[doc = "Checks if the value of the field is `_16_BIT`"]
    #[inline(always)]
    pub fn is_16_bit(&self) -> bool {
        **self == BITS_A::_16_BIT
    }
}
impl core::ops::Deref for BITS_R {
    type Target = crate::FieldReader<u8, BITS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BITS` writer - Bits Per Transfer"]
pub struct BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> BITS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BITS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8 bits for transfer"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(BITS_A::_8_BIT)
    }
    #[doc = "9 bits for transfer"]
    #[inline(always)]
    pub fn _9_bit(self) -> &'a mut W {
        self.variant(BITS_A::_9_BIT)
    }
    #[doc = "10 bits for transfer"]
    #[inline(always)]
    pub fn _10_bit(self) -> &'a mut W {
        self.variant(BITS_A::_10_BIT)
    }
    #[doc = "11 bits for transfer"]
    #[inline(always)]
    pub fn _11_bit(self) -> &'a mut W {
        self.variant(BITS_A::_11_BIT)
    }
    #[doc = "12 bits for transfer"]
    #[inline(always)]
    pub fn _12_bit(self) -> &'a mut W {
        self.variant(BITS_A::_12_BIT)
    }
    #[doc = "13 bits for transfer"]
    #[inline(always)]
    pub fn _13_bit(self) -> &'a mut W {
        self.variant(BITS_A::_13_BIT)
    }
    #[doc = "14 bits for transfer"]
    #[inline(always)]
    pub fn _14_bit(self) -> &'a mut W {
        self.variant(BITS_A::_14_BIT)
    }
    #[doc = "15 bits for transfer"]
    #[inline(always)]
    pub fn _15_bit(self) -> &'a mut W {
        self.variant(BITS_A::_15_BIT)
    }
    #[doc = "16 bits for transfer"]
    #[inline(always)]
    pub fn _16_bit(self) -> &'a mut W {
        self.variant(BITS_A::_16_BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `SCBR` reader - Serial Clock Bit Rate"]
pub struct SCBR_R(crate::FieldReader<u8, u8>);
impl SCBR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCBR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCBR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCBR` writer - Serial Clock Bit Rate"]
pub struct SCBR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCBR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `DLYBS` reader - Delay Before SPCK"]
pub struct DLYBS_R(crate::FieldReader<u8, u8>);
impl DLYBS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DLYBS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLYBS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYBS` writer - Delay Before SPCK"]
pub struct DLYBS_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYBS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `DLYBCT` reader - Delay Between Consecutive Transfers"]
pub struct DLYBCT_R(crate::FieldReader<u8, u8>);
impl DLYBCT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DLYBCT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLYBCT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYBCT` writer - Delay Between Consecutive Transfers"]
pub struct DLYBCT_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYBCT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    pub fn ncpha(&self) -> NCPHA_R {
        NCPHA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Chip Select Not Active After Transfer (Ignored if CSAAT = 1)"]
    #[inline(always)]
    pub fn csnaat(&self) -> CSNAAT_R {
        CSNAAT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Chip Select Active After Transfer"]
    #[inline(always)]
    pub fn csaat(&self) -> CSAAT_R {
        CSAAT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Bits Per Transfer"]
    #[inline(always)]
    pub fn bits_(&self) -> BITS_R {
        BITS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Serial Clock Bit Rate"]
    #[inline(always)]
    pub fn scbr(&self) -> SCBR_R {
        SCBR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Delay Before SPCK"]
    #[inline(always)]
    pub fn dlybs(&self) -> DLYBS_R {
        DLYBS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    pub fn dlybct(&self) -> DLYBCT_R {
        DLYBCT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    pub fn ncpha(&mut self) -> NCPHA_W {
        NCPHA_W { w: self }
    }
    #[doc = "Bit 2 - Chip Select Not Active After Transfer (Ignored if CSAAT = 1)"]
    #[inline(always)]
    pub fn csnaat(&mut self) -> CSNAAT_W {
        CSNAAT_W { w: self }
    }
    #[doc = "Bit 3 - Chip Select Active After Transfer"]
    #[inline(always)]
    pub fn csaat(&mut self) -> CSAAT_W {
        CSAAT_W { w: self }
    }
    #[doc = "Bits 4:7 - Bits Per Transfer"]
    #[inline(always)]
    pub fn bits_(&mut self) -> BITS_W {
        BITS_W { w: self }
    }
    #[doc = "Bits 8:15 - Serial Clock Bit Rate"]
    #[inline(always)]
    pub fn scbr(&mut self) -> SCBR_W {
        SCBR_W { w: self }
    }
    #[doc = "Bits 16:23 - Delay Before SPCK"]
    #[inline(always)]
    pub fn dlybs(&mut self) -> DLYBS_W {
        DLYBS_W { w: self }
    }
    #[doc = "Bits 24:31 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    pub fn dlybct(&mut self) -> DLYBCT_W {
        DLYBCT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Chip Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_csr](index.html) module"]
pub struct SPI_CSR_SPEC;
impl crate::RegisterSpec for SPI_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_csr::R](R) reader structure"]
impl crate::Readable for SPI_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_csr::W](W) writer structure"]
impl crate::Writable for SPI_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_CSR[%s]
to value 0"]
impl crate::Resettable for SPI_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
