#[doc = "Reader of register EICCPUSEL0"]
pub type R = crate::R<u32, super::EICCPUSEL0>;
#[doc = "Writer for register EICCPUSEL0"]
pub type W = crate::W<u32, super::EICCPUSEL0>;
#[doc = "Register EICCPUSEL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::EICCPUSEL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTINT0`"]
pub type EXTINT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTINT0`"]
pub struct EXTINT0_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT0_W<'a> {
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
#[doc = "Reader of field `EXTINT1`"]
pub type EXTINT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTINT1`"]
pub struct EXTINT1_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT1_W<'a> {
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
#[doc = "Reader of field `EXTINT2`"]
pub type EXTINT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTINT2`"]
pub struct EXTINT2_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT2_W<'a> {
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
#[doc = "Reader of field `EXTINT3`"]
pub type EXTINT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTINT3`"]
pub struct EXTINT3_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT3_W<'a> {
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
#[doc = "Reader of field `EXTINT4`"]
pub type EXTINT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTINT4`"]
pub struct EXTINT4_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `EXTINT5`"]
pub type EXTINT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTINT5`"]
pub struct EXTINT5_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `EXTINT6`"]
pub type EXTINT6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTINT6`"]
pub struct EXTINT6_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT6_W<'a> {
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
#[doc = "Reader of field `EXTINT7`"]
pub type EXTINT7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTINT7`"]
pub struct EXTINT7_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT7_W<'a> {
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
#[doc = "Reader of field `EXTINT8`"]
pub type EXTINT8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTINT8`"]
pub struct EXTINT8_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT8_W<'a> {
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
#[doc = "Reader of field `EXTINT9`"]
pub type EXTINT9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTINT9`"]
pub struct EXTINT9_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT9_W<'a> {
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
#[doc = "Reader of field `EXTINT10`"]
pub type EXTINT10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTINT10`"]
pub struct EXTINT10_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT10_W<'a> {
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
#[doc = "Reader of field `EXTINT11`"]
pub type EXTINT11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTINT11`"]
pub struct EXTINT11_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT11_W<'a> {
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
#[doc = "Reader of field `EXTINT12`"]
pub type EXTINT12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTINT12`"]
pub struct EXTINT12_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `EXTINT13`"]
pub type EXTINT13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTINT13`"]
pub struct EXTINT13_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `EXTINT14`"]
pub type EXTINT14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTINT14`"]
pub struct EXTINT14_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `EXTINT15`"]
pub type EXTINT15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTINT15`"]
pub struct EXTINT15_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTINT15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - External Interrupt 0 CPU Select"]
    #[inline(always)]
    pub fn extint0(&self) -> EXTINT0_R {
        EXTINT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Interrupt 1 CPU Select"]
    #[inline(always)]
    pub fn extint1(&self) -> EXTINT1_R {
        EXTINT1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - External Interrupt 2 CPU Select"]
    #[inline(always)]
    pub fn extint2(&self) -> EXTINT2_R {
        EXTINT2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - External Interrupt 3 CPU Select"]
    #[inline(always)]
    pub fn extint3(&self) -> EXTINT3_R {
        EXTINT3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - External Interrupt 4 CPU Select"]
    #[inline(always)]
    pub fn extint4(&self) -> EXTINT4_R {
        EXTINT4_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - External Interrupt 5 CPU Select"]
    #[inline(always)]
    pub fn extint5(&self) -> EXTINT5_R {
        EXTINT5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - External Interrupt 6 CPU Select"]
    #[inline(always)]
    pub fn extint6(&self) -> EXTINT6_R {
        EXTINT6_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - External Interrupt 7 CPU Select"]
    #[inline(always)]
    pub fn extint7(&self) -> EXTINT7_R {
        EXTINT7_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - External Interrupt 8 CPU Select"]
    #[inline(always)]
    pub fn extint8(&self) -> EXTINT8_R {
        EXTINT8_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - External Interrupt 9 CPU Select"]
    #[inline(always)]
    pub fn extint9(&self) -> EXTINT9_R {
        EXTINT9_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - External Interrupt 10 CPU Select"]
    #[inline(always)]
    pub fn extint10(&self) -> EXTINT10_R {
        EXTINT10_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 22 - External Interrupt 11 CPU Select"]
    #[inline(always)]
    pub fn extint11(&self) -> EXTINT11_R {
        EXTINT11_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - External Interrupt 12 CPU Select"]
    #[inline(always)]
    pub fn extint12(&self) -> EXTINT12_R {
        EXTINT12_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - External Interrupt 13 CPU Select"]
    #[inline(always)]
    pub fn extint13(&self) -> EXTINT13_R {
        EXTINT13_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - External Interrupt 14 CPU Select"]
    #[inline(always)]
    pub fn extint14(&self) -> EXTINT14_R {
        EXTINT14_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - External Interrupt 15 CPU Select"]
    #[inline(always)]
    pub fn extint15(&self) -> EXTINT15_R {
        EXTINT15_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Interrupt 0 CPU Select"]
    #[inline(always)]
    pub fn extint0(&mut self) -> EXTINT0_W {
        EXTINT0_W { w: self }
    }
    #[doc = "Bit 2 - External Interrupt 1 CPU Select"]
    #[inline(always)]
    pub fn extint1(&mut self) -> EXTINT1_W {
        EXTINT1_W { w: self }
    }
    #[doc = "Bit 4 - External Interrupt 2 CPU Select"]
    #[inline(always)]
    pub fn extint2(&mut self) -> EXTINT2_W {
        EXTINT2_W { w: self }
    }
    #[doc = "Bit 6 - External Interrupt 3 CPU Select"]
    #[inline(always)]
    pub fn extint3(&mut self) -> EXTINT3_W {
        EXTINT3_W { w: self }
    }
    #[doc = "Bit 8 - External Interrupt 4 CPU Select"]
    #[inline(always)]
    pub fn extint4(&mut self) -> EXTINT4_W {
        EXTINT4_W { w: self }
    }
    #[doc = "Bit 10 - External Interrupt 5 CPU Select"]
    #[inline(always)]
    pub fn extint5(&mut self) -> EXTINT5_W {
        EXTINT5_W { w: self }
    }
    #[doc = "Bit 12 - External Interrupt 6 CPU Select"]
    #[inline(always)]
    pub fn extint6(&mut self) -> EXTINT6_W {
        EXTINT6_W { w: self }
    }
    #[doc = "Bit 14 - External Interrupt 7 CPU Select"]
    #[inline(always)]
    pub fn extint7(&mut self) -> EXTINT7_W {
        EXTINT7_W { w: self }
    }
    #[doc = "Bit 16 - External Interrupt 8 CPU Select"]
    #[inline(always)]
    pub fn extint8(&mut self) -> EXTINT8_W {
        EXTINT8_W { w: self }
    }
    #[doc = "Bit 18 - External Interrupt 9 CPU Select"]
    #[inline(always)]
    pub fn extint9(&mut self) -> EXTINT9_W {
        EXTINT9_W { w: self }
    }
    #[doc = "Bit 20 - External Interrupt 10 CPU Select"]
    #[inline(always)]
    pub fn extint10(&mut self) -> EXTINT10_W {
        EXTINT10_W { w: self }
    }
    #[doc = "Bit 22 - External Interrupt 11 CPU Select"]
    #[inline(always)]
    pub fn extint11(&mut self) -> EXTINT11_W {
        EXTINT11_W { w: self }
    }
    #[doc = "Bit 24 - External Interrupt 12 CPU Select"]
    #[inline(always)]
    pub fn extint12(&mut self) -> EXTINT12_W {
        EXTINT12_W { w: self }
    }
    #[doc = "Bit 26 - External Interrupt 13 CPU Select"]
    #[inline(always)]
    pub fn extint13(&mut self) -> EXTINT13_W {
        EXTINT13_W { w: self }
    }
    #[doc = "Bit 28 - External Interrupt 14 CPU Select"]
    #[inline(always)]
    pub fn extint14(&mut self) -> EXTINT14_W {
        EXTINT14_W { w: self }
    }
    #[doc = "Bit 30 - External Interrupt 15 CPU Select"]
    #[inline(always)]
    pub fn extint15(&mut self) -> EXTINT15_W {
        EXTINT15_W { w: self }
    }
}
