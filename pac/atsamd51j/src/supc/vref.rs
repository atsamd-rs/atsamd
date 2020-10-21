#[doc = "Reader of register VREF"]
pub type R = crate::R<u32, super::VREF>;
#[doc = "Writer for register VREF"]
pub type W = crate::W<u32, super::VREF>;
#[doc = "Register VREF `reset()`'s with value 0"]
impl crate::ResetValue for super::VREF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSEN`"]
pub type TSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSEN`"]
pub struct TSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEN_W<'a> {
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
#[doc = "Reader of field `VREFOE`"]
pub type VREFOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VREFOE`"]
pub struct VREFOE_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFOE_W<'a> {
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
#[doc = "Reader of field `TSSEL`"]
pub type TSSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSSEL`"]
pub struct TSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSSEL_W<'a> {
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
#[doc = "Reader of field `ONDEMAND`"]
pub type ONDEMAND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ONDEMAND`"]
pub struct ONDEMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> ONDEMAND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Voltage Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: 1.0V voltage reference typical value"]
    _1V0 = 0,
    #[doc = "1: 1.1V voltage reference typical value"]
    _1V1 = 1,
    #[doc = "2: 1.2V voltage reference typical value"]
    _1V2 = 2,
    #[doc = "3: 1.25V voltage reference typical value"]
    _1V25 = 3,
    #[doc = "4: 2.0V voltage reference typical value"]
    _2V0 = 4,
    #[doc = "5: 2.2V voltage reference typical value"]
    _2V2 = 5,
    #[doc = "6: 2.4V voltage reference typical value"]
    _2V4 = 6,
    #[doc = "7: 2.5V voltage reference typical value"]
    _2V5 = 7,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SEL`"]
pub type SEL_R = crate::R<u8, SEL_A>;
impl SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SEL_A::_1V0),
            1 => Val(SEL_A::_1V1),
            2 => Val(SEL_A::_1V2),
            3 => Val(SEL_A::_1V25),
            4 => Val(SEL_A::_2V0),
            5 => Val(SEL_A::_2V2),
            6 => Val(SEL_A::_2V4),
            7 => Val(SEL_A::_2V5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1V0`"]
    #[inline(always)]
    pub fn is_1v0(&self) -> bool {
        *self == SEL_A::_1V0
    }
    #[doc = "Checks if the value of the field is `_1V1`"]
    #[inline(always)]
    pub fn is_1v1(&self) -> bool {
        *self == SEL_A::_1V1
    }
    #[doc = "Checks if the value of the field is `_1V2`"]
    #[inline(always)]
    pub fn is_1v2(&self) -> bool {
        *self == SEL_A::_1V2
    }
    #[doc = "Checks if the value of the field is `_1V25`"]
    #[inline(always)]
    pub fn is_1v25(&self) -> bool {
        *self == SEL_A::_1V25
    }
    #[doc = "Checks if the value of the field is `_2V0`"]
    #[inline(always)]
    pub fn is_2v0(&self) -> bool {
        *self == SEL_A::_2V0
    }
    #[doc = "Checks if the value of the field is `_2V2`"]
    #[inline(always)]
    pub fn is_2v2(&self) -> bool {
        *self == SEL_A::_2V2
    }
    #[doc = "Checks if the value of the field is `_2V4`"]
    #[inline(always)]
    pub fn is_2v4(&self) -> bool {
        *self == SEL_A::_2V4
    }
    #[doc = "Checks if the value of the field is `_2V5`"]
    #[inline(always)]
    pub fn is_2v5(&self) -> bool {
        *self == SEL_A::_2V5
    }
}
#[doc = "Write proxy for field `SEL`"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1.0V voltage reference typical value"]
    #[inline(always)]
    pub fn _1v0(self) -> &'a mut W {
        self.variant(SEL_A::_1V0)
    }
    #[doc = "1.1V voltage reference typical value"]
    #[inline(always)]
    pub fn _1v1(self) -> &'a mut W {
        self.variant(SEL_A::_1V1)
    }
    #[doc = "1.2V voltage reference typical value"]
    #[inline(always)]
    pub fn _1v2(self) -> &'a mut W {
        self.variant(SEL_A::_1V2)
    }
    #[doc = "1.25V voltage reference typical value"]
    #[inline(always)]
    pub fn _1v25(self) -> &'a mut W {
        self.variant(SEL_A::_1V25)
    }
    #[doc = "2.0V voltage reference typical value"]
    #[inline(always)]
    pub fn _2v0(self) -> &'a mut W {
        self.variant(SEL_A::_2V0)
    }
    #[doc = "2.2V voltage reference typical value"]
    #[inline(always)]
    pub fn _2v2(self) -> &'a mut W {
        self.variant(SEL_A::_2V2)
    }
    #[doc = "2.4V voltage reference typical value"]
    #[inline(always)]
    pub fn _2v4(self) -> &'a mut W {
        self.variant(SEL_A::_2V4)
    }
    #[doc = "2.5V voltage reference typical value"]
    #[inline(always)]
    pub fn _2v5(self) -> &'a mut W {
        self.variant(SEL_A::_2V5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Temperature Sensor Output Enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Voltage Reference Output Enable"]
    #[inline(always)]
    pub fn vrefoe(&self) -> VREFOE_R {
        VREFOE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Temperature Sensor Selection"]
    #[inline(always)]
    pub fn tssel(&self) -> TSSEL_R {
        TSSEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - On Demand Contrl"]
    #[inline(always)]
    pub fn ondemand(&self) -> ONDEMAND_R {
        ONDEMAND_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Voltage Reference Selection"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Temperature Sensor Output Enable"]
    #[inline(always)]
    pub fn tsen(&mut self) -> TSEN_W {
        TSEN_W { w: self }
    }
    #[doc = "Bit 2 - Voltage Reference Output Enable"]
    #[inline(always)]
    pub fn vrefoe(&mut self) -> VREFOE_W {
        VREFOE_W { w: self }
    }
    #[doc = "Bit 3 - Temperature Sensor Selection"]
    #[inline(always)]
    pub fn tssel(&mut self) -> TSSEL_W {
        TSSEL_W { w: self }
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
    #[doc = "Bit 7 - On Demand Contrl"]
    #[inline(always)]
    pub fn ondemand(&mut self) -> ONDEMAND_W {
        ONDEMAND_W { w: self }
    }
    #[doc = "Bits 16:19 - Voltage Reference Selection"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
}
