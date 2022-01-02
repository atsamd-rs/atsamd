#[doc = "Register `CTRLB` reader"]
pub struct R(crate::R<CTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB` writer"]
pub struct W(crate::W<CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLB_SPEC>;
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
impl From<crate::W<CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GP0EN` reader - General Purpose 0 Enable"]
pub struct GP0EN_R(crate::FieldReader<bool, bool>);
impl GP0EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GP0EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP0EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP0EN` writer - General Purpose 0 Enable"]
pub struct GP0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GP0EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Field `GP2EN` reader - General Purpose 2 Enable"]
pub struct GP2EN_R(crate::FieldReader<bool, bool>);
impl GP2EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GP2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP2EN` writer - General Purpose 2 Enable"]
pub struct GP2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GP2EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `DEBMAJ` reader - Debouncer Majority Enable"]
pub struct DEBMAJ_R(crate::FieldReader<bool, bool>);
impl DEBMAJ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEBMAJ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEBMAJ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEBMAJ` writer - Debouncer Majority Enable"]
pub struct DEBMAJ_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBMAJ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `DEBASYNC` reader - Debouncer Asynchronous Enable"]
pub struct DEBASYNC_R(crate::FieldReader<bool, bool>);
impl DEBASYNC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEBASYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEBASYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEBASYNC` writer - Debouncer Asynchronous Enable"]
pub struct DEBASYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBASYNC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `RTCOUT` reader - RTC Output Enable"]
pub struct RTCOUT_R(crate::FieldReader<bool, bool>);
impl RTCOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTCOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCOUT` writer - RTC Output Enable"]
pub struct RTCOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub struct DMAEN_R(crate::FieldReader<bool, bool>);
impl DMAEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
#[doc = "Debounce Freqnuency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DEBF_A {
    #[doc = "0: CLK_RTC_DEB = CLK_RTC/2"]
    DIV2 = 0,
    #[doc = "1: CLK_RTC_DEB = CLK_RTC/4"]
    DIV4 = 1,
    #[doc = "2: CLK_RTC_DEB = CLK_RTC/8"]
    DIV8 = 2,
    #[doc = "3: CLK_RTC_DEB = CLK_RTC/16"]
    DIV16 = 3,
    #[doc = "4: CLK_RTC_DEB = CLK_RTC/32"]
    DIV32 = 4,
    #[doc = "5: CLK_RTC_DEB = CLK_RTC/64"]
    DIV64 = 5,
    #[doc = "6: CLK_RTC_DEB = CLK_RTC/128"]
    DIV128 = 6,
    #[doc = "7: CLK_RTC_DEB = CLK_RTC/256"]
    DIV256 = 7,
}
impl From<DEBF_A> for u8 {
    #[inline(always)]
    fn from(variant: DEBF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DEBF` reader - Debounce Freqnuency"]
pub struct DEBF_R(crate::FieldReader<u8, DEBF_A>);
impl DEBF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DEBF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBF_A {
        match self.bits {
            0 => DEBF_A::DIV2,
            1 => DEBF_A::DIV4,
            2 => DEBF_A::DIV8,
            3 => DEBF_A::DIV16,
            4 => DEBF_A::DIV32,
            5 => DEBF_A::DIV64,
            6 => DEBF_A::DIV128,
            7 => DEBF_A::DIV256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == DEBF_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == DEBF_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == DEBF_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == DEBF_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        **self == DEBF_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        **self == DEBF_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        **self == DEBF_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        **self == DEBF_A::DIV256
    }
}
impl core::ops::Deref for DEBF_R {
    type Target = crate::FieldReader<u8, DEBF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEBF` writer - Debounce Freqnuency"]
pub struct DEBF_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEBF_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(DEBF_A::DIV2)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(DEBF_A::DIV4)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(DEBF_A::DIV8)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(DEBF_A::DIV16)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(DEBF_A::DIV32)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(DEBF_A::DIV64)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(DEBF_A::DIV128)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(DEBF_A::DIV256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u16 & 0x07) << 8);
        self.w
    }
}
#[doc = "Active Layer Freqnuency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACTF_A {
    #[doc = "0: CLK_RTC_OUT = CLK_RTC/2"]
    DIV2 = 0,
    #[doc = "1: CLK_RTC_OUT = CLK_RTC/4"]
    DIV4 = 1,
    #[doc = "2: CLK_RTC_OUT = CLK_RTC/8"]
    DIV8 = 2,
    #[doc = "3: CLK_RTC_OUT = CLK_RTC/16"]
    DIV16 = 3,
    #[doc = "4: CLK_RTC_OUT = CLK_RTC/32"]
    DIV32 = 4,
    #[doc = "5: CLK_RTC_OUT = CLK_RTC/64"]
    DIV64 = 5,
    #[doc = "6: CLK_RTC_OUT = CLK_RTC/128"]
    DIV128 = 6,
    #[doc = "7: CLK_RTC_OUT = CLK_RTC/256"]
    DIV256 = 7,
}
impl From<ACTF_A> for u8 {
    #[inline(always)]
    fn from(variant: ACTF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ACTF` reader - Active Layer Freqnuency"]
pub struct ACTF_R(crate::FieldReader<u8, ACTF_A>);
impl ACTF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ACTF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTF_A {
        match self.bits {
            0 => ACTF_A::DIV2,
            1 => ACTF_A::DIV4,
            2 => ACTF_A::DIV8,
            3 => ACTF_A::DIV16,
            4 => ACTF_A::DIV32,
            5 => ACTF_A::DIV64,
            6 => ACTF_A::DIV128,
            7 => ACTF_A::DIV256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == ACTF_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == ACTF_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == ACTF_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == ACTF_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        **self == ACTF_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        **self == ACTF_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        **self == ACTF_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        **self == ACTF_A::DIV256
    }
}
impl core::ops::Deref for ACTF_R {
    type Target = crate::FieldReader<u8, ACTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTF` writer - Active Layer Freqnuency"]
pub struct ACTF_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACTF_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(ACTF_A::DIV2)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(ACTF_A::DIV4)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(ACTF_A::DIV8)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(ACTF_A::DIV16)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(ACTF_A::DIV32)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(ACTF_A::DIV64)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(ACTF_A::DIV128)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(ACTF_A::DIV256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u16 & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - General Purpose 0 Enable"]
    #[inline(always)]
    pub fn gp0en(&self) -> GP0EN_R {
        GP0EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - General Purpose 2 Enable"]
    #[inline(always)]
    pub fn gp2en(&self) -> GP2EN_R {
        GP2EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Debouncer Majority Enable"]
    #[inline(always)]
    pub fn debmaj(&self) -> DEBMAJ_R {
        DEBMAJ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Debouncer Asynchronous Enable"]
    #[inline(always)]
    pub fn debasync(&self) -> DEBASYNC_R {
        DEBASYNC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RTC Output Enable"]
    #[inline(always)]
    pub fn rtcout(&self) -> RTCOUT_R {
        RTCOUT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Debounce Freqnuency"]
    #[inline(always)]
    pub fn debf(&self) -> DEBF_R {
        DEBF_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Active Layer Freqnuency"]
    #[inline(always)]
    pub fn actf(&self) -> ACTF_R {
        ACTF_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - General Purpose 0 Enable"]
    #[inline(always)]
    pub fn gp0en(&mut self) -> GP0EN_W {
        GP0EN_W { w: self }
    }
    #[doc = "Bit 1 - General Purpose 2 Enable"]
    #[inline(always)]
    pub fn gp2en(&mut self) -> GP2EN_W {
        GP2EN_W { w: self }
    }
    #[doc = "Bit 4 - Debouncer Majority Enable"]
    #[inline(always)]
    pub fn debmaj(&mut self) -> DEBMAJ_W {
        DEBMAJ_W { w: self }
    }
    #[doc = "Bit 5 - Debouncer Asynchronous Enable"]
    #[inline(always)]
    pub fn debasync(&mut self) -> DEBASYNC_W {
        DEBASYNC_W { w: self }
    }
    #[doc = "Bit 6 - RTC Output Enable"]
    #[inline(always)]
    pub fn rtcout(&mut self) -> RTCOUT_W {
        RTCOUT_W { w: self }
    }
    #[doc = "Bit 7 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bits 8:10 - Debounce Freqnuency"]
    #[inline(always)]
    pub fn debf(&mut self) -> DEBF_W {
        DEBF_W { w: self }
    }
    #[doc = "Bits 12:14 - Active Layer Freqnuency"]
    #[inline(always)]
    pub fn actf(&mut self) -> ACTF_W {
        ACTF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MODE0 Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctrlb::R](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
