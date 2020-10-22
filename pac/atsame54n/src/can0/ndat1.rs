#[doc = "Reader of register NDAT1"]
pub type R = crate::R<u32, super::NDAT1>;
#[doc = "Writer for register NDAT1"]
pub type W = crate::W<u32, super::NDAT1>;
#[doc = "Register NDAT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::NDAT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ND0`"]
pub type ND0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND0`"]
pub struct ND0_W<'a> {
    w: &'a mut W,
}
impl<'a> ND0_W<'a> {
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
#[doc = "Reader of field `ND1`"]
pub type ND1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND1`"]
pub struct ND1_W<'a> {
    w: &'a mut W,
}
impl<'a> ND1_W<'a> {
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
#[doc = "Reader of field `ND2`"]
pub type ND2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND2`"]
pub struct ND2_W<'a> {
    w: &'a mut W,
}
impl<'a> ND2_W<'a> {
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
#[doc = "Reader of field `ND3`"]
pub type ND3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND3`"]
pub struct ND3_W<'a> {
    w: &'a mut W,
}
impl<'a> ND3_W<'a> {
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
#[doc = "Reader of field `ND4`"]
pub type ND4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND4`"]
pub struct ND4_W<'a> {
    w: &'a mut W,
}
impl<'a> ND4_W<'a> {
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
#[doc = "Reader of field `ND5`"]
pub type ND5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND5`"]
pub struct ND5_W<'a> {
    w: &'a mut W,
}
impl<'a> ND5_W<'a> {
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
#[doc = "Reader of field `ND6`"]
pub type ND6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND6`"]
pub struct ND6_W<'a> {
    w: &'a mut W,
}
impl<'a> ND6_W<'a> {
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
#[doc = "Reader of field `ND7`"]
pub type ND7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND7`"]
pub struct ND7_W<'a> {
    w: &'a mut W,
}
impl<'a> ND7_W<'a> {
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
#[doc = "Reader of field `ND8`"]
pub type ND8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND8`"]
pub struct ND8_W<'a> {
    w: &'a mut W,
}
impl<'a> ND8_W<'a> {
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
#[doc = "Reader of field `ND9`"]
pub type ND9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND9`"]
pub struct ND9_W<'a> {
    w: &'a mut W,
}
impl<'a> ND9_W<'a> {
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
#[doc = "Reader of field `ND10`"]
pub type ND10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND10`"]
pub struct ND10_W<'a> {
    w: &'a mut W,
}
impl<'a> ND10_W<'a> {
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
#[doc = "Reader of field `ND11`"]
pub type ND11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND11`"]
pub struct ND11_W<'a> {
    w: &'a mut W,
}
impl<'a> ND11_W<'a> {
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
#[doc = "Reader of field `ND12`"]
pub type ND12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND12`"]
pub struct ND12_W<'a> {
    w: &'a mut W,
}
impl<'a> ND12_W<'a> {
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
#[doc = "Reader of field `ND13`"]
pub type ND13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND13`"]
pub struct ND13_W<'a> {
    w: &'a mut W,
}
impl<'a> ND13_W<'a> {
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
#[doc = "Reader of field `ND14`"]
pub type ND14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND14`"]
pub struct ND14_W<'a> {
    w: &'a mut W,
}
impl<'a> ND14_W<'a> {
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
#[doc = "Reader of field `ND15`"]
pub type ND15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND15`"]
pub struct ND15_W<'a> {
    w: &'a mut W,
}
impl<'a> ND15_W<'a> {
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
#[doc = "Reader of field `ND16`"]
pub type ND16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND16`"]
pub struct ND16_W<'a> {
    w: &'a mut W,
}
impl<'a> ND16_W<'a> {
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
#[doc = "Reader of field `ND17`"]
pub type ND17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND17`"]
pub struct ND17_W<'a> {
    w: &'a mut W,
}
impl<'a> ND17_W<'a> {
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
#[doc = "Reader of field `ND18`"]
pub type ND18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND18`"]
pub struct ND18_W<'a> {
    w: &'a mut W,
}
impl<'a> ND18_W<'a> {
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
#[doc = "Reader of field `ND19`"]
pub type ND19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND19`"]
pub struct ND19_W<'a> {
    w: &'a mut W,
}
impl<'a> ND19_W<'a> {
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
#[doc = "Reader of field `ND20`"]
pub type ND20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND20`"]
pub struct ND20_W<'a> {
    w: &'a mut W,
}
impl<'a> ND20_W<'a> {
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
#[doc = "Reader of field `ND21`"]
pub type ND21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND21`"]
pub struct ND21_W<'a> {
    w: &'a mut W,
}
impl<'a> ND21_W<'a> {
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
#[doc = "Reader of field `ND22`"]
pub type ND22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND22`"]
pub struct ND22_W<'a> {
    w: &'a mut W,
}
impl<'a> ND22_W<'a> {
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
#[doc = "Reader of field `ND23`"]
pub type ND23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND23`"]
pub struct ND23_W<'a> {
    w: &'a mut W,
}
impl<'a> ND23_W<'a> {
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
#[doc = "Reader of field `ND24`"]
pub type ND24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND24`"]
pub struct ND24_W<'a> {
    w: &'a mut W,
}
impl<'a> ND24_W<'a> {
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
#[doc = "Reader of field `ND25`"]
pub type ND25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND25`"]
pub struct ND25_W<'a> {
    w: &'a mut W,
}
impl<'a> ND25_W<'a> {
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
#[doc = "Reader of field `ND26`"]
pub type ND26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND26`"]
pub struct ND26_W<'a> {
    w: &'a mut W,
}
impl<'a> ND26_W<'a> {
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
#[doc = "Reader of field `ND27`"]
pub type ND27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND27`"]
pub struct ND27_W<'a> {
    w: &'a mut W,
}
impl<'a> ND27_W<'a> {
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
#[doc = "Reader of field `ND28`"]
pub type ND28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND28`"]
pub struct ND28_W<'a> {
    w: &'a mut W,
}
impl<'a> ND28_W<'a> {
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
#[doc = "Reader of field `ND29`"]
pub type ND29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND29`"]
pub struct ND29_W<'a> {
    w: &'a mut W,
}
impl<'a> ND29_W<'a> {
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
#[doc = "Reader of field `ND30`"]
pub type ND30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND30`"]
pub struct ND30_W<'a> {
    w: &'a mut W,
}
impl<'a> ND30_W<'a> {
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
#[doc = "Reader of field `ND31`"]
pub type ND31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND31`"]
pub struct ND31_W<'a> {
    w: &'a mut W,
}
impl<'a> ND31_W<'a> {
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
    #[doc = "Bit 0 - New Data 0"]
    #[inline(always)]
    pub fn nd0(&self) -> ND0_R {
        ND0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - New Data 1"]
    #[inline(always)]
    pub fn nd1(&self) -> ND1_R {
        ND1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - New Data 2"]
    #[inline(always)]
    pub fn nd2(&self) -> ND2_R {
        ND2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - New Data 3"]
    #[inline(always)]
    pub fn nd3(&self) -> ND3_R {
        ND3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - New Data 4"]
    #[inline(always)]
    pub fn nd4(&self) -> ND4_R {
        ND4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - New Data 5"]
    #[inline(always)]
    pub fn nd5(&self) -> ND5_R {
        ND5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - New Data 6"]
    #[inline(always)]
    pub fn nd6(&self) -> ND6_R {
        ND6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - New Data 7"]
    #[inline(always)]
    pub fn nd7(&self) -> ND7_R {
        ND7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - New Data 8"]
    #[inline(always)]
    pub fn nd8(&self) -> ND8_R {
        ND8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - New Data 9"]
    #[inline(always)]
    pub fn nd9(&self) -> ND9_R {
        ND9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - New Data 10"]
    #[inline(always)]
    pub fn nd10(&self) -> ND10_R {
        ND10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - New Data 11"]
    #[inline(always)]
    pub fn nd11(&self) -> ND11_R {
        ND11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - New Data 12"]
    #[inline(always)]
    pub fn nd12(&self) -> ND12_R {
        ND12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - New Data 13"]
    #[inline(always)]
    pub fn nd13(&self) -> ND13_R {
        ND13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - New Data 14"]
    #[inline(always)]
    pub fn nd14(&self) -> ND14_R {
        ND14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - New Data 15"]
    #[inline(always)]
    pub fn nd15(&self) -> ND15_R {
        ND15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - New Data 16"]
    #[inline(always)]
    pub fn nd16(&self) -> ND16_R {
        ND16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - New Data 17"]
    #[inline(always)]
    pub fn nd17(&self) -> ND17_R {
        ND17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - New Data 18"]
    #[inline(always)]
    pub fn nd18(&self) -> ND18_R {
        ND18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - New Data 19"]
    #[inline(always)]
    pub fn nd19(&self) -> ND19_R {
        ND19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - New Data 20"]
    #[inline(always)]
    pub fn nd20(&self) -> ND20_R {
        ND20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - New Data 21"]
    #[inline(always)]
    pub fn nd21(&self) -> ND21_R {
        ND21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - New Data 22"]
    #[inline(always)]
    pub fn nd22(&self) -> ND22_R {
        ND22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - New Data 23"]
    #[inline(always)]
    pub fn nd23(&self) -> ND23_R {
        ND23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - New Data 24"]
    #[inline(always)]
    pub fn nd24(&self) -> ND24_R {
        ND24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - New Data 25"]
    #[inline(always)]
    pub fn nd25(&self) -> ND25_R {
        ND25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - New Data 26"]
    #[inline(always)]
    pub fn nd26(&self) -> ND26_R {
        ND26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - New Data 27"]
    #[inline(always)]
    pub fn nd27(&self) -> ND27_R {
        ND27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - New Data 28"]
    #[inline(always)]
    pub fn nd28(&self) -> ND28_R {
        ND28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - New Data 29"]
    #[inline(always)]
    pub fn nd29(&self) -> ND29_R {
        ND29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - New Data 30"]
    #[inline(always)]
    pub fn nd30(&self) -> ND30_R {
        ND30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - New Data 31"]
    #[inline(always)]
    pub fn nd31(&self) -> ND31_R {
        ND31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - New Data 0"]
    #[inline(always)]
    pub fn nd0(&mut self) -> ND0_W {
        ND0_W { w: self }
    }
    #[doc = "Bit 1 - New Data 1"]
    #[inline(always)]
    pub fn nd1(&mut self) -> ND1_W {
        ND1_W { w: self }
    }
    #[doc = "Bit 2 - New Data 2"]
    #[inline(always)]
    pub fn nd2(&mut self) -> ND2_W {
        ND2_W { w: self }
    }
    #[doc = "Bit 3 - New Data 3"]
    #[inline(always)]
    pub fn nd3(&mut self) -> ND3_W {
        ND3_W { w: self }
    }
    #[doc = "Bit 4 - New Data 4"]
    #[inline(always)]
    pub fn nd4(&mut self) -> ND4_W {
        ND4_W { w: self }
    }
    #[doc = "Bit 5 - New Data 5"]
    #[inline(always)]
    pub fn nd5(&mut self) -> ND5_W {
        ND5_W { w: self }
    }
    #[doc = "Bit 6 - New Data 6"]
    #[inline(always)]
    pub fn nd6(&mut self) -> ND6_W {
        ND6_W { w: self }
    }
    #[doc = "Bit 7 - New Data 7"]
    #[inline(always)]
    pub fn nd7(&mut self) -> ND7_W {
        ND7_W { w: self }
    }
    #[doc = "Bit 8 - New Data 8"]
    #[inline(always)]
    pub fn nd8(&mut self) -> ND8_W {
        ND8_W { w: self }
    }
    #[doc = "Bit 9 - New Data 9"]
    #[inline(always)]
    pub fn nd9(&mut self) -> ND9_W {
        ND9_W { w: self }
    }
    #[doc = "Bit 10 - New Data 10"]
    #[inline(always)]
    pub fn nd10(&mut self) -> ND10_W {
        ND10_W { w: self }
    }
    #[doc = "Bit 11 - New Data 11"]
    #[inline(always)]
    pub fn nd11(&mut self) -> ND11_W {
        ND11_W { w: self }
    }
    #[doc = "Bit 12 - New Data 12"]
    #[inline(always)]
    pub fn nd12(&mut self) -> ND12_W {
        ND12_W { w: self }
    }
    #[doc = "Bit 13 - New Data 13"]
    #[inline(always)]
    pub fn nd13(&mut self) -> ND13_W {
        ND13_W { w: self }
    }
    #[doc = "Bit 14 - New Data 14"]
    #[inline(always)]
    pub fn nd14(&mut self) -> ND14_W {
        ND14_W { w: self }
    }
    #[doc = "Bit 15 - New Data 15"]
    #[inline(always)]
    pub fn nd15(&mut self) -> ND15_W {
        ND15_W { w: self }
    }
    #[doc = "Bit 16 - New Data 16"]
    #[inline(always)]
    pub fn nd16(&mut self) -> ND16_W {
        ND16_W { w: self }
    }
    #[doc = "Bit 17 - New Data 17"]
    #[inline(always)]
    pub fn nd17(&mut self) -> ND17_W {
        ND17_W { w: self }
    }
    #[doc = "Bit 18 - New Data 18"]
    #[inline(always)]
    pub fn nd18(&mut self) -> ND18_W {
        ND18_W { w: self }
    }
    #[doc = "Bit 19 - New Data 19"]
    #[inline(always)]
    pub fn nd19(&mut self) -> ND19_W {
        ND19_W { w: self }
    }
    #[doc = "Bit 20 - New Data 20"]
    #[inline(always)]
    pub fn nd20(&mut self) -> ND20_W {
        ND20_W { w: self }
    }
    #[doc = "Bit 21 - New Data 21"]
    #[inline(always)]
    pub fn nd21(&mut self) -> ND21_W {
        ND21_W { w: self }
    }
    #[doc = "Bit 22 - New Data 22"]
    #[inline(always)]
    pub fn nd22(&mut self) -> ND22_W {
        ND22_W { w: self }
    }
    #[doc = "Bit 23 - New Data 23"]
    #[inline(always)]
    pub fn nd23(&mut self) -> ND23_W {
        ND23_W { w: self }
    }
    #[doc = "Bit 24 - New Data 24"]
    #[inline(always)]
    pub fn nd24(&mut self) -> ND24_W {
        ND24_W { w: self }
    }
    #[doc = "Bit 25 - New Data 25"]
    #[inline(always)]
    pub fn nd25(&mut self) -> ND25_W {
        ND25_W { w: self }
    }
    #[doc = "Bit 26 - New Data 26"]
    #[inline(always)]
    pub fn nd26(&mut self) -> ND26_W {
        ND26_W { w: self }
    }
    #[doc = "Bit 27 - New Data 27"]
    #[inline(always)]
    pub fn nd27(&mut self) -> ND27_W {
        ND27_W { w: self }
    }
    #[doc = "Bit 28 - New Data 28"]
    #[inline(always)]
    pub fn nd28(&mut self) -> ND28_W {
        ND28_W { w: self }
    }
    #[doc = "Bit 29 - New Data 29"]
    #[inline(always)]
    pub fn nd29(&mut self) -> ND29_W {
        ND29_W { w: self }
    }
    #[doc = "Bit 30 - New Data 30"]
    #[inline(always)]
    pub fn nd30(&mut self) -> ND30_W {
        ND30_W { w: self }
    }
    #[doc = "Bit 31 - New Data 31"]
    #[inline(always)]
    pub fn nd31(&mut self) -> ND31_W {
        ND31_W { w: self }
    }
}
