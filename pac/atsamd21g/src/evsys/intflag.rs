#[doc = "Reader of register INTFLAG"]
pub type R = crate::R<u32, super::INTFLAG>;
#[doc = "Writer for register INTFLAG"]
pub type W = crate::W<u32, super::INTFLAG>;
#[doc = "Register INTFLAG `reset()`'s with value 0"]
impl crate::ResetValue for super::INTFLAG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OVR0`"]
pub type OVR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVR0`"]
pub struct OVR0_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `OVR1`"]
pub type OVR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVR1`"]
pub struct OVR1_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `OVR2`"]
pub type OVR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVR2`"]
pub struct OVR2_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `OVR3`"]
pub type OVR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVR3`"]
pub struct OVR3_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `OVR4`"]
pub type OVR4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVR4`"]
pub struct OVR4_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `OVR5`"]
pub type OVR5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVR5`"]
pub struct OVR5_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `OVR6`"]
pub type OVR6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVR6`"]
pub struct OVR6_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `OVR7`"]
pub type OVR7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVR7`"]
pub struct OVR7_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `EVD0`"]
pub type EVD0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVD0`"]
pub struct EVD0_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `EVD1`"]
pub type EVD1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVD1`"]
pub struct EVD1_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `EVD2`"]
pub type EVD2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVD2`"]
pub struct EVD2_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `EVD3`"]
pub type EVD3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVD3`"]
pub struct EVD3_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `EVD4`"]
pub type EVD4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVD4`"]
pub struct EVD4_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `EVD5`"]
pub type EVD5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVD5`"]
pub struct EVD5_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `EVD6`"]
pub type EVD6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVD6`"]
pub struct EVD6_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `EVD7`"]
pub type EVD7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVD7`"]
pub struct EVD7_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `OVR8`"]
pub type OVR8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVR8`"]
pub struct OVR8_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `OVR9`"]
pub type OVR9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVR9`"]
pub struct OVR9_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `OVR10`"]
pub type OVR10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVR10`"]
pub struct OVR10_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `OVR11`"]
pub type OVR11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVR11`"]
pub struct OVR11_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `EVD8`"]
pub type EVD8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVD8`"]
pub struct EVD8_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `EVD9`"]
pub type EVD9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVD9`"]
pub struct EVD9_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `EVD10`"]
pub type EVD10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVD10`"]
pub struct EVD10_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `EVD11`"]
pub type EVD11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVD11`"]
pub struct EVD11_W<'a> {
    w: &'a mut W,
}
impl<'a> EVD11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl R {
    #[doc = "Bit 0 - Channel 0 Overrun"]
    #[inline(always)]
    pub fn ovr0(&self) -> OVR0_R {
        OVR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Overrun"]
    #[inline(always)]
    pub fn ovr1(&self) -> OVR1_R {
        OVR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Overrun"]
    #[inline(always)]
    pub fn ovr2(&self) -> OVR2_R {
        OVR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Overrun"]
    #[inline(always)]
    pub fn ovr3(&self) -> OVR3_R {
        OVR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Overrun"]
    #[inline(always)]
    pub fn ovr4(&self) -> OVR4_R {
        OVR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Overrun"]
    #[inline(always)]
    pub fn ovr5(&self) -> OVR5_R {
        OVR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Overrun"]
    #[inline(always)]
    pub fn ovr6(&self) -> OVR6_R {
        OVR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Overrun"]
    #[inline(always)]
    pub fn ovr7(&self) -> OVR7_R {
        OVR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel 0 Event Detection"]
    #[inline(always)]
    pub fn evd0(&self) -> EVD0_R {
        EVD0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel 1 Event Detection"]
    #[inline(always)]
    pub fn evd1(&self) -> EVD1_R {
        EVD1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel 2 Event Detection"]
    #[inline(always)]
    pub fn evd2(&self) -> EVD2_R {
        EVD2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel 3 Event Detection"]
    #[inline(always)]
    pub fn evd3(&self) -> EVD3_R {
        EVD3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Channel 4 Event Detection"]
    #[inline(always)]
    pub fn evd4(&self) -> EVD4_R {
        EVD4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Channel 5 Event Detection"]
    #[inline(always)]
    pub fn evd5(&self) -> EVD5_R {
        EVD5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Channel 6 Event Detection"]
    #[inline(always)]
    pub fn evd6(&self) -> EVD6_R {
        EVD6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Channel 7 Event Detection"]
    #[inline(always)]
    pub fn evd7(&self) -> EVD7_R {
        EVD7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel 8 Overrun"]
    #[inline(always)]
    pub fn ovr8(&self) -> OVR8_R {
        OVR8_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel 9 Overrun"]
    #[inline(always)]
    pub fn ovr9(&self) -> OVR9_R {
        OVR9_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel 10 Overrun"]
    #[inline(always)]
    pub fn ovr10(&self) -> OVR10_R {
        OVR10_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel 11 Overrun"]
    #[inline(always)]
    pub fn ovr11(&self) -> OVR11_R {
        OVR11_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Channel 8 Event Detection"]
    #[inline(always)]
    pub fn evd8(&self) -> EVD8_R {
        EVD8_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Channel 9 Event Detection"]
    #[inline(always)]
    pub fn evd9(&self) -> EVD9_R {
        EVD9_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Channel 10 Event Detection"]
    #[inline(always)]
    pub fn evd10(&self) -> EVD10_R {
        EVD10_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Channel 11 Event Detection"]
    #[inline(always)]
    pub fn evd11(&self) -> EVD11_R {
        EVD11_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Overrun"]
    #[inline(always)]
    pub fn ovr0(&mut self) -> OVR0_W {
        OVR0_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Overrun"]
    #[inline(always)]
    pub fn ovr1(&mut self) -> OVR1_W {
        OVR1_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Overrun"]
    #[inline(always)]
    pub fn ovr2(&mut self) -> OVR2_W {
        OVR2_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Overrun"]
    #[inline(always)]
    pub fn ovr3(&mut self) -> OVR3_W {
        OVR3_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Overrun"]
    #[inline(always)]
    pub fn ovr4(&mut self) -> OVR4_W {
        OVR4_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Overrun"]
    #[inline(always)]
    pub fn ovr5(&mut self) -> OVR5_W {
        OVR5_W { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Overrun"]
    #[inline(always)]
    pub fn ovr6(&mut self) -> OVR6_W {
        OVR6_W { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Overrun"]
    #[inline(always)]
    pub fn ovr7(&mut self) -> OVR7_W {
        OVR7_W { w: self }
    }
    #[doc = "Bit 8 - Channel 0 Event Detection"]
    #[inline(always)]
    pub fn evd0(&mut self) -> EVD0_W {
        EVD0_W { w: self }
    }
    #[doc = "Bit 9 - Channel 1 Event Detection"]
    #[inline(always)]
    pub fn evd1(&mut self) -> EVD1_W {
        EVD1_W { w: self }
    }
    #[doc = "Bit 10 - Channel 2 Event Detection"]
    #[inline(always)]
    pub fn evd2(&mut self) -> EVD2_W {
        EVD2_W { w: self }
    }
    #[doc = "Bit 11 - Channel 3 Event Detection"]
    #[inline(always)]
    pub fn evd3(&mut self) -> EVD3_W {
        EVD3_W { w: self }
    }
    #[doc = "Bit 12 - Channel 4 Event Detection"]
    #[inline(always)]
    pub fn evd4(&mut self) -> EVD4_W {
        EVD4_W { w: self }
    }
    #[doc = "Bit 13 - Channel 5 Event Detection"]
    #[inline(always)]
    pub fn evd5(&mut self) -> EVD5_W {
        EVD5_W { w: self }
    }
    #[doc = "Bit 14 - Channel 6 Event Detection"]
    #[inline(always)]
    pub fn evd6(&mut self) -> EVD6_W {
        EVD6_W { w: self }
    }
    #[doc = "Bit 15 - Channel 7 Event Detection"]
    #[inline(always)]
    pub fn evd7(&mut self) -> EVD7_W {
        EVD7_W { w: self }
    }
    #[doc = "Bit 16 - Channel 8 Overrun"]
    #[inline(always)]
    pub fn ovr8(&mut self) -> OVR8_W {
        OVR8_W { w: self }
    }
    #[doc = "Bit 17 - Channel 9 Overrun"]
    #[inline(always)]
    pub fn ovr9(&mut self) -> OVR9_W {
        OVR9_W { w: self }
    }
    #[doc = "Bit 18 - Channel 10 Overrun"]
    #[inline(always)]
    pub fn ovr10(&mut self) -> OVR10_W {
        OVR10_W { w: self }
    }
    #[doc = "Bit 19 - Channel 11 Overrun"]
    #[inline(always)]
    pub fn ovr11(&mut self) -> OVR11_W {
        OVR11_W { w: self }
    }
    #[doc = "Bit 24 - Channel 8 Event Detection"]
    #[inline(always)]
    pub fn evd8(&mut self) -> EVD8_W {
        EVD8_W { w: self }
    }
    #[doc = "Bit 25 - Channel 9 Event Detection"]
    #[inline(always)]
    pub fn evd9(&mut self) -> EVD9_W {
        EVD9_W { w: self }
    }
    #[doc = "Bit 26 - Channel 10 Event Detection"]
    #[inline(always)]
    pub fn evd10(&mut self) -> EVD10_W {
        EVD10_W { w: self }
    }
    #[doc = "Bit 27 - Channel 11 Event Detection"]
    #[inline(always)]
    pub fn evd11(&mut self) -> EVD11_W {
        EVD11_W { w: self }
    }
}
