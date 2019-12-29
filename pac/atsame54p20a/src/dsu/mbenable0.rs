#[doc = "Reader of register MBENABLE0"]
pub type R = crate::R<u32, super::MBENABLE0>;
#[doc = "Writer for register MBENABLE0"]
pub type W = crate::W<u32, super::MBENABLE0>;
#[doc = "Register MBENABLE0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MBENABLE0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENABLE0`"]
pub type ENABLE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE0`"]
pub struct ENABLE0_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE0_W<'a> {
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
#[doc = "Reader of field `ENABLE1`"]
pub type ENABLE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE1`"]
pub struct ENABLE1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE1_W<'a> {
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
#[doc = "Reader of field `ENABLE2`"]
pub type ENABLE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE2`"]
pub struct ENABLE2_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE2_W<'a> {
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
#[doc = "Reader of field `ENABLE3`"]
pub type ENABLE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE3`"]
pub struct ENABLE3_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE3_W<'a> {
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
#[doc = "Reader of field `ENABLE4`"]
pub type ENABLE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE4`"]
pub struct ENABLE4_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE4_W<'a> {
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
#[doc = "Reader of field `ENABLE5`"]
pub type ENABLE5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE5`"]
pub struct ENABLE5_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE5_W<'a> {
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
#[doc = "Reader of field `ENABLE6`"]
pub type ENABLE6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE6`"]
pub struct ENABLE6_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE6_W<'a> {
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
#[doc = "Reader of field `ENABLE7`"]
pub type ENABLE7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE7`"]
pub struct ENABLE7_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE7_W<'a> {
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
#[doc = "Reader of field `ENABLE8`"]
pub type ENABLE8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE8`"]
pub struct ENABLE8_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE8_W<'a> {
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
#[doc = "Reader of field `ENABLE9`"]
pub type ENABLE9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE9`"]
pub struct ENABLE9_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE9_W<'a> {
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
#[doc = "Reader of field `ENABLE10`"]
pub type ENABLE10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE10`"]
pub struct ENABLE10_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE10_W<'a> {
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
#[doc = "Reader of field `ENABLE11`"]
pub type ENABLE11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE11`"]
pub struct ENABLE11_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE11_W<'a> {
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
#[doc = "Reader of field `ENABLE12`"]
pub type ENABLE12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE12`"]
pub struct ENABLE12_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE12_W<'a> {
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
#[doc = "Reader of field `ENABLE13`"]
pub type ENABLE13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE13`"]
pub struct ENABLE13_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE13_W<'a> {
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
#[doc = "Reader of field `ENABLE14`"]
pub type ENABLE14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE14`"]
pub struct ENABLE14_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE14_W<'a> {
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
#[doc = "Reader of field `ENABLE15`"]
pub type ENABLE15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE15`"]
pub struct ENABLE15_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE15_W<'a> {
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
#[doc = "Reader of field `ENABLE16`"]
pub type ENABLE16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE16`"]
pub struct ENABLE16_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE16_W<'a> {
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
#[doc = "Reader of field `ENABLE17`"]
pub type ENABLE17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE17`"]
pub struct ENABLE17_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE17_W<'a> {
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
#[doc = "Reader of field `ENABLE18`"]
pub type ENABLE18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE18`"]
pub struct ENABLE18_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE18_W<'a> {
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
#[doc = "Reader of field `ENABLE19`"]
pub type ENABLE19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE19`"]
pub struct ENABLE19_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE19_W<'a> {
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
#[doc = "Reader of field `ENABLE20`"]
pub type ENABLE20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE20`"]
pub struct ENABLE20_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE20_W<'a> {
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
#[doc = "Reader of field `ENABLE21`"]
pub type ENABLE21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE21`"]
pub struct ENABLE21_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE21_W<'a> {
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
#[doc = "Reader of field `ENABLE22`"]
pub type ENABLE22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE22`"]
pub struct ENABLE22_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE22_W<'a> {
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
#[doc = "Reader of field `ENABLE23`"]
pub type ENABLE23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE23`"]
pub struct ENABLE23_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE23_W<'a> {
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
#[doc = "Reader of field `ENABLE24`"]
pub type ENABLE24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE24`"]
pub struct ENABLE24_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE24_W<'a> {
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
#[doc = "Reader of field `ENABLE25`"]
pub type ENABLE25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE25`"]
pub struct ENABLE25_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE25_W<'a> {
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
#[doc = "Reader of field `ENABLE26`"]
pub type ENABLE26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE26`"]
pub struct ENABLE26_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE26_W<'a> {
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
#[doc = "Reader of field `ENABLE27`"]
pub type ENABLE27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE27`"]
pub struct ENABLE27_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE27_W<'a> {
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
#[doc = "Reader of field `ENABLE28`"]
pub type ENABLE28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE28`"]
pub struct ENABLE28_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE28_W<'a> {
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
    #[doc = "Bit 0 - Memory 0 MBIST Enable"]
    #[inline(always)]
    pub fn enable0(&self) -> ENABLE0_R {
        ENABLE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Memory 1 MBIST Enable"]
    #[inline(always)]
    pub fn enable1(&self) -> ENABLE1_R {
        ENABLE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Memory 2 MBIST Enable"]
    #[inline(always)]
    pub fn enable2(&self) -> ENABLE2_R {
        ENABLE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Memory 3 MBIST Enable"]
    #[inline(always)]
    pub fn enable3(&self) -> ENABLE3_R {
        ENABLE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Memory 4 MBIST Enable"]
    #[inline(always)]
    pub fn enable4(&self) -> ENABLE4_R {
        ENABLE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Memory 5 MBIST Enable"]
    #[inline(always)]
    pub fn enable5(&self) -> ENABLE5_R {
        ENABLE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Memory 6 MBIST Enable"]
    #[inline(always)]
    pub fn enable6(&self) -> ENABLE6_R {
        ENABLE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Memory 7 MBIST Enable"]
    #[inline(always)]
    pub fn enable7(&self) -> ENABLE7_R {
        ENABLE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Memory 8 MBIST Enable"]
    #[inline(always)]
    pub fn enable8(&self) -> ENABLE8_R {
        ENABLE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Memory 9 MBIST Enable"]
    #[inline(always)]
    pub fn enable9(&self) -> ENABLE9_R {
        ENABLE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Memory 10 MBIST Enable"]
    #[inline(always)]
    pub fn enable10(&self) -> ENABLE10_R {
        ENABLE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Memory 11 MBIST Enable"]
    #[inline(always)]
    pub fn enable11(&self) -> ENABLE11_R {
        ENABLE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Memory 12 MBIST Enable"]
    #[inline(always)]
    pub fn enable12(&self) -> ENABLE12_R {
        ENABLE12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Memory 13 MBIST Enable"]
    #[inline(always)]
    pub fn enable13(&self) -> ENABLE13_R {
        ENABLE13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Memory 14 MBIST Enable"]
    #[inline(always)]
    pub fn enable14(&self) -> ENABLE14_R {
        ENABLE14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Memory 15 MBIST Enable"]
    #[inline(always)]
    pub fn enable15(&self) -> ENABLE15_R {
        ENABLE15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Memory 16 MBIST Enable"]
    #[inline(always)]
    pub fn enable16(&self) -> ENABLE16_R {
        ENABLE16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Memory 17 MBIST Enable"]
    #[inline(always)]
    pub fn enable17(&self) -> ENABLE17_R {
        ENABLE17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Memory 18 MBIST Enable"]
    #[inline(always)]
    pub fn enable18(&self) -> ENABLE18_R {
        ENABLE18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Memory 19 MBIST Enable"]
    #[inline(always)]
    pub fn enable19(&self) -> ENABLE19_R {
        ENABLE19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Memory 20 MBIST Enable"]
    #[inline(always)]
    pub fn enable20(&self) -> ENABLE20_R {
        ENABLE20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Memory 21 MBIST Enable"]
    #[inline(always)]
    pub fn enable21(&self) -> ENABLE21_R {
        ENABLE21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Memory 22 MBIST Enable"]
    #[inline(always)]
    pub fn enable22(&self) -> ENABLE22_R {
        ENABLE22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Memory 23 MBIST Enable"]
    #[inline(always)]
    pub fn enable23(&self) -> ENABLE23_R {
        ENABLE23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Memory 24 MBIST Enable"]
    #[inline(always)]
    pub fn enable24(&self) -> ENABLE24_R {
        ENABLE24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Memory 25 MBIST Enable"]
    #[inline(always)]
    pub fn enable25(&self) -> ENABLE25_R {
        ENABLE25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Memory 26 MBIST Enable"]
    #[inline(always)]
    pub fn enable26(&self) -> ENABLE26_R {
        ENABLE26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Memory 27 MBIST Enable"]
    #[inline(always)]
    pub fn enable27(&self) -> ENABLE27_R {
        ENABLE27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Memory 28 MBIST Enable"]
    #[inline(always)]
    pub fn enable28(&self) -> ENABLE28_R {
        ENABLE28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Memory 0 MBIST Enable"]
    #[inline(always)]
    pub fn enable0(&mut self) -> ENABLE0_W {
        ENABLE0_W { w: self }
    }
    #[doc = "Bit 1 - Memory 1 MBIST Enable"]
    #[inline(always)]
    pub fn enable1(&mut self) -> ENABLE1_W {
        ENABLE1_W { w: self }
    }
    #[doc = "Bit 2 - Memory 2 MBIST Enable"]
    #[inline(always)]
    pub fn enable2(&mut self) -> ENABLE2_W {
        ENABLE2_W { w: self }
    }
    #[doc = "Bit 3 - Memory 3 MBIST Enable"]
    #[inline(always)]
    pub fn enable3(&mut self) -> ENABLE3_W {
        ENABLE3_W { w: self }
    }
    #[doc = "Bit 4 - Memory 4 MBIST Enable"]
    #[inline(always)]
    pub fn enable4(&mut self) -> ENABLE4_W {
        ENABLE4_W { w: self }
    }
    #[doc = "Bit 5 - Memory 5 MBIST Enable"]
    #[inline(always)]
    pub fn enable5(&mut self) -> ENABLE5_W {
        ENABLE5_W { w: self }
    }
    #[doc = "Bit 6 - Memory 6 MBIST Enable"]
    #[inline(always)]
    pub fn enable6(&mut self) -> ENABLE6_W {
        ENABLE6_W { w: self }
    }
    #[doc = "Bit 7 - Memory 7 MBIST Enable"]
    #[inline(always)]
    pub fn enable7(&mut self) -> ENABLE7_W {
        ENABLE7_W { w: self }
    }
    #[doc = "Bit 8 - Memory 8 MBIST Enable"]
    #[inline(always)]
    pub fn enable8(&mut self) -> ENABLE8_W {
        ENABLE8_W { w: self }
    }
    #[doc = "Bit 9 - Memory 9 MBIST Enable"]
    #[inline(always)]
    pub fn enable9(&mut self) -> ENABLE9_W {
        ENABLE9_W { w: self }
    }
    #[doc = "Bit 10 - Memory 10 MBIST Enable"]
    #[inline(always)]
    pub fn enable10(&mut self) -> ENABLE10_W {
        ENABLE10_W { w: self }
    }
    #[doc = "Bit 11 - Memory 11 MBIST Enable"]
    #[inline(always)]
    pub fn enable11(&mut self) -> ENABLE11_W {
        ENABLE11_W { w: self }
    }
    #[doc = "Bit 12 - Memory 12 MBIST Enable"]
    #[inline(always)]
    pub fn enable12(&mut self) -> ENABLE12_W {
        ENABLE12_W { w: self }
    }
    #[doc = "Bit 13 - Memory 13 MBIST Enable"]
    #[inline(always)]
    pub fn enable13(&mut self) -> ENABLE13_W {
        ENABLE13_W { w: self }
    }
    #[doc = "Bit 14 - Memory 14 MBIST Enable"]
    #[inline(always)]
    pub fn enable14(&mut self) -> ENABLE14_W {
        ENABLE14_W { w: self }
    }
    #[doc = "Bit 15 - Memory 15 MBIST Enable"]
    #[inline(always)]
    pub fn enable15(&mut self) -> ENABLE15_W {
        ENABLE15_W { w: self }
    }
    #[doc = "Bit 16 - Memory 16 MBIST Enable"]
    #[inline(always)]
    pub fn enable16(&mut self) -> ENABLE16_W {
        ENABLE16_W { w: self }
    }
    #[doc = "Bit 17 - Memory 17 MBIST Enable"]
    #[inline(always)]
    pub fn enable17(&mut self) -> ENABLE17_W {
        ENABLE17_W { w: self }
    }
    #[doc = "Bit 18 - Memory 18 MBIST Enable"]
    #[inline(always)]
    pub fn enable18(&mut self) -> ENABLE18_W {
        ENABLE18_W { w: self }
    }
    #[doc = "Bit 19 - Memory 19 MBIST Enable"]
    #[inline(always)]
    pub fn enable19(&mut self) -> ENABLE19_W {
        ENABLE19_W { w: self }
    }
    #[doc = "Bit 20 - Memory 20 MBIST Enable"]
    #[inline(always)]
    pub fn enable20(&mut self) -> ENABLE20_W {
        ENABLE20_W { w: self }
    }
    #[doc = "Bit 21 - Memory 21 MBIST Enable"]
    #[inline(always)]
    pub fn enable21(&mut self) -> ENABLE21_W {
        ENABLE21_W { w: self }
    }
    #[doc = "Bit 22 - Memory 22 MBIST Enable"]
    #[inline(always)]
    pub fn enable22(&mut self) -> ENABLE22_W {
        ENABLE22_W { w: self }
    }
    #[doc = "Bit 23 - Memory 23 MBIST Enable"]
    #[inline(always)]
    pub fn enable23(&mut self) -> ENABLE23_W {
        ENABLE23_W { w: self }
    }
    #[doc = "Bit 24 - Memory 24 MBIST Enable"]
    #[inline(always)]
    pub fn enable24(&mut self) -> ENABLE24_W {
        ENABLE24_W { w: self }
    }
    #[doc = "Bit 25 - Memory 25 MBIST Enable"]
    #[inline(always)]
    pub fn enable25(&mut self) -> ENABLE25_W {
        ENABLE25_W { w: self }
    }
    #[doc = "Bit 26 - Memory 26 MBIST Enable"]
    #[inline(always)]
    pub fn enable26(&mut self) -> ENABLE26_W {
        ENABLE26_W { w: self }
    }
    #[doc = "Bit 27 - Memory 27 MBIST Enable"]
    #[inline(always)]
    pub fn enable27(&mut self) -> ENABLE27_W {
        ENABLE27_W { w: self }
    }
    #[doc = "Bit 28 - Memory 28 MBIST Enable"]
    #[inline(always)]
    pub fn enable28(&mut self) -> ENABLE28_W {
        ENABLE28_W { w: self }
    }
}
