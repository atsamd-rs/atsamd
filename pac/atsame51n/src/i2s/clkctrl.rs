#[doc = "Register `CLKCTRL[%s]` reader"]
pub struct R(crate::R<CLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKCTRL[%s]` writer"]
pub struct W(crate::W<CLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKCTRL_SPEC>;
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
impl From<crate::W<CLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Slot Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SLOTSIZE_A {
    #[doc = "0: 8-bit Slot for Clock Unit n"]
    _8 = 0,
    #[doc = "1: 16-bit Slot for Clock Unit n"]
    _16 = 1,
    #[doc = "2: 24-bit Slot for Clock Unit n"]
    _24 = 2,
    #[doc = "3: 32-bit Slot for Clock Unit n"]
    _32 = 3,
}
impl From<SLOTSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SLOTSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SLOTSIZE` reader - Slot Size"]
pub struct SLOTSIZE_R(crate::FieldReader<u8, SLOTSIZE_A>);
impl SLOTSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLOTSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLOTSIZE_A {
        match self.bits {
            0 => SLOTSIZE_A::_8,
            1 => SLOTSIZE_A::_16,
            2 => SLOTSIZE_A::_24,
            3 => SLOTSIZE_A::_32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        **self == SLOTSIZE_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        **self == SLOTSIZE_A::_16
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        **self == SLOTSIZE_A::_24
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        **self == SLOTSIZE_A::_32
    }
}
impl core::ops::Deref for SLOTSIZE_R {
    type Target = crate::FieldReader<u8, SLOTSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOTSIZE` writer - Slot Size"]
pub struct SLOTSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOTSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLOTSIZE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "8-bit Slot for Clock Unit n"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(SLOTSIZE_A::_8)
    }
    #[doc = "16-bit Slot for Clock Unit n"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(SLOTSIZE_A::_16)
    }
    #[doc = "24-bit Slot for Clock Unit n"]
    #[inline(always)]
    pub fn _24(self) -> &'a mut W {
        self.variant(SLOTSIZE_A::_24)
    }
    #[doc = "32-bit Slot for Clock Unit n"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(SLOTSIZE_A::_32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `NBSLOTS` reader - Number of Slots in Frame"]
pub struct NBSLOTS_R(crate::FieldReader<u8, u8>);
impl NBSLOTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NBSLOTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBSLOTS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBSLOTS` writer - Number of Slots in Frame"]
pub struct NBSLOTS_W<'a> {
    w: &'a mut W,
}
impl<'a> NBSLOTS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
#[doc = "Frame Sync Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FSWIDTH_A {
    #[doc = "0: Frame Sync Pulse is 1 Slot wide (default for I2S protocol)"]
    SLOT = 0,
    #[doc = "1: Frame Sync Pulse is half a Frame wide"]
    HALF = 1,
    #[doc = "2: Frame Sync Pulse is 1 Bit wide"]
    BIT = 2,
    #[doc = "3: Clock Unit n operates in Burst mode, with a 1-bit wide Frame Sync pulse per Data sample, only when Data transfer is requested"]
    BURST = 3,
}
impl From<FSWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: FSWIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FSWIDTH` reader - Frame Sync Width"]
pub struct FSWIDTH_R(crate::FieldReader<u8, FSWIDTH_A>);
impl FSWIDTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FSWIDTH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSWIDTH_A {
        match self.bits {
            0 => FSWIDTH_A::SLOT,
            1 => FSWIDTH_A::HALF,
            2 => FSWIDTH_A::BIT,
            3 => FSWIDTH_A::BURST,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SLOT`"]
    #[inline(always)]
    pub fn is_slot(&self) -> bool {
        **self == FSWIDTH_A::SLOT
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        **self == FSWIDTH_A::HALF
    }
    #[doc = "Checks if the value of the field is `BIT`"]
    #[inline(always)]
    pub fn is_bit_(&self) -> bool {
        **self == FSWIDTH_A::BIT
    }
    #[doc = "Checks if the value of the field is `BURST`"]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        **self == FSWIDTH_A::BURST
    }
}
impl core::ops::Deref for FSWIDTH_R {
    type Target = crate::FieldReader<u8, FSWIDTH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSWIDTH` writer - Frame Sync Width"]
pub struct FSWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> FSWIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSWIDTH_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Frame Sync Pulse is 1 Slot wide (default for I2S protocol)"]
    #[inline(always)]
    pub fn slot(self) -> &'a mut W {
        self.variant(FSWIDTH_A::SLOT)
    }
    #[doc = "Frame Sync Pulse is half a Frame wide"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(FSWIDTH_A::HALF)
    }
    #[doc = "Frame Sync Pulse is 1 Bit wide"]
    #[inline(always)]
    pub fn bit_(self) -> &'a mut W {
        self.variant(FSWIDTH_A::BIT)
    }
    #[doc = "Clock Unit n operates in Burst mode, with a 1-bit wide Frame Sync pulse per Data sample, only when Data transfer is requested"]
    #[inline(always)]
    pub fn burst(self) -> &'a mut W {
        self.variant(FSWIDTH_A::BURST)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Data Delay from Frame Sync\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BITDELAY_A {
    #[doc = "0: Left Justified (0 Bit Delay)"]
    LJ = 0,
    #[doc = "1: I2S (1 Bit Delay)"]
    I2S = 1,
}
impl From<BITDELAY_A> for bool {
    #[inline(always)]
    fn from(variant: BITDELAY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BITDELAY` reader - Data Delay from Frame Sync"]
pub struct BITDELAY_R(crate::FieldReader<bool, BITDELAY_A>);
impl BITDELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BITDELAY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BITDELAY_A {
        match self.bits {
            false => BITDELAY_A::LJ,
            true => BITDELAY_A::I2S,
        }
    }
    #[doc = "Checks if the value of the field is `LJ`"]
    #[inline(always)]
    pub fn is_lj(&self) -> bool {
        **self == BITDELAY_A::LJ
    }
    #[doc = "Checks if the value of the field is `I2S`"]
    #[inline(always)]
    pub fn is_i2s(&self) -> bool {
        **self == BITDELAY_A::I2S
    }
}
impl core::ops::Deref for BITDELAY_R {
    type Target = crate::FieldReader<bool, BITDELAY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BITDELAY` writer - Data Delay from Frame Sync"]
pub struct BITDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> BITDELAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BITDELAY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Left Justified (0 Bit Delay)"]
    #[inline(always)]
    pub fn lj(self) -> &'a mut W {
        self.variant(BITDELAY_A::LJ)
    }
    #[doc = "I2S (1 Bit Delay)"]
    #[inline(always)]
    pub fn i2s(self) -> &'a mut W {
        self.variant(BITDELAY_A::I2S)
    }
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
#[doc = "Frame Sync Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSSEL_A {
    #[doc = "0: Divided Serial Clock n is used as Frame Sync n source"]
    SCKDIV = 0,
    #[doc = "1: FSn input pin is used as Frame Sync n source"]
    FSPIN = 1,
}
impl From<FSSEL_A> for bool {
    #[inline(always)]
    fn from(variant: FSSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSSEL` reader - Frame Sync Select"]
pub struct FSSEL_R(crate::FieldReader<bool, FSSEL_A>);
impl FSSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSSEL_A {
        match self.bits {
            false => FSSEL_A::SCKDIV,
            true => FSSEL_A::FSPIN,
        }
    }
    #[doc = "Checks if the value of the field is `SCKDIV`"]
    #[inline(always)]
    pub fn is_sckdiv(&self) -> bool {
        **self == FSSEL_A::SCKDIV
    }
    #[doc = "Checks if the value of the field is `FSPIN`"]
    #[inline(always)]
    pub fn is_fspin(&self) -> bool {
        **self == FSSEL_A::FSPIN
    }
}
impl core::ops::Deref for FSSEL_R {
    type Target = crate::FieldReader<bool, FSSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSSEL` writer - Frame Sync Select"]
pub struct FSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Divided Serial Clock n is used as Frame Sync n source"]
    #[inline(always)]
    pub fn sckdiv(self) -> &'a mut W {
        self.variant(FSSEL_A::SCKDIV)
    }
    #[doc = "FSn input pin is used as Frame Sync n source"]
    #[inline(always)]
    pub fn fspin(self) -> &'a mut W {
        self.variant(FSSEL_A::FSPIN)
    }
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
#[doc = "Field `FSINV` reader - Frame Sync Invert"]
pub struct FSINV_R(crate::FieldReader<bool, bool>);
impl FSINV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSINV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSINV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSINV` writer - Frame Sync Invert"]
pub struct FSINV_W<'a> {
    w: &'a mut W,
}
impl<'a> FSINV_W<'a> {
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
#[doc = "Field `FSOUTINV` reader - Frame Sync Output Invert"]
pub struct FSOUTINV_R(crate::FieldReader<bool, bool>);
impl FSOUTINV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSOUTINV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSOUTINV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSOUTINV` writer - Frame Sync Output Invert"]
pub struct FSOUTINV_W<'a> {
    w: &'a mut W,
}
impl<'a> FSOUTINV_W<'a> {
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
#[doc = "Serial Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCKSEL_A {
    #[doc = "0: Divided Master Clock n is used as Serial Clock n source"]
    MCKDIV = 0,
    #[doc = "1: SCKn input pin is used as Serial Clock n source"]
    SCKPIN = 1,
}
impl From<SCKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SCKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCKSEL` reader - Serial Clock Select"]
pub struct SCKSEL_R(crate::FieldReader<bool, SCKSEL_A>);
impl SCKSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCKSEL_A {
        match self.bits {
            false => SCKSEL_A::MCKDIV,
            true => SCKSEL_A::SCKPIN,
        }
    }
    #[doc = "Checks if the value of the field is `MCKDIV`"]
    #[inline(always)]
    pub fn is_mckdiv(&self) -> bool {
        **self == SCKSEL_A::MCKDIV
    }
    #[doc = "Checks if the value of the field is `SCKPIN`"]
    #[inline(always)]
    pub fn is_sckpin(&self) -> bool {
        **self == SCKSEL_A::SCKPIN
    }
}
impl core::ops::Deref for SCKSEL_R {
    type Target = crate::FieldReader<bool, SCKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCKSEL` writer - Serial Clock Select"]
pub struct SCKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCKSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Divided Master Clock n is used as Serial Clock n source"]
    #[inline(always)]
    pub fn mckdiv(self) -> &'a mut W {
        self.variant(SCKSEL_A::MCKDIV)
    }
    #[doc = "SCKn input pin is used as Serial Clock n source"]
    #[inline(always)]
    pub fn sckpin(self) -> &'a mut W {
        self.variant(SCKSEL_A::SCKPIN)
    }
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
#[doc = "Field `SCKOUTINV` reader - Serial Clock Output Invert"]
pub struct SCKOUTINV_R(crate::FieldReader<bool, bool>);
impl SCKOUTINV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCKOUTINV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCKOUTINV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCKOUTINV` writer - Serial Clock Output Invert"]
pub struct SCKOUTINV_W<'a> {
    w: &'a mut W,
}
impl<'a> SCKOUTINV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Master Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCKSEL_A {
    #[doc = "0: GCLK_I2S_n is used as Master Clock n source"]
    GCLK = 0,
    #[doc = "1: MCKn input pin is used as Master Clock n source"]
    MCKPIN = 1,
}
impl From<MCKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: MCKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCKSEL` reader - Master Clock Select"]
pub struct MCKSEL_R(crate::FieldReader<bool, MCKSEL_A>);
impl MCKSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCKSEL_A {
        match self.bits {
            false => MCKSEL_A::GCLK,
            true => MCKSEL_A::MCKPIN,
        }
    }
    #[doc = "Checks if the value of the field is `GCLK`"]
    #[inline(always)]
    pub fn is_gclk(&self) -> bool {
        **self == MCKSEL_A::GCLK
    }
    #[doc = "Checks if the value of the field is `MCKPIN`"]
    #[inline(always)]
    pub fn is_mckpin(&self) -> bool {
        **self == MCKSEL_A::MCKPIN
    }
}
impl core::ops::Deref for MCKSEL_R {
    type Target = crate::FieldReader<bool, MCKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCKSEL` writer - Master Clock Select"]
