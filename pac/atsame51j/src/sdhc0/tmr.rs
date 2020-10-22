#[doc = "Reader of register TMR"]
pub type R = crate::R<u16, super::TMR>;
#[doc = "Writer for register TMR"]
pub type W = crate::W<u16, super::TMR>;
#[doc = "Register TMR `reset()`'s with value 0"]
impl crate::ResetValue for super::TMR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: No data transfer or Non DMA data transfer"]
    DISABLE = 0,
    #[doc = "1: DMA data transfer"]
    ENABLE = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAEN`"]
pub type DMAEN_R = crate::R<bool, DMAEN_A>;
impl DMAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::DISABLE,
            true => DMAEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMAEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMAEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `DMAEN`"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No data transfer or Non DMA data transfer"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMAEN_A::DISABLE)
    }
    #[doc = "DMA data transfer"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMAEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Block Count Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<BCEN_A> for bool {
    #[inline(always)]
    fn from(variant: BCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BCEN`"]
pub type BCEN_R = crate::R<bool, BCEN_A>;
impl BCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCEN_A {
        match self.bits {
            false => BCEN_A::DISABLE,
            true => BCEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BCEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BCEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `BCEN`"]
pub struct BCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BCEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BCEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Auto Command Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACMDEN_A {
    #[doc = "0: Auto Command Disabled"]
    DISABLED = 0,
    #[doc = "1: Auto CMD12 Enable"]
    CMD12 = 1,
    #[doc = "2: Auto CMD23 Enable"]
    CMD23 = 2,
}
impl From<ACMDEN_A> for u8 {
    #[inline(always)]
    fn from(variant: ACMDEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ACMDEN`"]
pub type ACMDEN_R = crate::R<u8, ACMDEN_A>;
impl ACMDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ACMDEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ACMDEN_A::DISABLED),
            1 => Val(ACMDEN_A::CMD12),
            2 => Val(ACMDEN_A::CMD23),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ACMDEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `CMD12`"]
    #[inline(always)]
    pub fn is_cmd12(&self) -> bool {
        *self == ACMDEN_A::CMD12
    }
    #[doc = "Checks if the value of the field is `CMD23`"]
    #[inline(always)]
    pub fn is_cmd23(&self) -> bool {
        *self == ACMDEN_A::CMD23
    }
}
#[doc = "Write proxy for field `ACMDEN`"]
pub struct ACMDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMDEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Auto Command Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ACMDEN_A::DISABLED)
    }
    #[doc = "Auto CMD12 Enable"]
    #[inline(always)]
    pub fn cmd12(self) -> &'a mut W {
        self.variant(ACMDEN_A::CMD12)
    }
    #[doc = "Auto CMD23 Enable"]
    #[inline(always)]
    pub fn cmd23(self) -> &'a mut W {
        self.variant(ACMDEN_A::CMD23)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "Data Transfer Direction Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTDSEL_A {
    #[doc = "0: Write (Host to Card)"]
    WRITE = 0,
    #[doc = "1: Read (Card to Host)"]
    READ = 1,
}
impl From<DTDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DTDSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DTDSEL`"]
pub type DTDSEL_R = crate::R<bool, DTDSEL_A>;
impl DTDSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTDSEL_A {
        match self.bits {
            false => DTDSEL_A::WRITE,
            true => DTDSEL_A::READ,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == DTDSEL_A::WRITE
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == DTDSEL_A::READ
    }
}
#[doc = "Write proxy for field `DTDSEL`"]
pub struct DTDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTDSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTDSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write (Host to Card)"]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(DTDSEL_A::WRITE)
    }
    #[doc = "Read (Card to Host)"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(DTDSEL_A::READ)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Multi/Single Block Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSBSEL_A {
    #[doc = "0: Single Block"]
    SINGLE = 0,
    #[doc = "1: Multiple Block"]
    MULTIPLE = 1,
}
impl From<MSBSEL_A> for bool {
    #[inline(always)]
    fn from(variant: MSBSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSBSEL`"]
pub type MSBSEL_R = crate::R<bool, MSBSEL_A>;
impl MSBSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSBSEL_A {
        match self.bits {
            false => MSBSEL_A::SINGLE,
            true => MSBSEL_A::MULTIPLE,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == MSBSEL_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `MULTIPLE`"]
    #[inline(always)]
    pub fn is_multiple(&self) -> bool {
        *self == MSBSEL_A::MULTIPLE
    }
}
#[doc = "Write proxy for field `MSBSEL`"]
pub struct MSBSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MSBSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSBSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Single Block"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(MSBSEL_A::SINGLE)
    }
    #[doc = "Multiple Block"]
    #[inline(always)]
    pub fn multiple(self) -> &'a mut W {
        self.variant(MSBSEL_A::MULTIPLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    pub fn bcen(&self) -> BCEN_R {
        BCEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Auto Command Enable"]
    #[inline(always)]
    pub fn acmden(&self) -> ACMDEN_R {
        ACMDEN_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Data Transfer Direction Selection"]
    #[inline(always)]
    pub fn dtdsel(&self) -> DTDSEL_R {
        DTDSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Multi/Single Block Selection"]
    #[inline(always)]
    pub fn msbsel(&self) -> MSBSEL_R {
        MSBSEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    pub fn bcen(&mut self) -> BCEN_W {
        BCEN_W { w: self }
    }
    #[doc = "Bits 2:3 - Auto Command Enable"]
    #[inline(always)]
    pub fn acmden(&mut self) -> ACMDEN_W {
        ACMDEN_W { w: self }
    }
    #[doc = "Bit 4 - Data Transfer Direction Selection"]
    #[inline(always)]
    pub fn dtdsel(&mut self) -> DTDSEL_W {
        DTDSEL_W { w: self }
    }
    #[doc = "Bit 5 - Multi/Single Block Selection"]
    #[inline(always)]
    pub fn msbsel(&mut self) -> MSBSEL_W {
        MSBSEL_W { w: self }
    }
}
