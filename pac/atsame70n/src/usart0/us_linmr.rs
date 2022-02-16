#[doc = "Register `US_LINMR` reader"]
pub struct R(crate::R<US_LINMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_LINMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_LINMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_LINMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_LINMR` writer"]
pub struct W(crate::W<US_LINMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_LINMR_SPEC>;
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
impl From<crate::W<US_LINMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_LINMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LIN Node Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NACT_A {
    #[doc = "0: The USART transmits the response."]
    PUBLISH = 0,
    #[doc = "1: The USART receives the response."]
    SUBSCRIBE = 1,
    #[doc = "2: The USART does not transmit and does not receive the response."]
    IGNORE = 2,
}
impl From<NACT_A> for u8 {
    #[inline(always)]
    fn from(variant: NACT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NACT` reader - LIN Node Action"]
pub struct NACT_R(crate::FieldReader<u8, NACT_A>);
impl NACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NACT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NACT_A> {
        match self.bits {
            0 => Some(NACT_A::PUBLISH),
            1 => Some(NACT_A::SUBSCRIBE),
            2 => Some(NACT_A::IGNORE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PUBLISH`"]
    #[inline(always)]
    pub fn is_publish(&self) -> bool {
        **self == NACT_A::PUBLISH
    }
    #[doc = "Checks if the value of the field is `SUBSCRIBE`"]
    #[inline(always)]
    pub fn is_subscribe(&self) -> bool {
        **self == NACT_A::SUBSCRIBE
    }
    #[doc = "Checks if the value of the field is `IGNORE`"]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        **self == NACT_A::IGNORE
    }
}
impl core::ops::Deref for NACT_R {
    type Target = crate::FieldReader<u8, NACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NACT` writer - LIN Node Action"]
pub struct NACT_W<'a> {
    w: &'a mut W,
}
impl<'a> NACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NACT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The USART transmits the response."]
    #[inline(always)]
    pub fn publish(self) -> &'a mut W {
        self.variant(NACT_A::PUBLISH)
    }
    #[doc = "The USART receives the response."]
    #[inline(always)]
    pub fn subscribe(self) -> &'a mut W {
        self.variant(NACT_A::SUBSCRIBE)
    }
    #[doc = "The USART does not transmit and does not receive the response."]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut W {
        self.variant(NACT_A::IGNORE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `PARDIS` reader - Parity Disable"]
pub struct PARDIS_R(crate::FieldReader<bool, bool>);
impl PARDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PARDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARDIS` writer - Parity Disable"]
pub struct PARDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PARDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CHKDIS` reader - Checksum Disable"]
pub struct CHKDIS_R(crate::FieldReader<bool, bool>);
impl CHKDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHKDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHKDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHKDIS` writer - Checksum Disable"]
pub struct CHKDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CHKDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CHKTYP` reader - Checksum Type"]
pub struct CHKTYP_R(crate::FieldReader<bool, bool>);
impl CHKTYP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHKTYP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHKTYP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHKTYP` writer - Checksum Type"]
pub struct CHKTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> CHKTYP_W<'a> {
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
#[doc = "Field `DLM` reader - Data Length Mode"]
pub struct DLM_R(crate::FieldReader<bool, bool>);
impl DLM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DLM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLM` writer - Data Length Mode"]
pub struct DLM_W<'a> {
    w: &'a mut W,
}
impl<'a> DLM_W<'a> {
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
#[doc = "Field `FSDIS` reader - Frame Slot Mode Disable"]
pub struct FSDIS_R(crate::FieldReader<bool, bool>);
impl FSDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSDIS` writer - Frame Slot Mode Disable"]
pub struct FSDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FSDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `WKUPTYP` reader - Wakeup Signal Type"]
pub struct WKUPTYP_R(crate::FieldReader<bool, bool>);
impl WKUPTYP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKUPTYP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPTYP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPTYP` writer - Wakeup Signal Type"]
pub struct WKUPTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPTYP_W<'a> {
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
#[doc = "Field `DLC` reader - Data Length Control"]
pub struct DLC_R(crate::FieldReader<u8, u8>);
impl DLC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DLC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLC` writer - Data Length Control"]
pub struct DLC_W<'a> {
    w: &'a mut W,
}
impl<'a> DLC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `PDCM` reader - DMAC Mode"]
pub struct PDCM_R(crate::FieldReader<bool, bool>);
impl PDCM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDCM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDCM` writer - DMAC Mode"]
pub struct PDCM_W<'a> {
    w: &'a mut W,
}
impl<'a> PDCM_W<'a> {
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
#[doc = "Field `SYNCDIS` reader - Synchronization Disable"]
pub struct SYNCDIS_R(crate::FieldReader<bool, bool>);
impl SYNCDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYNCDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCDIS` writer - Synchronization Disable"]
pub struct SYNCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCDIS_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - LIN Node Action"]
    #[inline(always)]
    pub fn nact(&self) -> NACT_R {
        NACT_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Parity Disable"]
    #[inline(always)]
    pub fn pardis(&self) -> PARDIS_R {
        PARDIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Checksum Disable"]
    #[inline(always)]
    pub fn chkdis(&self) -> CHKDIS_R {
        CHKDIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Checksum Type"]
    #[inline(always)]
    pub fn chktyp(&self) -> CHKTYP_R {
        CHKTYP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Data Length Mode"]
    #[inline(always)]
    pub fn dlm(&self) -> DLM_R {
        DLM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Frame Slot Mode Disable"]
    #[inline(always)]
    pub fn fsdis(&self) -> FSDIS_R {
        FSDIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Wakeup Signal Type"]
    #[inline(always)]
    pub fn wkuptyp(&self) -> WKUPTYP_R {
        WKUPTYP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Data Length Control"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - DMAC Mode"]
    #[inline(always)]
    pub fn pdcm(&self) -> PDCM_R {
        PDCM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Synchronization Disable"]
    #[inline(always)]
    pub fn syncdis(&self) -> SYNCDIS_R {
        SYNCDIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - LIN Node Action"]
    #[inline(always)]
    pub fn nact(&mut self) -> NACT_W {
        NACT_W { w: self }
    }
    #[doc = "Bit 2 - Parity Disable"]
    #[inline(always)]
    pub fn pardis(&mut self) -> PARDIS_W {
        PARDIS_W { w: self }
    }
    #[doc = "Bit 3 - Checksum Disable"]
    #[inline(always)]
    pub fn chkdis(&mut self) -> CHKDIS_W {
        CHKDIS_W { w: self }
    }
    #[doc = "Bit 4 - Checksum Type"]
    #[inline(always)]
    pub fn chktyp(&mut self) -> CHKTYP_W {
        CHKTYP_W { w: self }
    }
    #[doc = "Bit 5 - Data Length Mode"]
    #[inline(always)]
    pub fn dlm(&mut self) -> DLM_W {
        DLM_W { w: self }
    }
    #[doc = "Bit 6 - Frame Slot Mode Disable"]
    #[inline(always)]
    pub fn fsdis(&mut self) -> FSDIS_W {
        FSDIS_W { w: self }
    }
    #[doc = "Bit 7 - Wakeup Signal Type"]
    #[inline(always)]
    pub fn wkuptyp(&mut self) -> WKUPTYP_W {
        WKUPTYP_W { w: self }
    }
    #[doc = "Bits 8:15 - Data Length Control"]
    #[inline(always)]
    pub fn dlc(&mut self) -> DLC_W {
        DLC_W { w: self }
    }
    #[doc = "Bit 16 - DMAC Mode"]
    #[inline(always)]
    pub fn pdcm(&mut self) -> PDCM_W {
        PDCM_W { w: self }
    }
    #[doc = "Bit 17 - Synchronization Disable"]
    #[inline(always)]
    pub fn syncdis(&mut self) -> SYNCDIS_W {
        SYNCDIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LIN Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_linmr](index.html) module"]
pub struct US_LINMR_SPEC;
impl crate::RegisterSpec for US_LINMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_linmr::R](R) reader structure"]
impl crate::Readable for US_LINMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_linmr::W](W) writer structure"]
impl crate::Writable for US_LINMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets US_LINMR to value 0"]
impl crate::Resettable for US_LINMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
