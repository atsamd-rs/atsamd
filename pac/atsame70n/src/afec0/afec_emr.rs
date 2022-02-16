#[doc = "Register `AFEC_EMR` reader"]
pub struct R(crate::R<AFEC_EMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFEC_EMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFEC_EMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFEC_EMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFEC_EMR` writer"]
pub struct W(crate::W<AFEC_EMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFEC_EMR_SPEC>;
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
impl From<crate::W<AFEC_EMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFEC_EMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Comparison Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPMODE_A {
    #[doc = "0: Generates an event when the converted data is lower than the low threshold of the window."]
    LOW = 0,
    #[doc = "1: Generates an event when the converted data is higher than the high threshold of the window."]
    HIGH = 1,
    #[doc = "2: Generates an event when the converted data is in the comparison window."]
    IN = 2,
    #[doc = "3: Generates an event when the converted data is out of the comparison window."]
    OUT = 3,
}
impl From<CMPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMPMODE` reader - Comparison Mode"]
pub struct CMPMODE_R(crate::FieldReader<u8, CMPMODE_A>);
impl CMPMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CMPMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPMODE_A {
        match self.bits {
            0 => CMPMODE_A::LOW,
            1 => CMPMODE_A::HIGH,
            2 => CMPMODE_A::IN,
            3 => CMPMODE_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == CMPMODE_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == CMPMODE_A::HIGH
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        **self == CMPMODE_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        **self == CMPMODE_A::OUT
    }
}
impl core::ops::Deref for CMPMODE_R {
    type Target = crate::FieldReader<u8, CMPMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPMODE` writer - Comparison Mode"]
pub struct CMPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPMODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CMPMODE_A::LOW)
    }
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CMPMODE_A::HIGH)
    }
    #[doc = "Generates an event when the converted data is in the comparison window."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(CMPMODE_A::IN)
    }
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(CMPMODE_A::OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `CMPSEL` reader - Comparison Selected Channel"]
pub struct CMPSEL_R(crate::FieldReader<u8, u8>);
impl CMPSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CMPSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPSEL` writer - Comparison Selected Channel"]
pub struct CMPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | ((value as u32 & 0x1f) << 3);
        self.w
    }
}
#[doc = "Field `CMPALL` reader - Compare All Channels"]
pub struct CMPALL_R(crate::FieldReader<bool, bool>);
impl CMPALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPALL` writer - Compare All Channels"]
pub struct CMPALL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPALL_W<'a> {
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
#[doc = "Field `CMPFILTER` reader - Compare Event Filtering"]
pub struct CMPFILTER_R(crate::FieldReader<u8, u8>);
impl CMPFILTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CMPFILTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPFILTER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPFILTER` writer - Compare Event Filtering"]
pub struct CMPFILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPFILTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RES_A {
    #[doc = "0: 12-bit resolution, AFE sample rate is maximum (no averaging)."]
    NO_AVERAGE = 0,
    #[doc = "2: 13-bit resolution, AFE sample rate divided by 4 (averaging)."]
    OSR4 = 2,
    #[doc = "3: 14-bit resolution, AFE sample rate divided by 16 (averaging)."]
    OSR16 = 3,
    #[doc = "4: 15-bit resolution, AFE sample rate divided by 64 (averaging)."]
    OSR64 = 4,
    #[doc = "5: 16-bit resolution, AFE sample rate divided by 256 (averaging)."]
    OSR256 = 5,
}
impl From<RES_A> for u8 {
    #[inline(always)]
    fn from(variant: RES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RES` reader - Resolution"]
pub struct RES_R(crate::FieldReader<u8, RES_A>);
impl RES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RES_A> {
        match self.bits {
            0 => Some(RES_A::NO_AVERAGE),
            2 => Some(RES_A::OSR4),
            3 => Some(RES_A::OSR16),
            4 => Some(RES_A::OSR64),
            5 => Some(RES_A::OSR256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_AVERAGE`"]
    #[inline(always)]
    pub fn is_no_average(&self) -> bool {
        **self == RES_A::NO_AVERAGE
    }
    #[doc = "Checks if the value of the field is `OSR4`"]
    #[inline(always)]
    pub fn is_osr4(&self) -> bool {
        **self == RES_A::OSR4
    }
    #[doc = "Checks if the value of the field is `OSR16`"]
    #[inline(always)]
    pub fn is_osr16(&self) -> bool {
        **self == RES_A::OSR16
    }
    #[doc = "Checks if the value of the field is `OSR64`"]
    #[inline(always)]
    pub fn is_osr64(&self) -> bool {
        **self == RES_A::OSR64
    }
    #[doc = "Checks if the value of the field is `OSR256`"]
    #[inline(always)]
    pub fn is_osr256(&self) -> bool {
        **self == RES_A::OSR256
    }
}
impl core::ops::Deref for RES_R {
    type Target = crate::FieldReader<u8, RES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RES` writer - Resolution"]
pub struct RES_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RES_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "12-bit resolution, AFE sample rate is maximum (no averaging)."]
    #[inline(always)]
    pub fn no_average(self) -> &'a mut W {
        self.variant(RES_A::NO_AVERAGE)
    }
    #[doc = "13-bit resolution, AFE sample rate divided by 4 (averaging)."]
    #[inline(always)]
    pub fn osr4(self) -> &'a mut W {
        self.variant(RES_A::OSR4)
    }
    #[doc = "14-bit resolution, AFE sample rate divided by 16 (averaging)."]
    #[inline(always)]
    pub fn osr16(self) -> &'a mut W {
        self.variant(RES_A::OSR16)
    }
    #[doc = "15-bit resolution, AFE sample rate divided by 64 (averaging)."]
    #[inline(always)]
    pub fn osr64(self) -> &'a mut W {
        self.variant(RES_A::OSR64)
    }
    #[doc = "16-bit resolution, AFE sample rate divided by 256 (averaging)."]
    #[inline(always)]
    pub fn osr256(self) -> &'a mut W {
        self.variant(RES_A::OSR256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `TAG` reader - TAG of the AFEC_LDCR"]
pub struct TAG_R(crate::FieldReader<bool, bool>);
impl TAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAG` writer - TAG of the AFEC_LDCR"]
pub struct TAG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAG_W<'a> {
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
#[doc = "Field `STM` reader - Single Trigger Mode"]
pub struct STM_R(crate::FieldReader<bool, bool>);
impl STM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STM` writer - Single Trigger Mode"]
pub struct STM_W<'a> {
    w: &'a mut W,
}
impl<'a> STM_W<'a> {
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
#[doc = "Sign Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SIGNMODE_A {
    #[doc = "0: Single-Ended channels: Unsigned conversions.Differential channels: Signed conversions."]
    SE_UNSG_DF_SIGN = 0,
    #[doc = "1: Single-Ended channels: Signed conversions.Differential channels: Unsigned conversions."]
    SE_SIGN_DF_UNSG = 1,
    #[doc = "2: All channels: Unsigned conversions."]
    ALL_UNSIGNED = 2,
    #[doc = "3: All channels: Signed conversions."]
    ALL_SIGNED = 3,
}
impl From<SIGNMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: SIGNMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SIGNMODE` reader - Sign Mode"]
pub struct SIGNMODE_R(crate::FieldReader<u8, SIGNMODE_A>);
impl SIGNMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SIGNMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIGNMODE_A {
        match self.bits {
            0 => SIGNMODE_A::SE_UNSG_DF_SIGN,
            1 => SIGNMODE_A::SE_SIGN_DF_UNSG,
            2 => SIGNMODE_A::ALL_UNSIGNED,
            3 => SIGNMODE_A::ALL_SIGNED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SE_UNSG_DF_SIGN`"]
    #[inline(always)]
    pub fn is_se_unsg_df_sign(&self) -> bool {
        **self == SIGNMODE_A::SE_UNSG_DF_SIGN
    }
    #[doc = "Checks if the value of the field is `SE_SIGN_DF_UNSG`"]
    #[inline(always)]
    pub fn is_se_sign_df_unsg(&self) -> bool {
        **self == SIGNMODE_A::SE_SIGN_DF_UNSG
    }
    #[doc = "Checks if the value of the field is `ALL_UNSIGNED`"]
    #[inline(always)]
    pub fn is_all_unsigned(&self) -> bool {
        **self == SIGNMODE_A::ALL_UNSIGNED
    }
    #[doc = "Checks if the value of the field is `ALL_SIGNED`"]
    #[inline(always)]
    pub fn is_all_signed(&self) -> bool {
        **self == SIGNMODE_A::ALL_SIGNED
    }
}
impl core::ops::Deref for SIGNMODE_R {
    type Target = crate::FieldReader<u8, SIGNMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIGNMODE` writer - Sign Mode"]
pub struct SIGNMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGNMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIGNMODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Single-Ended channels: Unsigned conversions.Differential channels: Signed conversions."]
    #[inline(always)]
    pub fn se_unsg_df_sign(self) -> &'a mut W {
        self.variant(SIGNMODE_A::SE_UNSG_DF_SIGN)
    }
    #[doc = "Single-Ended channels: Signed conversions.Differential channels: Unsigned conversions."]
    #[inline(always)]
    pub fn se_sign_df_unsg(self) -> &'a mut W {
        self.variant(SIGNMODE_A::SE_SIGN_DF_UNSG)
    }
    #[doc = "All channels: Unsigned conversions."]
    #[inline(always)]
    pub fn all_unsigned(self) -> &'a mut W {
        self.variant(SIGNMODE_A::ALL_UNSIGNED)
    }
    #[doc = "All channels: Signed conversions."]
    #[inline(always)]
    pub fn all_signed(self) -> &'a mut W {
        self.variant(SIGNMODE_A::ALL_SIGNED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&self) -> CMPMODE_R {
        CMPMODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 3:7 - Comparison Selected Channel"]
    #[inline(always)]
    pub fn cmpsel(&self) -> CMPSEL_R {
        CMPSEL_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 9 - Compare All Channels"]
    #[inline(always)]
    pub fn cmpall(&self) -> CMPALL_R {
        CMPALL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Compare Event Filtering"]
    #[inline(always)]
    pub fn cmpfilter(&self) -> CMPFILTER_R {
        CMPFILTER_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:18 - Resolution"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 24 - TAG of the AFEC_LDCR"]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Single Trigger Mode"]
    #[inline(always)]
    pub fn stm(&self) -> STM_R {
        STM_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - Sign Mode"]
    #[inline(always)]
    pub fn signmode(&self) -> SIGNMODE_R {
        SIGNMODE_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&mut self) -> CMPMODE_W {
        CMPMODE_W { w: self }
    }
    #[doc = "Bits 3:7 - Comparison Selected Channel"]
    #[inline(always)]
    pub fn cmpsel(&mut self) -> CMPSEL_W {
        CMPSEL_W { w: self }
    }
    #[doc = "Bit 9 - Compare All Channels"]
    #[inline(always)]
    pub fn cmpall(&mut self) -> CMPALL_W {
        CMPALL_W { w: self }
    }
    #[doc = "Bits 12:13 - Compare Event Filtering"]
    #[inline(always)]
    pub fn cmpfilter(&mut self) -> CMPFILTER_W {
        CMPFILTER_W { w: self }
    }
    #[doc = "Bits 16:18 - Resolution"]
    #[inline(always)]
    pub fn res(&mut self) -> RES_W {
        RES_W { w: self }
    }
    #[doc = "Bit 24 - TAG of the AFEC_LDCR"]
    #[inline(always)]
    pub fn tag(&mut self) -> TAG_W {
        TAG_W { w: self }
    }
    #[doc = "Bit 25 - Single Trigger Mode"]
    #[inline(always)]
    pub fn stm(&mut self) -> STM_W {
        STM_W { w: self }
    }
    #[doc = "Bits 28:29 - Sign Mode"]
    #[inline(always)]
    pub fn signmode(&mut self) -> SIGNMODE_W {
        SIGNMODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Extended Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_emr](index.html) module"]
pub struct AFEC_EMR_SPEC;
impl crate::RegisterSpec for AFEC_EMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afec_emr::R](R) reader structure"]
impl crate::Readable for AFEC_EMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afec_emr::W](W) writer structure"]
impl crate::Writable for AFEC_EMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFEC_EMR to value 0"]
impl crate::Resettable for AFEC_EMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
