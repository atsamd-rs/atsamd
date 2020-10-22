#[doc = "Reader of register TXBAR"]
pub type R = crate::R<u32, super::TXBAR>;
#[doc = "Writer for register TXBAR"]
pub type W = crate::W<u32, super::TXBAR>;
#[doc = "Register TXBAR `reset()`'s with value 0"]
impl crate::ResetValue for super::TXBAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AR0`"]
pub type AR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR0`"]
pub struct AR0_W<'a> {
    w: &'a mut W,
}
impl<'a> AR0_W<'a> {
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
#[doc = "Reader of field `AR1`"]
pub type AR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR1`"]
pub struct AR1_W<'a> {
    w: &'a mut W,
}
impl<'a> AR1_W<'a> {
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
#[doc = "Reader of field `AR2`"]
pub type AR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR2`"]
pub struct AR2_W<'a> {
    w: &'a mut W,
}
impl<'a> AR2_W<'a> {
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
#[doc = "Reader of field `AR3`"]
pub type AR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR3`"]
pub struct AR3_W<'a> {
    w: &'a mut W,
}
impl<'a> AR3_W<'a> {
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
#[doc = "Reader of field `AR4`"]
pub type AR4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR4`"]
pub struct AR4_W<'a> {
    w: &'a mut W,
}
impl<'a> AR4_W<'a> {
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
#[doc = "Reader of field `AR5`"]
pub type AR5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR5`"]
pub struct AR5_W<'a> {
    w: &'a mut W,
}
impl<'a> AR5_W<'a> {
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
#[doc = "Reader of field `AR6`"]
pub type AR6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR6`"]
pub struct AR6_W<'a> {
    w: &'a mut W,
}
impl<'a> AR6_W<'a> {
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
#[doc = "Reader of field `AR7`"]
pub type AR7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR7`"]
pub struct AR7_W<'a> {
    w: &'a mut W,
}
impl<'a> AR7_W<'a> {
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
#[doc = "Reader of field `AR8`"]
pub type AR8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR8`"]
pub struct AR8_W<'a> {
    w: &'a mut W,
}
impl<'a> AR8_W<'a> {
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
#[doc = "Reader of field `AR9`"]
pub type AR9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR9`"]
pub struct AR9_W<'a> {
    w: &'a mut W,
}
impl<'a> AR9_W<'a> {
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
#[doc = "Reader of field `AR10`"]
pub type AR10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR10`"]
pub struct AR10_W<'a> {
    w: &'a mut W,
}
impl<'a> AR10_W<'a> {
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
#[doc = "Reader of field `AR11`"]
pub type AR11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR11`"]
pub struct AR11_W<'a> {
    w: &'a mut W,
}
impl<'a> AR11_W<'a> {
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
#[doc = "Reader of field `AR12`"]
pub type AR12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR12`"]
pub struct AR12_W<'a> {
    w: &'a mut W,
}
impl<'a> AR12_W<'a> {
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
#[doc = "Reader of field `AR13`"]
pub type AR13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR13`"]
pub struct AR13_W<'a> {
    w: &'a mut W,
}
impl<'a> AR13_W<'a> {
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
#[doc = "Reader of field `AR14`"]
pub type AR14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR14`"]
pub struct AR14_W<'a> {
    w: &'a mut W,
}
impl<'a> AR14_W<'a> {
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
#[doc = "Reader of field `AR15`"]
pub type AR15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR15`"]
pub struct AR15_W<'a> {
    w: &'a mut W,
}
impl<'a> AR15_W<'a> {
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
#[doc = "Reader of field `AR16`"]
pub type AR16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR16`"]
pub struct AR16_W<'a> {
    w: &'a mut W,
}
impl<'a> AR16_W<'a> {
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
#[doc = "Reader of field `AR17`"]
pub type AR17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR17`"]
pub struct AR17_W<'a> {
    w: &'a mut W,
}
impl<'a> AR17_W<'a> {
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
#[doc = "Reader of field `AR18`"]
pub type AR18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR18`"]
pub struct AR18_W<'a> {
    w: &'a mut W,
}
impl<'a> AR18_W<'a> {
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
#[doc = "Reader of field `AR19`"]
pub type AR19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR19`"]
pub struct AR19_W<'a> {
    w: &'a mut W,
}
impl<'a> AR19_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `AR20`"]
pub type AR20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR20`"]
pub struct AR20_W<'a> {
    w: &'a mut W,
}
impl<'a> AR20_W<'a> {
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
#[doc = "Reader of field `AR21`"]
pub type AR21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR21`"]
pub struct AR21_W<'a> {
    w: &'a mut W,
}
impl<'a> AR21_W<'a> {
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
#[doc = "Reader of field `AR22`"]
pub type AR22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR22`"]
pub struct AR22_W<'a> {
    w: &'a mut W,
}
impl<'a> AR22_W<'a> {
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
#[doc = "Reader of field `AR23`"]
pub type AR23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR23`"]
pub struct AR23_W<'a> {
    w: &'a mut W,
}
impl<'a> AR23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `AR24`"]
pub type AR24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR24`"]
pub struct AR24_W<'a> {
    w: &'a mut W,
}
impl<'a> AR24_W<'a> {
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
#[doc = "Reader of field `AR25`"]
pub type AR25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR25`"]
pub struct AR25_W<'a> {
    w: &'a mut W,
}
impl<'a> AR25_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `AR26`"]
pub type AR26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR26`"]
pub struct AR26_W<'a> {
    w: &'a mut W,
}
impl<'a> AR26_W<'a> {
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
#[doc = "Reader of field `AR27`"]
pub type AR27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR27`"]
pub struct AR27_W<'a> {
    w: &'a mut W,
}
impl<'a> AR27_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `AR28`"]
pub type AR28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR28`"]
pub struct AR28_W<'a> {
    w: &'a mut W,
}
impl<'a> AR28_W<'a> {
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
#[doc = "Reader of field `AR29`"]
pub type AR29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR29`"]
pub struct AR29_W<'a> {
    w: &'a mut W,
}
impl<'a> AR29_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `AR30`"]
pub type AR30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR30`"]
pub struct AR30_W<'a> {
    w: &'a mut W,
}
impl<'a> AR30_W<'a> {
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
#[doc = "Reader of field `AR31`"]
pub type AR31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR31`"]
pub struct AR31_W<'a> {
    w: &'a mut W,
}
impl<'a> AR31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Add Request 0"]
    #[inline(always)]
    pub fn ar0(&self) -> AR0_R {
        AR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Add Request 1"]
    #[inline(always)]
    pub fn ar1(&self) -> AR1_R {
        AR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Add Request 2"]
    #[inline(always)]
    pub fn ar2(&self) -> AR2_R {
        AR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Add Request 3"]
    #[inline(always)]
    pub fn ar3(&self) -> AR3_R {
        AR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Add Request 4"]
    #[inline(always)]
    pub fn ar4(&self) -> AR4_R {
        AR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Add Request 5"]
    #[inline(always)]
    pub fn ar5(&self) -> AR5_R {
        AR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Add Request 6"]
    #[inline(always)]
    pub fn ar6(&self) -> AR6_R {
        AR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Add Request 7"]
    #[inline(always)]
    pub fn ar7(&self) -> AR7_R {
        AR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Add Request 8"]
    #[inline(always)]
    pub fn ar8(&self) -> AR8_R {
        AR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Add Request 9"]
    #[inline(always)]
    pub fn ar9(&self) -> AR9_R {
        AR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Add Request 10"]
    #[inline(always)]
    pub fn ar10(&self) -> AR10_R {
        AR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Add Request 11"]
    #[inline(always)]
    pub fn ar11(&self) -> AR11_R {
        AR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Add Request 12"]
    #[inline(always)]
    pub fn ar12(&self) -> AR12_R {
        AR12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Add Request 13"]
    #[inline(always)]
    pub fn ar13(&self) -> AR13_R {
        AR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Add Request 14"]
    #[inline(always)]
    pub fn ar14(&self) -> AR14_R {
        AR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Add Request 15"]
    #[inline(always)]
    pub fn ar15(&self) -> AR15_R {
        AR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Add Request 16"]
    #[inline(always)]
    pub fn ar16(&self) -> AR16_R {
        AR16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Add Request 17"]
    #[inline(always)]
    pub fn ar17(&self) -> AR17_R {
        AR17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Add Request 18"]
    #[inline(always)]
    pub fn ar18(&self) -> AR18_R {
        AR18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Add Request 19"]
    #[inline(always)]
    pub fn ar19(&self) -> AR19_R {
        AR19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Add Request 20"]
    #[inline(always)]
    pub fn ar20(&self) -> AR20_R {
        AR20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Add Request 21"]
    #[inline(always)]
    pub fn ar21(&self) -> AR21_R {
        AR21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Add Request 22"]
    #[inline(always)]
    pub fn ar22(&self) -> AR22_R {
        AR22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Add Request 23"]
    #[inline(always)]
    pub fn ar23(&self) -> AR23_R {
        AR23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Add Request 24"]
    #[inline(always)]
    pub fn ar24(&self) -> AR24_R {
        AR24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Add Request 25"]
    #[inline(always)]
    pub fn ar25(&self) -> AR25_R {
        AR25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Add Request 26"]
    #[inline(always)]
    pub fn ar26(&self) -> AR26_R {
        AR26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Add Request 27"]
    #[inline(always)]
    pub fn ar27(&self) -> AR27_R {
        AR27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Add Request 28"]
    #[inline(always)]
    pub fn ar28(&self) -> AR28_R {
        AR28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Add Request 29"]
    #[inline(always)]
    pub fn ar29(&self) -> AR29_R {
        AR29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Add Request 30"]
    #[inline(always)]
    pub fn ar30(&self) -> AR30_R {
        AR30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Add Request 31"]
    #[inline(always)]
    pub fn ar31(&self) -> AR31_R {
        AR31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Add Request 0"]
    #[inline(always)]
    pub fn ar0(&mut self) -> AR0_W {
        AR0_W { w: self }
    }
    #[doc = "Bit 1 - Add Request 1"]
    #[inline(always)]
    pub fn ar1(&mut self) -> AR1_W {
        AR1_W { w: self }
    }
    #[doc = "Bit 2 - Add Request 2"]
    #[inline(always)]
    pub fn ar2(&mut self) -> AR2_W {
        AR2_W { w: self }
    }
    #[doc = "Bit 3 - Add Request 3"]
    #[inline(always)]
    pub fn ar3(&mut self) -> AR3_W {
        AR3_W { w: self }
    }
    #[doc = "Bit 4 - Add Request 4"]
    #[inline(always)]
    pub fn ar4(&mut self) -> AR4_W {
        AR4_W { w: self }
    }
    #[doc = "Bit 5 - Add Request 5"]
    #[inline(always)]
    pub fn ar5(&mut self) -> AR5_W {
        AR5_W { w: self }
    }
    #[doc = "Bit 6 - Add Request 6"]
    #[inline(always)]
    pub fn ar6(&mut self) -> AR6_W {
        AR6_W { w: self }
    }
    #[doc = "Bit 7 - Add Request 7"]
    #[inline(always)]
    pub fn ar7(&mut self) -> AR7_W {
        AR7_W { w: self }
    }
    #[doc = "Bit 8 - Add Request 8"]
    #[inline(always)]
    pub fn ar8(&mut self) -> AR8_W {
        AR8_W { w: self }
    }
    #[doc = "Bit 9 - Add Request 9"]
    #[inline(always)]
    pub fn ar9(&mut self) -> AR9_W {
        AR9_W { w: self }
    }
    #[doc = "Bit 10 - Add Request 10"]
    #[inline(always)]
    pub fn ar10(&mut self) -> AR10_W {
        AR10_W { w: self }
    }
    #[doc = "Bit 11 - Add Request 11"]
    #[inline(always)]
    pub fn ar11(&mut self) -> AR11_W {
        AR11_W { w: self }
    }
    #[doc = "Bit 12 - Add Request 12"]
    #[inline(always)]
    pub fn ar12(&mut self) -> AR12_W {
        AR12_W { w: self }
    }
    #[doc = "Bit 13 - Add Request 13"]
    #[inline(always)]
    pub fn ar13(&mut self) -> AR13_W {
        AR13_W { w: self }
    }
    #[doc = "Bit 14 - Add Request 14"]
    #[inline(always)]
    pub fn ar14(&mut self) -> AR14_W {
        AR14_W { w: self }
    }
    #[doc = "Bit 15 - Add Request 15"]
    #[inline(always)]
    pub fn ar15(&mut self) -> AR15_W {
        AR15_W { w: self }
    }
    #[doc = "Bit 16 - Add Request 16"]
    #[inline(always)]
    pub fn ar16(&mut self) -> AR16_W {
        AR16_W { w: self }
    }
    #[doc = "Bit 17 - Add Request 17"]
    #[inline(always)]
    pub fn ar17(&mut self) -> AR17_W {
        AR17_W { w: self }
    }
    #[doc = "Bit 18 - Add Request 18"]
    #[inline(always)]
    pub fn ar18(&mut self) -> AR18_W {
        AR18_W { w: self }
    }
    #[doc = "Bit 19 - Add Request 19"]
    #[inline(always)]
    pub fn ar19(&mut self) -> AR19_W {
        AR19_W { w: self }
    }
    #[doc = "Bit 20 - Add Request 20"]
    #[inline(always)]
    pub fn ar20(&mut self) -> AR20_W {
        AR20_W { w: self }
    }
    #[doc = "Bit 21 - Add Request 21"]
    #[inline(always)]
    pub fn ar21(&mut self) -> AR21_W {
        AR21_W { w: self }
    }
    #[doc = "Bit 22 - Add Request 22"]
    #[inline(always)]
    pub fn ar22(&mut self) -> AR22_W {
        AR22_W { w: self }
    }
    #[doc = "Bit 23 - Add Request 23"]
    #[inline(always)]
    pub fn ar23(&mut self) -> AR23_W {
        AR23_W { w: self }
    }
    #[doc = "Bit 24 - Add Request 24"]
    #[inline(always)]
    pub fn ar24(&mut self) -> AR24_W {
        AR24_W { w: self }
    }
    #[doc = "Bit 25 - Add Request 25"]
    #[inline(always)]
    pub fn ar25(&mut self) -> AR25_W {
        AR25_W { w: self }
    }
    #[doc = "Bit 26 - Add Request 26"]
    #[inline(always)]
    pub fn ar26(&mut self) -> AR26_W {
        AR26_W { w: self }
    }
    #[doc = "Bit 27 - Add Request 27"]
    #[inline(always)]
    pub fn ar27(&mut self) -> AR27_W {
        AR27_W { w: self }
    }
    #[doc = "Bit 28 - Add Request 28"]
    #[inline(always)]
    pub fn ar28(&mut self) -> AR28_W {
        AR28_W { w: self }
    }
    #[doc = "Bit 29 - Add Request 29"]
    #[inline(always)]
    pub fn ar29(&mut self) -> AR29_W {
        AR29_W { w: self }
    }
    #[doc = "Bit 30 - Add Request 30"]
    #[inline(always)]
    pub fn ar30(&mut self) -> AR30_W {
        AR30_W { w: self }
    }
    #[doc = "Bit 31 - Add Request 31"]
    #[inline(always)]
    pub fn ar31(&mut self) -> AR31_W {
        AR31_W { w: self }
    }
}
