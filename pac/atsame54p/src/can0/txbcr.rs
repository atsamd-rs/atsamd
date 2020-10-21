#[doc = "Reader of register TXBCR"]
pub type R = crate::R<u32, super::TXBCR>;
#[doc = "Writer for register TXBCR"]
pub type W = crate::W<u32, super::TXBCR>;
#[doc = "Register TXBCR `reset()`'s with value 0"]
impl crate::ResetValue for super::TXBCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CR0`"]
pub type CR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR0`"]
pub struct CR0_W<'a> {
    w: &'a mut W,
}
impl<'a> CR0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR1`"]
pub type CR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR1`"]
pub struct CR1_W<'a> {
    w: &'a mut W,
}
impl<'a> CR1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR2`"]
pub type CR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR2`"]
pub struct CR2_W<'a> {
    w: &'a mut W,
}
impl<'a> CR2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR3`"]
pub type CR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR3`"]
pub struct CR3_W<'a> {
    w: &'a mut W,
}
impl<'a> CR3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR4`"]
pub type CR4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR4`"]
pub struct CR4_W<'a> {
    w: &'a mut W,
}
impl<'a> CR4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR5`"]
pub type CR5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR5`"]
pub struct CR5_W<'a> {
    w: &'a mut W,
}
impl<'a> CR5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR6`"]
pub type CR6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR6`"]
pub struct CR6_W<'a> {
    w: &'a mut W,
}
impl<'a> CR6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR7`"]
pub type CR7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR7`"]
pub struct CR7_W<'a> {
    w: &'a mut W,
}
impl<'a> CR7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR8`"]
pub type CR8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR8`"]
pub struct CR8_W<'a> {
    w: &'a mut W,
}
impl<'a> CR8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR9`"]
pub type CR9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR9`"]
pub struct CR9_W<'a> {
    w: &'a mut W,
}
impl<'a> CR9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR10`"]
pub type CR10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR10`"]
pub struct CR10_W<'a> {
    w: &'a mut W,
}
impl<'a> CR10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR11`"]
pub type CR11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR11`"]
pub struct CR11_W<'a> {
    w: &'a mut W,
}
impl<'a> CR11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR12`"]
pub type CR12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR12`"]
pub struct CR12_W<'a> {
    w: &'a mut W,
}
impl<'a> CR12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR13`"]
pub type CR13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR13`"]
pub struct CR13_W<'a> {
    w: &'a mut W,
}
impl<'a> CR13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR14`"]
pub type CR14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR14`"]
pub struct CR14_W<'a> {
    w: &'a mut W,
}
impl<'a> CR14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR15`"]
pub type CR15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR15`"]
pub struct CR15_W<'a> {
    w: &'a mut W,
}
impl<'a> CR15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR16`"]
pub type CR16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR16`"]
pub struct CR16_W<'a> {
    w: &'a mut W,
}
impl<'a> CR16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR17`"]
pub type CR17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR17`"]
pub struct CR17_W<'a> {
    w: &'a mut W,
}
impl<'a> CR17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR18`"]
pub type CR18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR18`"]
pub struct CR18_W<'a> {
    w: &'a mut W,
}
impl<'a> CR18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR19`"]
pub type CR19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR19`"]
pub struct CR19_W<'a> {
    w: &'a mut W,
}
impl<'a> CR19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR20`"]
pub type CR20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR20`"]
pub struct CR20_W<'a> {
    w: &'a mut W,
}
impl<'a> CR20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR21`"]
pub type CR21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR21`"]
pub struct CR21_W<'a> {
    w: &'a mut W,
}
impl<'a> CR21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR22`"]
pub type CR22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR22`"]
pub struct CR22_W<'a> {
    w: &'a mut W,
}
impl<'a> CR22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR23`"]
pub type CR23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR23`"]
pub struct CR23_W<'a> {
    w: &'a mut W,
}
impl<'a> CR23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR24`"]
pub type CR24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR24`"]
pub struct CR24_W<'a> {
    w: &'a mut W,
}
impl<'a> CR24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR25`"]
pub type CR25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR25`"]
pub struct CR25_W<'a> {
    w: &'a mut W,
}
impl<'a> CR25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR26`"]
pub type CR26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR26`"]
pub struct CR26_W<'a> {
    w: &'a mut W,
}
impl<'a> CR26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR27`"]
pub type CR27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR27`"]
pub struct CR27_W<'a> {
    w: &'a mut W,
}
impl<'a> CR27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR28`"]
pub type CR28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR28`"]
pub struct CR28_W<'a> {
    w: &'a mut W,
}
impl<'a> CR28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR29`"]
pub type CR29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR29`"]
pub struct CR29_W<'a> {
    w: &'a mut W,
}
impl<'a> CR29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR30`"]
pub type CR30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR30`"]
pub struct CR30_W<'a> {
    w: &'a mut W,
}
impl<'a> CR30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CR31`"]
pub type CR31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR31`"]
pub struct CR31_W<'a> {
    w: &'a mut W,
}
impl<'a> CR31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - Cancellation Request 0"]
    #[inline(always)]
    pub fn cr0(&self) -> CR0_R {
        CR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Cancellation Request 1"]
    #[inline(always)]
    pub fn cr1(&self) -> CR1_R {
        CR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Cancellation Request 2"]
    #[inline(always)]
    pub fn cr2(&self) -> CR2_R {
        CR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Cancellation Request 3"]
    #[inline(always)]
    pub fn cr3(&self) -> CR3_R {
        CR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Cancellation Request 4"]
    #[inline(always)]
    pub fn cr4(&self) -> CR4_R {
        CR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Cancellation Request 5"]
    #[inline(always)]
    pub fn cr5(&self) -> CR5_R {
        CR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Cancellation Request 6"]
    #[inline(always)]
    pub fn cr6(&self) -> CR6_R {
        CR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Cancellation Request 7"]
    #[inline(always)]
    pub fn cr7(&self) -> CR7_R {
        CR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Cancellation Request 8"]
    #[inline(always)]
    pub fn cr8(&self) -> CR8_R {
        CR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Cancellation Request 9"]
    #[inline(always)]
    pub fn cr9(&self) -> CR9_R {
        CR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Cancellation Request 10"]
    #[inline(always)]
    pub fn cr10(&self) -> CR10_R {
        CR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Cancellation Request 11"]
    #[inline(always)]
    pub fn cr11(&self) -> CR11_R {
        CR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Cancellation Request 12"]
    #[inline(always)]
    pub fn cr12(&self) -> CR12_R {
        CR12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Cancellation Request 13"]
    #[inline(always)]
    pub fn cr13(&self) -> CR13_R {
        CR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Cancellation Request 14"]
    #[inline(always)]
    pub fn cr14(&self) -> CR14_R {
        CR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Cancellation Request 15"]
    #[inline(always)]
    pub fn cr15(&self) -> CR15_R {
        CR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Cancellation Request 16"]
    #[inline(always)]
    pub fn cr16(&self) -> CR16_R {
        CR16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Cancellation Request 17"]
    #[inline(always)]
    pub fn cr17(&self) -> CR17_R {
        CR17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Cancellation Request 18"]
    #[inline(always)]
    pub fn cr18(&self) -> CR18_R {
        CR18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Cancellation Request 19"]
    #[inline(always)]
    pub fn cr19(&self) -> CR19_R {
        CR19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Cancellation Request 20"]
    #[inline(always)]
    pub fn cr20(&self) -> CR20_R {
        CR20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Cancellation Request 21"]
    #[inline(always)]
    pub fn cr21(&self) -> CR21_R {
        CR21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Cancellation Request 22"]
    #[inline(always)]
    pub fn cr22(&self) -> CR22_R {
        CR22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Cancellation Request 23"]
    #[inline(always)]
    pub fn cr23(&self) -> CR23_R {
        CR23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Cancellation Request 24"]
    #[inline(always)]
    pub fn cr24(&self) -> CR24_R {
        CR24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Cancellation Request 25"]
    #[inline(always)]
    pub fn cr25(&self) -> CR25_R {
        CR25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Cancellation Request 26"]
    #[inline(always)]
    pub fn cr26(&self) -> CR26_R {
        CR26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Cancellation Request 27"]
    #[inline(always)]
    pub fn cr27(&self) -> CR27_R {
        CR27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Cancellation Request 28"]
    #[inline(always)]
    pub fn cr28(&self) -> CR28_R {
        CR28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Cancellation Request 29"]
    #[inline(always)]
    pub fn cr29(&self) -> CR29_R {
        CR29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Cancellation Request 30"]
    #[inline(always)]
    pub fn cr30(&self) -> CR30_R {
        CR30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Cancellation Request 31"]
    #[inline(always)]
    pub fn cr31(&self) -> CR31_R {
        CR31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cancellation Request 0"]
    #[inline(always)]
    pub fn cr0(&mut self) -> CR0_W {
        CR0_W { w: self }
    }
    #[doc = "Bit 1 - Cancellation Request 1"]
    #[inline(always)]
    pub fn cr1(&mut self) -> CR1_W {
        CR1_W { w: self }
    }
    #[doc = "Bit 2 - Cancellation Request 2"]
    #[inline(always)]
    pub fn cr2(&mut self) -> CR2_W {
        CR2_W { w: self }
    }
    #[doc = "Bit 3 - Cancellation Request 3"]
    #[inline(always)]
    pub fn cr3(&mut self) -> CR3_W {
        CR3_W { w: self }
    }
    #[doc = "Bit 4 - Cancellation Request 4"]
    #[inline(always)]
    pub fn cr4(&mut self) -> CR4_W {
        CR4_W { w: self }
    }
    #[doc = "Bit 5 - Cancellation Request 5"]
    #[inline(always)]
    pub fn cr5(&mut self) -> CR5_W {
        CR5_W { w: self }
    }
    #[doc = "Bit 6 - Cancellation Request 6"]
    #[inline(always)]
    pub fn cr6(&mut self) -> CR6_W {
        CR6_W { w: self }
    }
    #[doc = "Bit 7 - Cancellation Request 7"]
    #[inline(always)]
    pub fn cr7(&mut self) -> CR7_W {
        CR7_W { w: self }
    }
    #[doc = "Bit 8 - Cancellation Request 8"]
    #[inline(always)]
    pub fn cr8(&mut self) -> CR8_W {
        CR8_W { w: self }
    }
    #[doc = "Bit 9 - Cancellation Request 9"]
    #[inline(always)]
    pub fn cr9(&mut self) -> CR9_W {
        CR9_W { w: self }
    }
    #[doc = "Bit 10 - Cancellation Request 10"]
    #[inline(always)]
    pub fn cr10(&mut self) -> CR10_W {
        CR10_W { w: self }
    }
    #[doc = "Bit 11 - Cancellation Request 11"]
    #[inline(always)]
    pub fn cr11(&mut self) -> CR11_W {
        CR11_W { w: self }
    }
    #[doc = "Bit 12 - Cancellation Request 12"]
    #[inline(always)]
    pub fn cr12(&mut self) -> CR12_W {
        CR12_W { w: self }
    }
    #[doc = "Bit 13 - Cancellation Request 13"]
    #[inline(always)]
    pub fn cr13(&mut self) -> CR13_W {
        CR13_W { w: self }
    }
    #[doc = "Bit 14 - Cancellation Request 14"]
    #[inline(always)]
    pub fn cr14(&mut self) -> CR14_W {
        CR14_W { w: self }
    }
    #[doc = "Bit 15 - Cancellation Request 15"]
    #[inline(always)]
    pub fn cr15(&mut self) -> CR15_W {
        CR15_W { w: self }
    }
    #[doc = "Bit 16 - Cancellation Request 16"]
    #[inline(always)]
    pub fn cr16(&mut self) -> CR16_W {
        CR16_W { w: self }
    }
    #[doc = "Bit 17 - Cancellation Request 17"]
    #[inline(always)]
    pub fn cr17(&mut self) -> CR17_W {
        CR17_W { w: self }
    }
    #[doc = "Bit 18 - Cancellation Request 18"]
    #[inline(always)]
    pub fn cr18(&mut self) -> CR18_W {
        CR18_W { w: self }
    }
    #[doc = "Bit 19 - Cancellation Request 19"]
    #[inline(always)]
    pub fn cr19(&mut self) -> CR19_W {
        CR19_W { w: self }
    }
    #[doc = "Bit 20 - Cancellation Request 20"]
    #[inline(always)]
    pub fn cr20(&mut self) -> CR20_W {
        CR20_W { w: self }
    }
    #[doc = "Bit 21 - Cancellation Request 21"]
    #[inline(always)]
    pub fn cr21(&mut self) -> CR21_W {
        CR21_W { w: self }
    }
    #[doc = "Bit 22 - Cancellation Request 22"]
    #[inline(always)]
    pub fn cr22(&mut self) -> CR22_W {
        CR22_W { w: self }
    }
    #[doc = "Bit 23 - Cancellation Request 23"]
    #[inline(always)]
    pub fn cr23(&mut self) -> CR23_W {
        CR23_W { w: self }
    }
    #[doc = "Bit 24 - Cancellation Request 24"]
    #[inline(always)]
    pub fn cr24(&mut self) -> CR24_W {
        CR24_W { w: self }
    }
    #[doc = "Bit 25 - Cancellation Request 25"]
    #[inline(always)]
    pub fn cr25(&mut self) -> CR25_W {
        CR25_W { w: self }
    }
    #[doc = "Bit 26 - Cancellation Request 26"]
    #[inline(always)]
    pub fn cr26(&mut self) -> CR26_W {
        CR26_W { w: self }
    }
    #[doc = "Bit 27 - Cancellation Request 27"]
    #[inline(always)]
    pub fn cr27(&mut self) -> CR27_W {
        CR27_W { w: self }
    }
    #[doc = "Bit 28 - Cancellation Request 28"]
    #[inline(always)]
    pub fn cr28(&mut self) -> CR28_W {
        CR28_W { w: self }
    }
    #[doc = "Bit 29 - Cancellation Request 29"]
    #[inline(always)]
    pub fn cr29(&mut self) -> CR29_W {
        CR29_W { w: self }
    }
    #[doc = "Bit 30 - Cancellation Request 30"]
    #[inline(always)]
    pub fn cr30(&mut self) -> CR30_W {
        CR30_W { w: self }
    }
    #[doc = "Bit 31 - Cancellation Request 31"]
    #[inline(always)]
    pub fn cr31(&mut self) -> CR31_W {
        CR31_W { w: self }
    }
}
