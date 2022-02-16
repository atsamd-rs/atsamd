#[doc = "Register `DACC_MR` reader"]
pub struct R(crate::R<DACC_MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DACC_MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DACC_MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DACC_MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DACC_MR` writer"]
pub struct W(crate::W<DACC_MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DACC_MR_SPEC>;
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
impl From<crate::W<DACC_MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DACC_MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Max Speed Mode for Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAXS0_A {
    #[doc = "0: External trigger mode or Free-running mode enabled. (See TRGENx.DACC_TRIGR.)"]
    TRIG_EVENT = 0,
    #[doc = "1: Max speed mode enabled."]
    MAXIMUM = 1,
}
impl From<MAXS0_A> for bool {
    #[inline(always)]
    fn from(variant: MAXS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAXS0` reader - Max Speed Mode for Channel 0"]
pub struct MAXS0_R(crate::FieldReader<bool, MAXS0_A>);
impl MAXS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAXS0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAXS0_A {
        match self.bits {
            false => MAXS0_A::TRIG_EVENT,
            true => MAXS0_A::MAXIMUM,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG_EVENT`"]
    #[inline(always)]
    pub fn is_trig_event(&self) -> bool {
        **self == MAXS0_A::TRIG_EVENT
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        **self == MAXS0_A::MAXIMUM
    }
}
impl core::ops::Deref for MAXS0_R {
    type Target = crate::FieldReader<bool, MAXS0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAXS0` writer - Max Speed Mode for Channel 0"]
pub struct MAXS0_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAXS0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External trigger mode or Free-running mode enabled. (See TRGENx.DACC_TRIGR.)"]
    #[inline(always)]
    pub fn trig_event(self) -> &'a mut W {
        self.variant(MAXS0_A::TRIG_EVENT)
    }
    #[doc = "Max speed mode enabled."]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(MAXS0_A::MAXIMUM)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Max Speed Mode for Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAXS1_A {
    #[doc = "0: External trigger mode or Free-running mode enabled. (See TRGENx.DACC_TRIGR.)"]
    TRIG_EVENT = 0,
    #[doc = "1: Max speed mode enabled."]
    MAXIMUM = 1,
}
impl From<MAXS1_A> for bool {
    #[inline(always)]
    fn from(variant: MAXS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAXS1` reader - Max Speed Mode for Channel 1"]
pub struct MAXS1_R(crate::FieldReader<bool, MAXS1_A>);
impl MAXS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAXS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAXS1_A {
        match self.bits {
            false => MAXS1_A::TRIG_EVENT,
            true => MAXS1_A::MAXIMUM,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG_EVENT`"]
    #[inline(always)]
    pub fn is_trig_event(&self) -> bool {
        **self == MAXS1_A::TRIG_EVENT
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        **self == MAXS1_A::MAXIMUM
    }
}
impl core::ops::Deref for MAXS1_R {
    type Target = crate::FieldReader<bool, MAXS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAXS1` writer - Max Speed Mode for Channel 1"]
pub struct MAXS1_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAXS1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External trigger mode or Free-running mode enabled. (See TRGENx.DACC_TRIGR.)"]
    #[inline(always)]
    pub fn trig_event(self) -> &'a mut W {
        self.variant(MAXS1_A::TRIG_EVENT)
    }
    #[doc = "Max speed mode enabled."]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(MAXS1_A::MAXIMUM)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Word Transfer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WORD_A {
    #[doc = "0: One data to convert is written to the FIFO per access to DACC."]
    DISABLED = 0,
    #[doc = "1: Two data to convert are written to the FIFO per access to DACC (reduces the number of requests to DMA and the number of system bus accesses)."]
    ENABLED = 1,
}
impl From<WORD_A> for bool {
    #[inline(always)]
    fn from(variant: WORD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WORD` reader - Word Transfer Mode"]
pub struct WORD_R(crate::FieldReader<bool, WORD_A>);
impl WORD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WORD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WORD_A {
        match self.bits {
            false => WORD_A::DISABLED,
            true => WORD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WORD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WORD_A::ENABLED
    }
}
impl core::ops::Deref for WORD_R {
    type Target = crate::FieldReader<bool, WORD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WORD` writer - Word Transfer Mode"]
pub struct WORD_W<'a> {
    w: &'a mut W,
}
impl<'a> WORD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WORD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "One data to convert is written to the FIFO per access to DACC."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WORD_A::DISABLED)
    }
    #[doc = "Two data to convert are written to the FIFO per access to DACC (reduces the number of requests to DMA and the number of system bus accesses)."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WORD_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `ZERO` reader - Must always be written to 0."]
pub struct ZERO_R(crate::FieldReader<bool, bool>);
impl ZERO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ZERO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZERO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZERO` writer - Must always be written to 0."]
pub struct ZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> ZERO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Differential Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIFF_A {
    #[doc = "0: DAC0 and DAC1 are single-ended outputs."]
    DISABLED = 0,
    #[doc = "1: DACP and DACN are differential outputs. The differential level is configured by the channel 0 value."]
    ENABLED = 1,
}
impl From<DIFF_A> for bool {
    #[inline(always)]
    fn from(variant: DIFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIFF` reader - Differential Mode"]
pub struct DIFF_R(crate::FieldReader<bool, DIFF_A>);
impl DIFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIFF_A {
        match self.bits {
            false => DIFF_A::DISABLED,
            true => DIFF_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DIFF_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DIFF_A::ENABLED
    }
}
impl core::ops::Deref for DIFF_R {
    type Target = crate::FieldReader<bool, DIFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFF` writer - Differential Mode"]
pub struct DIFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DAC0 and DAC1 are single-ended outputs."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIFF_A::DISABLED)
    }
    #[doc = "DACP and DACN are differential outputs. The differential level is configured by the channel 0 value."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIFF_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `PRESCALER` reader - Peripheral Clock to DAC Clock Ratio"]
pub struct PRESCALER_R(crate::FieldReader<u8, u8>);
impl PRESCALER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRESCALER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESCALER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRESCALER` writer - Peripheral Clock to DAC Clock Ratio"]
pub struct PRESCALER_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Max Speed Mode for Channel 0"]
    #[inline(always)]
    pub fn maxs0(&self) -> MAXS0_R {
        MAXS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Max Speed Mode for Channel 1"]
    #[inline(always)]
    pub fn maxs1(&self) -> MAXS1_R {
        MAXS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Word Transfer Mode"]
    #[inline(always)]
    pub fn word(&self) -> WORD_R {
        WORD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Must always be written to 0."]
    #[inline(always)]
    pub fn zero(&self) -> ZERO_R {
        ZERO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Differential Mode"]
    #[inline(always)]
    pub fn diff(&self) -> DIFF_R {
        DIFF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Peripheral Clock to DAC Clock Ratio"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Max Speed Mode for Channel 0"]
    #[inline(always)]
    pub fn maxs0(&mut self) -> MAXS0_W {
        MAXS0_W { w: self }
    }
    #[doc = "Bit 1 - Max Speed Mode for Channel 1"]
    #[inline(always)]
    pub fn maxs1(&mut self) -> MAXS1_W {
        MAXS1_W { w: self }
    }
    #[doc = "Bit 4 - Word Transfer Mode"]
    #[inline(always)]
    pub fn word(&mut self) -> WORD_W {
        WORD_W { w: self }
    }
    #[doc = "Bit 5 - Must always be written to 0."]
    #[inline(always)]
    pub fn zero(&mut self) -> ZERO_W {
        ZERO_W { w: self }
    }
    #[doc = "Bit 23 - Differential Mode"]
    #[inline(always)]
    pub fn diff(&mut self) -> DIFF_W {
        DIFF_W { w: self }
    }
    #[doc = "Bits 24:27 - Peripheral Clock to DAC Clock Ratio"]
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W {
        PRESCALER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacc_mr](index.html) module"]
pub struct DACC_MR_SPEC;
impl crate::RegisterSpec for DACC_MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dacc_mr::R](R) reader structure"]
impl crate::Readable for DACC_MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dacc_mr::W](W) writer structure"]
impl crate::Writable for DACC_MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DACC_MR to value 0"]
impl crate::Resettable for DACC_MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
