#[doc = "Reader of register MBSTATUS0"]
pub type R = crate::R<u32, super::MBSTATUS0>;
#[doc = "Writer for register MBSTATUS0"]
pub type W = crate::W<u32, super::MBSTATUS0>;
#[doc = "Register MBSTATUS0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MBSTATUS0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STATUS0`"]
pub type STATUS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS0`"]
pub struct STATUS0_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS0_W<'a> {
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
#[doc = "Reader of field `STATUS1`"]
pub type STATUS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS1`"]
pub struct STATUS1_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS1_W<'a> {
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
#[doc = "Reader of field `STATUS2`"]
pub type STATUS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS2`"]
pub struct STATUS2_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS2_W<'a> {
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
#[doc = "Reader of field `STATUS3`"]
pub type STATUS3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS3`"]
pub struct STATUS3_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS3_W<'a> {
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
#[doc = "Reader of field `STATUS4`"]
pub type STATUS4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS4`"]
pub struct STATUS4_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS4_W<'a> {
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
#[doc = "Reader of field `STATUS5`"]
pub type STATUS5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS5`"]
pub struct STATUS5_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS5_W<'a> {
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
#[doc = "Reader of field `STATUS6`"]
pub type STATUS6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS6`"]
pub struct STATUS6_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS6_W<'a> {
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
#[doc = "Reader of field `STATUS7`"]
pub type STATUS7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS7`"]
pub struct STATUS7_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS7_W<'a> {
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
#[doc = "Reader of field `STATUS8`"]
pub type STATUS8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS8`"]
pub struct STATUS8_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS8_W<'a> {
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
#[doc = "Reader of field `STATUS9`"]
pub type STATUS9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS9`"]
pub struct STATUS9_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS9_W<'a> {
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
#[doc = "Reader of field `STATUS10`"]
pub type STATUS10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS10`"]
pub struct STATUS10_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS10_W<'a> {
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
#[doc = "Reader of field `STATUS11`"]
pub type STATUS11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS11`"]
pub struct STATUS11_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS11_W<'a> {
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
#[doc = "Reader of field `STATUS12`"]
pub type STATUS12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS12`"]
pub struct STATUS12_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS12_W<'a> {
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
#[doc = "Reader of field `STATUS13`"]
pub type STATUS13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS13`"]
pub struct STATUS13_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS13_W<'a> {
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
#[doc = "Reader of field `STATUS14`"]
pub type STATUS14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS14`"]
pub struct STATUS14_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS14_W<'a> {
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
#[doc = "Reader of field `STATUS15`"]
pub type STATUS15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS15`"]
pub struct STATUS15_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS15_W<'a> {
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
#[doc = "Reader of field `STATUS16`"]
pub type STATUS16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS16`"]
pub struct STATUS16_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS16_W<'a> {
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
#[doc = "Reader of field `STATUS17`"]
pub type STATUS17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS17`"]
pub struct STATUS17_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS17_W<'a> {
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
#[doc = "Reader of field `STATUS18`"]
pub type STATUS18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS18`"]
pub struct STATUS18_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS18_W<'a> {
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
#[doc = "Reader of field `STATUS19`"]
pub type STATUS19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS19`"]
pub struct STATUS19_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS19_W<'a> {
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
#[doc = "Reader of field `STATUS20`"]
pub type STATUS20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS20`"]
pub struct STATUS20_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS20_W<'a> {
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
#[doc = "Reader of field `STATUS21`"]
pub type STATUS21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS21`"]
pub struct STATUS21_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS21_W<'a> {
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
#[doc = "Reader of field `STATUS22`"]
pub type STATUS22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS22`"]
pub struct STATUS22_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS22_W<'a> {
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
#[doc = "Reader of field `STATUS23`"]
pub type STATUS23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS23`"]
pub struct STATUS23_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS23_W<'a> {
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
#[doc = "Reader of field `STATUS24`"]
pub type STATUS24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS24`"]
pub struct STATUS24_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS24_W<'a> {
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
#[doc = "Reader of field `STATUS25`"]
pub type STATUS25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS25`"]
pub struct STATUS25_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS25_W<'a> {
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
#[doc = "Reader of field `STATUS26`"]
pub type STATUS26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS26`"]
pub struct STATUS26_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS26_W<'a> {
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
#[doc = "Reader of field `STATUS27`"]
pub type STATUS27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS27`"]
pub struct STATUS27_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS27_W<'a> {
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
#[doc = "Reader of field `STATUS28`"]
pub type STATUS28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS28`"]
pub struct STATUS28_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS28_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Memory 0 MBIST Status"]
    #[inline(always)]
    pub fn status0(&self) -> STATUS0_R {
        STATUS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Memory 1 MBIST Status"]
    #[inline(always)]
    pub fn status1(&self) -> STATUS1_R {
        STATUS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Memory 2 MBIST Status"]
    #[inline(always)]
    pub fn status2(&self) -> STATUS2_R {
        STATUS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Memory 3 MBIST Status"]
    #[inline(always)]
    pub fn status3(&self) -> STATUS3_R {
        STATUS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Memory 4 MBIST Status"]
    #[inline(always)]
    pub fn status4(&self) -> STATUS4_R {
        STATUS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Memory 5 MBIST Status"]
    #[inline(always)]
    pub fn status5(&self) -> STATUS5_R {
        STATUS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Memory 6 MBIST Status"]
    #[inline(always)]
    pub fn status6(&self) -> STATUS6_R {
        STATUS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Memory 7 MBIST Status"]
    #[inline(always)]
    pub fn status7(&self) -> STATUS7_R {
        STATUS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Memory 8 MBIST Status"]
    #[inline(always)]
    pub fn status8(&self) -> STATUS8_R {
        STATUS8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Memory 9 MBIST Status"]
    #[inline(always)]
    pub fn status9(&self) -> STATUS9_R {
        STATUS9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Memory 10 MBIST Status"]
    #[inline(always)]
    pub fn status10(&self) -> STATUS10_R {
        STATUS10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Memory 11 MBIST Status"]
    #[inline(always)]
    pub fn status11(&self) -> STATUS11_R {
        STATUS11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Memory 12 MBIST Status"]
    #[inline(always)]
    pub fn status12(&self) -> STATUS12_R {
        STATUS12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Memory 13 MBIST Status"]
    #[inline(always)]
    pub fn status13(&self) -> STATUS13_R {
        STATUS13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Memory 14 MBIST Status"]
    #[inline(always)]
    pub fn status14(&self) -> STATUS14_R {
        STATUS14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Memory 15 MBIST Status"]
    #[inline(always)]
    pub fn status15(&self) -> STATUS15_R {
        STATUS15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Memory 16 MBIST Status"]
    #[inline(always)]
    pub fn status16(&self) -> STATUS16_R {
        STATUS16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Memory 17 MBIST Status"]
    #[inline(always)]
    pub fn status17(&self) -> STATUS17_R {
        STATUS17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Memory 18 MBIST Status"]
    #[inline(always)]
    pub fn status18(&self) -> STATUS18_R {
        STATUS18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Memory 19 MBIST Status"]
    #[inline(always)]
    pub fn status19(&self) -> STATUS19_R {
        STATUS19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Memory 20 MBIST Status"]
    #[inline(always)]
    pub fn status20(&self) -> STATUS20_R {
        STATUS20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Memory 21 MBIST Status"]
    #[inline(always)]
    pub fn status21(&self) -> STATUS21_R {
        STATUS21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Memory 22 MBIST Status"]
    #[inline(always)]
    pub fn status22(&self) -> STATUS22_R {
        STATUS22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Memory 23 MBIST Status"]
    #[inline(always)]
    pub fn status23(&self) -> STATUS23_R {
        STATUS23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Memory 24 MBIST Status"]
    #[inline(always)]
    pub fn status24(&self) -> STATUS24_R {
        STATUS24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Memory 25 MBIST Status"]
    #[inline(always)]
    pub fn status25(&self) -> STATUS25_R {
        STATUS25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Memory 26 MBIST Status"]
    #[inline(always)]
    pub fn status26(&self) -> STATUS26_R {
        STATUS26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Memory 27 MBIST Status"]
    #[inline(always)]
    pub fn status27(&self) -> STATUS27_R {
        STATUS27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Memory 28 MBIST Status"]
    #[inline(always)]
    pub fn status28(&self) -> STATUS28_R {
        STATUS28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Memory 0 MBIST Status"]
    #[inline(always)]
    pub fn status0(&mut self) -> STATUS0_W {
        STATUS0_W { w: self }
    }
    #[doc = "Bit 1 - Memory 1 MBIST Status"]
    #[inline(always)]
    pub fn status1(&mut self) -> STATUS1_W {
        STATUS1_W { w: self }
    }
    #[doc = "Bit 2 - Memory 2 MBIST Status"]
    #[inline(always)]
    pub fn status2(&mut self) -> STATUS2_W {
        STATUS2_W { w: self }
    }
    #[doc = "Bit 3 - Memory 3 MBIST Status"]
    #[inline(always)]
    pub fn status3(&mut self) -> STATUS3_W {
        STATUS3_W { w: self }
    }
    #[doc = "Bit 4 - Memory 4 MBIST Status"]
    #[inline(always)]
    pub fn status4(&mut self) -> STATUS4_W {
        STATUS4_W { w: self }
    }
    #[doc = "Bit 5 - Memory 5 MBIST Status"]
    #[inline(always)]
    pub fn status5(&mut self) -> STATUS5_W {
        STATUS5_W { w: self }
    }
    #[doc = "Bit 6 - Memory 6 MBIST Status"]
    #[inline(always)]
    pub fn status6(&mut self) -> STATUS6_W {
        STATUS6_W { w: self }
    }
    #[doc = "Bit 7 - Memory 7 MBIST Status"]
    #[inline(always)]
    pub fn status7(&mut self) -> STATUS7_W {
        STATUS7_W { w: self }
    }
    #[doc = "Bit 8 - Memory 8 MBIST Status"]
    #[inline(always)]
    pub fn status8(&mut self) -> STATUS8_W {
        STATUS8_W { w: self }
    }
    #[doc = "Bit 9 - Memory 9 MBIST Status"]
    #[inline(always)]
    pub fn status9(&mut self) -> STATUS9_W {
        STATUS9_W { w: self }
    }
    #[doc = "Bit 10 - Memory 10 MBIST Status"]
    #[inline(always)]
    pub fn status10(&mut self) -> STATUS10_W {
        STATUS10_W { w: self }
    }
    #[doc = "Bit 11 - Memory 11 MBIST Status"]
    #[inline(always)]
    pub fn status11(&mut self) -> STATUS11_W {
        STATUS11_W { w: self }
    }
    #[doc = "Bit 12 - Memory 12 MBIST Status"]
    #[inline(always)]
    pub fn status12(&mut self) -> STATUS12_W {
        STATUS12_W { w: self }
    }
    #[doc = "Bit 13 - Memory 13 MBIST Status"]
    #[inline(always)]
    pub fn status13(&mut self) -> STATUS13_W {
        STATUS13_W { w: self }
    }
    #[doc = "Bit 14 - Memory 14 MBIST Status"]
    #[inline(always)]
    pub fn status14(&mut self) -> STATUS14_W {
        STATUS14_W { w: self }
    }
    #[doc = "Bit 15 - Memory 15 MBIST Status"]
    #[inline(always)]
    pub fn status15(&mut self) -> STATUS15_W {
        STATUS15_W { w: self }
    }
    #[doc = "Bit 16 - Memory 16 MBIST Status"]
    #[inline(always)]
    pub fn status16(&mut self) -> STATUS16_W {
        STATUS16_W { w: self }
    }
    #[doc = "Bit 17 - Memory 17 MBIST Status"]
    #[inline(always)]
    pub fn status17(&mut self) -> STATUS17_W {
        STATUS17_W { w: self }
    }
    #[doc = "Bit 18 - Memory 18 MBIST Status"]
    #[inline(always)]
    pub fn status18(&mut self) -> STATUS18_W {
        STATUS18_W { w: self }
    }
    #[doc = "Bit 19 - Memory 19 MBIST Status"]
    #[inline(always)]
    pub fn status19(&mut self) -> STATUS19_W {
        STATUS19_W { w: self }
    }
    #[doc = "Bit 20 - Memory 20 MBIST Status"]
    #[inline(always)]
    pub fn status20(&mut self) -> STATUS20_W {
        STATUS20_W { w: self }
    }
    #[doc = "Bit 21 - Memory 21 MBIST Status"]
    #[inline(always)]
    pub fn status21(&mut self) -> STATUS21_W {
        STATUS21_W { w: self }
    }
    #[doc = "Bit 22 - Memory 22 MBIST Status"]
    #[inline(always)]
    pub fn status22(&mut self) -> STATUS22_W {
        STATUS22_W { w: self }
    }
    #[doc = "Bit 23 - Memory 23 MBIST Status"]
    #[inline(always)]
    pub fn status23(&mut self) -> STATUS23_W {
        STATUS23_W { w: self }
    }
    #[doc = "Bit 24 - Memory 24 MBIST Status"]
    #[inline(always)]
    pub fn status24(&mut self) -> STATUS24_W {
        STATUS24_W { w: self }
    }
    #[doc = "Bit 25 - Memory 25 MBIST Status"]
    #[inline(always)]
    pub fn status25(&mut self) -> STATUS25_W {
        STATUS25_W { w: self }
    }
    #[doc = "Bit 26 - Memory 26 MBIST Status"]
    #[inline(always)]
    pub fn status26(&mut self) -> STATUS26_W {
        STATUS26_W { w: self }
    }
    #[doc = "Bit 27 - Memory 27 MBIST Status"]
    #[inline(always)]
    pub fn status27(&mut self) -> STATUS27_W {
        STATUS27_W { w: self }
    }
    #[doc = "Bit 28 - Memory 28 MBIST Status"]
    #[inline(always)]
    pub fn status28(&mut self) -> STATUS28_W {
        STATUS28_W { w: self }
    }
}
