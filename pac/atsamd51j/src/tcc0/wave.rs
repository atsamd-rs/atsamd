#[doc = "Reader of register WAVE"]
pub type R = crate::R<u32, super::WAVE>;
#[doc = "Writer for register WAVE"]
pub type W = crate::W<u32, super::WAVE>;
#[doc = "Register WAVE `reset()`'s with value 0"]
impl crate::ResetValue for super::WAVE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Waveform Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WAVEGEN_A {
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
impl From<WAVEGEN_A> for u8 {
    #[inline(always)]
    fn from(variant: WAVEGEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WAVEGEN`"]
pub type WAVEGEN_R = crate::R<u8, WAVEGEN_A>;
impl WAVEGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WAVEGEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WAVEGEN_A::NFRQ),
            1 => Val(WAVEGEN_A::MFRQ),
            2 => Val(WAVEGEN_A::NPWM),
            4 => Val(WAVEGEN_A::DSCRITICAL),
            5 => Val(WAVEGEN_A::DSBOTTOM),
            6 => Val(WAVEGEN_A::DSBOTH),
            7 => Val(WAVEGEN_A::DSTOP),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NFRQ`"]
    #[inline(always)]
    pub fn is_nfrq(&self) -> bool {
        *self == WAVEGEN_A::NFRQ
    }
    #[doc = "Checks if the value of the field is `MFRQ`"]
    #[inline(always)]
    pub fn is_mfrq(&self) -> bool {
        *self == WAVEGEN_A::MFRQ
    }
    #[doc = "Checks if the value of the field is `NPWM`"]
    #[inline(always)]
    pub fn is_npwm(&self) -> bool {
        *self == WAVEGEN_A::NPWM
    }
    #[doc = "Checks if the value of the field is `DSCRITICAL`"]
    #[inline(always)]
    pub fn is_dscritical(&self) -> bool {
        *self == WAVEGEN_A::DSCRITICAL
    }
    #[doc = "Checks if the value of the field is `DSBOTTOM`"]
    #[inline(always)]
    pub fn is_dsbottom(&self) -> bool {
        *self == WAVEGEN_A::DSBOTTOM
    }
    #[doc = "Checks if the value of the field is `DSBOTH`"]
    #[inline(always)]
    pub fn is_dsboth(&self) -> bool {
        *self == WAVEGEN_A::DSBOTH
    }
    #[doc = "Checks if the value of the field is `DSTOP`"]
    #[inline(always)]
    pub fn is_dstop(&self) -> bool {
        *self == WAVEGEN_A::DSTOP
    }
}
#[doc = "Write proxy for field `WAVEGEN`"]
pub struct WAVEGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVEGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAVEGEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Normal frequency"]
    #[inline(always)]
    pub fn nfrq(self) -> &'a mut W {
        self.variant(WAVEGEN_A::NFRQ)
    }
    #[doc = "Match frequency"]
    #[inline(always)]
    pub fn mfrq(self) -> &'a mut W {
        self.variant(WAVEGEN_A::MFRQ)
    }
    #[doc = "Normal PWM"]
    #[inline(always)]
    pub fn npwm(self) -> &'a mut W {
        self.variant(WAVEGEN_A::NPWM)
    }
    #[doc = "Dual-slope critical"]
    #[inline(always)]
    pub fn dscritical(self) -> &'a mut W {
        self.variant(WAVEGEN_A::DSCRITICAL)
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO"]
    #[inline(always)]
    pub fn dsbottom(self) -> &'a mut W {
        self.variant(WAVEGEN_A::DSBOTTOM)
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO or TOP"]
    #[inline(always)]
    pub fn dsboth(self) -> &'a mut W {
        self.variant(WAVEGEN_A::DSBOTH)
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches TOP"]
    #[inline(always)]
    pub fn dstop(self) -> &'a mut W {
        self.variant(WAVEGEN_A::DSTOP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Ramp Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RAMP_A {
    #[doc = "0: RAMP1 operation"]
    RAMP1 = 0,
    #[doc = "1: Alternative RAMP2 operation"]
    RAMP2A = 1,
    #[doc = "2: RAMP2 operation"]
    RAMP2 = 2,
    #[doc = "3: Critical RAMP2 operation"]
    RAMP2C = 3,
}
impl From<RAMP_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RAMP`"]
pub type RAMP_R = crate::R<u8, RAMP_A>;
impl RAMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMP_A {
        match self.bits {
            0 => RAMP_A::RAMP1,
            1 => RAMP_A::RAMP2A,
            2 => RAMP_A::RAMP2,
            3 => RAMP_A::RAMP2C,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RAMP1`"]
    #[inline(always)]
    pub fn is_ramp1(&self) -> bool {
        *self == RAMP_A::RAMP1
    }
    #[doc = "Checks if the value of the field is `RAMP2A`"]
    #[inline(always)]
    pub fn is_ramp2a(&self) -> bool {
        *self == RAMP_A::RAMP2A
    }
    #[doc = "Checks if the value of the field is `RAMP2`"]
    #[inline(always)]
    pub fn is_ramp2(&self) -> bool {
        *self == RAMP_A::RAMP2
    }
    #[doc = "Checks if the value of the field is `RAMP2C`"]
    #[inline(always)]
    pub fn is_ramp2c(&self) -> bool {
        *self == RAMP_A::RAMP2C
    }
}
#[doc = "Write proxy for field `RAMP`"]
pub struct RAMP_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAMP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "RAMP1 operation"]
    #[inline(always)]
    pub fn ramp1(self) -> &'a mut W {
        self.variant(RAMP_A::RAMP1)
    }
    #[doc = "Alternative RAMP2 operation"]
    #[inline(always)]
    pub fn ramp2a(self) -> &'a mut W {
        self.variant(RAMP_A::RAMP2A)
    }
    #[doc = "RAMP2 operation"]
    #[inline(always)]
    pub fn ramp2(self) -> &'a mut W {
        self.variant(RAMP_A::RAMP2)
    }
    #[doc = "Critical RAMP2 operation"]
    #[inline(always)]
    pub fn ramp2c(self) -> &'a mut W {
        self.variant(RAMP_A::RAMP2C)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `CIPEREN`"]
