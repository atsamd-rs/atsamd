#[doc = "Register `WAVEB` reader"]
pub struct R(crate::R<WAVEB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAVEB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAVEB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAVEB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAVEB` writer"]
pub struct W(crate::W<WAVEB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAVEB_SPEC>;
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
impl From<crate::W<WAVEB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAVEB_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `WAVEGENB` reader - Waveform Generation Buffer"]
pub struct WAVEGENB_R(crate::FieldReader<u8, WAVEGENB_A>);
impl WAVEGENB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WAVEGENB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WAVEGENB_A> {
        match self.bits {
            0 => Some(WAVEGENB_A::NFRQ),
            1 => Some(WAVEGENB_A::MFRQ),
            2 => Some(WAVEGENB_A::NPWM),
            4 => Some(WAVEGENB_A::DSCRITICAL),
            5 => Some(WAVEGENB_A::DSBOTTOM),
            6 => Some(WAVEGENB_A::DSBOTH),
            7 => Some(WAVEGENB_A::DSTOP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NFRQ`"]
    #[inline(always)]
    pub fn is_nfrq(&self) -> bool {
        **self == WAVEGENB_A::NFRQ
    }
    #[doc = "Checks if the value of the field is `MFRQ`"]
    #[inline(always)]
    pub fn is_mfrq(&self) -> bool {
        **self == WAVEGENB_A::MFRQ
    }
    #[doc = "Checks if the value of the field is `NPWM`"]
    #[inline(always)]
    pub fn is_npwm(&self) -> bool {
        **self == WAVEGENB_A::NPWM
    }
    #[doc = "Checks if the value of the field is `DSCRITICAL`"]
    #[inline(always)]
    pub fn is_dscritical(&self) -> bool {
        **self == WAVEGENB_A::DSCRITICAL
    }
    #[doc = "Checks if the value of the field is `DSBOTTOM`"]
    #[inline(always)]
    pub fn is_dsbottom(&self) -> bool {
        **self == WAVEGENB_A::DSBOTTOM
    }
    #[doc = "Checks if the value of the field is `DSBOTH`"]
    #[inline(always)]
    pub fn is_dsboth(&self) -> bool {
        **self == WAVEGENB_A::DSBOTH
    }
    #[doc = "Checks if the value of the field is `DSTOP`"]
    #[inline(always)]
    pub fn is_dstop(&self) -> bool {
        **self == WAVEGENB_A::DSTOP
    }
}
impl core::ops::Deref for WAVEGENB_R {
    type Target = crate::FieldReader<u8, WAVEGENB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAVEGENB` writer - Waveform Generation Buffer"]
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
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
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
#[doc = "Field `RAMPB` reader - Ramp Mode Buffer"]
pub struct RAMPB_R(crate::FieldReader<u8, RAMPB_A>);
impl RAMPB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RAMPB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RAMPB_A> {
        match self.bits {
            0 => Some(RAMPB_A::RAMP1),
            1 => Some(RAMPB_A::RAMP2A),
            2 => Some(RAMPB_A::RAMP2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RAMP1`"]
    #[inline(always)]
    pub fn is_ramp1(&self) -> bool {
        **self == RAMPB_A::RAMP1
    }
    #[doc = "Checks if the value of the field is `RAMP2A`"]
    #[inline(always)]
    pub fn is_ramp2a(&self) -> bool {
        **self == RAMPB_A::RAMP2A
    }
    #[doc = "Checks if the value of the field is `RAMP2`"]
    #[inline(always)]
    pub fn is_ramp2(&self) -> bool {
        **self == RAMPB_A::RAMP2
    }
}
impl core::ops::Deref for RAMPB_R {
    type Target = crate::FieldReader<u8, RAMPB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMPB` writer - Ramp Mode Buffer"]
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
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `CIPERENB` reader - Circular Period Enable Buffer"]
pub struct CIPERENB_R(crate::FieldReader<bool, bool>);
impl CIPERENB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CIPERENB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CIPERENB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CIPERENB` writer - Circular Period Enable Buffer"]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `CICCENB0` reader - Circular Channel 0 Enable Buffer"]
pub struct CICCENB0_R(crate::FieldReader<bool, bool>);
impl CICCENB0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CICCENB0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CICCENB0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CICCENB0` writer - Circular Channel 0 Enable Buffer"]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `CICCENB1` reader - Circular Channel 1 Enable Buffer"]
pub struct CICCENB1_R(crate::FieldReader<bool, bool>);
impl CICCENB1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CICCENB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CICCENB1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CICCENB1` writer - Circular Channel 1 Enable Buffer"]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CICCENB2` reader - Circular Channel 2 Enable Buffer"]
pub struct CICCENB2_R(crate::FieldReader<bool, bool>);
impl CICCENB2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CICCENB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CICCENB2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CICCENB2` writer - Circular Channel 2 Enable Buffer"]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `CICCENB3` reader - Circular Channel 3 Enable Buffer"]
pub struct CICCENB3_R(crate::FieldReader<bool, bool>);
impl CICCENB3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CICCENB3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CICCENB3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CICCENB3` writer - Circular Channel 3 Enable Buffer"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `POLB0` reader - Channel 0 Polarity Buffer"]
pub struct POLB0_R(crate::FieldReader<bool, bool>);
impl POLB0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POLB0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POLB0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POLB0` writer - Channel 0 Polarity Buffer"]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `POLB1` reader - Channel 1 Polarity Buffer"]
pub struct POLB1_R(crate::FieldReader<bool, bool>);
impl POLB1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POLB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POLB1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POLB1` writer - Channel 1 Polarity Buffer"]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `POLB2` reader - Channel 2 Polarity Buffer"]
pub struct POLB2_R(crate::FieldReader<bool, bool>);
impl POLB2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POLB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POLB2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POLB2` writer - Channel 2 Polarity Buffer"]
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `POLB3` reader - Channel 3 Polarity Buffer"]
pub struct POLB3_R(crate::FieldReader<bool, bool>);
impl POLB3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POLB3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POLB3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POLB3` writer - Channel 3 Polarity Buffer"]
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `SWAPB0` reader - Swap DTI Output Pair 0 Buffer"]
pub struct SWAPB0_R(crate::FieldReader<bool, bool>);
impl SWAPB0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWAPB0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWAPB0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWAPB0` writer - Swap DTI Output Pair 0 Buffer"]
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `SWAPB1` reader - Swap DTI Output Pair 1 Buffer"]
pub struct SWAPB1_R(crate::FieldReader<bool, bool>);
impl SWAPB1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWAPB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWAPB1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWAPB1` writer - Swap DTI Output Pair 1 Buffer"]
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `SWAPB2` reader - Swap DTI Output Pair 2 Buffer"]
pub struct SWAPB2_R(crate::FieldReader<bool, bool>);
impl SWAPB2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWAPB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWAPB2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWAPB2` writer - Swap DTI Output Pair 2 Buffer"]
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `SWAPB3` reader - Swap DTI Output Pair 3 Buffer"]
pub struct SWAPB3_R(crate::FieldReader<bool, bool>);
impl SWAPB3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWAPB3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWAPB3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWAPB3` writer - Swap DTI Output Pair 3 Buffer"]
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Waveform Control Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [waveb](index.html) module"]
pub struct WAVEB_SPEC;
impl crate::RegisterSpec for WAVEB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [waveb::R](R) reader structure"]
impl crate::Readable for WAVEB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [waveb::W](W) writer structure"]
impl crate::Writable for WAVEB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAVEB to value 0"]
impl crate::Resettable for WAVEB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
