#[doc = "Reader of register CR"]
pub type R = crate::R<u16, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u16, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Response Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RESPTYP_A {
    #[doc = "0: No response"]
    NONE = 0,
    #[doc = "1: 136-bit response"]
    _136_BIT = 1,
    #[doc = "2: 48-bit response"]
    _48_BIT = 2,
    #[doc = "3: 48-bit response check busy after response"]
    _48_BIT_BUSY = 3,
}
impl From<RESPTYP_A> for u8 {
    #[inline(always)]
    fn from(variant: RESPTYP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RESPTYP`"]
pub type RESPTYP_R = crate::R<u8, RESPTYP_A>;
impl RESPTYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESPTYP_A {
        match self.bits {
            0 => RESPTYP_A::NONE,
            1 => RESPTYP_A::_136_BIT,
            2 => RESPTYP_A::_48_BIT,
            3 => RESPTYP_A::_48_BIT_BUSY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RESPTYP_A::NONE
    }
    #[doc = "Checks if the value of the field is `_136_BIT`"]
    #[inline(always)]
    pub fn is_136_bit(&self) -> bool {
        *self == RESPTYP_A::_136_BIT
    }
    #[doc = "Checks if the value of the field is `_48_BIT`"]
    #[inline(always)]
    pub fn is_48_bit(&self) -> bool {
        *self == RESPTYP_A::_48_BIT
    }
    #[doc = "Checks if the value of the field is `_48_BIT_BUSY`"]
    #[inline(always)]
    pub fn is_48_bit_busy(&self) -> bool {
        *self == RESPTYP_A::_48_BIT_BUSY
    }
}
#[doc = "Write proxy for field `RESPTYP`"]
pub struct RESPTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> RESPTYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESPTYP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No response"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(RESPTYP_A::NONE)
    }
    #[doc = "136-bit response"]
    #[inline(always)]
    pub fn _136_bit(self) -> &'a mut W {
        self.variant(RESPTYP_A::_136_BIT)
    }
    #[doc = "48-bit response"]
    #[inline(always)]
    pub fn _48_bit(self) -> &'a mut W {
        self.variant(RESPTYP_A::_48_BIT)
    }
    #[doc = "48-bit response check busy after response"]
    #[inline(always)]
    pub fn _48_bit_busy(self) -> &'a mut W {
        self.variant(RESPTYP_A::_48_BIT_BUSY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
#[doc = "Command CRC Check Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDCCEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<CMDCCEN_A> for bool {
    #[inline(always)]
    fn from(variant: CMDCCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMDCCEN`"]
pub type CMDCCEN_R = crate::R<bool, CMDCCEN_A>;
impl CMDCCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDCCEN_A {
        match self.bits {
            false => CMDCCEN_A::DISABLE,
            true => CMDCCEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CMDCCEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CMDCCEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `CMDCCEN`"]
pub struct CMDCCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDCCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDCCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CMDCCEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CMDCCEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Command Index Check Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDICEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<CMDICEN_A> for bool {
    #[inline(always)]
    fn from(variant: CMDICEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMDICEN`"]
pub type CMDICEN_R = crate::R<bool, CMDICEN_A>;
impl CMDICEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDICEN_A {
        match self.bits {
            false => CMDICEN_A::DISABLE,
            true => CMDICEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CMDICEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CMDICEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `CMDICEN`"]
pub struct CMDICEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDICEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDICEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CMDICEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CMDICEN_A::ENABLE)
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
#[doc = "Data Present Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPSEL_A {
    #[doc = "0: No Data Present"]
    NO_DATA = 0,
    #[doc = "1: Data Present"]
    DATA = 1,
}
impl From<DPSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DPSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DPSEL`"]
pub type DPSEL_R = crate::R<bool, DPSEL_A>;
impl DPSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPSEL_A {
        match self.bits {
            false => DPSEL_A::NO_DATA,
            true => DPSEL_A::DATA,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == DPSEL_A::NO_DATA
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == DPSEL_A::DATA
    }
}
#[doc = "Write proxy for field `DPSEL`"]
pub struct DPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DPSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Data Present"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(DPSEL_A::NO_DATA)
    }
    #[doc = "Data Present"]
    #[inline(always)]
    pub fn data(self) -> &'a mut W {
        self.variant(DPSEL_A::DATA)
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
#[doc = "Command Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMDTYP_A {
    #[doc = "0: Other commands"]
    NORMAL = 0,
    #[doc = "1: CMD52 for writing Bus Suspend in CCCR"]
    SUSPEND = 1,
    #[doc = "2: CMD52 for writing Function Select in CCCR"]
    RESUME = 2,
    #[doc = "3: CMD12, CMD52 for writing I/O Abort in CCCR"]
    ABORT = 3,
}
impl From<CMDTYP_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDTYP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CMDTYP`"]
pub type CMDTYP_R = crate::R<u8, CMDTYP_A>;
impl CMDTYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDTYP_A {
        match self.bits {
            0 => CMDTYP_A::NORMAL,
            1 => CMDTYP_A::SUSPEND,
            2 => CMDTYP_A::RESUME,
            3 => CMDTYP_A::ABORT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CMDTYP_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == CMDTYP_A::SUSPEND
    }
    #[doc = "Checks if the value of the field is `RESUME`"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == CMDTYP_A::RESUME
    }
    #[doc = "Checks if the value of the field is `ABORT`"]
    #[inline(always)]
    pub fn is_abort(&self) -> bool {
        *self == CMDTYP_A::ABORT
    }
}
#[doc = "Write proxy for field `CMDTYP`"]
pub struct CMDTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDTYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDTYP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Other commands"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(CMDTYP_A::NORMAL)
    }
    #[doc = "CMD52 for writing Bus Suspend in CCCR"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut W {
        self.variant(CMDTYP_A::SUSPEND)
    }
    #[doc = "CMD52 for writing Function Select in CCCR"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut W {
        self.variant(CMDTYP_A::RESUME)
    }
    #[doc = "CMD12, CMD52 for writing I/O Abort in CCCR"]
    #[inline(always)]
    pub fn abort(self) -> &'a mut W {
        self.variant(CMDTYP_A::ABORT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `CMDIDX`"]
pub type CMDIDX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMDIDX`"]
pub struct CMDIDX_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDIDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u16) & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Response Type"]
    #[inline(always)]
    pub fn resptyp(&self) -> RESPTYP_R {
        RESPTYP_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 3 - Command CRC Check Enable"]
    #[inline(always)]
    pub fn cmdccen(&self) -> CMDCCEN_R {
        CMDCCEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Command Index Check Enable"]
    #[inline(always)]
    pub fn cmdicen(&self) -> CMDICEN_R {
        CMDICEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Data Present Select"]
    #[inline(always)]
    pub fn dpsel(&self) -> DPSEL_R {
        DPSEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Command Type"]
    #[inline(always)]
    pub fn cmdtyp(&self) -> CMDTYP_R {
        CMDTYP_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:13 - Command Index"]
    #[inline(always)]
    pub fn cmdidx(&self) -> CMDIDX_R {
        CMDIDX_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Response Type"]
    #[inline(always)]
    pub fn resptyp(&mut self) -> RESPTYP_W {
        RESPTYP_W { w: self }
    }
    #[doc = "Bit 3 - Command CRC Check Enable"]
    #[inline(always)]
    pub fn cmdccen(&mut self) -> CMDCCEN_W {
        CMDCCEN_W { w: self }
    }
    #[doc = "Bit 4 - Command Index Check Enable"]
    #[inline(always)]
    pub fn cmdicen(&mut self) -> CMDICEN_W {
        CMDICEN_W { w: self }
    }
    #[doc = "Bit 5 - Data Present Select"]
    #[inline(always)]
    pub fn dpsel(&mut self) -> DPSEL_W {
        DPSEL_W { w: self }
    }
    #[doc = "Bits 6:7 - Command Type"]
    #[inline(always)]
    pub fn cmdtyp(&mut self) -> CMDTYP_W {
        CMDTYP_W { w: self }
    }
    #[doc = "Bits 8:13 - Command Index"]
    #[inline(always)]
    pub fn cmdidx(&mut self) -> CMDIDX_W {
        CMDIDX_W { w: self }
    }
}
