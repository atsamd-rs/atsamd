#[doc = "Reader of register CTRLA"]
pub type R = crate::R<u32, super::CTRLA>;
#[doc = "Writer for register CTRLA"]
pub type W = crate::W<u32, super::CTRLA>;
#[doc = "Register CTRLA `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SWRST`"]
pub type SWRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWRST`"]
pub struct SWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Operation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: QDEC operating mode"]
    QDEC = 0,
    #[doc = "1: HALL operating mode"]
    HALL = 1,
    #[doc = "2: COUNTER operating mode"]
    COUNTER = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::QDEC),
            1 => Val(MODE_A::HALL),
            2 => Val(MODE_A::COUNTER),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `QDEC`"]
    #[inline(always)]
    pub fn is_qdec(&self) -> bool {
        *self == MODE_A::QDEC
    }
    #[doc = "Checks if the value of the field is `HALL`"]
    #[inline(always)]
    pub fn is_hall(&self) -> bool {
        *self == MODE_A::HALL
    }
    #[doc = "Checks if the value of the field is `COUNTER`"]
    #[inline(always)]
    pub fn is_counter(&self) -> bool {
        *self == MODE_A::COUNTER
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "QDEC operating mode"]
    #[inline(always)]
    pub fn qdec(self) -> &'a mut W {
        self.variant(MODE_A::QDEC)
    }
    #[doc = "HALL operating mode"]
    #[inline(always)]
    pub fn hall(self) -> &'a mut W {
        self.variant(MODE_A::HALL)
    }
    #[doc = "COUNTER operating mode"]
    #[inline(always)]
    pub fn counter(self) -> &'a mut W {
        self.variant(MODE_A::COUNTER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `RUNSTDBY`"]
pub type RUNSTDBY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RUNSTDBY`"]
pub struct RUNSTDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNSTDBY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "PDEC Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CONF_A {
    #[doc = "0: Quadrature decoder direction"]
    X4 = 0,
    #[doc = "1: Secure Quadrature decoder direction"]
    X4S = 1,
    #[doc = "2: Decoder direction"]
    X2 = 2,
    #[doc = "3: Secure decoder direction"]
    X2S = 3,
    #[doc = "4: Auto correction mode"]
    AUTOC = 4,
}
impl From<CONF_A> for u8 {
    #[inline(always)]
    fn from(variant: CONF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CONF`"]
pub type CONF_R = crate::R<u8, CONF_A>;
impl CONF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CONF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CONF_A::X4),
            1 => Val(CONF_A::X4S),
            2 => Val(CONF_A::X2),
            3 => Val(CONF_A::X2S),
            4 => Val(CONF_A::AUTOC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `X4`"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == CONF_A::X4
    }
    #[doc = "Checks if the value of the field is `X4S`"]
    #[inline(always)]
    pub fn is_x4s(&self) -> bool {
        *self == CONF_A::X4S
    }
    #[doc = "Checks if the value of the field is `X2`"]
    #[inline(always)]
    pub fn is_x2(&self) -> bool {
        *self == CONF_A::X2
    }
    #[doc = "Checks if the value of the field is `X2S`"]
    #[inline(always)]
    pub fn is_x2s(&self) -> bool {
        *self == CONF_A::X2S
    }
    #[doc = "Checks if the value of the field is `AUTOC`"]
    #[inline(always)]
    pub fn is_autoc(&self) -> bool {
        *self == CONF_A::AUTOC
    }
}
#[doc = "Write proxy for field `CONF`"]
pub struct CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> CONF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Quadrature decoder direction"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut W {
        self.variant(CONF_A::X4)
    }
    #[doc = "Secure Quadrature decoder direction"]
    #[inline(always)]
    pub fn x4s(self) -> &'a mut W {
        self.variant(CONF_A::X4S)
    }
    #[doc = "Decoder direction"]
    #[inline(always)]
    pub fn x2(self) -> &'a mut W {
        self.variant(CONF_A::X2)
    }
    #[doc = "Secure decoder direction"]
    #[inline(always)]
    pub fn x2s(self) -> &'a mut W {
        self.variant(CONF_A::X2S)
    }
    #[doc = "Auto correction mode"]
    #[inline(always)]
    pub fn autoc(self) -> &'a mut W {
        self.variant(CONF_A::AUTOC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `ALOCK`"]
pub type ALOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALOCK`"]
pub struct ALOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> ALOCK_W<'a> {
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
#[doc = "Reader of field `SWAP`"]
pub type SWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWAP`"]
pub struct SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `PEREN`"]
pub type PEREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEREN`"]
pub struct PEREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREN_W<'a> {
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
#[doc = "Reader of field `PINEN0`"]
pub type PINEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINEN0`"]
pub struct PINEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PINEN0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `PINEN1`"]
pub type PINEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINEN1`"]
pub struct PINEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PINEN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `PINEN2`"]
pub type PINEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINEN2`"]
pub struct PINEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PINEN2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `PINVEN0`"]
pub type PINVEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVEN0`"]
pub struct PINVEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVEN0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `PINVEN1`"]
pub type PINVEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVEN1`"]
pub struct PINVEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVEN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `PINVEN2`"]
pub type PINVEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVEN2`"]
pub struct PINVEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVEN2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `ANGULAR`"]
pub type ANGULAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ANGULAR`"]
pub struct ANGULAR_W<'a> {
    w: &'a mut W,
}
impl<'a> ANGULAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `MAXCMP`"]
pub type MAXCMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAXCMP`"]
pub struct MAXCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXCMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Operation Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - PDEC Configuration"]
    #[inline(always)]
    pub fn conf(&self) -> CONF_R {
        CONF_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Auto Lock"]
    #[inline(always)]
    pub fn alock(&self) -> ALOCK_R {
        ALOCK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PDEC Phase A and B Swap"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Period Enable"]
    #[inline(always)]
    pub fn peren(&self) -> PEREN_R {
        PEREN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PDEC Input From Pin 0 Enable"]
    #[inline(always)]
    pub fn pinen0(&self) -> PINEN0_R {
        PINEN0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PDEC Input From Pin 1 Enable"]
    #[inline(always)]
    pub fn pinen1(&self) -> PINEN1_R {
        PINEN1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PDEC Input From Pin 2 Enable"]
    #[inline(always)]
    pub fn pinen2(&self) -> PINEN2_R {
        PINEN2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - IO Pin 0 Invert Enable"]
    #[inline(always)]
    pub fn pinven0(&self) -> PINVEN0_R {
        PINVEN0_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - IO Pin 1 Invert Enable"]
    #[inline(always)]
    pub fn pinven1(&self) -> PINVEN1_R {
        PINVEN1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - IO Pin 2 Invert Enable"]
    #[inline(always)]
    pub fn pinven2(&self) -> PINVEN2_R {
        PINVEN2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Angular Counter Length"]
    #[inline(always)]
    pub fn angular(&self) -> ANGULAR_R {
        ANGULAR_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:31 - Maximum Consecutive Missing Pulses"]
    #[inline(always)]
    pub fn maxcmp(&self) -> MAXCMP_R {
        MAXCMP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 2:3 - Operation Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
    #[doc = "Bits 8:10 - PDEC Configuration"]
    #[inline(always)]
    pub fn conf(&mut self) -> CONF_W {
        CONF_W { w: self }
    }
    #[doc = "Bit 11 - Auto Lock"]
    #[inline(always)]
    pub fn alock(&mut self) -> ALOCK_W {
        ALOCK_W { w: self }
    }
    #[doc = "Bit 14 - PDEC Phase A and B Swap"]
    #[inline(always)]
    pub fn swap(&mut self) -> SWAP_W {
        SWAP_W { w: self }
    }
    #[doc = "Bit 15 - Period Enable"]
    #[inline(always)]
    pub fn peren(&mut self) -> PEREN_W {
        PEREN_W { w: self }
    }
    #[doc = "Bit 16 - PDEC Input From Pin 0 Enable"]
    #[inline(always)]
    pub fn pinen0(&mut self) -> PINEN0_W {
        PINEN0_W { w: self }
    }
    #[doc = "Bit 17 - PDEC Input From Pin 1 Enable"]
    #[inline(always)]
    pub fn pinen1(&mut self) -> PINEN1_W {
        PINEN1_W { w: self }
    }
    #[doc = "Bit 18 - PDEC Input From Pin 2 Enable"]
    #[inline(always)]
    pub fn pinen2(&mut self) -> PINEN2_W {
        PINEN2_W { w: self }
    }
    #[doc = "Bit 20 - IO Pin 0 Invert Enable"]
    #[inline(always)]
    pub fn pinven0(&mut self) -> PINVEN0_W {
        PINVEN0_W { w: self }
    }
    #[doc = "Bit 21 - IO Pin 1 Invert Enable"]
    #[inline(always)]
    pub fn pinven1(&mut self) -> PINVEN1_W {
        PINVEN1_W { w: self }
    }
    #[doc = "Bit 22 - IO Pin 2 Invert Enable"]
    #[inline(always)]
    pub fn pinven2(&mut self) -> PINVEN2_W {
        PINVEN2_W { w: self }
    }
    #[doc = "Bits 24:26 - Angular Counter Length"]
    #[inline(always)]
    pub fn angular(&mut self) -> ANGULAR_W {
        ANGULAR_W { w: self }
    }
    #[doc = "Bits 28:31 - Maximum Consecutive Missing Pulses"]
    #[inline(always)]
    pub fn maxcmp(&mut self) -> MAXCMP_W {
        MAXCMP_W { w: self }
    }
}
