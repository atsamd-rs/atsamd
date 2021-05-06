#[doc = "Reader of register DPLLCTRLB"]
pub type R = crate::R<u32, super::DPLLCTRLB>;
#[doc = "Writer for register DPLLCTRLB"]
pub type W = crate::W<u32, super::DPLLCTRLB>;
#[doc = "Register DPLLCTRLB `reset()`'s with value 0"]
impl crate::ResetValue for super::DPLLCTRLB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Proportional Integral Filter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FILTER_A {
    #[doc = "0: Default filter mode"]
    DEFAULT = 0,
    #[doc = "1: Low bandwidth filter"]
    LBFILT = 1,
    #[doc = "2: High bandwidth filter"]
    HBFILT = 2,
    #[doc = "3: High damping filter"]
    HDFILT = 3,
}
impl From<FILTER_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FILTER`"]
pub type FILTER_R = crate::R<u8, FILTER_A>;
impl FILTER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTER_A {
        match self.bits {
            0 => FILTER_A::DEFAULT,
            1 => FILTER_A::LBFILT,
            2 => FILTER_A::HBFILT,
            3 => FILTER_A::HDFILT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == FILTER_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `LBFILT`"]
    #[inline(always)]
    pub fn is_lbfilt(&self) -> bool {
        *self == FILTER_A::LBFILT
    }
    #[doc = "Checks if the value of the field is `HBFILT`"]
    #[inline(always)]
    pub fn is_hbfilt(&self) -> bool {
        *self == FILTER_A::HBFILT
    }
    #[doc = "Checks if the value of the field is `HDFILT`"]
    #[inline(always)]
    pub fn is_hdfilt(&self) -> bool {
        *self == FILTER_A::HDFILT
    }
}
#[doc = "Write proxy for field `FILTER`"]
pub struct FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTER_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Default filter mode"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(FILTER_A::DEFAULT)
    }
    #[doc = "Low bandwidth filter"]
    #[inline(always)]
    pub fn lbfilt(self) -> &'a mut W {
        self.variant(FILTER_A::LBFILT)
    }
    #[doc = "High bandwidth filter"]
    #[inline(always)]
    pub fn hbfilt(self) -> &'a mut W {
        self.variant(FILTER_A::HBFILT)
    }
    #[doc = "High damping filter"]
    #[inline(always)]
    pub fn hdfilt(self) -> &'a mut W {
        self.variant(FILTER_A::HDFILT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `LPEN`"]
pub type LPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPEN`"]
pub struct LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `WUF`"]
pub type WUF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUF`"]
pub struct WUF_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reference Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFCLK_A {
    #[doc = "0: CLK_DPLL_REF0 clock reference"]
    REF0 = 0,
    #[doc = "1: CLK_DPLL_REF1 clock reference"]
    REF1 = 1,
    #[doc = "2: GCLK_DPLL clock reference"]
    GCLK = 2,
}
impl From<REFCLK_A> for u8 {
    #[inline(always)]
    fn from(variant: REFCLK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REFCLK`"]
