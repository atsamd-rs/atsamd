#[doc = "Reader of register TXBCIE"]
pub type R = crate::R<u32, super::TXBCIE>;
#[doc = "Writer for register TXBCIE"]
pub type W = crate::W<u32, super::TXBCIE>;
#[doc = "Register TXBCIE `reset()`'s with value 0"]
impl crate::ResetValue for super::TXBCIE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CFIE0`"]
pub type CFIE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE0`"]
pub struct CFIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE1`"]
pub type CFIE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE1`"]
pub struct CFIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE2`"]
pub type CFIE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE2`"]
pub struct CFIE2_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE3`"]
pub type CFIE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE3`"]
pub struct CFIE3_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE4`"]
pub type CFIE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE4`"]
pub struct CFIE4_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE5`"]
pub type CFIE5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE5`"]
pub struct CFIE5_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE6`"]
pub type CFIE6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE6`"]
pub struct CFIE6_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE7`"]
pub type CFIE7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE7`"]
pub struct CFIE7_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE8`"]
pub type CFIE8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE8`"]
pub struct CFIE8_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE9`"]
pub type CFIE9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE9`"]
pub struct CFIE9_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE10`"]
pub type CFIE10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE10`"]
pub struct CFIE10_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE11`"]
pub type CFIE11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE11`"]
pub struct CFIE11_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE12`"]
pub type CFIE12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE12`"]
pub struct CFIE12_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE13`"]
pub type CFIE13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE13`"]
pub struct CFIE13_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE14`"]
pub type CFIE14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE14`"]
pub struct CFIE14_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE15`"]
pub type CFIE15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE15`"]
pub struct CFIE15_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE16`"]
pub type CFIE16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE16`"]
pub struct CFIE16_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE17`"]
pub type CFIE17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE17`"]
pub struct CFIE17_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE18`"]
pub type CFIE18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE18`"]
pub struct CFIE18_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE19`"]
pub type CFIE19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE19`"]
pub struct CFIE19_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE20`"]
pub type CFIE20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE20`"]
pub struct CFIE20_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE21`"]
pub type CFIE21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE21`"]
pub struct CFIE21_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE22`"]
pub type CFIE22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE22`"]
pub struct CFIE22_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE23`"]
pub type CFIE23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE23`"]
pub struct CFIE23_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE24`"]
pub type CFIE24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE24`"]
pub struct CFIE24_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE25`"]
pub type CFIE25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE25`"]
pub struct CFIE25_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE26`"]
pub type CFIE26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE26`"]
pub struct CFIE26_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE27`"]
pub type CFIE27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE27`"]
pub struct CFIE27_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE28`"]
pub type CFIE28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE28`"]
pub struct CFIE28_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE29`"]
pub type CFIE29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE29`"]
pub struct CFIE29_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE30`"]
pub type CFIE30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE30`"]
pub struct CFIE30_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CFIE31`"]
pub type CFIE31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFIE31`"]
pub struct CFIE31_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - Cancellation Finished Interrupt Enable 0"]
    #[inline(always)]
    pub fn cfie0(&self) -> CFIE0_R {
        CFIE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Cancellation Finished Interrupt Enable 1"]
    #[inline(always)]
    pub fn cfie1(&self) -> CFIE1_R {
        CFIE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Cancellation Finished Interrupt Enable 2"]
    #[inline(always)]
    pub fn cfie2(&self) -> CFIE2_R {
        CFIE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Cancellation Finished Interrupt Enable 3"]
    #[inline(always)]
    pub fn cfie3(&self) -> CFIE3_R {
        CFIE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Cancellation Finished Interrupt Enable 4"]
    #[inline(always)]
    pub fn cfie4(&self) -> CFIE4_R {
        CFIE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Cancellation Finished Interrupt Enable 5"]
    #[inline(always)]
    pub fn cfie5(&self) -> CFIE5_R {
        CFIE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Cancellation Finished Interrupt Enable 6"]
    #[inline(always)]
    pub fn cfie6(&self) -> CFIE6_R {
        CFIE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Cancellation Finished Interrupt Enable 7"]
    #[inline(always)]
    pub fn cfie7(&self) -> CFIE7_R {
        CFIE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Cancellation Finished Interrupt Enable 8"]
    #[inline(always)]
    pub fn cfie8(&self) -> CFIE8_R {
        CFIE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Cancellation Finished Interrupt Enable 9"]
    #[inline(always)]
    pub fn cfie9(&self) -> CFIE9_R {
        CFIE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Cancellation Finished Interrupt Enable 10"]
    #[inline(always)]
    pub fn cfie10(&self) -> CFIE10_R {
        CFIE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Cancellation Finished Interrupt Enable 11"]
    #[inline(always)]
    pub fn cfie11(&self) -> CFIE11_R {
        CFIE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Cancellation Finished Interrupt Enable 12"]
    #[inline(always)]
    pub fn cfie12(&self) -> CFIE12_R {
        CFIE12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Cancellation Finished Interrupt Enable 13"]
    #[inline(always)]
    pub fn cfie13(&self) -> CFIE13_R {
        CFIE13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Cancellation Finished Interrupt Enable 14"]
    #[inline(always)]
    pub fn cfie14(&self) -> CFIE14_R {
        CFIE14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Cancellation Finished Interrupt Enable 15"]
    #[inline(always)]
    pub fn cfie15(&self) -> CFIE15_R {
        CFIE15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Cancellation Finished Interrupt Enable 16"]
    #[inline(always)]
    pub fn cfie16(&self) -> CFIE16_R {
        CFIE16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Cancellation Finished Interrupt Enable 17"]
    #[inline(always)]
    pub fn cfie17(&self) -> CFIE17_R {
        CFIE17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Cancellation Finished Interrupt Enable 18"]
    #[inline(always)]
    pub fn cfie18(&self) -> CFIE18_R {
        CFIE18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Cancellation Finished Interrupt Enable 19"]
    #[inline(always)]
    pub fn cfie19(&self) -> CFIE19_R {
        CFIE19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Cancellation Finished Interrupt Enable 20"]
    #[inline(always)]
    pub fn cfie20(&self) -> CFIE20_R {
        CFIE20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Cancellation Finished Interrupt Enable 21"]
    #[inline(always)]
    pub fn cfie21(&self) -> CFIE21_R {
        CFIE21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Cancellation Finished Interrupt Enable 22"]
    #[inline(always)]
    pub fn cfie22(&self) -> CFIE22_R {
        CFIE22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Cancellation Finished Interrupt Enable 23"]
    #[inline(always)]
    pub fn cfie23(&self) -> CFIE23_R {
        CFIE23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Cancellation Finished Interrupt Enable 24"]
    #[inline(always)]
    pub fn cfie24(&self) -> CFIE24_R {
        CFIE24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Cancellation Finished Interrupt Enable 25"]
    #[inline(always)]
    pub fn cfie25(&self) -> CFIE25_R {
        CFIE25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Cancellation Finished Interrupt Enable 26"]
    #[inline(always)]
    pub fn cfie26(&self) -> CFIE26_R {
        CFIE26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Cancellation Finished Interrupt Enable 27"]
    #[inline(always)]
    pub fn cfie27(&self) -> CFIE27_R {
        CFIE27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Cancellation Finished Interrupt Enable 28"]
    #[inline(always)]
    pub fn cfie28(&self) -> CFIE28_R {
        CFIE28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Cancellation Finished Interrupt Enable 29"]
    #[inline(always)]
    pub fn cfie29(&self) -> CFIE29_R {
        CFIE29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Cancellation Finished Interrupt Enable 30"]
    #[inline(always)]
    pub fn cfie30(&self) -> CFIE30_R {
        CFIE30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Cancellation Finished Interrupt Enable 31"]
    #[inline(always)]
    pub fn cfie31(&self) -> CFIE31_R {
        CFIE31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cancellation Finished Interrupt Enable 0"]
    #[inline(always)]
    pub fn cfie0(&mut self) -> CFIE0_W {
        CFIE0_W { w: self }
    }
    #[doc = "Bit 1 - Cancellation Finished Interrupt Enable 1"]
    #[inline(always)]
    pub fn cfie1(&mut self) -> CFIE1_W {
        CFIE1_W { w: self }
    }
    #[doc = "Bit 2 - Cancellation Finished Interrupt Enable 2"]
    #[inline(always)]
    pub fn cfie2(&mut self) -> CFIE2_W {
        CFIE2_W { w: self }
    }
    #[doc = "Bit 3 - Cancellation Finished Interrupt Enable 3"]
    #[inline(always)]
    pub fn cfie3(&mut self) -> CFIE3_W {
        CFIE3_W { w: self }
    }
    #[doc = "Bit 4 - Cancellation Finished Interrupt Enable 4"]
    #[inline(always)]
    pub fn cfie4(&mut self) -> CFIE4_W {
        CFIE4_W { w: self }
    }
    #[doc = "Bit 5 - Cancellation Finished Interrupt Enable 5"]
    #[inline(always)]
    pub fn cfie5(&mut self) -> CFIE5_W {
        CFIE5_W { w: self }
    }
    #[doc = "Bit 6 - Cancellation Finished Interrupt Enable 6"]
    #[inline(always)]
    pub fn cfie6(&mut self) -> CFIE6_W {
        CFIE6_W { w: self }
    }
    #[doc = "Bit 7 - Cancellation Finished Interrupt Enable 7"]
    #[inline(always)]
    pub fn cfie7(&mut self) -> CFIE7_W {
        CFIE7_W { w: self }
    }
    #[doc = "Bit 8 - Cancellation Finished Interrupt Enable 8"]
    #[inline(always)]
    pub fn cfie8(&mut self) -> CFIE8_W {
        CFIE8_W { w: self }
    }
    #[doc = "Bit 9 - Cancellation Finished Interrupt Enable 9"]
    #[inline(always)]
    pub fn cfie9(&mut self) -> CFIE9_W {
        CFIE9_W { w: self }
    }
    #[doc = "Bit 10 - Cancellation Finished Interrupt Enable 10"]
    #[inline(always)]
    pub fn cfie10(&mut self) -> CFIE10_W {
        CFIE10_W { w: self }
    }
    #[doc = "Bit 11 - Cancellation Finished Interrupt Enable 11"]
    #[inline(always)]
    pub fn cfie11(&mut self) -> CFIE11_W {
        CFIE11_W { w: self }
    }
    #[doc = "Bit 12 - Cancellation Finished Interrupt Enable 12"]
    #[inline(always)]
    pub fn cfie12(&mut self) -> CFIE12_W {
        CFIE12_W { w: self }
    }
    #[doc = "Bit 13 - Cancellation Finished Interrupt Enable 13"]
    #[inline(always)]
    pub fn cfie13(&mut self) -> CFIE13_W {
        CFIE13_W { w: self }
    }
    #[doc = "Bit 14 - Cancellation Finished Interrupt Enable 14"]
    #[inline(always)]
    pub fn cfie14(&mut self) -> CFIE14_W {
        CFIE14_W { w: self }
    }
    #[doc = "Bit 15 - Cancellation Finished Interrupt Enable 15"]
    #[inline(always)]
    pub fn cfie15(&mut self) -> CFIE15_W {
        CFIE15_W { w: self }
    }
    #[doc = "Bit 16 - Cancellation Finished Interrupt Enable 16"]
    #[inline(always)]
    pub fn cfie16(&mut self) -> CFIE16_W {
        CFIE16_W { w: self }
    }
    #[doc = "Bit 17 - Cancellation Finished Interrupt Enable 17"]
    #[inline(always)]
    pub fn cfie17(&mut self) -> CFIE17_W {
        CFIE17_W { w: self }
    }
    #[doc = "Bit 18 - Cancellation Finished Interrupt Enable 18"]
    #[inline(always)]
    pub fn cfie18(&mut self) -> CFIE18_W {
        CFIE18_W { w: self }
    }
    #[doc = "Bit 19 - Cancellation Finished Interrupt Enable 19"]
    #[inline(always)]
    pub fn cfie19(&mut self) -> CFIE19_W {
        CFIE19_W { w: self }
    }
    #[doc = "Bit 20 - Cancellation Finished Interrupt Enable 20"]
    #[inline(always)]
    pub fn cfie20(&mut self) -> CFIE20_W {
        CFIE20_W { w: self }
    }
    #[doc = "Bit 21 - Cancellation Finished Interrupt Enable 21"]
    #[inline(always)]
    pub fn cfie21(&mut self) -> CFIE21_W {
        CFIE21_W { w: self }
    }
    #[doc = "Bit 22 - Cancellation Finished Interrupt Enable 22"]
    #[inline(always)]
    pub fn cfie22(&mut self) -> CFIE22_W {
        CFIE22_W { w: self }
    }
    #[doc = "Bit 23 - Cancellation Finished Interrupt Enable 23"]
    #[inline(always)]
    pub fn cfie23(&mut self) -> CFIE23_W {
        CFIE23_W { w: self }
    }
    #[doc = "Bit 24 - Cancellation Finished Interrupt Enable 24"]
    #[inline(always)]
    pub fn cfie24(&mut self) -> CFIE24_W {
        CFIE24_W { w: self }
    }
    #[doc = "Bit 25 - Cancellation Finished Interrupt Enable 25"]
    #[inline(always)]
    pub fn cfie25(&mut self) -> CFIE25_W {
        CFIE25_W { w: self }
    }
    #[doc = "Bit 26 - Cancellation Finished Interrupt Enable 26"]
    #[inline(always)]
    pub fn cfie26(&mut self) -> CFIE26_W {
        CFIE26_W { w: self }
    }
    #[doc = "Bit 27 - Cancellation Finished Interrupt Enable 27"]
    #[inline(always)]
    pub fn cfie27(&mut self) -> CFIE27_W {
        CFIE27_W { w: self }
    }
    #[doc = "Bit 28 - Cancellation Finished Interrupt Enable 28"]
    #[inline(always)]
    pub fn cfie28(&mut self) -> CFIE28_W {
        CFIE28_W { w: self }
    }
    #[doc = "Bit 29 - Cancellation Finished Interrupt Enable 29"]
    #[inline(always)]
    pub fn cfie29(&mut self) -> CFIE29_W {
        CFIE29_W { w: self }
    }
    #[doc = "Bit 30 - Cancellation Finished Interrupt Enable 30"]
    #[inline(always)]
    pub fn cfie30(&mut self) -> CFIE30_W {
        CFIE30_W { w: self }
    }
    #[doc = "Bit 31 - Cancellation Finished Interrupt Enable 31"]
    #[inline(always)]
    pub fn cfie31(&mut self) -> CFIE31_W {
        CFIE31_W { w: self }
    }
}
