#[doc = "Reader of register PATTBUF"]
pub type R = crate::R<u16, super::PATTBUF>;
#[doc = "Writer for register PATTBUF"]
pub type W = crate::W<u16, super::PATTBUF>;
#[doc = "Register PATTBUF `reset()`'s with value 0"]
impl crate::ResetValue for super::PATTBUF {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PGEB0`"]
pub type PGEB0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGEB0`"]
pub struct PGEB0_W<'a> {
    w: &'a mut W,
}
impl<'a> PGEB0_W<'a> {
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
#[doc = "Reader of field `PGEB1`"]
pub type PGEB1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGEB1`"]
pub struct PGEB1_W<'a> {
    w: &'a mut W,
}
impl<'a> PGEB1_W<'a> {
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
#[doc = "Reader of field `PGEB2`"]
pub type PGEB2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGEB2`"]
pub struct PGEB2_W<'a> {
    w: &'a mut W,
}
impl<'a> PGEB2_W<'a> {
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
#[doc = "Reader of field `PGEB3`"]
pub type PGEB3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGEB3`"]
pub struct PGEB3_W<'a> {
    w: &'a mut W,
}
impl<'a> PGEB3_W<'a> {
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
#[doc = "Reader of field `PGEB4`"]
pub type PGEB4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGEB4`"]
pub struct PGEB4_W<'a> {
    w: &'a mut W,
}
impl<'a> PGEB4_W<'a> {
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
#[doc = "Reader of field `PGEB5`"]
pub type PGEB5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGEB5`"]
pub struct PGEB5_W<'a> {
    w: &'a mut W,
}
impl<'a> PGEB5_W<'a> {
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
#[doc = "Reader of field `PGEB6`"]
pub type PGEB6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGEB6`"]
pub struct PGEB6_W<'a> {
    w: &'a mut W,
}
impl<'a> PGEB6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `PGEB7`"]
pub type PGEB7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGEB7`"]
pub struct PGEB7_W<'a> {
    w: &'a mut W,
}
impl<'a> PGEB7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `PGVB0`"]
pub type PGVB0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGVB0`"]
pub struct PGVB0_W<'a> {
    w: &'a mut W,
}
impl<'a> PGVB0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `PGVB1`"]
pub type PGVB1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGVB1`"]
pub struct PGVB1_W<'a> {
    w: &'a mut W,
}
impl<'a> PGVB1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `PGVB2`"]
pub type PGVB2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGVB2`"]
pub struct PGVB2_W<'a> {
    w: &'a mut W,
}
impl<'a> PGVB2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `PGVB3`"]
pub type PGVB3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGVB3`"]
pub struct PGVB3_W<'a> {
    w: &'a mut W,
}
impl<'a> PGVB3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `PGVB4`"]
pub type PGVB4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGVB4`"]
pub struct PGVB4_W<'a> {
    w: &'a mut W,
}
impl<'a> PGVB4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `PGVB5`"]
pub type PGVB5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGVB5`"]
pub struct PGVB5_W<'a> {
    w: &'a mut W,
}
impl<'a> PGVB5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `PGVB6`"]
pub type PGVB6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGVB6`"]
pub struct PGVB6_W<'a> {
    w: &'a mut W,
}
impl<'a> PGVB6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `PGVB7`"]
pub type PGVB7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGVB7`"]
pub struct PGVB7_W<'a> {
    w: &'a mut W,
}
impl<'a> PGVB7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Pattern Generator 0 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb0(&self) -> PGEB0_R {
        PGEB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pattern Generator 1 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb1(&self) -> PGEB1_R {
        PGEB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pattern Generator 2 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb2(&self) -> PGEB2_R {
        PGEB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pattern Generator 3 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb3(&self) -> PGEB3_R {
        PGEB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pattern Generator 4 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb4(&self) -> PGEB4_R {
        PGEB4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pattern Generator 5 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb5(&self) -> PGEB5_R {
        PGEB5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Pattern Generator 6 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb6(&self) -> PGEB6_R {
        PGEB6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Pattern Generator 7 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb7(&self) -> PGEB7_R {
        PGEB7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pattern Generator 0 Output Enable"]
    #[inline(always)]
    pub fn pgvb0(&self) -> PGVB0_R {
        PGVB0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pattern Generator 1 Output Enable"]
    #[inline(always)]
    pub fn pgvb1(&self) -> PGVB1_R {
        PGVB1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Pattern Generator 2 Output Enable"]
    #[inline(always)]
    pub fn pgvb2(&self) -> PGVB2_R {
        PGVB2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Pattern Generator 3 Output Enable"]
    #[inline(always)]
    pub fn pgvb3(&self) -> PGVB3_R {
        PGVB3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pattern Generator 4 Output Enable"]
    #[inline(always)]
    pub fn pgvb4(&self) -> PGVB4_R {
        PGVB4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pattern Generator 5 Output Enable"]
    #[inline(always)]
    pub fn pgvb5(&self) -> PGVB5_R {
        PGVB5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Pattern Generator 6 Output Enable"]
    #[inline(always)]
    pub fn pgvb6(&self) -> PGVB6_R {
        PGVB6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Pattern Generator 7 Output Enable"]
    #[inline(always)]
    pub fn pgvb7(&self) -> PGVB7_R {
        PGVB7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pattern Generator 0 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb0(&mut self) -> PGEB0_W {
        PGEB0_W { w: self }
    }
    #[doc = "Bit 1 - Pattern Generator 1 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb1(&mut self) -> PGEB1_W {
        PGEB1_W { w: self }
    }
    #[doc = "Bit 2 - Pattern Generator 2 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb2(&mut self) -> PGEB2_W {
        PGEB2_W { w: self }
    }
    #[doc = "Bit 3 - Pattern Generator 3 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb3(&mut self) -> PGEB3_W {
        PGEB3_W { w: self }
    }
    #[doc = "Bit 4 - Pattern Generator 4 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb4(&mut self) -> PGEB4_W {
        PGEB4_W { w: self }
    }
    #[doc = "Bit 5 - Pattern Generator 5 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb5(&mut self) -> PGEB5_W {
        PGEB5_W { w: self }
    }
    #[doc = "Bit 6 - Pattern Generator 6 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb6(&mut self) -> PGEB6_W {
        PGEB6_W { w: self }
    }
    #[doc = "Bit 7 - Pattern Generator 7 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb7(&mut self) -> PGEB7_W {
        PGEB7_W { w: self }
    }
    #[doc = "Bit 8 - Pattern Generator 0 Output Enable"]
    #[inline(always)]
    pub fn pgvb0(&mut self) -> PGVB0_W {
        PGVB0_W { w: self }
    }
    #[doc = "Bit 9 - Pattern Generator 1 Output Enable"]
    #[inline(always)]
    pub fn pgvb1(&mut self) -> PGVB1_W {
        PGVB1_W { w: self }
    }
    #[doc = "Bit 10 - Pattern Generator 2 Output Enable"]
    #[inline(always)]
    pub fn pgvb2(&mut self) -> PGVB2_W {
        PGVB2_W { w: self }
    }
    #[doc = "Bit 11 - Pattern Generator 3 Output Enable"]
    #[inline(always)]
    pub fn pgvb3(&mut self) -> PGVB3_W {
        PGVB3_W { w: self }
    }
    #[doc = "Bit 12 - Pattern Generator 4 Output Enable"]
    #[inline(always)]
    pub fn pgvb4(&mut self) -> PGVB4_W {
        PGVB4_W { w: self }
    }
    #[doc = "Bit 13 - Pattern Generator 5 Output Enable"]
    #[inline(always)]
    pub fn pgvb5(&mut self) -> PGVB5_W {
        PGVB5_W { w: self }
    }
    #[doc = "Bit 14 - Pattern Generator 6 Output Enable"]
    #[inline(always)]
    pub fn pgvb6(&mut self) -> PGVB6_W {
        PGVB6_W { w: self }
    }
    #[doc = "Bit 15 - Pattern Generator 7 Output Enable"]
    #[inline(always)]
    pub fn pgvb7(&mut self) -> PGVB7_W {
        PGVB7_W { w: self }
    }
}