pub type REFCLK_R = crate::R<u8, REFCLK_A>;
impl REFCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REFCLK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REFCLK_A::REF0),
            1 => Val(REFCLK_A::REF1),
            2 => Val(REFCLK_A::GCLK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `REF0`"]
    #[inline(always)]
    pub fn is_ref0(&self) -> bool {
        *self == REFCLK_A::REF0
    }
    #[doc = "Checks if the value of the field is `REF1`"]
    #[inline(always)]
    pub fn is_ref1(&self) -> bool {
        *self == REFCLK_A::REF1
    }
    #[doc = "Checks if the value of the field is `GCLK`"]
    #[inline(always)]
    pub fn is_gclk(&self) -> bool {
        *self == REFCLK_A::GCLK
    }
}
#[doc = "Write proxy for field `REFCLK`"]
pub struct REFCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> REFCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFCLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CLK_DPLL_REF0 clock reference"]
    #[inline(always)]
    pub fn ref0(self) -> &'a mut W {
        self.variant(REFCLK_A::REF0)
    }
    #[doc = "CLK_DPLL_REF1 clock reference"]
    #[inline(always)]
    pub fn ref1(self) -> &'a mut W {
        self.variant(REFCLK_A::REF1)
    }
    #[doc = "GCLK_DPLL clock reference"]
    #[inline(always)]
    pub fn gclk(self) -> &'a mut W {
        self.variant(REFCLK_A::GCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Lock Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LTIME_A {
    #[doc = "0: Default\tNo time-out"]
    NONE = 0,
    #[doc = "4: 8MS\tTime-out if no lock within 8 ms"]
    _8MS = 4,
    #[doc = "5: 9MS\tTime-out if no lock within 9 ms"]
    _9MS = 5,
    #[doc = "6: 10MS\tTime-out if no lock within 10 ms"]
    _10MS = 6,
    #[doc = "7: 11MS\tTime-out if no lock within 11 ms"]
    _11MS = 7,
}
impl From<LTIME_A> for u8 {
    #[inline(always)]
    fn from(variant: LTIME_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LTIME`"]
pub type LTIME_R = crate::R<u8, LTIME_A>;
impl LTIME_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LTIME_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LTIME_A::NONE),
            4 => Val(LTIME_A::_8MS),
            5 => Val(LTIME_A::_9MS),
            6 => Val(LTIME_A::_10MS),
            7 => Val(LTIME_A::_11MS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == LTIME_A::NONE
    }
    #[doc = "Checks if the value of the field is `_8MS`"]
    #[inline(always)]
    pub fn is_8ms(&self) -> bool {
        *self == LTIME_A::_8MS
    }
    #[doc = "Checks if the value of the field is `_9MS`"]
    #[inline(always)]
    pub fn is_9ms(&self) -> bool {
        *self == LTIME_A::_9MS
    }
    #[doc = "Checks if the value of the field is `_10MS`"]
    #[inline(always)]
    pub fn is_10ms(&self) -> bool {
        *self == LTIME_A::_10MS
    }
    #[doc = "Checks if the value of the field is `_11MS`"]
    #[inline(always)]
    pub fn is_11ms(&self) -> bool {
        *self == LTIME_A::_11MS
    }
}
#[doc = "Write proxy for field `LTIME`"]
pub struct LTIME_W<'a> {
    w: &'a mut W,
}
impl<'a> LTIME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LTIME_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Default No time-out"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(LTIME_A::NONE)
    }
    #[doc = "8MS Time-out if no lock within 8 ms"]
    #[inline(always)]
    pub fn _8ms(self) -> &'a mut W {
        self.variant(LTIME_A::_8MS)
    }
    #[doc = "9MS Time-out if no lock within 9 ms"]
    #[inline(always)]
    pub fn _9ms(self) -> &'a mut W {
        self.variant(LTIME_A::_9MS)
    }
    #[doc = "10MS Time-out if no lock within 10 ms"]
    #[inline(always)]
    pub fn _10ms(self) -> &'a mut W {
        self.variant(LTIME_A::_10MS)
    }
    #[doc = "11MS Time-out if no lock within 11 ms"]
    #[inline(always)]
    pub fn _11ms(self) -> &'a mut W {
        self.variant(LTIME_A::_11MS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `LBYPASS`"]
pub type LBYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LBYPASS`"]
pub struct LBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> LBYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `DIV`"]
pub type DIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DIV`"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | (((value as u32) & 0x07ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Proportional Integral Filter Selection"]
    #[inline(always)]
    pub fn filter(&self) -> FILTER_R {
        FILTER_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Low-Power Enable"]
    #[inline(always)]
    pub fn lpen(&self) -> LPEN_R {
        LPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Wake Up Fast"]
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Reference Clock Selection"]
    #[inline(always)]
    pub fn refclk(&self) -> REFCLK_R {
        REFCLK_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - Lock Time"]
    #[inline(always)]
    pub fn ltime(&self) -> LTIME_R {
        LTIME_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Lock Bypass"]
    #[inline(always)]
    pub fn lbypass(&self) -> LBYPASS_R {
        LBYPASS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:26 - Clock Divider"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Proportional Integral Filter Selection"]
    #[inline(always)]
    pub fn filter(&mut self) -> FILTER_W {
        FILTER_W { w: self }
    }
    #[doc = "Bit 2 - Low-Power Enable"]
    #[inline(always)]
    pub fn lpen(&mut self) -> LPEN_W {
        LPEN_W { w: self }
    }
    #[doc = "Bit 3 - Wake Up Fast"]
    #[inline(always)]
    pub fn wuf(&mut self) -> WUF_W {
        WUF_W { w: self }
    }
    #[doc = "Bits 4:5 - Reference Clock Selection"]
    #[inline(always)]
    pub fn refclk(&mut self) -> REFCLK_W {
        REFCLK_W { w: self }
    }
    #[doc = "Bits 8:10 - Lock Time"]
    #[inline(always)]
    pub fn ltime(&mut self) -> LTIME_W {
        LTIME_W { w: self }
    }
    #[doc = "Bit 12 - Lock Bypass"]
    #[inline(always)]
    pub fn lbypass(&mut self) -> LBYPASS_W {
        LBYPASS_W { w: self }
    }
    #[doc = "Bits 16:26 - Clock Divider"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
}
