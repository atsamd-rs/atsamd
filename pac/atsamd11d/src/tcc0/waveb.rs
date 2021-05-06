#[doc = "Reader of register WAVEB"]
pub type R = crate::R<u32, super::WAVEB>;
#[doc = "Writer for register WAVEB"]
pub type W = crate::W<u32, super::WAVEB>;
#[doc = "Register WAVEB `reset()`'s with value 0"]
impl crate::ResetValue for super::WAVEB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Waveform Generation Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WAVEGENB_A {
    #[doc = "0: Normal frequency"]
    NFRQ = 0,
    #[doc = "1: Match frequency"]
    MFRQ = 1,
    #[doc = "2: Normal PWM"]
    NPWM = 2,
    #[doc = "4: Dual-slope critical"]
    DSCRITICAL = 4,
    #[doc = "5: Dual-slope with interrupt/event condition when COUNT reaches ZERO"]
    DSBOTTOM = 5,
    #[doc = "6: Dual-slope with interrupt/event condition when COUNT reaches ZERO or TOP"]
    DSBOTH = 6,
    #[doc = "7: Dual-slope with interrupt/event condition when COUNT reaches TOP"]
    DSTOP = 7,
}
impl From<WAVEGENB_A> for u8 {
    #[inline(always)]
    fn from(variant: WAVEGENB_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WAVEGENB`"]
pub type WAVEGENB_R = crate::R<u8, WAVEGENB_A>;
impl WAVEGENB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WAVEGENB_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WAVEGENB_A::NFRQ),
            1 => Val(WAVEGENB_A::MFRQ),
            2 => Val(WAVEGENB_A::NPWM),
            4 => Val(WAVEGENB_A::DSCRITICAL),
            5 => Val(WAVEGENB_A::DSBOTTOM),
            6 => Val(WAVEGENB_A::DSBOTH),
            7 => Val(WAVEGENB_A::DSTOP),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NFRQ`"]
    #[inline(always)]
    pub fn is_nfrq(&self) -> bool {
        *self == WAVEGENB_A::NFRQ
    }
    #[doc = "Checks if the value of the field is `MFRQ`"]
    #[inline(always)]
    pub fn is_mfrq(&self) -> bool {
        *self == WAVEGENB_A::MFRQ
    }
    #[doc = "Checks if the value of the field is `NPWM`"]
    #[inline(always)]
    pub fn is_npwm(&self) -> bool {
        *self == WAVEGENB_A::NPWM
    }
    #[doc = "Checks if the value of the field is `DSCRITICAL`"]
    #[inline(always)]
    pub fn is_dscritical(&self) -> bool {
        *self == WAVEGENB_A::DSCRITICAL
    }
    #[doc = "Checks if the value of the field is `DSBOTTOM`"]
    #[inline(always)]
    pub fn is_dsbottom(&self) -> bool {
        *self == WAVEGENB_A::DSBOTTOM
    }
    #[doc = "Checks if the value of the field is `DSBOTH`"]
    #[inline(always)]
    pub fn is_dsboth(&self) -> bool {
        *self == WAVEGENB_A::DSBOTH
    }
    #[doc = "Checks if the value of the field is `DSTOP`"]
    #[inline(always)]
    pub fn is_dstop(&self) -> bool {
        *self == WAVEGENB_A::DSTOP
    }
}
#[doc = "Write proxy for field `WAVEGENB`"]
pub struct WAVEGENB_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVEGENB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAVEGENB_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Normal frequency"]
    #[inline(always)]
    pub fn nfrq(self) -> &'a mut W {
        self.variant(WAVEGENB_A::NFRQ)
    }
    #[doc = "Match frequency"]
    #[inline(always)]
    pub fn mfrq(self) -> &'a mut W {
        self.variant(WAVEGENB_A::MFRQ)
    }
    #[doc = "Normal PWM"]
    #[inline(always)]
    pub fn npwm(self) -> &'a mut W {
        self.variant(WAVEGENB_A::NPWM)
    }
    #[doc = "Dual-slope critical"]
    #[inline(always)]
    pub fn dscritical(self) -> &'a mut W {
        self.variant(WAVEGENB_A::DSCRITICAL)
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO"]
    #[inline(always)]
    pub fn dsbottom(self) -> &'a mut W {
        self.variant(WAVEGENB_A::DSBOTTOM)
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO or TOP"]
    #[inline(always)]
    pub fn dsboth(self) -> &'a mut W {
        self.variant(WAVEGENB_A::DSBOTH)
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches TOP"]
    #[inline(always)]
    pub fn dstop(self) -> &'a mut W {
        self.variant(WAVEGENB_A::DSTOP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Ramp Mode Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RAMPB_A {
    #[doc = "0: RAMP1 operation"]
    RAMP1 = 0,
    #[doc = "1: Alternative RAMP2 operation"]
    RAMP2A = 1,
    #[doc = "2: RAMP2 operation"]
    RAMP2 = 2,
}
impl From<RAMPB_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMPB_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RAMPB`"]
pub type RAMPB_R = crate::R<u8, RAMPB_A>;
impl RAMPB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RAMPB_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RAMPB_A::RAMP1),
            1 => Val(RAMPB_A::RAMP2A),
            2 => Val(RAMPB_A::RAMP2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RAMP1`"]
    #[inline(always)]
    pub fn is_ramp1(&self) -> bool {
        *self == RAMPB_A::RAMP1
    }
    #[doc = "Checks if the value of the field is `RAMP2A`"]
    #[inline(always)]
    pub fn is_ramp2a(&self) -> bool {
        *self == RAMPB_A::RAMP2A
    }
    #[doc = "Checks if the value of the field is `RAMP2`"]
    #[inline(always)]
    pub fn is_ramp2(&self) -> bool {
        *self == RAMPB_A::RAMP2
    }
}
#[doc = "Write proxy for field `RAMPB`"]
pub struct RAMPB_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMPB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAMPB_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "RAMP1 operation"]
    #[inline(always)]
    pub fn ramp1(self) -> &'a mut W {
        self.variant(RAMPB_A::RAMP1)
    }
    #[doc = "Alternative RAMP2 operation"]
    #[inline(always)]
    pub fn ramp2a(self) -> &'a mut W {
        self.variant(RAMPB_A::RAMP2A)
    }
    #[doc = "RAMP2 operation"]
    #[inline(always)]
    pub fn ramp2(self) -> &'a mut W {
        self.variant(RAMPB_A::RAMP2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `CIPERENB`"]
pub type CIPERENB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CIPERENB`"]
pub struct CIPERENB_W<'a> {
    w: &'a mut W,
}
impl<'a> CIPERENB_W<'a> {
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
#[doc = "Reader of field `CICCENB0`"]
pub type CICCENB0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CICCENB0`"]
pub struct CICCENB0_W<'a> {
    w: &'a mut W,
}
impl<'a> CICCENB0_W<'a> {
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
#[doc = "Reader of field `CICCENB1`"]
pub type CICCENB1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CICCENB1`"]
pub struct CICCENB1_W<'a> {
    w: &'a mut W,
}
impl<'a> CICCENB1_W<'a> {
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
#[doc = "Reader of field `CICCENB2`"]
pub type CICCENB2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CICCENB2`"]
pub struct CICCENB2_W<'a> {
    w: &'a mut W,
}
impl<'a> CICCENB2_W<'a> {
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
#[doc = "Reader of field `CICCENB3`"]
pub type CICCENB3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CICCENB3`"]
pub struct CICCENB3_W<'a> {
    w: &'a mut W,
}
impl<'a> CICCENB3_W<'a> {
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
#[doc = "Reader of field `POLB0`"]
pub type POLB0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POLB0`"]
pub struct POLB0_W<'a> {
    w: &'a mut W,
}
impl<'a> POLB0_W<'a> {
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
#[doc = "Reader of field `POLB1`"]
pub type POLB1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POLB1`"]
pub struct POLB1_W<'a> {
    w: &'a mut W,
}
impl<'a> POLB1_W<'a> {
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
#[doc = "Reader of field `POLB2`"]
pub type POLB2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POLB2`"]
pub struct POLB2_W<'a> {
    w: &'a mut W,
}
impl<'a> POLB2_W<'a> {
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
#[doc = "Reader of field `POLB3`"]
pub type POLB3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POLB3`"]
pub struct POLB3_W<'a> {
    w: &'a mut W,
}
impl<'a> POLB3_W<'a> {
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
#[doc = "Reader of field `SWAPB0`"]
pub type SWAPB0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWAPB0`"]
pub struct SWAPB0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAPB0_W<'a> {
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
#[doc = "Reader of field `SWAPB1`"]
pub type SWAPB1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWAPB1`"]
pub struct SWAPB1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAPB1_W<'a> {
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
#[doc = "Reader of field `SWAPB2`"]
pub type SWAPB2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWAPB2`"]
pub struct SWAPB2_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAPB2_W<'a> {
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
#[doc = "Reader of field `SWAPB3`"]
pub type SWAPB3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWAPB3`"]
pub struct SWAPB3_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAPB3_W<'a> {
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
    #[doc = "Bits 0:2 - Waveform Generation Buffer"]
    #[inline(always)]
    pub fn wavegenb(&self) -> WAVEGENB_R {
        WAVEGENB_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:5 - Ramp Mode Buffer"]
    #[inline(always)]
    pub fn rampb(&self) -> RAMPB_R {
        RAMPB_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Circular Period Enable Buffer"]
    #[inline(always)]
    pub fn ciperenb(&self) -> CIPERENB_R {
        CIPERENB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Circular Channel 0 Enable Buffer"]
    #[inline(always)]
    pub fn ciccenb0(&self) -> CICCENB0_R {
        CICCENB0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Circular Channel 1 Enable Buffer"]
    #[inline(always)]
    pub fn ciccenb1(&self) -> CICCENB1_R {
        CICCENB1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Circular Channel 2 Enable Buffer"]
    #[inline(always)]
    pub fn ciccenb2(&self) -> CICCENB2_R {
        CICCENB2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Circular Channel 3 Enable Buffer"]
    #[inline(always)]
    pub fn ciccenb3(&self) -> CICCENB3_R {
        CICCENB3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel 0 Polarity Buffer"]
    #[inline(always)]
    pub fn polb0(&self) -> POLB0_R {
        POLB0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel 1 Polarity Buffer"]
    #[inline(always)]
    pub fn polb1(&self) -> POLB1_R {
        POLB1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel 2 Polarity Buffer"]
    #[inline(always)]
    pub fn polb2(&self) -> POLB2_R {
        POLB2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel 3 Polarity Buffer"]
    #[inline(always)]
    pub fn polb3(&self) -> POLB3_R {
        POLB3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Swap DTI Output Pair 0 Buffer"]
    #[inline(always)]
    pub fn swapb0(&self) -> SWAPB0_R {
        SWAPB0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Swap DTI Output Pair 1 Buffer"]
    #[inline(always)]
    pub fn swapb1(&self) -> SWAPB1_R {
        SWAPB1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Swap DTI Output Pair 2 Buffer"]
    #[inline(always)]
    pub fn swapb2(&self) -> SWAPB2_R {
        SWAPB2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Swap DTI Output Pair 3 Buffer"]
    #[inline(always)]
    pub fn swapb3(&self) -> SWAPB3_R {
        SWAPB3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Waveform Generation Buffer"]
    #[inline(always)]
    pub fn wavegenb(&mut self) -> WAVEGENB_W {
        WAVEGENB_W { w: self }
    }
    #[doc = "Bits 4:5 - Ramp Mode Buffer"]
    #[inline(always)]
    pub fn rampb(&mut self) -> RAMPB_W {
        RAMPB_W { w: self }
    }
    #[doc = "Bit 7 - Circular Period Enable Buffer"]
    #[inline(always)]
    pub fn ciperenb(&mut self) -> CIPERENB_W {
        CIPERENB_W { w: self }
    }
    #[doc = "Bit 8 - Circular Channel 0 Enable Buffer"]
    #[inline(always)]
    pub fn ciccenb0(&mut self) -> CICCENB0_W {
        CICCENB0_W { w: self }
    }
    #[doc = "Bit 9 - Circular Channel 1 Enable Buffer"]
    #[inline(always)]
    pub fn ciccenb1(&mut self) -> CICCENB1_W {
        CICCENB1_W { w: self }
    }
    #[doc = "Bit 10 - Circular Channel 2 Enable Buffer"]
    #[inline(always)]
    pub fn ciccenb2(&mut self) -> CICCENB2_W {
        CICCENB2_W { w: self }
    }
    #[doc = "Bit 11 - Circular Channel 3 Enable Buffer"]
    #[inline(always)]
    pub fn ciccenb3(&mut self) -> CICCENB3_W {
        CICCENB3_W { w: self }
    }
    #[doc = "Bit 16 - Channel 0 Polarity Buffer"]
    #[inline(always)]
    pub fn polb0(&mut self) -> POLB0_W {
        POLB0_W { w: self }
    }
    #[doc = "Bit 17 - Channel 1 Polarity Buffer"]
    #[inline(always)]
    pub fn polb1(&mut self) -> POLB1_W {
        POLB1_W { w: self }
    }
    #[doc = "Bit 18 - Channel 2 Polarity Buffer"]
    #[inline(always)]
    pub fn polb2(&mut self) -> POLB2_W {
        POLB2_W { w: self }
    }
    #[doc = "Bit 19 - Channel 3 Polarity Buffer"]
    #[inline(always)]
    pub fn polb3(&mut self) -> POLB3_W {
        POLB3_W { w: self }
    }
    #[doc = "Bit 24 - Swap DTI Output Pair 0 Buffer"]
    #[inline(always)]
    pub fn swapb0(&mut self) -> SWAPB0_W {
        SWAPB0_W { w: self }
    }
    #[doc = "Bit 25 - Swap DTI Output Pair 1 Buffer"]
    #[inline(always)]
    pub fn swapb1(&mut self) -> SWAPB1_W {
        SWAPB1_W { w: self }
    }
    #[doc = "Bit 26 - Swap DTI Output Pair 2 Buffer"]
    #[inline(always)]
    pub fn swapb2(&mut self) -> SWAPB2_W {
        SWAPB2_W { w: self }
    }
    #[doc = "Bit 27 - Swap DTI Output Pair 3 Buffer"]
    #[inline(always)]
    pub fn swapb3(&mut self) -> SWAPB3_W {
        SWAPB3_W { w: self }
    }
}
