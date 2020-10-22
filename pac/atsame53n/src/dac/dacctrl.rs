#[doc = "Reader of register DACCTRL[%s]"]
pub type R = crate::R<u16, super::DACCTRL>;
#[doc = "Writer for register DACCTRL[%s]"]
pub type W = crate::W<u16, super::DACCTRL>;
#[doc = "Register DACCTRL[%s]
`reset()`'s with value 0"]
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
    #[doc = "0: 100kSPS"]
    CC100K = 0,
    #[doc = "1: 500kSPS"]
    CC1M = 1,
    #[doc = "2: 1MSPS"]
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
    #[doc = "100kSPS"]
    #[inline(always)]
    pub fn cc100k(self) -> &'a mut W {
        self.variant(CCTRL_A::CC100K)
    }
    #[doc = "500kSPS"]
    #[inline(always)]
    pub fn cc1m(self) -> &'a mut W {
        self.variant(CCTRL_A::CC1M)
    }
    #[doc = "1MSPS"]
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
#[doc = "Refresh period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFRESH_A {
    #[doc = "0: Do not Refresh"]
    REFRESH_0 = 0,
    #[doc = "1: Refresh every 30 us"]
    REFRESH_1 = 1,
    #[doc = "2: Refresh every 60 us"]
    REFRESH_2 = 2,
    #[doc = "3: Refresh every 90 us"]
    REFRESH_3 = 3,
    #[doc = "4: Refresh every 120 us"]
    REFRESH_4 = 4,
    #[doc = "5: Refresh every 150 us"]
    REFRESH_5 = 5,
    #[doc = "6: Refresh every 180 us"]
    REFRESH_6 = 6,
    #[doc = "7: Refresh every 210 us"]
    REFRESH_7 = 7,
    #[doc = "8: Refresh every 240 us"]
    REFRESH_8 = 8,
    #[doc = "9: Refresh every 270 us"]
    REFRESH_9 = 9,
    #[doc = "10: Refresh every 300 us"]
    REFRESH_10 = 10,
    #[doc = "11: Refresh every 330 us"]
    REFRESH_11 = 11,
    #[doc = "12: Refresh every 360 us"]
    REFRESH_12 = 12,
    #[doc = "13: Refresh every 390 us"]
    REFRESH_13 = 13,
    #[doc = "14: Refresh every 420 us"]
    REFRESH_14 = 14,
    #[doc = "15: Refresh every 450 us"]
    REFRESH_15 = 15,
}
impl From<REFRESH_A> for u8 {
    #[inline(always)]
    fn from(variant: REFRESH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REFRESH`"]
pub type REFRESH_R = crate::R<u8, REFRESH_A>;
impl REFRESH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFRESH_A {
        match self.bits {
            0 => REFRESH_A::REFRESH_0,
            1 => REFRESH_A::REFRESH_1,
            2 => REFRESH_A::REFRESH_2,
            3 => REFRESH_A::REFRESH_3,
            4 => REFRESH_A::REFRESH_4,
            5 => REFRESH_A::REFRESH_5,
            6 => REFRESH_A::REFRESH_6,
            7 => REFRESH_A::REFRESH_7,
            8 => REFRESH_A::REFRESH_8,
            9 => REFRESH_A::REFRESH_9,
            10 => REFRESH_A::REFRESH_10,
            11 => REFRESH_A::REFRESH_11,
            12 => REFRESH_A::REFRESH_12,
            13 => REFRESH_A::REFRESH_13,
            14 => REFRESH_A::REFRESH_14,
            15 => REFRESH_A::REFRESH_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REFRESH_0`"]
    #[inline(always)]
    pub fn is_refresh_0(&self) -> bool {
        *self == REFRESH_A::REFRESH_0
    }
    #[doc = "Checks if the value of the field is `REFRESH_1`"]
    #[inline(always)]
    pub fn is_refresh_1(&self) -> bool {
        *self == REFRESH_A::REFRESH_1
    }
    #[doc = "Checks if the value of the field is `REFRESH_2`"]
    #[inline(always)]
    pub fn is_refresh_2(&self) -> bool {
        *self == REFRESH_A::REFRESH_2
    }
    #[doc = "Checks if the value of the field is `REFRESH_3`"]
    #[inline(always)]
    pub fn is_refresh_3(&self) -> bool {
        *self == REFRESH_A::REFRESH_3
    }
    #[doc = "Checks if the value of the field is `REFRESH_4`"]
    #[inline(always)]
    pub fn is_refresh_4(&self) -> bool {
        *self == REFRESH_A::REFRESH_4
    }
    #[doc = "Checks if the value of the field is `REFRESH_5`"]
    #[inline(always)]
    pub fn is_refresh_5(&self) -> bool {
        *self == REFRESH_A::REFRESH_5
    }
    #[doc = "Checks if the value of the field is `REFRESH_6`"]
    #[inline(always)]
    pub fn is_refresh_6(&self) -> bool {
        *self == REFRESH_A::REFRESH_6
    }
    #[doc = "Checks if the value of the field is `REFRESH_7`"]
    #[inline(always)]
    pub fn is_refresh_7(&self) -> bool {
        *self == REFRESH_A::REFRESH_7
    }
    #[doc = "Checks if the value of the field is `REFRESH_8`"]
    #[inline(always)]
    pub fn is_refresh_8(&self) -> bool {
        *self == REFRESH_A::REFRESH_8
    }
    #[doc = "Checks if the value of the field is `REFRESH_9`"]
    #[inline(always)]
    pub fn is_refresh_9(&self) -> bool {
        *self == REFRESH_A::REFRESH_9
    }
    #[doc = "Checks if the value of the field is `REFRESH_10`"]
    #[inline(always)]
    pub fn is_refresh_10(&self) -> bool {
        *self == REFRESH_A::REFRESH_10
    }
    #[doc = "Checks if the value of the field is `REFRESH_11`"]
    #[inline(always)]
    pub fn is_refresh_11(&self) -> bool {
        *self == REFRESH_A::REFRESH_11
    }
    #[doc = "Checks if the value of the field is `REFRESH_12`"]
    #[inline(always)]
    pub fn is_refresh_12(&self) -> bool {
        *self == REFRESH_A::REFRESH_12
    }
    #[doc = "Checks if the value of the field is `REFRESH_13`"]
    #[inline(always)]
    pub fn is_refresh_13(&self) -> bool {
        *self == REFRESH_A::REFRESH_13
    }
    #[doc = "Checks if the value of the field is `REFRESH_14`"]
    #[inline(always)]
    pub fn is_refresh_14(&self) -> bool {
        *self == REFRESH_A::REFRESH_14
    }
    #[doc = "Checks if the value of the field is `REFRESH_15`"]
    #[inline(always)]
    pub fn is_refresh_15(&self) -> bool {
        *self == REFRESH_A::REFRESH_15
    }
}
#[doc = "Write proxy for field `REFRESH`"]
pub struct REFRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> REFRESH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFRESH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Do not Refresh"]
    #[inline(always)]
    pub fn refresh_0(self) -> &'a mut W {
        self.variant(REFRESH_A::REFRESH_0)
    }
    #[doc = "Refresh every 30 us"]
    #[inline(always)]
    pub fn refresh_1(self) -> &'a mut W {
        self.variant(REFRESH_A::REFRESH_1)
    }
    #[doc = "Refresh every 60 us"]
    #[inline(always)]
    pub fn refresh_2(self) -> &'a mut W {
        self.variant(REFRESH_A::REFRESH_2)
    }
    #[doc = "Refresh every 90 us"]
    #[inline(always)]
    pub fn refresh_3(self) -> &'a mut W {
        self.variant(REFRESH_A::REFRESH_3)
    }
    #[doc = "Refresh every 120 us"]
    #[inline(always)]
    pub fn refresh_4(self) -> &'a mut W {
        self.variant(REFRESH_A::REFRESH_4)
    }
    #[doc = "Refresh every 150 us"]
    #[inline(always)]
    pub fn refresh_5(self) -> &'a mut W {
        self.variant(REFRESH_A::REFRESH_5)
    }
    #[doc = "Refresh every 180 us"]
    #[inline(always)]
    pub fn refresh_6(self) -> &'a mut W {
        self.variant(REFRESH_A::REFRESH_6)
    }
    #[doc = "Refresh every 210 us"]
    #[inline(always)]
    pub fn refresh_7(self) -> &'a mut W {
        self.variant(REFRESH_A::REFRESH_7)
    }
    #[doc = "Refresh every 240 us"]
    #[inline(always)]
    pub fn refresh_8(self) -> &'a mut W {
        self.variant(REFRESH_A::REFRESH_8)
    }
    #[doc = "Refresh every 270 us"]
    #[inline(always)]
    pub fn refresh_9(self) -> &'a mut W {
        self.variant(REFRESH_A::REFRESH_9)
    }
    #[doc = "Refresh every 300 us"]
    #[inline(always)]
    pub fn refresh_10(self) -> &'a mut W {
        self.variant(REFRESH_A::REFRESH_10)
    }
    #[doc = "Refresh every 330 us"]
    #[inline(always)]
    pub fn refresh_11(self) -> &'a mut W {
        self.variant(REFRESH_A::REFRESH_11)
    }
    #[doc = "Refresh every 360 us"]
    #[inline(always)]
    pub fn refresh_12(self) -> &'a mut W {
        self.variant(REFRESH_A::REFRESH_12)
    }
    #[doc = "Refresh every 390 us"]
    #[inline(always)]
    pub fn refresh_13(self) -> &'a mut W {
        self.variant(REFRESH_A::REFRESH_13)
    }
    #[doc = "Refresh every 420 us"]
    #[inline(always)]
    pub fn refresh_14(self) -> &'a mut W {
        self.variant(REFRESH_A::REFRESH_14)
    }
    #[doc = "Refresh every 450 us"]
    #[inline(always)]
    pub fn refresh_15(self) -> &'a mut W {
        self.variant(REFRESH_A::REFRESH_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Sampling Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSR_A {
    #[doc = "0: No Over Sampling"]
    OSR_1 = 0,
    #[doc = "1: 2x Over Sampling Ratio"]
    OSR_2 = 1,
    #[doc = "2: 4x Over Sampling Ratio"]
    OSR_4 = 2,
    #[doc = "3: 8x Over Sampling Ratio"]
    OSR_8 = 3,
    #[doc = "4: 16x Over Sampling Ratio"]
    OSR_16 = 4,
    #[doc = "5: 32x Over Sampling Ratio"]
    OSR_32 = 5,
}
impl From<OSR_A> for u8 {
    #[inline(always)]
    fn from(variant: OSR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OSR`"]
pub type OSR_R = crate::R<u8, OSR_A>;
impl OSR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OSR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OSR_A::OSR_1),
            1 => Val(OSR_A::OSR_2),
            2 => Val(OSR_A::OSR_4),
            3 => Val(OSR_A::OSR_8),
            4 => Val(OSR_A::OSR_16),
            5 => Val(OSR_A::OSR_32),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OSR_1`"]
    #[inline(always)]
    pub fn is_osr_1(&self) -> bool {
        *self == OSR_A::OSR_1
    }
    #[doc = "Checks if the value of the field is `OSR_2`"]
    #[inline(always)]
    pub fn is_osr_2(&self) -> bool {
        *self == OSR_A::OSR_2
    }
    #[doc = "Checks if the value of the field is `OSR_4`"]
    #[inline(always)]
    pub fn is_osr_4(&self) -> bool {
        *self == OSR_A::OSR_4
    }
    #[doc = "Checks if the value of the field is `OSR_8`"]
    #[inline(always)]
    pub fn is_osr_8(&self) -> bool {
        *self == OSR_A::OSR_8
    }
    #[doc = "Checks if the value of the field is `OSR_16`"]
    #[inline(always)]
    pub fn is_osr_16(&self) -> bool {
        *self == OSR_A::OSR_16
    }
    #[doc = "Checks if the value of the field is `OSR_32`"]
    #[inline(always)]
    pub fn is_osr_32(&self) -> bool {
        *self == OSR_A::OSR_32
    }
}
#[doc = "Write proxy for field `OSR`"]
pub struct OSR_W<'a> {
    w: &'a mut W,
}
impl<'a> OSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No Over Sampling"]
    #[inline(always)]
    pub fn osr_1(self) -> &'a mut W {
        self.variant(OSR_A::OSR_1)
    }
    #[doc = "2x Over Sampling Ratio"]
    #[inline(always)]
    pub fn osr_2(self) -> &'a mut W {
        self.variant(OSR_A::OSR_2)
    }
    #[doc = "4x Over Sampling Ratio"]
    #[inline(always)]
    pub fn osr_4(self) -> &'a mut W {
        self.variant(OSR_A::OSR_4)
    }
    #[doc = "8x Over Sampling Ratio"]
    #[inline(always)]
    pub fn osr_8(self) -> &'a mut W {
        self.variant(OSR_A::OSR_8)
    }
    #[doc = "16x Over Sampling Ratio"]
    #[inline(always)]
    pub fn osr_16(self) -> &'a mut W {
        self.variant(OSR_A::OSR_16)
    }
    #[doc = "32x Over Sampling Ratio"]
    #[inline(always)]
    pub fn osr_32(self) -> &'a mut W {
        self.variant(OSR_A::OSR_32)
    }
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
