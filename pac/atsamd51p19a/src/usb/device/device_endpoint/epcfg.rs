#[doc = "Reader of register EPCFG"]
pub type R = crate::R<u8, super::EPCFG>;
#[doc = "Writer for register EPCFG"]
pub type W = crate::W<u8, super::EPCFG>;
#[doc = "Register EPCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::EPCFG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EPTYPE0`"]
pub type EPTYPE0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EPTYPE0`"]
pub struct EPTYPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTYPE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `EPTYPE1`"]
pub type EPTYPE1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EPTYPE1`"]
pub struct EPTYPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTYPE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u8) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `NYETDIS`"]
pub type NYETDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NYETDIS`"]
pub struct NYETDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NYETDIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - End Point Type0"]
    #[inline(always)]
    pub fn eptype0(&self) -> EPTYPE0_R {
        EPTYPE0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - End Point Type1"]
    #[inline(always)]
    pub fn eptype1(&self) -> EPTYPE1_R {
        EPTYPE1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - NYET Token Disable"]
    #[inline(always)]
    pub fn nyetdis(&self) -> NYETDIS_R {
        NYETDIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - End Point Type0"]
    #[inline(always)]
    pub fn eptype0(&mut self) -> EPTYPE0_W {
        EPTYPE0_W { w: self }
    }
    #[doc = "Bits 4:6 - End Point Type1"]
    #[inline(always)]
    pub fn eptype1(&mut self) -> EPTYPE1_W {
        EPTYPE1_W { w: self }
    }
    #[doc = "Bit 7 - NYET Token Disable"]
    #[inline(always)]
    pub fn nyetdis(&mut self) -> NYETDIS_W {
        NYETDIS_W { w: self }
    }
}
