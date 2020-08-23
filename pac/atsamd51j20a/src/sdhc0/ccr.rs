#[doc = "Reader of register CCR"]
pub type R = crate::R<u16, super::CCR>;
#[doc = "Writer for register CCR"]
pub type W = crate::W<u16, super::CCR>;
#[doc = "Register CCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CCR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Internal Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTCLKEN_A {
    #[doc = "0: Stop"]
    OFF = 0,
    #[doc = "1: Oscillate"]
    ON = 1,
}
impl From<INTCLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: INTCLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INTCLKEN`"]
pub type INTCLKEN_R = crate::R<bool, INTCLKEN_A>;
impl INTCLKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTCLKEN_A {
        match self.bits {
            false => INTCLKEN_A::OFF,
            true => INTCLKEN_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == INTCLKEN_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == INTCLKEN_A::ON
    }
}
#[doc = "Write proxy for field `INTCLKEN`"]
pub struct INTCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTCLKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(INTCLKEN_A::OFF)
    }
    #[doc = "Oscillate"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(INTCLKEN_A::ON)
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
#[doc = "Internal Clock Stable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTCLKS_A {
    #[doc = "0: Not Ready"]
    NOT_READY = 0,
    #[doc = "1: Ready"]
    READY = 1,
}
impl From<INTCLKS_A> for bool {
    #[inline(always)]
    fn from(variant: INTCLKS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INTCLKS`"]
pub type INTCLKS_R = crate::R<bool, INTCLKS_A>;
impl INTCLKS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTCLKS_A {
        match self.bits {
            false => INTCLKS_A::NOT_READY,
            true => INTCLKS_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_READY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == INTCLKS_A::NOT_READY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == INTCLKS_A::READY
    }
}
#[doc = "Write proxy for field `INTCLKS`"]
pub struct INTCLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCLKS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTCLKS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not Ready"]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(INTCLKS_A::NOT_READY)
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(INTCLKS_A::READY)
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
#[doc = "SD Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDCLKEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<SDCLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: SDCLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDCLKEN`"]
pub type SDCLKEN_R = crate::R<bool, SDCLKEN_A>;
impl SDCLKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDCLKEN_A {
        match self.bits {
            false => SDCLKEN_A::DISABLE,
            true => SDCLKEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SDCLKEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SDCLKEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `SDCLKEN`"]
pub struct SDCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDCLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDCLKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDCLKEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDCLKEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Clock Generator Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKGSEL_A {
    #[doc = "0: Divided Clock Mode"]
    DIV = 0,
    #[doc = "1: Programmable Clock Mode"]
    PROG = 1,
}
impl From<CLKGSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CLKGSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLKGSEL`"]
pub type CLKGSEL_R = crate::R<bool, CLKGSEL_A>;
impl CLKGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKGSEL_A {
        match self.bits {
            false => CLKGSEL_A::DIV,
            true => CLKGSEL_A::PROG,
        }
    }
    #[doc = "Checks if the value of the field is `DIV`"]
    #[inline(always)]
    pub fn is_div(&self) -> bool {
        *self == CLKGSEL_A::DIV
    }
    #[doc = "Checks if the value of the field is `PROG`"]
    #[inline(always)]
    pub fn is_prog(&self) -> bool {
        *self == CLKGSEL_A::PROG
    }
}
#[doc = "Write proxy for field `CLKGSEL`"]
pub struct CLKGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKGSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Divided Clock Mode"]
    #[inline(always)]
    pub fn div(self) -> &'a mut W {
        self.variant(CLKGSEL_A::DIV)
    }
    #[doc = "Programmable Clock Mode"]
    #[inline(always)]
    pub fn prog(self) -> &'a mut W {
        self.variant(CLKGSEL_A::PROG)
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
#[doc = "Reader of field `USDCLKFSEL`"]
pub type USDCLKFSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USDCLKFSEL`"]
pub struct USDCLKFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USDCLKFSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `SDCLKFSEL`"]
pub type SDCLKFSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDCLKFSEL`"]
pub struct SDCLKFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDCLKFSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline(always)]
    pub fn intclken(&self) -> INTCLKEN_R {
        INTCLKEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Internal Clock Stable"]
    #[inline(always)]
    pub fn intclks(&self) -> INTCLKS_R {
        INTCLKS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SD Clock Enable"]
    #[inline(always)]
    pub fn sdclken(&self) -> SDCLKEN_R {
        SDCLKEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clock Generator Select"]
    #[inline(always)]
    pub fn clkgsel(&self) -> CLKGSEL_R {
        CLKGSEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Upper Bits of SDCLK Frequency Select"]
    #[inline(always)]
    pub fn usdclkfsel(&self) -> USDCLKFSEL_R {
        USDCLKFSEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline(always)]
    pub fn sdclkfsel(&self) -> SDCLKFSEL_R {
        SDCLKFSEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline(always)]
    pub fn intclken(&mut self) -> INTCLKEN_W {
        INTCLKEN_W { w: self }
    }
    #[doc = "Bit 1 - Internal Clock Stable"]
    #[inline(always)]
    pub fn intclks(&mut self) -> INTCLKS_W {
        INTCLKS_W { w: self }
    }
    #[doc = "Bit 2 - SD Clock Enable"]
    #[inline(always)]
    pub fn sdclken(&mut self) -> SDCLKEN_W {
        SDCLKEN_W { w: self }
    }
    #[doc = "Bit 5 - Clock Generator Select"]
    #[inline(always)]
    pub fn clkgsel(&mut self) -> CLKGSEL_W {
        CLKGSEL_W { w: self }
    }
    #[doc = "Bits 6:7 - Upper Bits of SDCLK Frequency Select"]
    #[inline(always)]
    pub fn usdclkfsel(&mut self) -> USDCLKFSEL_W {
        USDCLKFSEL_W { w: self }
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline(always)]
    pub fn sdclkfsel(&mut self) -> SDCLKFSEL_W {
        SDCLKFSEL_W { w: self }
    }
}
