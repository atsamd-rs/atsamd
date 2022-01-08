#[doc = "Register `WAVE` reader"]
pub struct R(crate::R<WAVE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAVE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAVE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAVE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAVE` writer"]
pub struct W(crate::W<WAVE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAVE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<WAVE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAVE_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `WAVEGEN` reader - Waveform Generation"]
pub struct WAVEGEN_R(crate::FieldReader<u8, WAVEGEN_A>);
impl WAVEGEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WAVEGEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WAVEGEN_A> {
        match self.bits {
            0 => Some(WAVEGEN_A::NFRQ),
            1 => Some(WAVEGEN_A::MFRQ),
            2 => Some(WAVEGEN_A::NPWM),
            4 => Some(WAVEGEN_A::DSCRITICAL),
            5 => Some(WAVEGEN_A::DSBOTTOM),
            6 => Some(WAVEGEN_A::DSBOTH),
            7 => Some(WAVEGEN_A::DSTOP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NFRQ`"]
    #[inline(always)]
    pub fn is_nfrq(&self) -> bool {
        **self == WAVEGEN_A::NFRQ
    }
    #[doc = "Checks if the value of the field is `MFRQ`"]
    #[inline(always)]
    pub fn is_mfrq(&self) -> bool {
        **self == WAVEGEN_A::MFRQ
    }
    #[doc = "Checks if the value of the field is `NPWM`"]
    #[inline(always)]
    pub fn is_npwm(&self) -> bool {
        **self == WAVEGEN_A::NPWM
    }
    #[doc = "Checks if the value of the field is `DSCRITICAL`"]
    #[inline(always)]
    pub fn is_dscritical(&self) -> bool {
        **self == WAVEGEN_A::DSCRITICAL
    }
    #[doc = "Checks if the value of the field is `DSBOTTOM`"]
    #[inline(always)]
    pub fn is_dsbottom(&self) -> bool {
        **self == WAVEGEN_A::DSBOTTOM
    }
    #[doc = "Checks if the value of the field is `DSBOTH`"]
    #[inline(always)]
    pub fn is_dsboth(&self) -> bool {
        **self == WAVEGEN_A::DSBOTH
    }
    #[doc = "Checks if the value of the field is `DSTOP`"]
    #[inline(always)]
    pub fn is_dstop(&self) -> bool {
        **self == WAVEGEN_A::DSTOP
    }
}
impl core::ops::Deref for WAVEGEN_R {
    type Target = crate::FieldReader<u8, WAVEGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAVEGEN` writer - Waveform Generation"]
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
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
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
#[doc = "Field `RAMP` reader - Ramp Mode"]
pub struct RAMP_R(crate::FieldReader<u8, RAMP_A>);
impl RAMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RAMP_R(crate::FieldReader::new(bits))
    }
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
        **self == RAMP_A::RAMP1
    }
    #[doc = "Checks if the value of the field is `RAMP2A`"]
    #[inline(always)]
    pub fn is_ramp2a(&self) -> bool {
        **self == RAMP_A::RAMP2A
    }
    #[doc = "Checks if the value of the field is `RAMP2`"]
    #[inline(always)]
    pub fn is_ramp2(&self) -> bool {
        **self == RAMP_A::RAMP2
    }
    #[doc = "Checks if the value of the field is `RAMP2C`"]
    #[inline(always)]
    pub fn is_ramp2c(&self) -> bool {
        **self == RAMP_A::RAMP2C
    }
}
impl core::ops::Deref for RAMP_R {
    type Target = crate::FieldReader<u8, RAMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMP` writer - Ramp Mode"]
pub struct RAMP_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAMP_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `CIPEREN` reader - Circular period Enable"]
pub struct CIPEREN_R(crate::FieldReader<bool, bool>);
impl CIPEREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CIPEREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CIPEREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CIPEREN` writer - Circular period Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `CICCEN0` reader - Circular Channel 0 Enable"]
pub struct CICCEN0_R(crate::FieldReader<bool, bool>);
impl CICCEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CICCEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CICCEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CICCEN0` writer - Circular Channel 0 Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `CICCEN1` reader - Circular Channel 1 Enable"]
pub struct CICCEN1_R(crate::FieldReader<bool, bool>);
impl CICCEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CICCEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CICCEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CICCEN1` writer - Circular Channel 1 Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CICCEN2` reader - Circular Channel 2 Enable"]
pub struct CICCEN2_R(crate::FieldReader<bool, bool>);
impl CICCEN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CICCEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CICCEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CICCEN2` writer - Circular Channel 2 Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `CICCEN3` reader - Circular Channel 3 Enable"]
pub struct CICCEN3_R(crate::FieldReader<bool, bool>);
impl CICCEN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CICCEN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CICCEN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CICCEN3` writer - Circular Channel 3 Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `POL0` reader - Channel 0 Polarity"]
pub struct POL0_R(crate::FieldReader<bool, bool>);
impl POL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL0` writer - Channel 0 Polarity"]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `POL1` reader - Channel 1 Polarity"]
pub struct POL1_R(crate::FieldReader<bool, bool>);
impl POL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL1` writer - Channel 1 Polarity"]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `POL2` reader - Channel 2 Polarity"]
pub struct POL2_R(crate::FieldReader<bool, bool>);
impl POL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL2` writer - Channel 2 Polarity"]
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `POL3` reader - Channel 3 Polarity"]
pub struct POL3_R(crate::FieldReader<bool, bool>);
impl POL3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL3` writer - Channel 3 Polarity"]
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `POL4` reader - Channel 4 Polarity"]
pub struct POL4_R(crate::FieldReader<bool, bool>);
impl POL4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL4` writer - Channel 4 Polarity"]
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `POL5` reader - Channel 5 Polarity"]
pub struct POL5_R(crate::FieldReader<bool, bool>);
impl POL5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL5` writer - Channel 5 Polarity"]
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `SWAP0` reader - Swap DTI Output Pair 0"]
pub struct SWAP0_R(crate::FieldReader<bool, bool>);
impl SWAP0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWAP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWAP0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWAP0` writer - Swap DTI Output Pair 0"]
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `SWAP1` reader - Swap DTI Output Pair 1"]
pub struct SWAP1_R(crate::FieldReader<bool, bool>);
impl SWAP1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWAP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWAP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWAP1` writer - Swap DTI Output Pair 1"]
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `SWAP2` reader - Swap DTI Output Pair 2"]
pub struct SWAP2_R(crate::FieldReader<bool, bool>);
impl SWAP2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWAP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWAP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWAP2` writer - Swap DTI Output Pair 2"]
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `SWAP3` reader - Swap DTI Output Pair 3"]
pub struct SWAP3_R(crate::FieldReader<bool, bool>);
impl SWAP3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWAP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWAP3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWAP3` writer - Swap DTI Output Pair 3"]
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Waveform Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wave](index.html) module"]
pub struct WAVE_SPEC;
impl crate::RegisterSpec for WAVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wave::R](R) reader structure"]
impl crate::Readable for WAVE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wave::W](W) writer structure"]
impl crate::Writable for WAVE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAVE to value 0"]
impl crate::Resettable for WAVE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
