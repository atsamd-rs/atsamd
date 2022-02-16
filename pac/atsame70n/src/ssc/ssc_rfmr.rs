#[doc = "Register `SSC_RFMR` reader"]
pub struct R(crate::R<SSC_RFMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSC_RFMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSC_RFMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSC_RFMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSC_RFMR` writer"]
pub struct W(crate::W<SSC_RFMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSC_RFMR_SPEC>;
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
impl From<crate::W<SSC_RFMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSC_RFMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATLEN` reader - Data Length"]
pub struct DATLEN_R(crate::FieldReader<u8, u8>);
impl DATLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATLEN` writer - Data Length"]
pub struct DATLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `LOOP` reader - Loop Mode"]
pub struct LOOP_R(crate::FieldReader<bool, bool>);
impl LOOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOOP` writer - Loop Mode"]
pub struct LOOP_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOP_W<'a> {
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
#[doc = "Field `MSBF` reader - Most Significant Bit First"]
pub struct MSBF_R(crate::FieldReader<bool, bool>);
impl MSBF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MSBF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSBF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSBF` writer - Most Significant Bit First"]
pub struct MSBF_W<'a> {
    w: &'a mut W,
}
impl<'a> MSBF_W<'a> {
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
#[doc = "Field `DATNB` reader - Data Number per Frame"]
pub struct DATNB_R(crate::FieldReader<u8, u8>);
impl DATNB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATNB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATNB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATNB` writer - Data Number per Frame"]
pub struct DATNB_W<'a> {
    w: &'a mut W,
}
impl<'a> DATNB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `FSLEN` reader - Receive Frame Sync Length"]
pub struct FSLEN_R(crate::FieldReader<u8, u8>);
impl FSLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FSLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSLEN` writer - Receive Frame Sync Length"]
pub struct FSLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FSLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Receive Frame Sync Output Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FSOS_A {
    #[doc = "0: None, RF pin is an input"]
    NONE = 0,
    #[doc = "1: Negative Pulse, RF pin is an output"]
    NEGATIVE = 1,
    #[doc = "2: Positive Pulse, RF pin is an output"]
    POSITIVE = 2,
    #[doc = "3: Driven Low during data transfer, RF pin is an output"]
    LOW = 3,
    #[doc = "4: Driven High during data transfer, RF pin is an output"]
    HIGH = 4,
    #[doc = "5: Toggling at each start of data transfer, RF pin is an output"]
    TOGGLING = 5,
}
impl From<FSOS_A> for u8 {
    #[inline(always)]
    fn from(variant: FSOS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FSOS` reader - Receive Frame Sync Output Selection"]