pub struct MCKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCKSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "GCLK_I2S_n is used as Master Clock n source"]
    #[inline(always)]
    pub fn gclk(self) -> &'a mut W {
        self.variant(MCKSEL_A::GCLK)
    }
    #[doc = "MCKn input pin is used as Master Clock n source"]
    #[inline(always)]
    pub fn mckpin(self) -> &'a mut W {
        self.variant(MCKSEL_A::MCKPIN)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `MCKEN` reader - Master Clock Enable"]
pub struct MCKEN_R(crate::FieldReader<bool, bool>);
impl MCKEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCKEN` writer - Master Clock Enable"]
pub struct MCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCKEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `MCKOUTINV` reader - Master Clock Output Invert"]
pub struct MCKOUTINV_R(crate::FieldReader<bool, bool>);
impl MCKOUTINV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCKOUTINV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCKOUTINV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCKOUTINV` writer - Master Clock Output Invert"]
pub struct MCKOUTINV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCKOUTINV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `MCKDIV` reader - Master Clock Division Factor"]
pub struct MCKDIV_R(crate::FieldReader<u8, u8>);
impl MCKDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MCKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCKDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCKDIV` writer - Master Clock Division Factor"]
pub struct MCKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `MCKOUTDIV` reader - Master Clock Output Division Factor"]
pub struct MCKOUTDIV_R(crate::FieldReader<u8, u8>);
impl MCKOUTDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MCKOUTDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCKOUTDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCKOUTDIV` writer - Master Clock Output Division Factor"]
pub struct MCKOUTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCKOUTDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Slot Size"]
    #[inline(always)]
    pub fn slotsize(&self) -> SLOTSIZE_R {
        SLOTSIZE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4 - Number of Slots in Frame"]
    #[inline(always)]
    pub fn nbslots(&self) -> NBSLOTS_R {
        NBSLOTS_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 5:6 - Frame Sync Width"]
    #[inline(always)]
    pub fn fswidth(&self) -> FSWIDTH_R {
        FSWIDTH_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Data Delay from Frame Sync"]
    #[inline(always)]
    pub fn bitdelay(&self) -> BITDELAY_R {
        BITDELAY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Frame Sync Select"]
    #[inline(always)]
    pub fn fssel(&self) -> FSSEL_R {
        FSSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Frame Sync Invert"]
    #[inline(always)]
    pub fn fsinv(&self) -> FSINV_R {
        FSINV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Frame Sync Output Invert"]
    #[inline(always)]
    pub fn fsoutinv(&self) -> FSOUTINV_R {
        FSOUTINV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Serial Clock Select"]
    #[inline(always)]
    pub fn scksel(&self) -> SCKSEL_R {
        SCKSEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Serial Clock Output Invert"]
    #[inline(always)]
    pub fn sckoutinv(&self) -> SCKOUTINV_R {
        SCKOUTINV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Master Clock Select"]
    #[inline(always)]
    pub fn mcksel(&self) -> MCKSEL_R {
        MCKSEL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Master Clock Enable"]
    #[inline(always)]
    pub fn mcken(&self) -> MCKEN_R {
        MCKEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Master Clock Output Invert"]
    #[inline(always)]
    pub fn mckoutinv(&self) -> MCKOUTINV_R {
        MCKOUTINV_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - Master Clock Division Factor"]
    #[inline(always)]
    pub fn mckdiv(&self) -> MCKDIV_R {
        MCKDIV_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Master Clock Output Division Factor"]
    #[inline(always)]
    pub fn mckoutdiv(&self) -> MCKOUTDIV_R {
        MCKOUTDIV_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Slot Size"]
    #[inline(always)]
    pub fn slotsize(&mut self) -> SLOTSIZE_W {
        SLOTSIZE_W { w: self }
    }
    #[doc = "Bits 2:4 - Number of Slots in Frame"]
    #[inline(always)]
    pub fn nbslots(&mut self) -> NBSLOTS_W {
        NBSLOTS_W { w: self }
    }
    #[doc = "Bits 5:6 - Frame Sync Width"]
    #[inline(always)]
    pub fn fswidth(&mut self) -> FSWIDTH_W {
        FSWIDTH_W { w: self }
    }
    #[doc = "Bit 7 - Data Delay from Frame Sync"]
    #[inline(always)]
    pub fn bitdelay(&mut self) -> BITDELAY_W {
        BITDELAY_W { w: self }
    }
    #[doc = "Bit 8 - Frame Sync Select"]
    #[inline(always)]
    pub fn fssel(&mut self) -> FSSEL_W {
        FSSEL_W { w: self }
    }
    #[doc = "Bit 9 - Frame Sync Invert"]
    #[inline(always)]
    pub fn fsinv(&mut self) -> FSINV_W {
        FSINV_W { w: self }
    }
    #[doc = "Bit 10 - Frame Sync Output Invert"]
    #[inline(always)]
    pub fn fsoutinv(&mut self) -> FSOUTINV_W {
        FSOUTINV_W { w: self }
    }
    #[doc = "Bit 11 - Serial Clock Select"]
    #[inline(always)]
    pub fn scksel(&mut self) -> SCKSEL_W {
        SCKSEL_W { w: self }
    }
    #[doc = "Bit 12 - Serial Clock Output Invert"]
    #[inline(always)]
    pub fn sckoutinv(&mut self) -> SCKOUTINV_W {
        SCKOUTINV_W { w: self }
    }
    #[doc = "Bit 13 - Master Clock Select"]
    #[inline(always)]
    pub fn mcksel(&mut self) -> MCKSEL_W {
        MCKSEL_W { w: self }
    }
    #[doc = "Bit 14 - Master Clock Enable"]
    #[inline(always)]
    pub fn mcken(&mut self) -> MCKEN_W {
        MCKEN_W { w: self }
    }
    #[doc = "Bit 15 - Master Clock Output Invert"]
    #[inline(always)]
    pub fn mckoutinv(&mut self) -> MCKOUTINV_W {
        MCKOUTINV_W { w: self }
    }
    #[doc = "Bits 16:21 - Master Clock Division Factor"]
    #[inline(always)]
    pub fn mckdiv(&mut self) -> MCKDIV_W {
        MCKDIV_W { w: self }
    }
    #[doc = "Bits 24:29 - Master Clock Output Division Factor"]
    #[inline(always)]
    pub fn mckoutdiv(&mut self) -> MCKOUTDIV_W {
        MCKOUTDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Unit n Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkctrl](index.html) module"]
pub struct CLKCTRL_SPEC;
impl crate::RegisterSpec for CLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkctrl::R](R) reader structure"]
impl crate::Readable for CLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkctrl::W](W) writer structure"]
impl crate::Writable for CLKCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKCTRL[%s]
to value 0"]
impl crate::Resettable for CLKCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
