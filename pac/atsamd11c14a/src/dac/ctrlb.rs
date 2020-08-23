#[doc = "Reader of register CTRLB"]
pub type R = crate::R<u8, super::CTRLB>;
#[doc = "Writer for register CTRLB"]
pub type W = crate::W<u8, super::CTRLB>;
#[doc = "Register CTRLB `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLB {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EOEN`"]
pub type EOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOEN`"]
pub struct EOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EOEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `IOEN`"]
pub type IOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOEN`"]
pub struct IOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `LEFTADJ`"]
pub type LEFTADJ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEFTADJ`"]
pub struct LEFTADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> LEFTADJ_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `VPD`"]
pub type VPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VPD`"]
pub struct VPD_W<'a> {
    w: &'a mut W,
}
impl<'a> VPD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `BDWP`"]
pub type BDWP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BDWP`"]
pub struct BDWP_W<'a> {
    w: &'a mut W,
}
impl<'a> BDWP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: Internal 1.0V reference"]
    INT1V = 0,
    #[doc = "1: AVCC"]
    AVCC = 1,
    #[doc = "2: External reference"]
    VREFP = 2,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REFSEL`"]
pub type REFSEL_R = crate::R<u8, REFSEL_A>;
impl REFSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REFSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REFSEL_A::INT1V),
            1 => Val(REFSEL_A::AVCC),
            2 => Val(REFSEL_A::VREFP),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INT1V`"]
    #[inline(always)]
    pub fn is_int1v(&self) -> bool {
        *self == REFSEL_A::INT1V
    }
    #[doc = "Checks if the value of the field is `AVCC`"]
    #[inline(always)]
    pub fn is_avcc(&self) -> bool {
        *self == REFSEL_A::AVCC
    }
    #[doc = "Checks if the value of the field is `VREFP`"]
    #[inline(always)]
    pub fn is_vrefp(&self) -> bool {
        *self == REFSEL_A::VREFP
    }
}
#[doc = "Write proxy for field `REFSEL`"]
pub struct REFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Internal 1.0V reference"]
    #[inline(always)]
    pub fn int1v(self) -> &'a mut W {
        self.variant(REFSEL_A::INT1V)
    }
    #[doc = "AVCC"]
    #[inline(always)]
    pub fn avcc(self) -> &'a mut W {
        self.variant(REFSEL_A::AVCC)
    }
    #[doc = "External reference"]
    #[inline(always)]
    pub fn vrefp(self) -> &'a mut W {
        self.variant(REFSEL_A::VREFP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u8) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - External Output Enable"]
    #[inline(always)]
    pub fn eoen(&self) -> EOEN_R {
        EOEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Internal Output Enable"]
    #[inline(always)]
    pub fn ioen(&self) -> IOEN_R {
        IOEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Left Adjusted Data"]
    #[inline(always)]
    pub fn leftadj(&self) -> LEFTADJ_R {
        LEFTADJ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Voltage Pump Disable"]
    #[inline(always)]
    pub fn vpd(&self) -> VPD_R {
        VPD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Bypass DATABUF Write Protection"]
    #[inline(always)]
    pub fn bdwp(&self) -> BDWP_R {
        BDWP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External Output Enable"]
    #[inline(always)]
    pub fn eoen(&mut self) -> EOEN_W {
        EOEN_W { w: self }
    }
    #[doc = "Bit 1 - Internal Output Enable"]
    #[inline(always)]
    pub fn ioen(&mut self) -> IOEN_W {
        IOEN_W { w: self }
    }
    #[doc = "Bit 2 - Left Adjusted Data"]
    #[inline(always)]
    pub fn leftadj(&mut self) -> LEFTADJ_W {
        LEFTADJ_W { w: self }
    }
    #[doc = "Bit 3 - Voltage Pump Disable"]
    #[inline(always)]
    pub fn vpd(&mut self) -> VPD_W {
        VPD_W { w: self }
    }
    #[doc = "Bit 4 - Bypass DATABUF Write Protection"]
    #[inline(always)]
    pub fn bdwp(&mut self) -> BDWP_W {
        BDWP_W { w: self }
    }
    #[doc = "Bits 6:7 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W { w: self }
    }
}