pub struct FSOS_R(crate::FieldReader<u8, FSOS_A>);
impl FSOS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FSOS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FSOS_A> {
        match self.bits {
            0 => Some(FSOS_A::NONE),
            1 => Some(FSOS_A::NEGATIVE),
            2 => Some(FSOS_A::POSITIVE),
            3 => Some(FSOS_A::LOW),
            4 => Some(FSOS_A::HIGH),
            5 => Some(FSOS_A::TOGGLING),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == FSOS_A::NONE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE`"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        **self == FSOS_A::NEGATIVE
    }
    #[doc = "Checks if the value of the field is `POSITIVE`"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        **self == FSOS_A::POSITIVE
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == FSOS_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == FSOS_A::HIGH
    }
    #[doc = "Checks if the value of the field is `TOGGLING`"]
    #[inline(always)]
    pub fn is_toggling(&self) -> bool {
        **self == FSOS_A::TOGGLING
    }
}
impl core::ops::Deref for FSOS_R {
    type Target = crate::FieldReader<u8, FSOS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSOS` writer - Receive Frame Sync Output Selection"]
pub struct FSOS_W<'a> {
    w: &'a mut W,
}
impl<'a> FSOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSOS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "None, RF pin is an input"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(FSOS_A::NONE)
    }
    #[doc = "Negative Pulse, RF pin is an output"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(FSOS_A::NEGATIVE)
    }
    #[doc = "Positive Pulse, RF pin is an output"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(FSOS_A::POSITIVE)
    }
    #[doc = "Driven Low during data transfer, RF pin is an output"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(FSOS_A::LOW)
    }
    #[doc = "Driven High during data transfer, RF pin is an output"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(FSOS_A::HIGH)
    }
    #[doc = "Toggling at each start of data transfer, RF pin is an output"]
    #[inline(always)]
    pub fn toggling(self) -> &'a mut W {
        self.variant(FSOS_A::TOGGLING)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Frame Sync Edge Detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSEDGE_A {
    #[doc = "0: Positive Edge Detection"]
    POSITIVE = 0,
    #[doc = "1: Negative Edge Detection"]
    NEGATIVE = 1,
}
impl From<FSEDGE_A> for bool {
    #[inline(always)]
    fn from(variant: FSEDGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSEDGE` reader - Frame Sync Edge Detection"]
pub struct FSEDGE_R(crate::FieldReader<bool, FSEDGE_A>);
impl FSEDGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSEDGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEDGE_A {
        match self.bits {
            false => FSEDGE_A::POSITIVE,
            true => FSEDGE_A::NEGATIVE,
        }
    }
    #[doc = "Checks if the value of the field is `POSITIVE`"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        **self == FSEDGE_A::POSITIVE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE`"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        **self == FSEDGE_A::NEGATIVE
    }
}
impl core::ops::Deref for FSEDGE_R {
    type Target = crate::FieldReader<bool, FSEDGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSEDGE` writer - Frame Sync Edge Detection"]
pub struct FSEDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> FSEDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSEDGE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Positive Edge Detection"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(FSEDGE_A::POSITIVE)
    }
    #[doc = "Negative Edge Detection"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(FSEDGE_A::NEGATIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `FSLEN_EXT` reader - FSLEN Field Extension"]
pub struct FSLEN_EXT_R(crate::FieldReader<u8, u8>);
impl FSLEN_EXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FSLEN_EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSLEN_EXT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSLEN_EXT` writer - FSLEN Field Extension"]
pub struct FSLEN_EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> FSLEN_EXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Data Length"]
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Loop Mode"]
    #[inline(always)]
    pub fn loop_(&self) -> LOOP_R {
        LOOP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Most Significant Bit First"]
    #[inline(always)]
    pub fn msbf(&self) -> MSBF_R {
        MSBF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Data Number per Frame"]
    #[inline(always)]
    pub fn datnb(&self) -> DATNB_R {
        DATNB_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Receive Frame Sync Length"]
    #[inline(always)]
    pub fn fslen(&self) -> FSLEN_R {
        FSLEN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Receive Frame Sync Output Selection"]
    #[inline(always)]
    pub fn fsos(&self) -> FSOS_R {
        FSOS_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 24 - Frame Sync Edge Detection"]
    #[inline(always)]
    pub fn fsedge(&self) -> FSEDGE_R {
        FSEDGE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 28:31 - FSLEN Field Extension"]
    #[inline(always)]
    pub fn fslen_ext(&self) -> FSLEN_EXT_R {
        FSLEN_EXT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Data Length"]
    #[inline(always)]
    pub fn datlen(&mut self) -> DATLEN_W {
        DATLEN_W { w: self }
    }
    #[doc = "Bit 5 - Loop Mode"]
    #[inline(always)]
    pub fn loop_(&mut self) -> LOOP_W {
        LOOP_W { w: self }
    }
    #[doc = "Bit 7 - Most Significant Bit First"]
    #[inline(always)]
    pub fn msbf(&mut self) -> MSBF_W {
        MSBF_W { w: self }
    }
    #[doc = "Bits 8:11 - Data Number per Frame"]
    #[inline(always)]
    pub fn datnb(&mut self) -> DATNB_W {
        DATNB_W { w: self }
    }
    #[doc = "Bits 16:19 - Receive Frame Sync Length"]
    #[inline(always)]
    pub fn fslen(&mut self) -> FSLEN_W {
        FSLEN_W { w: self }
    }
    #[doc = "Bits 20:22 - Receive Frame Sync Output Selection"]
    #[inline(always)]
    pub fn fsos(&mut self) -> FSOS_W {
        FSOS_W { w: self }
    }
    #[doc = "Bit 24 - Frame Sync Edge Detection"]
    #[inline(always)]
    pub fn fsedge(&mut self) -> FSEDGE_W {
        FSEDGE_W { w: self }
    }
    #[doc = "Bits 28:31 - FSLEN Field Extension"]
    #[inline(always)]
    pub fn fslen_ext(&mut self) -> FSLEN_EXT_W {
        FSLEN_EXT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Frame Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssc_rfmr](index.html) module"]
pub struct SSC_RFMR_SPEC;
impl crate::RegisterSpec for SSC_RFMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssc_rfmr::R](R) reader structure"]
impl crate::Readable for SSC_RFMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssc_rfmr::W](W) writer structure"]
impl crate::Writable for SSC_RFMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSC_RFMR to value 0"]
impl crate::Resettable for SSC_RFMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
