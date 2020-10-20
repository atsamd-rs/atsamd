#[doc = "Reader of register PATT"]
pub type R = crate::R<u16, super::PATT>;
#[doc = "Writer for register PATT"]
pub type W = crate::W<u16, super::PATT>;
#[doc = "Register PATT `reset()`'s with value 0"]
impl crate::ResetValue for super::PATT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PGE0`"]
pub type PGE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGE0`"]
pub struct PGE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PGE0_W<'a> {
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
#[doc = "Reader of field `PGE1`"]
pub type PGE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGE1`"]
pub struct PGE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PGE1_W<'a> {
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
#[doc = "Reader of field `PGE2`"]
pub type PGE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGE2`"]
pub struct PGE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PGE2_W<'a> {
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
#[doc = "Reader of field `PGE3`"]
pub type PGE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGE3`"]
pub struct PGE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PGE3_W<'a> {
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
#[doc = "Reader of field `PGE4`"]
pub type PGE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGE4`"]
pub struct PGE4_W<'a> {
    w: &'a mut W,
}
impl<'a> PGE4_W<'a> {
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
#[doc = "Reader of field `PGE5`"]
pub type PGE5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGE5`"]
pub struct PGE5_W<'a> {
    w: &'a mut W,
}
impl<'a> PGE5_W<'a> {
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
#[doc = "Reader of field `PGE6`"]
pub type PGE6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGE6`"]
pub struct PGE6_W<'a> {
    w: &'a mut W,
}
impl<'a> PGE6_W<'a> {
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
#[doc = "Reader of field `PGE7`"]
pub type PGE7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGE7`"]
pub struct PGE7_W<'a> {
    w: &'a mut W,
}
impl<'a> PGE7_W<'a> {
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
#[doc = "Reader of field `PGV0`"]
pub type PGV0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGV0`"]
pub struct PGV0_W<'a> {
    w: &'a mut W,
}
impl<'a> PGV0_W<'a> {
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
#[doc = "Reader of field `PGV1`"]
pub type PGV1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGV1`"]
pub struct PGV1_W<'a> {
    w: &'a mut W,
}
impl<'a> PGV1_W<'a> {
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
#[doc = "Reader of field `PGV2`"]
pub type PGV2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGV2`"]
pub struct PGV2_W<'a> {
    w: &'a mut W,
}
impl<'a> PGV2_W<'a> {
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
#[doc = "Reader of field `PGV3`"]
pub type PGV3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGV3`"]
pub struct PGV3_W<'a> {
    w: &'a mut W,
}
impl<'a> PGV3_W<'a> {
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
#[doc = "Reader of field `PGV4`"]
pub type PGV4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGV4`"]
pub struct PGV4_W<'a> {
    w: &'a mut W,
}
impl<'a> PGV4_W<'a> {
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
#[doc = "Reader of field `PGV5`"]
pub type PGV5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGV5`"]
pub struct PGV5_W<'a> {
    w: &'a mut W,
}
impl<'a> PGV5_W<'a> {
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
#[doc = "Reader of field `PGV6`"]
pub type PGV6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGV6`"]
pub struct PGV6_W<'a> {
    w: &'a mut W,
}
impl<'a> PGV6_W<'a> {
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
#[doc = "Reader of field `PGV7`"]
pub type PGV7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGV7`"]
pub struct PGV7_W<'a> {
    w: &'a mut W,
}
impl<'a> PGV7_W<'a> {
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
    #[doc = "Bit 0 - Pattern Generator 0 Output Enable"]
    #[inline(always)]
    pub fn pge0(&self) -> PGE0_R {
        PGE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pattern Generator 1 Output Enable"]
    #[inline(always)]
    pub fn pge1(&self) -> PGE1_R {
        PGE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pattern Generator 2 Output Enable"]
    #[inline(always)]
    pub fn pge2(&self) -> PGE2_R {
        PGE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pattern Generator 3 Output Enable"]
    #[inline(always)]
    pub fn pge3(&self) -> PGE3_R {
        PGE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pattern Generator 4 Output Enable"]
    #[inline(always)]
    pub fn pge4(&self) -> PGE4_R {
        PGE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pattern Generator 5 Output Enable"]
    #[inline(always)]
    pub fn pge5(&self) -> PGE5_R {
        PGE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Pattern Generator 6 Output Enable"]
    #[inline(always)]
    pub fn pge6(&self) -> PGE6_R {
        PGE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Pattern Generator 7 Output Enable"]
    #[inline(always)]
    pub fn pge7(&self) -> PGE7_R {
        PGE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pattern Generator 0 Output Value"]
    #[inline(always)]
    pub fn pgv0(&self) -> PGV0_R {
        PGV0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pattern Generator 1 Output Value"]
    #[inline(always)]
    pub fn pgv1(&self) -> PGV1_R {
        PGV1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Pattern Generator 2 Output Value"]
    #[inline(always)]
    pub fn pgv2(&self) -> PGV2_R {
        PGV2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Pattern Generator 3 Output Value"]
    #[inline(always)]
    pub fn pgv3(&self) -> PGV3_R {
        PGV3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pattern Generator 4 Output Value"]
    #[inline(always)]
    pub fn pgv4(&self) -> PGV4_R {
        PGV4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pattern Generator 5 Output Value"]
    #[inline(always)]
    pub fn pgv5(&self) -> PGV5_R {
        PGV5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Pattern Generator 6 Output Value"]
    #[inline(always)]
    pub fn pgv6(&self) -> PGV6_R {
        PGV6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Pattern Generator 7 Output Value"]
    #[inline(always)]
    pub fn pgv7(&self) -> PGV7_R {
        PGV7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pattern Generator 0 Output Enable"]
    #[inline(always)]
    pub fn pge0(&mut self) -> PGE0_W {
        PGE0_W { w: self }
    }
    #[doc = "Bit 1 - Pattern Generator 1 Output Enable"]
    #[inline(always)]
    pub fn pge1(&mut self) -> PGE1_W {
        PGE1_W { w: self }
    }
    #[doc = "Bit 2 - Pattern Generator 2 Output Enable"]
    #[inline(always)]
    pub fn pge2(&mut self) -> PGE2_W {
        PGE2_W { w: self }
    }
    #[doc = "Bit 3 - Pattern Generator 3 Output Enable"]
    #[inline(always)]
    pub fn pge3(&mut self) -> PGE3_W {
        PGE3_W { w: self }
    }
    #[doc = "Bit 4 - Pattern Generator 4 Output Enable"]
    #[inline(always)]
    pub fn pge4(&mut self) -> PGE4_W {
        PGE4_W { w: self }
    }
    #[doc = "Bit 5 - Pattern Generator 5 Output Enable"]
    #[inline(always)]
    pub fn pge5(&mut self) -> PGE5_W {
        PGE5_W { w: self }
    }
    #[doc = "Bit 6 - Pattern Generator 6 Output Enable"]
    #[inline(always)]
    pub fn pge6(&mut self) -> PGE6_W {
        PGE6_W { w: self }
    }
    #[doc = "Bit 7 - Pattern Generator 7 Output Enable"]
    #[inline(always)]
    pub fn pge7(&mut self) -> PGE7_W {
        PGE7_W { w: self }
    }
    #[doc = "Bit 8 - Pattern Generator 0 Output Value"]
    #[inline(always)]
    pub fn pgv0(&mut self) -> PGV0_W {
        PGV0_W { w: self }
    }
    #[doc = "Bit 9 - Pattern Generator 1 Output Value"]
    #[inline(always)]
    pub fn pgv1(&mut self) -> PGV1_W {
        PGV1_W { w: self }
    }
    #[doc = "Bit 10 - Pattern Generator 2 Output Value"]
    #[inline(always)]
    pub fn pgv2(&mut self) -> PGV2_W {
        PGV2_W { w: self }
    }
    #[doc = "Bit 11 - Pattern Generator 3 Output Value"]
    #[inline(always)]
    pub fn pgv3(&mut self) -> PGV3_W {
        PGV3_W { w: self }
    }
    #[doc = "Bit 12 - Pattern Generator 4 Output Value"]
    #[inline(always)]
    pub fn pgv4(&mut self) -> PGV4_W {
        PGV4_W { w: self }
    }
    #[doc = "Bit 13 - Pattern Generator 5 Output Value"]
    #[inline(always)]
    pub fn pgv5(&mut self) -> PGV5_W {
        PGV5_W { w: self }
    }
    #[doc = "Bit 14 - Pattern Generator 6 Output Value"]
    #[inline(always)]
    pub fn pgv6(&mut self) -> PGV6_W {
        PGV6_W { w: self }
    }
    #[doc = "Bit 15 - Pattern Generator 7 Output Value"]
    #[inline(always)]
    pub fn pgv7(&mut self) -> PGV7_W {
        PGV7_W { w: self }
    }
}
