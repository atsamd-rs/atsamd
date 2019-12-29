#[doc = "Reader of register SWTRIGCTRL"]
pub type R = crate::R<u32, super::SWTRIGCTRL>;
#[doc = "Writer for register SWTRIGCTRL"]
pub type W = crate::W<u32, super::SWTRIGCTRL>;
#[doc = "Register SWTRIGCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SWTRIGCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SWTRIG0`"]
pub type SWTRIG0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG0`"]
pub struct SWTRIG0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG0_W<'a> {
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
#[doc = "Reader of field `SWTRIG1`"]
pub type SWTRIG1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG1`"]
pub struct SWTRIG1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG1_W<'a> {
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
#[doc = "Reader of field `SWTRIG2`"]
pub type SWTRIG2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG2`"]
pub struct SWTRIG2_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG2_W<'a> {
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
#[doc = "Reader of field `SWTRIG3`"]
pub type SWTRIG3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG3`"]
pub struct SWTRIG3_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG3_W<'a> {
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
#[doc = "Reader of field `SWTRIG4`"]
pub type SWTRIG4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG4`"]
pub struct SWTRIG4_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG4_W<'a> {
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
#[doc = "Reader of field `SWTRIG5`"]
pub type SWTRIG5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG5`"]
pub struct SWTRIG5_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG5_W<'a> {
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
#[doc = "Reader of field `SWTRIG6`"]
pub type SWTRIG6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG6`"]
pub struct SWTRIG6_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG6_W<'a> {
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
#[doc = "Reader of field `SWTRIG7`"]
pub type SWTRIG7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG7`"]
pub struct SWTRIG7_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG7_W<'a> {
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
#[doc = "Reader of field `SWTRIG8`"]
pub type SWTRIG8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG8`"]
pub struct SWTRIG8_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG8_W<'a> {
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
#[doc = "Reader of field `SWTRIG9`"]
pub type SWTRIG9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG9`"]
pub struct SWTRIG9_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG9_W<'a> {
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
#[doc = "Reader of field `SWTRIG10`"]
pub type SWTRIG10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG10`"]
pub struct SWTRIG10_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG10_W<'a> {
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
#[doc = "Reader of field `SWTRIG11`"]
pub type SWTRIG11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG11`"]
pub struct SWTRIG11_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG11_W<'a> {
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
#[doc = "Reader of field `SWTRIG12`"]
pub type SWTRIG12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG12`"]
pub struct SWTRIG12_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG12_W<'a> {
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
#[doc = "Reader of field `SWTRIG13`"]
pub type SWTRIG13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG13`"]
pub struct SWTRIG13_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG13_W<'a> {
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
#[doc = "Reader of field `SWTRIG14`"]
pub type SWTRIG14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG14`"]
pub struct SWTRIG14_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG14_W<'a> {
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
#[doc = "Reader of field `SWTRIG15`"]
pub type SWTRIG15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG15`"]
pub struct SWTRIG15_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG15_W<'a> {
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
#[doc = "Reader of field `SWTRIG16`"]
pub type SWTRIG16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG16`"]
pub struct SWTRIG16_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG16_W<'a> {
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
#[doc = "Reader of field `SWTRIG17`"]
pub type SWTRIG17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG17`"]
pub struct SWTRIG17_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG17_W<'a> {
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
#[doc = "Reader of field `SWTRIG18`"]
pub type SWTRIG18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG18`"]
pub struct SWTRIG18_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG18_W<'a> {
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
#[doc = "Reader of field `SWTRIG19`"]
pub type SWTRIG19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG19`"]
pub struct SWTRIG19_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG19_W<'a> {
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
#[doc = "Reader of field `SWTRIG20`"]
pub type SWTRIG20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG20`"]
pub struct SWTRIG20_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG20_W<'a> {
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
#[doc = "Reader of field `SWTRIG21`"]
pub type SWTRIG21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG21`"]
pub struct SWTRIG21_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG21_W<'a> {
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
#[doc = "Reader of field `SWTRIG22`"]
pub type SWTRIG22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG22`"]
pub struct SWTRIG22_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG22_W<'a> {
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
#[doc = "Reader of field `SWTRIG23`"]
pub type SWTRIG23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG23`"]
pub struct SWTRIG23_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG23_W<'a> {
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
#[doc = "Reader of field `SWTRIG24`"]
pub type SWTRIG24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG24`"]
pub struct SWTRIG24_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG24_W<'a> {
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
#[doc = "Reader of field `SWTRIG25`"]
pub type SWTRIG25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG25`"]
pub struct SWTRIG25_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG25_W<'a> {
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
#[doc = "Reader of field `SWTRIG26`"]
pub type SWTRIG26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG26`"]
pub struct SWTRIG26_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG26_W<'a> {
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
#[doc = "Reader of field `SWTRIG27`"]
pub type SWTRIG27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG27`"]
pub struct SWTRIG27_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG27_W<'a> {
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
#[doc = "Reader of field `SWTRIG28`"]
pub type SWTRIG28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG28`"]
pub struct SWTRIG28_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG28_W<'a> {
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
#[doc = "Reader of field `SWTRIG29`"]
pub type SWTRIG29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG29`"]
pub struct SWTRIG29_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG29_W<'a> {
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
#[doc = "Reader of field `SWTRIG30`"]
pub type SWTRIG30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG30`"]
pub struct SWTRIG30_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG30_W<'a> {
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
#[doc = "Reader of field `SWTRIG31`"]
pub type SWTRIG31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG31`"]
pub struct SWTRIG31_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG31_W<'a> {
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
    #[doc = "Bit 0 - Channel 0 Software Trigger"]
    #[inline(always)]
    pub fn swtrig0(&self) -> SWTRIG0_R {
        SWTRIG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Software Trigger"]
    #[inline(always)]
    pub fn swtrig1(&self) -> SWTRIG1_R {
        SWTRIG1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Software Trigger"]
    #[inline(always)]
    pub fn swtrig2(&self) -> SWTRIG2_R {
        SWTRIG2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Software Trigger"]
    #[inline(always)]
    pub fn swtrig3(&self) -> SWTRIG3_R {
        SWTRIG3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Software Trigger"]
    #[inline(always)]
    pub fn swtrig4(&self) -> SWTRIG4_R {
        SWTRIG4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Software Trigger"]
    #[inline(always)]
    pub fn swtrig5(&self) -> SWTRIG5_R {
        SWTRIG5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Software Trigger"]
    #[inline(always)]
    pub fn swtrig6(&self) -> SWTRIG6_R {
        SWTRIG6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Software Trigger"]
    #[inline(always)]
    pub fn swtrig7(&self) -> SWTRIG7_R {
        SWTRIG7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel 8 Software Trigger"]
    #[inline(always)]
    pub fn swtrig8(&self) -> SWTRIG8_R {
        SWTRIG8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel 9 Software Trigger"]
    #[inline(always)]
    pub fn swtrig9(&self) -> SWTRIG9_R {
        SWTRIG9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel 10 Software Trigger"]
    #[inline(always)]
    pub fn swtrig10(&self) -> SWTRIG10_R {
        SWTRIG10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel 11 Software Trigger"]
    #[inline(always)]
    pub fn swtrig11(&self) -> SWTRIG11_R {
        SWTRIG11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Channel 12 Software Trigger"]
    #[inline(always)]
    pub fn swtrig12(&self) -> SWTRIG12_R {
        SWTRIG12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Channel 13 Software Trigger"]
    #[inline(always)]
    pub fn swtrig13(&self) -> SWTRIG13_R {
        SWTRIG13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Channel 14 Software Trigger"]
    #[inline(always)]
    pub fn swtrig14(&self) -> SWTRIG14_R {
        SWTRIG14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Channel 15 Software Trigger"]
    #[inline(always)]
    pub fn swtrig15(&self) -> SWTRIG15_R {
        SWTRIG15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel 16 Software Trigger"]
    #[inline(always)]
    pub fn swtrig16(&self) -> SWTRIG16_R {
        SWTRIG16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel 17 Software Trigger"]
    #[inline(always)]
    pub fn swtrig17(&self) -> SWTRIG17_R {
        SWTRIG17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel 18 Software Trigger"]
    #[inline(always)]
    pub fn swtrig18(&self) -> SWTRIG18_R {
        SWTRIG18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel 19 Software Trigger"]
    #[inline(always)]
    pub fn swtrig19(&self) -> SWTRIG19_R {
        SWTRIG19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Channel 20 Software Trigger"]
    #[inline(always)]
    pub fn swtrig20(&self) -> SWTRIG20_R {
        SWTRIG20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Channel 21 Software Trigger"]
    #[inline(always)]
    pub fn swtrig21(&self) -> SWTRIG21_R {
        SWTRIG21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Channel 22 Software Trigger"]
    #[inline(always)]
    pub fn swtrig22(&self) -> SWTRIG22_R {
        SWTRIG22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Channel 23 Software Trigger"]
    #[inline(always)]
    pub fn swtrig23(&self) -> SWTRIG23_R {
        SWTRIG23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Channel 24 Software Trigger"]
    #[inline(always)]
    pub fn swtrig24(&self) -> SWTRIG24_R {
        SWTRIG24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Channel 25 Software Trigger"]
    #[inline(always)]
    pub fn swtrig25(&self) -> SWTRIG25_R {
        SWTRIG25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Channel 26 Software Trigger"]
    #[inline(always)]
    pub fn swtrig26(&self) -> SWTRIG26_R {
        SWTRIG26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Channel 27 Software Trigger"]
    #[inline(always)]
    pub fn swtrig27(&self) -> SWTRIG27_R {
        SWTRIG27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Channel 28 Software Trigger"]
    #[inline(always)]
    pub fn swtrig28(&self) -> SWTRIG28_R {
        SWTRIG28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Channel 29 Software Trigger"]
    #[inline(always)]
    pub fn swtrig29(&self) -> SWTRIG29_R {
        SWTRIG29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Channel 30 Software Trigger"]
    #[inline(always)]
    pub fn swtrig30(&self) -> SWTRIG30_R {
        SWTRIG30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Channel 31 Software Trigger"]
    #[inline(always)]
    pub fn swtrig31(&self) -> SWTRIG31_R {
        SWTRIG31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Software Trigger"]
    #[inline(always)]
    pub fn swtrig0(&mut self) -> SWTRIG0_W {
        SWTRIG0_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Software Trigger"]
    #[inline(always)]
    pub fn swtrig1(&mut self) -> SWTRIG1_W {
        SWTRIG1_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Software Trigger"]
    #[inline(always)]
    pub fn swtrig2(&mut self) -> SWTRIG2_W {
        SWTRIG2_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Software Trigger"]
    #[inline(always)]
    pub fn swtrig3(&mut self) -> SWTRIG3_W {
        SWTRIG3_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Software Trigger"]
    #[inline(always)]
    pub fn swtrig4(&mut self) -> SWTRIG4_W {
        SWTRIG4_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Software Trigger"]
    #[inline(always)]
    pub fn swtrig5(&mut self) -> SWTRIG5_W {
        SWTRIG5_W { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Software Trigger"]
    #[inline(always)]
    pub fn swtrig6(&mut self) -> SWTRIG6_W {
        SWTRIG6_W { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Software Trigger"]
    #[inline(always)]
    pub fn swtrig7(&mut self) -> SWTRIG7_W {
        SWTRIG7_W { w: self }
    }
    #[doc = "Bit 8 - Channel 8 Software Trigger"]
    #[inline(always)]
    pub fn swtrig8(&mut self) -> SWTRIG8_W {
        SWTRIG8_W { w: self }
    }
    #[doc = "Bit 9 - Channel 9 Software Trigger"]
    #[inline(always)]
    pub fn swtrig9(&mut self) -> SWTRIG9_W {
        SWTRIG9_W { w: self }
    }
    #[doc = "Bit 10 - Channel 10 Software Trigger"]
    #[inline(always)]
    pub fn swtrig10(&mut self) -> SWTRIG10_W {
        SWTRIG10_W { w: self }
    }
    #[doc = "Bit 11 - Channel 11 Software Trigger"]
    #[inline(always)]
    pub fn swtrig11(&mut self) -> SWTRIG11_W {
        SWTRIG11_W { w: self }
    }
    #[doc = "Bit 12 - Channel 12 Software Trigger"]
    #[inline(always)]
    pub fn swtrig12(&mut self) -> SWTRIG12_W {
        SWTRIG12_W { w: self }
    }
    #[doc = "Bit 13 - Channel 13 Software Trigger"]
    #[inline(always)]
    pub fn swtrig13(&mut self) -> SWTRIG13_W {
        SWTRIG13_W { w: self }
    }
    #[doc = "Bit 14 - Channel 14 Software Trigger"]
    #[inline(always)]
    pub fn swtrig14(&mut self) -> SWTRIG14_W {
        SWTRIG14_W { w: self }
    }
    #[doc = "Bit 15 - Channel 15 Software Trigger"]
    #[inline(always)]
    pub fn swtrig15(&mut self) -> SWTRIG15_W {
        SWTRIG15_W { w: self }
    }
    #[doc = "Bit 16 - Channel 16 Software Trigger"]
    #[inline(always)]
    pub fn swtrig16(&mut self) -> SWTRIG16_W {
        SWTRIG16_W { w: self }
    }
    #[doc = "Bit 17 - Channel 17 Software Trigger"]
    #[inline(always)]
    pub fn swtrig17(&mut self) -> SWTRIG17_W {
        SWTRIG17_W { w: self }
    }
    #[doc = "Bit 18 - Channel 18 Software Trigger"]
    #[inline(always)]
    pub fn swtrig18(&mut self) -> SWTRIG18_W {
        SWTRIG18_W { w: self }
    }
    #[doc = "Bit 19 - Channel 19 Software Trigger"]
    #[inline(always)]
    pub fn swtrig19(&mut self) -> SWTRIG19_W {
        SWTRIG19_W { w: self }
    }
    #[doc = "Bit 20 - Channel 20 Software Trigger"]
    #[inline(always)]
    pub fn swtrig20(&mut self) -> SWTRIG20_W {
        SWTRIG20_W { w: self }
    }
    #[doc = "Bit 21 - Channel 21 Software Trigger"]
    #[inline(always)]
    pub fn swtrig21(&mut self) -> SWTRIG21_W {
        SWTRIG21_W { w: self }
    }
    #[doc = "Bit 22 - Channel 22 Software Trigger"]
    #[inline(always)]
    pub fn swtrig22(&mut self) -> SWTRIG22_W {
        SWTRIG22_W { w: self }
    }
    #[doc = "Bit 23 - Channel 23 Software Trigger"]
    #[inline(always)]
    pub fn swtrig23(&mut self) -> SWTRIG23_W {
        SWTRIG23_W { w: self }
    }
    #[doc = "Bit 24 - Channel 24 Software Trigger"]
    #[inline(always)]
    pub fn swtrig24(&mut self) -> SWTRIG24_W {
        SWTRIG24_W { w: self }
    }
    #[doc = "Bit 25 - Channel 25 Software Trigger"]
    #[inline(always)]
    pub fn swtrig25(&mut self) -> SWTRIG25_W {
        SWTRIG25_W { w: self }
    }
    #[doc = "Bit 26 - Channel 26 Software Trigger"]
    #[inline(always)]
    pub fn swtrig26(&mut self) -> SWTRIG26_W {
        SWTRIG26_W { w: self }
    }
    #[doc = "Bit 27 - Channel 27 Software Trigger"]
    #[inline(always)]
    pub fn swtrig27(&mut self) -> SWTRIG27_W {
        SWTRIG27_W { w: self }
    }
    #[doc = "Bit 28 - Channel 28 Software Trigger"]
    #[inline(always)]
    pub fn swtrig28(&mut self) -> SWTRIG28_W {
        SWTRIG28_W { w: self }
    }
    #[doc = "Bit 29 - Channel 29 Software Trigger"]
    #[inline(always)]
    pub fn swtrig29(&mut self) -> SWTRIG29_W {
        SWTRIG29_W { w: self }
    }
    #[doc = "Bit 30 - Channel 30 Software Trigger"]
    #[inline(always)]
    pub fn swtrig30(&mut self) -> SWTRIG30_W {
        SWTRIG30_W { w: self }
    }
    #[doc = "Bit 31 - Channel 31 Software Trigger"]
    #[inline(always)]
    pub fn swtrig31(&mut self) -> SWTRIG31_W {
        SWTRIG31_W { w: self }
    }
}
