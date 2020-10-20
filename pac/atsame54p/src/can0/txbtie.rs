#[doc = "Reader of register TXBTIE"]
pub type R = crate::R<u32, super::TXBTIE>;
#[doc = "Writer for register TXBTIE"]
pub type W = crate::W<u32, super::TXBTIE>;
#[doc = "Register TXBTIE `reset()`'s with value 0"]
impl crate::ResetValue for super::TXBTIE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIE0`"]
pub type TIE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE0`"]
pub struct TIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE0_W<'a> {
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
#[doc = "Reader of field `TIE1`"]
pub type TIE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE1`"]
pub struct TIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE1_W<'a> {
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
#[doc = "Reader of field `TIE2`"]
pub type TIE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE2`"]
pub struct TIE2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE2_W<'a> {
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
#[doc = "Reader of field `TIE3`"]
pub type TIE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE3`"]
pub struct TIE3_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE3_W<'a> {
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
#[doc = "Reader of field `TIE4`"]
pub type TIE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE4`"]
pub struct TIE4_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE4_W<'a> {
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
#[doc = "Reader of field `TIE5`"]
pub type TIE5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE5`"]
pub struct TIE5_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE5_W<'a> {
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
#[doc = "Reader of field `TIE6`"]
pub type TIE6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE6`"]
pub struct TIE6_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE6_W<'a> {
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
#[doc = "Reader of field `TIE7`"]
pub type TIE7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE7`"]
pub struct TIE7_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE7_W<'a> {
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
#[doc = "Reader of field `TIE8`"]
pub type TIE8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE8`"]
pub struct TIE8_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE8_W<'a> {
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
#[doc = "Reader of field `TIE9`"]
pub type TIE9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE9`"]
pub struct TIE9_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE9_W<'a> {
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
#[doc = "Reader of field `TIE10`"]
pub type TIE10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE10`"]
pub struct TIE10_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE10_W<'a> {
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
#[doc = "Reader of field `TIE11`"]
pub type TIE11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE11`"]
pub struct TIE11_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE11_W<'a> {
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
#[doc = "Reader of field `TIE12`"]
pub type TIE12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE12`"]
pub struct TIE12_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE12_W<'a> {
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
#[doc = "Reader of field `TIE13`"]
pub type TIE13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE13`"]
pub struct TIE13_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE13_W<'a> {
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
#[doc = "Reader of field `TIE14`"]
pub type TIE14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE14`"]
pub struct TIE14_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE14_W<'a> {
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
#[doc = "Reader of field `TIE15`"]
pub type TIE15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE15`"]
pub struct TIE15_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE15_W<'a> {
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
#[doc = "Reader of field `TIE16`"]
pub type TIE16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE16`"]
pub struct TIE16_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE16_W<'a> {
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
#[doc = "Reader of field `TIE17`"]
pub type TIE17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE17`"]
pub struct TIE17_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE17_W<'a> {
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
#[doc = "Reader of field `TIE18`"]
pub type TIE18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE18`"]
pub struct TIE18_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE18_W<'a> {
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
#[doc = "Reader of field `TIE19`"]
pub type TIE19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE19`"]
pub struct TIE19_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE19_W<'a> {
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
#[doc = "Reader of field `TIE20`"]
pub type TIE20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE20`"]
pub struct TIE20_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE20_W<'a> {
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
#[doc = "Reader of field `TIE21`"]
pub type TIE21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE21`"]
pub struct TIE21_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE21_W<'a> {
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
#[doc = "Reader of field `TIE22`"]
pub type TIE22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE22`"]
pub struct TIE22_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE22_W<'a> {
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
#[doc = "Reader of field `TIE23`"]
pub type TIE23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE23`"]
pub struct TIE23_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE23_W<'a> {
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
#[doc = "Reader of field `TIE24`"]
pub type TIE24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE24`"]
pub struct TIE24_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE24_W<'a> {
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
#[doc = "Reader of field `TIE25`"]
pub type TIE25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE25`"]
pub struct TIE25_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE25_W<'a> {
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
#[doc = "Reader of field `TIE26`"]
pub type TIE26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE26`"]
pub struct TIE26_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE26_W<'a> {
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
#[doc = "Reader of field `TIE27`"]
pub type TIE27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE27`"]
pub struct TIE27_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE27_W<'a> {
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
#[doc = "Reader of field `TIE28`"]
pub type TIE28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE28`"]
pub struct TIE28_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE28_W<'a> {
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
#[doc = "Reader of field `TIE29`"]
pub type TIE29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE29`"]
pub struct TIE29_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE29_W<'a> {
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
#[doc = "Reader of field `TIE30`"]
pub type TIE30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE30`"]
pub struct TIE30_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE30_W<'a> {
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
#[doc = "Reader of field `TIE31`"]
pub type TIE31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE31`"]
pub struct TIE31_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE31_W<'a> {
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
    #[doc = "Bit 0 - Transmission Interrupt Enable 0"]
    #[inline(always)]
    pub fn tie0(&self) -> TIE0_R {
        TIE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmission Interrupt Enable 1"]
    #[inline(always)]
    pub fn tie1(&self) -> TIE1_R {
        TIE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmission Interrupt Enable 2"]
    #[inline(always)]
    pub fn tie2(&self) -> TIE2_R {
        TIE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmission Interrupt Enable 3"]
    #[inline(always)]
    pub fn tie3(&self) -> TIE3_R {
        TIE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmission Interrupt Enable 4"]
    #[inline(always)]
    pub fn tie4(&self) -> TIE4_R {
        TIE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmission Interrupt Enable 5"]
    #[inline(always)]
    pub fn tie5(&self) -> TIE5_R {
        TIE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmission Interrupt Enable 6"]
    #[inline(always)]
    pub fn tie6(&self) -> TIE6_R {
        TIE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmission Interrupt Enable 7"]
    #[inline(always)]
    pub fn tie7(&self) -> TIE7_R {
        TIE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmission Interrupt Enable 8"]
    #[inline(always)]
    pub fn tie8(&self) -> TIE8_R {
        TIE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmission Interrupt Enable 9"]
    #[inline(always)]
    pub fn tie9(&self) -> TIE9_R {
        TIE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmission Interrupt Enable 10"]
    #[inline(always)]
    pub fn tie10(&self) -> TIE10_R {
        TIE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transmission Interrupt Enable 11"]
    #[inline(always)]
    pub fn tie11(&self) -> TIE11_R {
        TIE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Transmission Interrupt Enable 12"]
    #[inline(always)]
    pub fn tie12(&self) -> TIE12_R {
        TIE12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Transmission Interrupt Enable 13"]
    #[inline(always)]
    pub fn tie13(&self) -> TIE13_R {
        TIE13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Transmission Interrupt Enable 14"]
    #[inline(always)]
    pub fn tie14(&self) -> TIE14_R {
        TIE14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Transmission Interrupt Enable 15"]
    #[inline(always)]
    pub fn tie15(&self) -> TIE15_R {
        TIE15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Transmission Interrupt Enable 16"]
    #[inline(always)]
    pub fn tie16(&self) -> TIE16_R {
        TIE16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Transmission Interrupt Enable 17"]
    #[inline(always)]
    pub fn tie17(&self) -> TIE17_R {
        TIE17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Transmission Interrupt Enable 18"]
    #[inline(always)]
    pub fn tie18(&self) -> TIE18_R {
        TIE18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Transmission Interrupt Enable 19"]
    #[inline(always)]
    pub fn tie19(&self) -> TIE19_R {
        TIE19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Transmission Interrupt Enable 20"]
    #[inline(always)]
    pub fn tie20(&self) -> TIE20_R {
        TIE20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Transmission Interrupt Enable 21"]
    #[inline(always)]
    pub fn tie21(&self) -> TIE21_R {
        TIE21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Transmission Interrupt Enable 22"]
    #[inline(always)]
    pub fn tie22(&self) -> TIE22_R {
        TIE22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Transmission Interrupt Enable 23"]
    #[inline(always)]
    pub fn tie23(&self) -> TIE23_R {
        TIE23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Transmission Interrupt Enable 24"]
    #[inline(always)]
    pub fn tie24(&self) -> TIE24_R {
        TIE24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Transmission Interrupt Enable 25"]
    #[inline(always)]
    pub fn tie25(&self) -> TIE25_R {
        TIE25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Transmission Interrupt Enable 26"]
    #[inline(always)]
    pub fn tie26(&self) -> TIE26_R {
        TIE26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Transmission Interrupt Enable 27"]
    #[inline(always)]
    pub fn tie27(&self) -> TIE27_R {
        TIE27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Transmission Interrupt Enable 28"]
    #[inline(always)]
    pub fn tie28(&self) -> TIE28_R {
        TIE28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Transmission Interrupt Enable 29"]
    #[inline(always)]
    pub fn tie29(&self) -> TIE29_R {
        TIE29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Transmission Interrupt Enable 30"]
    #[inline(always)]
    pub fn tie30(&self) -> TIE30_R {
        TIE30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Transmission Interrupt Enable 31"]
    #[inline(always)]
    pub fn tie31(&self) -> TIE31_R {
        TIE31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmission Interrupt Enable 0"]
    #[inline(always)]
    pub fn tie0(&mut self) -> TIE0_W {
        TIE0_W { w: self }
    }
    #[doc = "Bit 1 - Transmission Interrupt Enable 1"]
    #[inline(always)]
    pub fn tie1(&mut self) -> TIE1_W {
        TIE1_W { w: self }
    }
    #[doc = "Bit 2 - Transmission Interrupt Enable 2"]
    #[inline(always)]
    pub fn tie2(&mut self) -> TIE2_W {
        TIE2_W { w: self }
    }
    #[doc = "Bit 3 - Transmission Interrupt Enable 3"]
    #[inline(always)]
    pub fn tie3(&mut self) -> TIE3_W {
        TIE3_W { w: self }
    }
    #[doc = "Bit 4 - Transmission Interrupt Enable 4"]
    #[inline(always)]
    pub fn tie4(&mut self) -> TIE4_W {
        TIE4_W { w: self }
    }
    #[doc = "Bit 5 - Transmission Interrupt Enable 5"]
    #[inline(always)]
    pub fn tie5(&mut self) -> TIE5_W {
        TIE5_W { w: self }
    }
    #[doc = "Bit 6 - Transmission Interrupt Enable 6"]
    #[inline(always)]
    pub fn tie6(&mut self) -> TIE6_W {
        TIE6_W { w: self }
    }
    #[doc = "Bit 7 - Transmission Interrupt Enable 7"]
    #[inline(always)]
    pub fn tie7(&mut self) -> TIE7_W {
        TIE7_W { w: self }
    }
    #[doc = "Bit 8 - Transmission Interrupt Enable 8"]
    #[inline(always)]
    pub fn tie8(&mut self) -> TIE8_W {
        TIE8_W { w: self }
    }
    #[doc = "Bit 9 - Transmission Interrupt Enable 9"]
    #[inline(always)]
    pub fn tie9(&mut self) -> TIE9_W {
        TIE9_W { w: self }
    }
    #[doc = "Bit 10 - Transmission Interrupt Enable 10"]
    #[inline(always)]
    pub fn tie10(&mut self) -> TIE10_W {
        TIE10_W { w: self }
    }
    #[doc = "Bit 11 - Transmission Interrupt Enable 11"]
    #[inline(always)]
    pub fn tie11(&mut self) -> TIE11_W {
        TIE11_W { w: self }
    }
    #[doc = "Bit 12 - Transmission Interrupt Enable 12"]
    #[inline(always)]
    pub fn tie12(&mut self) -> TIE12_W {
        TIE12_W { w: self }
    }
    #[doc = "Bit 13 - Transmission Interrupt Enable 13"]
    #[inline(always)]
    pub fn tie13(&mut self) -> TIE13_W {
        TIE13_W { w: self }
    }
    #[doc = "Bit 14 - Transmission Interrupt Enable 14"]
    #[inline(always)]
    pub fn tie14(&mut self) -> TIE14_W {
        TIE14_W { w: self }
    }
    #[doc = "Bit 15 - Transmission Interrupt Enable 15"]
    #[inline(always)]
    pub fn tie15(&mut self) -> TIE15_W {
        TIE15_W { w: self }
    }
    #[doc = "Bit 16 - Transmission Interrupt Enable 16"]
    #[inline(always)]
    pub fn tie16(&mut self) -> TIE16_W {
        TIE16_W { w: self }
    }
    #[doc = "Bit 17 - Transmission Interrupt Enable 17"]
    #[inline(always)]
    pub fn tie17(&mut self) -> TIE17_W {
        TIE17_W { w: self }
    }
    #[doc = "Bit 18 - Transmission Interrupt Enable 18"]
    #[inline(always)]
    pub fn tie18(&mut self) -> TIE18_W {
        TIE18_W { w: self }
    }
    #[doc = "Bit 19 - Transmission Interrupt Enable 19"]
    #[inline(always)]
    pub fn tie19(&mut self) -> TIE19_W {
        TIE19_W { w: self }
    }
    #[doc = "Bit 20 - Transmission Interrupt Enable 20"]
    #[inline(always)]
    pub fn tie20(&mut self) -> TIE20_W {
        TIE20_W { w: self }
    }
    #[doc = "Bit 21 - Transmission Interrupt Enable 21"]
    #[inline(always)]
    pub fn tie21(&mut self) -> TIE21_W {
        TIE21_W { w: self }
    }
    #[doc = "Bit 22 - Transmission Interrupt Enable 22"]
    #[inline(always)]
    pub fn tie22(&mut self) -> TIE22_W {
        TIE22_W { w: self }
    }
    #[doc = "Bit 23 - Transmission Interrupt Enable 23"]
    #[inline(always)]
    pub fn tie23(&mut self) -> TIE23_W {
        TIE23_W { w: self }
    }
    #[doc = "Bit 24 - Transmission Interrupt Enable 24"]
    #[inline(always)]
    pub fn tie24(&mut self) -> TIE24_W {
        TIE24_W { w: self }
    }
    #[doc = "Bit 25 - Transmission Interrupt Enable 25"]
    #[inline(always)]
    pub fn tie25(&mut self) -> TIE25_W {
        TIE25_W { w: self }
    }
    #[doc = "Bit 26 - Transmission Interrupt Enable 26"]
    #[inline(always)]
    pub fn tie26(&mut self) -> TIE26_W {
        TIE26_W { w: self }
    }
    #[doc = "Bit 27 - Transmission Interrupt Enable 27"]
    #[inline(always)]
    pub fn tie27(&mut self) -> TIE27_W {
        TIE27_W { w: self }
    }
    #[doc = "Bit 28 - Transmission Interrupt Enable 28"]
    #[inline(always)]
    pub fn tie28(&mut self) -> TIE28_W {
        TIE28_W { w: self }
    }
    #[doc = "Bit 29 - Transmission Interrupt Enable 29"]
    #[inline(always)]
    pub fn tie29(&mut self) -> TIE29_W {
        TIE29_W { w: self }
    }
    #[doc = "Bit 30 - Transmission Interrupt Enable 30"]
    #[inline(always)]
    pub fn tie30(&mut self) -> TIE30_W {
        TIE30_W { w: self }
    }
    #[doc = "Bit 31 - Transmission Interrupt Enable 31"]
    #[inline(always)]
    pub fn tie31(&mut self) -> TIE31_W {
        TIE31_W { w: self }
    }
}
