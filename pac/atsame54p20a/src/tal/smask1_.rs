#[doc = "Reader of register SMASK1_%s"]
pub type R = crate::R<u32, super::SMASK1_>;
#[doc = "Writer for register SMASK1_%s"]
pub type W = crate::W<u32, super::SMASK1_>;
#[doc = "Register SMASK1_%s `reset()`'s with value 0"]
impl crate::ResetValue for super::SMASK1_ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IPS0`"]
pub type IPS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS0`"]
pub struct IPS0_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS0_W<'a> {
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
#[doc = "Reader of field `IPS1`"]
pub type IPS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS1`"]
pub struct IPS1_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS1_W<'a> {
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
#[doc = "Reader of field `IPS2`"]
pub type IPS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS2`"]
pub struct IPS2_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS2_W<'a> {
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
#[doc = "Reader of field `IPS3`"]
pub type IPS3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS3`"]
pub struct IPS3_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS3_W<'a> {
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
#[doc = "Reader of field `IPS4`"]
pub type IPS4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS4`"]
pub struct IPS4_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS4_W<'a> {
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
#[doc = "Reader of field `IPS5`"]
pub type IPS5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS5`"]
pub struct IPS5_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS5_W<'a> {
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
#[doc = "Reader of field `IPS6`"]
pub type IPS6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS6`"]
pub struct IPS6_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS6_W<'a> {
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
#[doc = "Reader of field `IPS7`"]
pub type IPS7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS7`"]
pub struct IPS7_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS7_W<'a> {
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
#[doc = "Reader of field `IPS8`"]
pub type IPS8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS8`"]
pub struct IPS8_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS8_W<'a> {
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
#[doc = "Reader of field `IPS9`"]
pub type IPS9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS9`"]
pub struct IPS9_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS9_W<'a> {
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
#[doc = "Reader of field `IPS10`"]
pub type IPS10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS10`"]
pub struct IPS10_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS10_W<'a> {
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
#[doc = "Reader of field `IPS11`"]
pub type IPS11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS11`"]
pub struct IPS11_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS11_W<'a> {
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
#[doc = "Reader of field `IPS12`"]
pub type IPS12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS12`"]
pub struct IPS12_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS12_W<'a> {
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
#[doc = "Reader of field `IPS13`"]
pub type IPS13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS13`"]
pub struct IPS13_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS13_W<'a> {
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
#[doc = "Reader of field `IPS14`"]
pub type IPS14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS14`"]
pub struct IPS14_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS14_W<'a> {
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
#[doc = "Reader of field `IPS15`"]
pub type IPS15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS15`"]
pub struct IPS15_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS15_W<'a> {
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
#[doc = "Reader of field `IPS16`"]
pub type IPS16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS16`"]
pub struct IPS16_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS16_W<'a> {
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
#[doc = "Reader of field `IPS17`"]
pub type IPS17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS17`"]
pub struct IPS17_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS17_W<'a> {
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
#[doc = "Reader of field `IPS18`"]
pub type IPS18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS18`"]
pub struct IPS18_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS18_W<'a> {
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
#[doc = "Reader of field `IPS19`"]
pub type IPS19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS19`"]
pub struct IPS19_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS19_W<'a> {
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
#[doc = "Reader of field `IPS20`"]
pub type IPS20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS20`"]
pub struct IPS20_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS20_W<'a> {
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
#[doc = "Reader of field `IPS21`"]
pub type IPS21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS21`"]
pub struct IPS21_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS21_W<'a> {
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
#[doc = "Reader of field `IPS22`"]
pub type IPS22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS22`"]
pub struct IPS22_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS22_W<'a> {
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
#[doc = "Reader of field `IPS23`"]
pub type IPS23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS23`"]
pub struct IPS23_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS23_W<'a> {
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
#[doc = "Reader of field `IPS24`"]
pub type IPS24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS24`"]
pub struct IPS24_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS24_W<'a> {
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
#[doc = "Reader of field `IPS25`"]
pub type IPS25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS25`"]
pub struct IPS25_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS25_W<'a> {
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
#[doc = "Reader of field `IPS26`"]
pub type IPS26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS26`"]
pub struct IPS26_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS26_W<'a> {
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
#[doc = "Reader of field `IPS27`"]
pub type IPS27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS27`"]
pub struct IPS27_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS27_W<'a> {
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
#[doc = "Reader of field `IPS28`"]
pub type IPS28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS28`"]
pub struct IPS28_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS28_W<'a> {
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
#[doc = "Reader of field `IPS29`"]
pub type IPS29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS29`"]
pub struct IPS29_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS29_W<'a> {
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
#[doc = "Reader of field `IPS30`"]
pub type IPS30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS30`"]
pub struct IPS30_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS30_W<'a> {
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
#[doc = "Reader of field `IPS31`"]
pub type IPS31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS31`"]
pub struct IPS31_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS31_W<'a> {
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
    #[doc = "Bit 0 - Inter-Process Signal 0"]
    #[inline(always)]
    pub fn ips0(&self) -> IPS0_R {
        IPS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Inter-Process Signal 1"]
    #[inline(always)]
    pub fn ips1(&self) -> IPS1_R {
        IPS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Inter-Process Signal 2"]
    #[inline(always)]
    pub fn ips2(&self) -> IPS2_R {
        IPS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Inter-Process Signal 3"]
    #[inline(always)]
    pub fn ips3(&self) -> IPS3_R {
        IPS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Inter-Process Signal 4"]
    #[inline(always)]
    pub fn ips4(&self) -> IPS4_R {
        IPS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Inter-Process Signal 5"]
    #[inline(always)]
    pub fn ips5(&self) -> IPS5_R {
        IPS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Inter-Process Signal 6"]
    #[inline(always)]
    pub fn ips6(&self) -> IPS6_R {
        IPS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Inter-Process Signal 7"]
    #[inline(always)]
    pub fn ips7(&self) -> IPS7_R {
        IPS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Inter-Process Signal 8"]
    #[inline(always)]
    pub fn ips8(&self) -> IPS8_R {
        IPS8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Inter-Process Signal 9"]
    #[inline(always)]
    pub fn ips9(&self) -> IPS9_R {
        IPS9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Inter-Process Signal 10"]
    #[inline(always)]
    pub fn ips10(&self) -> IPS10_R {
        IPS10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Inter-Process Signal 11"]
    #[inline(always)]
    pub fn ips11(&self) -> IPS11_R {
        IPS11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Inter-Process Signal 12"]
    #[inline(always)]
    pub fn ips12(&self) -> IPS12_R {
        IPS12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Inter-Process Signal 13"]
    #[inline(always)]
    pub fn ips13(&self) -> IPS13_R {
        IPS13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Inter-Process Signal 14"]
    #[inline(always)]
    pub fn ips14(&self) -> IPS14_R {
        IPS14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Inter-Process Signal 15"]
    #[inline(always)]
    pub fn ips15(&self) -> IPS15_R {
        IPS15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Inter-Process Signal 16"]
    #[inline(always)]
    pub fn ips16(&self) -> IPS16_R {
        IPS16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Inter-Process Signal 17"]
    #[inline(always)]
    pub fn ips17(&self) -> IPS17_R {
        IPS17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Inter-Process Signal 18"]
    #[inline(always)]
    pub fn ips18(&self) -> IPS18_R {
        IPS18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Inter-Process Signal 19"]
    #[inline(always)]
    pub fn ips19(&self) -> IPS19_R {
        IPS19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Inter-Process Signal 20"]
    #[inline(always)]
    pub fn ips20(&self) -> IPS20_R {
        IPS20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Inter-Process Signal 21"]
    #[inline(always)]
    pub fn ips21(&self) -> IPS21_R {
        IPS21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Inter-Process Signal 22"]
    #[inline(always)]
    pub fn ips22(&self) -> IPS22_R {
        IPS22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Inter-Process Signal 23"]
    #[inline(always)]
    pub fn ips23(&self) -> IPS23_R {
        IPS23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Inter-Process Signal 24"]
    #[inline(always)]
    pub fn ips24(&self) -> IPS24_R {
        IPS24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Inter-Process Signal 25"]
    #[inline(always)]
    pub fn ips25(&self) -> IPS25_R {
        IPS25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Inter-Process Signal 26"]
    #[inline(always)]
    pub fn ips26(&self) -> IPS26_R {
        IPS26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Inter-Process Signal 27"]
    #[inline(always)]
    pub fn ips27(&self) -> IPS27_R {
        IPS27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Inter-Process Signal 28"]
    #[inline(always)]
    pub fn ips28(&self) -> IPS28_R {
        IPS28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Inter-Process Signal 29"]
    #[inline(always)]
    pub fn ips29(&self) -> IPS29_R {
        IPS29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Inter-Process Signal 30"]
    #[inline(always)]
    pub fn ips30(&self) -> IPS30_R {
        IPS30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Inter-Process Signal 31"]
    #[inline(always)]
    pub fn ips31(&self) -> IPS31_R {
        IPS31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Inter-Process Signal 0"]
    #[inline(always)]
    pub fn ips0(&mut self) -> IPS0_W {
        IPS0_W { w: self }
    }
    #[doc = "Bit 1 - Inter-Process Signal 1"]
    #[inline(always)]
    pub fn ips1(&mut self) -> IPS1_W {
        IPS1_W { w: self }
    }
    #[doc = "Bit 2 - Inter-Process Signal 2"]
    #[inline(always)]
    pub fn ips2(&mut self) -> IPS2_W {
        IPS2_W { w: self }
    }
    #[doc = "Bit 3 - Inter-Process Signal 3"]
    #[inline(always)]
    pub fn ips3(&mut self) -> IPS3_W {
        IPS3_W { w: self }
    }
    #[doc = "Bit 4 - Inter-Process Signal 4"]
    #[inline(always)]
    pub fn ips4(&mut self) -> IPS4_W {
        IPS4_W { w: self }
    }
    #[doc = "Bit 5 - Inter-Process Signal 5"]
    #[inline(always)]
    pub fn ips5(&mut self) -> IPS5_W {
        IPS5_W { w: self }
    }
    #[doc = "Bit 6 - Inter-Process Signal 6"]
    #[inline(always)]
    pub fn ips6(&mut self) -> IPS6_W {
        IPS6_W { w: self }
    }
    #[doc = "Bit 7 - Inter-Process Signal 7"]
    #[inline(always)]
    pub fn ips7(&mut self) -> IPS7_W {
        IPS7_W { w: self }
    }
    #[doc = "Bit 8 - Inter-Process Signal 8"]
    #[inline(always)]
    pub fn ips8(&mut self) -> IPS8_W {
        IPS8_W { w: self }
    }
    #[doc = "Bit 9 - Inter-Process Signal 9"]
    #[inline(always)]
    pub fn ips9(&mut self) -> IPS9_W {
        IPS9_W { w: self }
    }
    #[doc = "Bit 10 - Inter-Process Signal 10"]
    #[inline(always)]
    pub fn ips10(&mut self) -> IPS10_W {
        IPS10_W { w: self }
    }
    #[doc = "Bit 11 - Inter-Process Signal 11"]
    #[inline(always)]
    pub fn ips11(&mut self) -> IPS11_W {
        IPS11_W { w: self }
    }
    #[doc = "Bit 12 - Inter-Process Signal 12"]
    #[inline(always)]
    pub fn ips12(&mut self) -> IPS12_W {
        IPS12_W { w: self }
    }
    #[doc = "Bit 13 - Inter-Process Signal 13"]
    #[inline(always)]
    pub fn ips13(&mut self) -> IPS13_W {
        IPS13_W { w: self }
    }
    #[doc = "Bit 14 - Inter-Process Signal 14"]
    #[inline(always)]
    pub fn ips14(&mut self) -> IPS14_W {
        IPS14_W { w: self }
    }
    #[doc = "Bit 15 - Inter-Process Signal 15"]
    #[inline(always)]
    pub fn ips15(&mut self) -> IPS15_W {
        IPS15_W { w: self }
    }
    #[doc = "Bit 16 - Inter-Process Signal 16"]
    #[inline(always)]
    pub fn ips16(&mut self) -> IPS16_W {
        IPS16_W { w: self }
    }
    #[doc = "Bit 17 - Inter-Process Signal 17"]
    #[inline(always)]
    pub fn ips17(&mut self) -> IPS17_W {
        IPS17_W { w: self }
    }
    #[doc = "Bit 18 - Inter-Process Signal 18"]
    #[inline(always)]
    pub fn ips18(&mut self) -> IPS18_W {
        IPS18_W { w: self }
    }
    #[doc = "Bit 19 - Inter-Process Signal 19"]
    #[inline(always)]
    pub fn ips19(&mut self) -> IPS19_W {
        IPS19_W { w: self }
    }
    #[doc = "Bit 20 - Inter-Process Signal 20"]
    #[inline(always)]
    pub fn ips20(&mut self) -> IPS20_W {
        IPS20_W { w: self }
    }
    #[doc = "Bit 21 - Inter-Process Signal 21"]
    #[inline(always)]
    pub fn ips21(&mut self) -> IPS21_W {
        IPS21_W { w: self }
    }
    #[doc = "Bit 22 - Inter-Process Signal 22"]
    #[inline(always)]
    pub fn ips22(&mut self) -> IPS22_W {
        IPS22_W { w: self }
    }
    #[doc = "Bit 23 - Inter-Process Signal 23"]
    #[inline(always)]
    pub fn ips23(&mut self) -> IPS23_W {
        IPS23_W { w: self }
    }
    #[doc = "Bit 24 - Inter-Process Signal 24"]
    #[inline(always)]
    pub fn ips24(&mut self) -> IPS24_W {
        IPS24_W { w: self }
    }
    #[doc = "Bit 25 - Inter-Process Signal 25"]
    #[inline(always)]
    pub fn ips25(&mut self) -> IPS25_W {
        IPS25_W { w: self }
    }
    #[doc = "Bit 26 - Inter-Process Signal 26"]
    #[inline(always)]
    pub fn ips26(&mut self) -> IPS26_W {
        IPS26_W { w: self }
    }
    #[doc = "Bit 27 - Inter-Process Signal 27"]
    #[inline(always)]
    pub fn ips27(&mut self) -> IPS27_W {
        IPS27_W { w: self }
    }
    #[doc = "Bit 28 - Inter-Process Signal 28"]
    #[inline(always)]
    pub fn ips28(&mut self) -> IPS28_W {
        IPS28_W { w: self }
    }
    #[doc = "Bit 29 - Inter-Process Signal 29"]
    #[inline(always)]
    pub fn ips29(&mut self) -> IPS29_W {
        IPS29_W { w: self }
    }
    #[doc = "Bit 30 - Inter-Process Signal 30"]
    #[inline(always)]
    pub fn ips30(&mut self) -> IPS30_W {
        IPS30_W { w: self }
    }
    #[doc = "Bit 31 - Inter-Process Signal 31"]
    #[inline(always)]
    pub fn ips31(&mut self) -> IPS31_W {
        IPS31_W { w: self }
    }
}
