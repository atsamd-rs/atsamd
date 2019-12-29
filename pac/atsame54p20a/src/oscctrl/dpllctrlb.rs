#[doc = "Reader of register DPLLCTRLB%s"]
pub type R = crate::R<u32, super::DPLLCTRLB>;
#[doc = "Writer for register DPLLCTRLB%s"]
pub type W = crate::W<u32, super::DPLLCTRLB>;
#[doc = "Register DPLLCTRLB%s `reset()`'s with value 0x20"]
impl crate::ResetValue for super::DPLLCTRLB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "Reader of field `FILTER`"]
pub type FILTER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FILTER`"]
pub struct FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reference Clock Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFCLK_A {
    #[doc = "0: Dedicated GCLK clock reference"]
    GCLK = 0,
    #[doc = "1: XOSC32K clock reference"]
    XOSC32 = 1,
    #[doc = "2: XOSC0 clock reference"]
    XOSC0 = 2,
    #[doc = "3: XOSC1 clock reference"]
    XOSC1 = 3,
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
            0 => Val(REFCLK_A::GCLK),
            1 => Val(REFCLK_A::XOSC32),
            2 => Val(REFCLK_A::XOSC0),
            3 => Val(REFCLK_A::XOSC1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK`"]
    #[inline(always)]
    pub fn is_gclk(&self) -> bool {
        *self == REFCLK_A::GCLK
    }
    #[doc = "Checks if the value of the field is `XOSC32`"]
    #[inline(always)]
    pub fn is_xosc32(&self) -> bool {
        *self == REFCLK_A::XOSC32
    }
    #[doc = "Checks if the value of the field is `XOSC0`"]
    #[inline(always)]
    pub fn is_xosc0(&self) -> bool {
        *self == REFCLK_A::XOSC0
    }
    #[doc = "Checks if the value of the field is `XOSC1`"]
    #[inline(always)]
    pub fn is_xosc1(&self) -> bool {
        *self == REFCLK_A::XOSC1
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
    #[doc = "Dedicated GCLK clock reference"]
    #[inline(always)]
    pub fn gclk(self) -> &'a mut W {
        self.variant(REFCLK_A::GCLK)
    }
    #[doc = "XOSC32K clock reference"]
    #[inline(always)]
    pub fn xosc32(self) -> &'a mut W {
        self.variant(REFCLK_A::XOSC32)
    }
    #[doc = "XOSC0 clock reference"]
    #[inline(always)]
    pub fn xosc0(self) -> &'a mut W {
        self.variant(REFCLK_A::XOSC0)
    }
    #[doc = "XOSC1 clock reference"]
    #[inline(always)]
    pub fn xosc1(self) -> &'a mut W {
        self.variant(REFCLK_A::XOSC1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Lock Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LTIME_A {
    #[doc = "0: No time-out. Automatic lock"]
    DEFAULT = 0,
    #[doc = "4: Time-out if no lock within 800us"]
    _800US = 4,
    #[doc = "5: Time-out if no lock within 900us"]
    _900US = 5,
    #[doc = "6: Time-out if no lock within 1ms"]
    _1MS = 6,
    #[doc = "7: Time-out if no lock within 1.1ms"]
    _1P1MS = 7,
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
            0 => Val(LTIME_A::DEFAULT),
            4 => Val(LTIME_A::_800US),
            5 => Val(LTIME_A::_900US),
            6 => Val(LTIME_A::_1MS),
            7 => Val(LTIME_A::_1P1MS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == LTIME_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `_800US`"]
    #[inline(always)]
    pub fn is_800us(&self) -> bool {
        *self == LTIME_A::_800US
    }
    #[doc = "Checks if the value of the field is `_900US`"]
    #[inline(always)]
    pub fn is_900us(&self) -> bool {
        *self == LTIME_A::_900US
    }
    #[doc = "Checks if the value of the field is `_1MS`"]
    #[inline(always)]
    pub fn is_1ms(&self) -> bool {
        *self == LTIME_A::_1MS
    }
    #[doc = "Checks if the value of the field is `_1P1MS`"]
    #[inline(always)]
    pub fn is_1p1ms(&self) -> bool {
        *self == LTIME_A::_1P1MS
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
    #[doc = "No time-out. Automatic lock"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(LTIME_A::DEFAULT)
    }
    #[doc = "Time-out if no lock within 800us"]
    #[inline(always)]
    pub fn _800us(self) -> &'a mut W {
        self.variant(LTIME_A::_800US)
    }
    #[doc = "Time-out if no lock within 900us"]
    #[inline(always)]
    pub fn _900us(self) -> &'a mut W {
        self.variant(LTIME_A::_900US)
    }
    #[doc = "Time-out if no lock within 1ms"]
    #[inline(always)]
    pub fn _1ms(self) -> &'a mut W {
        self.variant(LTIME_A::_1MS)
    }
    #[doc = "Time-out if no lock within 1.1ms"]
    #[inline(always)]
    pub fn _1p1ms(self) -> &'a mut W {
        self.variant(LTIME_A::_1P1MS)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `DCOFILTER`"]
pub type DCOFILTER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DCOFILTER`"]
pub struct DCOFILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> DCOFILTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `DCOEN`"]
pub type DCOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCOEN`"]
pub struct DCOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCOEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
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
    #[doc = "Bits 0:3 - Proportional Integral Filter Selection"]
    #[inline(always)]
    pub fn filter(&self) -> FILTER_R {
        FILTER_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Wake Up Fast"]
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Reference Clock Selection"]
    #[inline(always)]
    pub fn refclk(&self) -> REFCLK_R {
        REFCLK_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Lock Time"]
    #[inline(always)]
    pub fn ltime(&self) -> LTIME_R {
        LTIME_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Lock Bypass"]
    #[inline(always)]
    pub fn lbypass(&self) -> LBYPASS_R {
        LBYPASS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - Sigma-Delta DCO Filter Selection"]
    #[inline(always)]
    pub fn dcofilter(&self) -> DCOFILTER_R {
        DCOFILTER_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - DCO Filter Enable"]
    #[inline(always)]
    pub fn dcoen(&self) -> DCOEN_R {
        DCOEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:26 - Clock Divider"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Proportional Integral Filter Selection"]
    #[inline(always)]
    pub fn filter(&mut self) -> FILTER_W {
        FILTER_W { w: self }
    }
    #[doc = "Bit 4 - Wake Up Fast"]
    #[inline(always)]
    pub fn wuf(&mut self) -> WUF_W {
        WUF_W { w: self }
    }
    #[doc = "Bits 5:7 - Reference Clock Selection"]
    #[inline(always)]
    pub fn refclk(&mut self) -> REFCLK_W {
        REFCLK_W { w: self }
    }
    #[doc = "Bits 8:10 - Lock Time"]
    #[inline(always)]
    pub fn ltime(&mut self) -> LTIME_W {
        LTIME_W { w: self }
    }
    #[doc = "Bit 11 - Lock Bypass"]
    #[inline(always)]
    pub fn lbypass(&mut self) -> LBYPASS_W {
        LBYPASS_W { w: self }
    }
    #[doc = "Bits 12:14 - Sigma-Delta DCO Filter Selection"]
    #[inline(always)]
    pub fn dcofilter(&mut self) -> DCOFILTER_W {
        DCOFILTER_W { w: self }
    }
    #[doc = "Bit 15 - DCO Filter Enable"]
    #[inline(always)]
    pub fn dcoen(&mut self) -> DCOEN_W {
        DCOEN_W { w: self }
    }
    #[doc = "Bits 16:26 - Clock Divider"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
}
