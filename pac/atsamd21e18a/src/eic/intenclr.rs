#[doc = "Reader of register INTENCLR"]
pub type R = crate::R<u32, super::INTENCLR>;
#[doc = "Writer for register INTENCLR"]
pub type W = crate::W<u32, super::INTENCLR>;
#[doc = "Register INTENCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENCLR {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - External Interrupt 0 Enable"]
    #[inline(always)]
    pub fn extint0(&self) -> EXTINT0_R {
        EXTINT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External Interrupt 1 Enable"]
    #[inline(always)]
    pub fn extint1(&self) -> EXTINT1_R {
        EXTINT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Interrupt 2 Enable"]
    #[inline(always)]
    pub fn extint2(&self) -> EXTINT2_R {
        EXTINT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Interrupt 3 Enable"]
    #[inline(always)]
    pub fn extint3(&self) -> EXTINT3_R {
        EXTINT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - External Interrupt 4 Enable"]
    #[inline(always)]
    pub fn extint4(&self) -> EXTINT4_R {
        EXTINT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - External Interrupt 5 Enable"]
    #[inline(always)]
    pub fn extint5(&self) -> EXTINT5_R {
        EXTINT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - External Interrupt 6 Enable"]
    #[inline(always)]
    pub fn extint6(&self) -> EXTINT6_R {
        EXTINT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - External Interrupt 7 Enable"]
    #[inline(always)]
    pub fn extint7(&self) -> EXTINT7_R {
        EXTINT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - External Interrupt 8 Enable"]
    #[inline(always)]
    pub fn extint8(&self) -> EXTINT8_R {
        EXTINT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - External Interrupt 9 Enable"]
    #[inline(always)]
    pub fn extint9(&self) -> EXTINT9_R {
        EXTINT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - External Interrupt 10 Enable"]
    #[inline(always)]
    pub fn extint10(&self) -> EXTINT10_R {
        EXTINT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - External Interrupt 11 Enable"]
    #[inline(always)]
    pub fn extint11(&self) -> EXTINT11_R {
        EXTINT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - External Interrupt 12 Enable"]
    #[inline(always)]
    pub fn extint12(&self) -> EXTINT12_R {
        EXTINT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - External Interrupt 13 Enable"]
    #[inline(always)]
    pub fn extint13(&self) -> EXTINT13_R {
        EXTINT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - External Interrupt 14 Enable"]
    #[inline(always)]
    pub fn extint14(&self) -> EXTINT14_R {
        EXTINT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - External Interrupt 15 Enable"]
    #[inline(always)]
    pub fn extint15(&self) -> EXTINT15_R {
        EXTINT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Interrupt 0 Enable"]
    #[inline(always)]
    pub fn extint0(&mut self) -> EXTINT0_W {
        EXTINT0_W { w: self }
    }
    #[doc = "Bit 1 - External Interrupt 1 Enable"]
    #[inline(always)]
    pub fn extint1(&mut self) -> EXTINT1_W {
        EXTINT1_W { w: self }
    }
    #[doc = "Bit 2 - External Interrupt 2 Enable"]
    #[inline(always)]
    pub fn extint2(&mut self) -> EXTINT2_W {
        EXTINT2_W { w: self }
    }
    #[doc = "Bit 3 - External Interrupt 3 Enable"]
    #[inline(always)]
    pub fn extint3(&mut self) -> EXTINT3_W {
        EXTINT3_W { w: self }
    }
    #[doc = "Bit 4 - External Interrupt 4 Enable"]
    #[inline(always)]
    pub fn extint4(&mut self) -> EXTINT4_W {
        EXTINT4_W { w: self }
    }
    #[doc = "Bit 5 - External Interrupt 5 Enable"]
    #[inline(always)]
    pub fn extint5(&mut self) -> EXTINT5_W {
        EXTINT5_W { w: self }
    }
    #[doc = "Bit 6 - External Interrupt 6 Enable"]
    #[inline(always)]
    pub fn extint6(&mut self) -> EXTINT6_W {
        EXTINT6_W { w: self }
    }
    #[doc = "Bit 7 - External Interrupt 7 Enable"]
    #[inline(always)]
    pub fn extint7(&mut self) -> EXTINT7_W {
        EXTINT7_W { w: self }
    }
    #[doc = "Bit 8 - External Interrupt 8 Enable"]
    #[inline(always)]
    pub fn extint8(&mut self) -> EXTINT8_W {
        EXTINT8_W { w: self }
    }
    #[doc = "Bit 9 - External Interrupt 9 Enable"]
    #[inline(always)]
    pub fn extint9(&mut self) -> EXTINT9_W {
        EXTINT9_W { w: self }
    }
    #[doc = "Bit 10 - External Interrupt 10 Enable"]
    #[inline(always)]
    pub fn extint10(&mut self) -> EXTINT10_W {
        EXTINT10_W { w: self }
    }
    #[doc = "Bit 11 - External Interrupt 11 Enable"]
    #[inline(always)]
    pub fn extint11(&mut self) -> EXTINT11_W {
        EXTINT11_W { w: self }
    }
    #[doc = "Bit 12 - External Interrupt 12 Enable"]
    #[inline(always)]
    pub fn extint12(&mut self) -> EXTINT12_W {
        EXTINT12_W { w: self }
    }
    #[doc = "Bit 13 - External Interrupt 13 Enable"]
    #[inline(always)]
    pub fn extint13(&mut self) -> EXTINT13_W {
        EXTINT13_W { w: self }
    }
    #[doc = "Bit 14 - External Interrupt 14 Enable"]
    #[inline(always)]
    pub fn extint14(&mut self) -> EXTINT14_W {
        EXTINT14_W { w: self }
    }
    #[doc = "Bit 15 - External Interrupt 15 Enable"]
    #[inline(always)]
    pub fn extint15(&mut self) -> EXTINT15_W {
        EXTINT15_W { w: self }
    }
}