pub type CIPEREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CIPEREN`"]
pub struct CIPEREN_W<'a> {
    w: &'a mut W,
}
impl<'a> CIPEREN_W<'a> {
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
#[doc = "Reader of field `CICCEN0`"]
pub type CICCEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CICCEN0`"]
pub struct CICCEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CICCEN0_W<'a> {
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
#[doc = "Reader of field `CICCEN1`"]
pub type CICCEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CICCEN1`"]
pub struct CICCEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CICCEN1_W<'a> {
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
#[doc = "Reader of field `CICCEN2`"]
pub type CICCEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CICCEN2`"]
pub struct CICCEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> CICCEN2_W<'a> {
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
#[doc = "Reader of field `CICCEN3`"]
pub type CICCEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CICCEN3`"]
pub struct CICCEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> CICCEN3_W<'a> {
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
#[doc = "Reader of field `POL0`"]
pub type POL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POL0`"]
pub struct POL0_W<'a> {
    w: &'a mut W,
}
impl<'a> POL0_W<'a> {
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
#[doc = "Reader of field `POL1`"]
pub type POL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POL1`"]
pub struct POL1_W<'a> {
    w: &'a mut W,
}
impl<'a> POL1_W<'a> {
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
#[doc = "Reader of field `POL2`"]
pub type POL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POL2`"]
pub struct POL2_W<'a> {
    w: &'a mut W,
}
impl<'a> POL2_W<'a> {
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
#[doc = "Reader of field `POL3`"]
pub type POL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POL3`"]
pub struct POL3_W<'a> {
    w: &'a mut W,
}
impl<'a> POL3_W<'a> {
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
#[doc = "Reader of field `POL4`"]
pub type POL4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POL4`"]
pub struct POL4_W<'a> {
    w: &'a mut W,
}
impl<'a> POL4_W<'a> {
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
#[doc = "Reader of field `POL5`"]
pub type POL5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POL5`"]
pub struct POL5_W<'a> {
    w: &'a mut W,
}
impl<'a> POL5_W<'a> {
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
#[doc = "Reader of field `SWAP0`"]
pub type SWAP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWAP0`"]
pub struct SWAP0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAP0_W<'a> {
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
#[doc = "Reader of field `SWAP1`"]
pub type SWAP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWAP1`"]
pub struct SWAP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAP1_W<'a> {
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
#[doc = "Reader of field `SWAP2`"]
pub type SWAP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWAP2`"]
pub struct SWAP2_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAP2_W<'a> {
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
#[doc = "Reader of field `SWAP3`"]
pub type SWAP3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWAP3`"]
pub struct SWAP3_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAP3_W<'a> {
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
    #[doc = "Bits 0:2 - Waveform Generation"]
    #[inline(always)]
    pub fn wavegen(&self) -> WAVEGEN_R {
        WAVEGEN_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:5 - Ramp Mode"]
    #[inline(always)]
    pub fn ramp(&self) -> RAMP_R {
        RAMP_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Circular period Enable"]
    #[inline(always)]
    pub fn ciperen(&self) -> CIPEREN_R {
        CIPEREN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Circular Channel 0 Enable"]
    #[inline(always)]
    pub fn ciccen0(&self) -> CICCEN0_R {
        CICCEN0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Circular Channel 1 Enable"]
    #[inline(always)]
    pub fn ciccen1(&self) -> CICCEN1_R {
        CICCEN1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Circular Channel 2 Enable"]
    #[inline(always)]
    pub fn ciccen2(&self) -> CICCEN2_R {
        CICCEN2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Circular Channel 3 Enable"]
    #[inline(always)]
    pub fn ciccen3(&self) -> CICCEN3_R {
        CICCEN3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel 0 Polarity"]
    #[inline(always)]
    pub fn pol0(&self) -> POL0_R {
        POL0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel 1 Polarity"]
    #[inline(always)]
    pub fn pol1(&self) -> POL1_R {
        POL1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel 2 Polarity"]
    #[inline(always)]
    pub fn pol2(&self) -> POL2_R {
        POL2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel 3 Polarity"]
    #[inline(always)]
    pub fn pol3(&self) -> POL3_R {
        POL3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Channel 4 Polarity"]
    #[inline(always)]
    pub fn pol4(&self) -> POL4_R {
        POL4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Channel 5 Polarity"]
    #[inline(always)]
    pub fn pol5(&self) -> POL5_R {
        POL5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Swap DTI Output Pair 0"]
    #[inline(always)]
    pub fn swap0(&self) -> SWAP0_R {
        SWAP0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Swap DTI Output Pair 1"]
    #[inline(always)]
    pub fn swap1(&self) -> SWAP1_R {
        SWAP1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Swap DTI Output Pair 2"]
    #[inline(always)]
    pub fn swap2(&self) -> SWAP2_R {
        SWAP2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Swap DTI Output Pair 3"]
    #[inline(always)]
    pub fn swap3(&self) -> SWAP3_R {
        SWAP3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Waveform Generation"]
    #[inline(always)]
    pub fn wavegen(&mut self) -> WAVEGEN_W {
        WAVEGEN_W { w: self }
    }
    #[doc = "Bits 4:5 - Ramp Mode"]
    #[inline(always)]
    pub fn ramp(&mut self) -> RAMP_W {
        RAMP_W { w: self }
    }
    #[doc = "Bit 7 - Circular period Enable"]
    #[inline(always)]
    pub fn ciperen(&mut self) -> CIPEREN_W {
        CIPEREN_W { w: self }
    }
    #[doc = "Bit 8 - Circular Channel 0 Enable"]
    #[inline(always)]
    pub fn ciccen0(&mut self) -> CICCEN0_W {
        CICCEN0_W { w: self }
    }
    #[doc = "Bit 9 - Circular Channel 1 Enable"]
    #[inline(always)]
    pub fn ciccen1(&mut self) -> CICCEN1_W {
        CICCEN1_W { w: self }
    }
    #[doc = "Bit 10 - Circular Channel 2 Enable"]
    #[inline(always)]
    pub fn ciccen2(&mut self) -> CICCEN2_W {
        CICCEN2_W { w: self }
    }
    #[doc = "Bit 11 - Circular Channel 3 Enable"]
    #[inline(always)]
    pub fn ciccen3(&mut self) -> CICCEN3_W {
        CICCEN3_W { w: self }
    }
    #[doc = "Bit 16 - Channel 0 Polarity"]
    #[inline(always)]
    pub fn pol0(&mut self) -> POL0_W {
        POL0_W { w: self }
    }
    #[doc = "Bit 17 - Channel 1 Polarity"]
    #[inline(always)]
    pub fn pol1(&mut self) -> POL1_W {
        POL1_W { w: self }
    }
    #[doc = "Bit 18 - Channel 2 Polarity"]
    #[inline(always)]
    pub fn pol2(&mut self) -> POL2_W {
        POL2_W { w: self }
    }
    #[doc = "Bit 19 - Channel 3 Polarity"]
    #[inline(always)]
    pub fn pol3(&mut self) -> POL3_W {
        POL3_W { w: self }
    }
    #[doc = "Bit 20 - Channel 4 Polarity"]
    #[inline(always)]
    pub fn pol4(&mut self) -> POL4_W {
        POL4_W { w: self }
    }
    #[doc = "Bit 21 - Channel 5 Polarity"]
    #[inline(always)]
    pub fn pol5(&mut self) -> POL5_W {
        POL5_W { w: self }
    }
    #[doc = "Bit 24 - Swap DTI Output Pair 0"]
    #[inline(always)]
    pub fn swap0(&mut self) -> SWAP0_W {
        SWAP0_W { w: self }
    }
    #[doc = "Bit 25 - Swap DTI Output Pair 1"]
    #[inline(always)]
    pub fn swap1(&mut self) -> SWAP1_W {
        SWAP1_W { w: self }
    }
    #[doc = "Bit 26 - Swap DTI Output Pair 2"]
    #[inline(always)]
    pub fn swap2(&mut self) -> SWAP2_W {
        SWAP2_W { w: self }
    }
    #[doc = "Bit 27 - Swap DTI Output Pair 3"]
    #[inline(always)]
    pub fn swap3(&mut self) -> SWAP3_W {
        SWAP3_W { w: self }
    }
}
