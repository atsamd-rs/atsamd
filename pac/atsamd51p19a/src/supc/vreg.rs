#[doc = "Reader of register VREG"]
pub type R = crate::R<u32, super::VREG>;
#[doc = "Writer for register VREG"]
pub type W = crate::W<u32, super::VREG>;
#[doc = "Register VREG `reset()`'s with value 0x02"]
impl crate::ResetValue for super::VREG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
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
#[doc = "Voltage Regulator Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_A {
    #[doc = "0: LDO selection"]
    LDO = 0,
    #[doc = "1: Buck selection"]
    BUCK = 1,
}
impl From<SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SEL`"]
pub type SEL_R = crate::R<bool, SEL_A>;
impl SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            false => SEL_A::LDO,
            true => SEL_A::BUCK,
        }
    }
    #[doc = "Checks if the value of the field is `LDO`"]
    #[inline(always)]
    pub fn is_ldo(&self) -> bool {
        *self == SEL_A::LDO
    }
    #[doc = "Checks if the value of the field is `BUCK`"]
    #[inline(always)]
    pub fn is_buck(&self) -> bool {
        *self == SEL_A::BUCK
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
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LDO selection"]
    #[inline(always)]
    pub fn ldo(self) -> &'a mut W {
        self.variant(SEL_A::LDO)
    }
    #[doc = "Buck selection"]
    #[inline(always)]
    pub fn buck(self) -> &'a mut W {
        self.variant(SEL_A::BUCK)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `RUNBKUP`"]
pub type RUNBKUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RUNBKUP`"]
pub struct RUNBKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNBKUP_W<'a> {
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
#[doc = "Reader of field `VSEN`"]
pub type VSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VSEN`"]
pub struct VSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VSEN_W<'a> {
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
#[doc = "Reader of field `VSPER`"]
pub type VSPER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VSPER`"]
pub struct VSPER_W<'a> {
    w: &'a mut W,
}
impl<'a> VSPER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Voltage Regulator Selection"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Run in Backup mode"]
    #[inline(always)]
    pub fn runbkup(&self) -> RUNBKUP_R {
        RUNBKUP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Voltage Scaling Enable"]
    #[inline(always)]
    pub fn vsen(&self) -> VSEN_R {
        VSEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Voltage Scaling Period"]
    #[inline(always)]
    pub fn vsper(&self) -> VSPER_R {
        VSPER_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - Voltage Regulator Selection"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Bit 7 - Run in Backup mode"]
    #[inline(always)]
    pub fn runbkup(&mut self) -> RUNBKUP_W {
        RUNBKUP_W { w: self }
    }
    #[doc = "Bit 16 - Voltage Scaling Enable"]
    #[inline(always)]
    pub fn vsen(&mut self) -> VSEN_W {
        VSEN_W { w: self }
    }
    #[doc = "Bits 24:26 - Voltage Scaling Period"]
    #[inline(always)]
    pub fn vsper(&mut self) -> VSPER_W {
        VSPER_W { w: self }
    }
}
