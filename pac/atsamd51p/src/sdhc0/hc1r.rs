#[doc = "Reader of register HC1R"]
pub type R = crate::R<u8, super::HC1R>;
#[doc = "Writer for register HC1R"]
pub type W = crate::W<u8, super::HC1R>;
#[doc = "Register HC1R `reset()`'s with value 0"]
impl crate::ResetValue for super::HC1R {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LED Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEDCTRL_A {
    #[doc = "0: LED off"]
    OFF = 0,
    #[doc = "1: LED on"]
    ON = 1,
}
impl From<LEDCTRL_A> for bool {
    #[inline(always)]
    fn from(variant: LEDCTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEDCTRL`"]
pub type LEDCTRL_R = crate::R<bool, LEDCTRL_A>;
impl LEDCTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEDCTRL_A {
        match self.bits {
            false => LEDCTRL_A::OFF,
            true => LEDCTRL_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LEDCTRL_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == LEDCTRL_A::ON
    }
}
#[doc = "Write proxy for field `LEDCTRL`"]
pub struct LEDCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDCTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEDCTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LED off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(LEDCTRL_A::OFF)
    }
    #[doc = "LED on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(LEDCTRL_A::ON)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
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
#[doc = "Reader of field `DW`"]
pub type DW_R = crate::R<bool, DW_A>;
impl DW_R {
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
        *self == DW_A::_1BIT
    }
    #[doc = "Checks if the value of the field is `_4BIT`"]
    #[inline(always)]
    pub fn is_4bit(&self) -> bool {
        *self == DW_A::_4BIT
    }
}
#[doc = "Write proxy for field `DW`"]
pub struct DW_W<'a> {
    w: &'a mut W,
}
impl<'a> DW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
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
#[doc = "Reader of field `HSEN`"]
pub type HSEN_R = crate::R<bool, HSEN_A>;
impl HSEN_R {
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
        *self == HSEN_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == HSEN_A::HIGH
    }
}
#[doc = "Write proxy for field `HSEN`"]
pub struct HSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
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
#[doc = "Reader of field `DMASEL`"]
pub type DMASEL_R = crate::R<u8, DMASEL_A>;
impl DMASEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DMASEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DMASEL_A::SDMA),
            2 => Val(DMASEL_A::_32BIT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SDMA`"]
    #[inline(always)]
    pub fn is_sdma(&self) -> bool {
        *self == DMASEL_A::SDMA
    }
    #[doc = "Checks if the value of the field is `_32BIT`"]
    #[inline(always)]
    pub fn is_32bit(&self) -> bool {
        *self == DMASEL_A::_32BIT
    }
}
#[doc = "Write proxy for field `DMASEL`"]
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
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u8) & 0x03) << 3);
        self.w
    }
}
#[doc = "Card Detect Test Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARDDTL_A {
    #[doc = "0: No Card"]
    NO = 0,
    #[doc = "1: Card Inserted"]
    YES = 1,
}
impl From<CARDDTL_A> for bool {
    #[inline(always)]
    fn from(variant: CARDDTL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CARDDTL`"]
pub type CARDDTL_R = crate::R<bool, CARDDTL_A>;
impl CARDDTL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CARDDTL_A {
        match self.bits {
            false => CARDDTL_A::NO,
            true => CARDDTL_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == CARDDTL_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == CARDDTL_A::YES
    }
}
#[doc = "Write proxy for field `CARDDTL`"]
pub struct CARDDTL_W<'a> {
    w: &'a mut W,
}
impl<'a> CARDDTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARDDTL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Card"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(CARDDTL_A::NO)
    }
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(CARDDTL_A::YES)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Card Detect Signal Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARDDSEL_A {
    #[doc = "0: SDCD# is selected (for normal use)"]
    NORMAL = 0,
    #[doc = "1: The Card Select Test Level is selected (for test purpose)"]
    TEST = 1,
}
impl From<CARDDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CARDDSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CARDDSEL`"]
pub type CARDDSEL_R = crate::R<bool, CARDDSEL_A>;
impl CARDDSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CARDDSEL_A {
        match self.bits {
            false => CARDDSEL_A::NORMAL,
            true => CARDDSEL_A::TEST,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CARDDSEL_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `TEST`"]
    #[inline(always)]
    pub fn is_test(&self) -> bool {
        *self == CARDDSEL_A::TEST
    }
}
#[doc = "Write proxy for field `CARDDSEL`"]
pub struct CARDDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CARDDSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARDDSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SDCD# is selected (for normal use)"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(CARDDSEL_A::NORMAL)
    }
    #[doc = "The Card Select Test Level is selected (for test purpose)"]
    #[inline(always)]
    pub fn test(self) -> &'a mut W {
        self.variant(CARDDSEL_A::TEST)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LED Control"]
    #[inline(always)]
    pub fn ledctrl(&self) -> LEDCTRL_R {
        LEDCTRL_R::new((self.bits & 0x01) != 0)
    }
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
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline(always)]
    pub fn carddtl(&self) -> CARDDTL_R {
        CARDDTL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Card Detect Signal Selection"]
    #[inline(always)]
    pub fn carddsel(&self) -> CARDDSEL_R {
        CARDDSEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LED Control"]
    #[inline(always)]
    pub fn ledctrl(&mut self) -> LEDCTRL_W {
        LEDCTRL_W { w: self }
    }
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
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline(always)]
    pub fn carddtl(&mut self) -> CARDDTL_W {
        CARDDTL_W { w: self }
    }
    #[doc = "Bit 7 - Card Detect Signal Selection"]
    #[inline(always)]
    pub fn carddsel(&mut self) -> CARDDSEL_W {
        CARDDSEL_W { w: self }
    }
}
