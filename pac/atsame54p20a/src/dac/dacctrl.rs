#[doc = "Reader of register DACCTRL%s"]
pub type R = crate::R<u16, super::DACCTRL>;
#[doc = "Writer for register DACCTRL%s"]
pub type W = crate::W<u16, super::DACCTRL>;
#[doc = "Register DACCTRL%s `reset()`'s with value 0"]
impl crate::ResetValue for super::DACCTRL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Current Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CCTRL_A {
    #[doc = "0: GCLK_DAC <= 1.2MHz (100kSPS)"]
    CC100K = 0,
    #[doc = "1: 1.2MHz < GCLK_DAC  <= 6MHz (500kSPS)"]
    CC1M = 1,
    #[doc = "2: 6MHz < GCLK_DAC <= 12MHz (1MSPS)"]
    CC12M = 2,
}
impl From<CCTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: CCTRL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CCTRL`"]
pub type CCTRL_R = crate::R<u8, CCTRL_A>;
impl CCTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CCTRL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CCTRL_A::CC100K),
            1 => Val(CCTRL_A::CC1M),
            2 => Val(CCTRL_A::CC12M),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CC100K`"]
    #[inline(always)]
    pub fn is_cc100k(&self) -> bool {
        *self == CCTRL_A::CC100K
    }
    #[doc = "Checks if the value of the field is `CC1M`"]
    #[inline(always)]
    pub fn is_cc1m(&self) -> bool {
        *self == CCTRL_A::CC1M
    }
    #[doc = "Checks if the value of the field is `CC12M`"]
    #[inline(always)]
    pub fn is_cc12m(&self) -> bool {
        *self == CCTRL_A::CC12M
    }
}
#[doc = "Write proxy for field `CCTRL`"]
pub struct CCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CCTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCTRL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GCLK_DAC <= 1.2MHz (100kSPS)"]
    #[inline(always)]
    pub fn cc100k(self) -> &'a mut W {
        self.variant(CCTRL_A::CC100K)
    }
    #[doc = "1.2MHz < GCLK_DAC <= 6MHz (500kSPS)"]
    #[inline(always)]
    pub fn cc1m(self) -> &'a mut W {
        self.variant(CCTRL_A::CC1M)
    }
    #[doc = "6MHz < GCLK_DAC <= 12MHz (1MSPS)"]
    #[inline(always)]
    pub fn cc12m(self) -> &'a mut W {
        self.variant(CCTRL_A::CC12M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `FEXT`"]
pub type FEXT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEXT`"]
pub struct FEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> FEXT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RUNSTDBY`"]
pub type RUNSTDBY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RUNSTDBY`"]
pub struct RUNSTDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNSTDBY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `DITHER`"]
pub type DITHER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DITHER`"]
pub struct DITHER_W<'a> {
    w: &'a mut W,
}
impl<'a> DITHER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `REFRESH`"]
pub type REFRESH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REFRESH`"]
pub struct REFRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> REFRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `OSR`"]
pub type OSR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSR`"]
pub struct OSR_W<'a> {
    w: &'a mut W,
}
impl<'a> OSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u16) & 0x07) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Left Adjusted Data"]
    #[inline(always)]
    pub fn leftadj(&self) -> LEFTADJ_R {
        LEFTADJ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable DAC0"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Current Control"]
    #[inline(always)]
    pub fn cctrl(&self) -> CCTRL_R {
        CCTRL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Standalone Filter"]
    #[inline(always)]
    pub fn fext(&self) -> FEXT_R {
        FEXT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Dithering Mode"]
    #[inline(always)]
    pub fn dither(&self) -> DITHER_R {
        DITHER_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Refresh period"]
    #[inline(always)]
    pub fn refresh(&self) -> REFRESH_R {
        REFRESH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 13:15 - Sampling Rate"]
    #[inline(always)]
    pub fn osr(&self) -> OSR_R {
        OSR_R::new(((self.bits >> 13) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Left Adjusted Data"]
    #[inline(always)]
    pub fn leftadj(&mut self) -> LEFTADJ_W {
        LEFTADJ_W { w: self }
    }
    #[doc = "Bit 1 - Enable DAC0"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 2:3 - Current Control"]
    #[inline(always)]
    pub fn cctrl(&mut self) -> CCTRL_W {
        CCTRL_W { w: self }
    }
    #[doc = "Bit 5 - Standalone Filter"]
    #[inline(always)]
    pub fn fext(&mut self) -> FEXT_W {
        FEXT_W { w: self }
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
    #[doc = "Bit 7 - Dithering Mode"]
    #[inline(always)]
    pub fn dither(&mut self) -> DITHER_W {
        DITHER_W { w: self }
    }
    #[doc = "Bits 8:11 - Refresh period"]
    #[inline(always)]
    pub fn refresh(&mut self) -> REFRESH_W {
        REFRESH_W { w: self }
    }
    #[doc = "Bits 13:15 - Sampling Rate"]
    #[inline(always)]
    pub fn osr(&mut self) -> OSR_W {
        OSR_W { w: self }
    }
}
