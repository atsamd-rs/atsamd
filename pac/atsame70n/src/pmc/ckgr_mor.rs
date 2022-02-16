#[doc = "Register `CKGR_MOR` reader"]
pub struct R(crate::R<CKGR_MOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKGR_MOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKGR_MOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKGR_MOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKGR_MOR` writer"]
pub struct W(crate::W<CKGR_MOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKGR_MOR_SPEC>;
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
impl From<crate::W<CKGR_MOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKGR_MOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MOSCXTEN` reader - Main Crystal Oscillator Enable"]
pub struct MOSCXTEN_R(crate::FieldReader<bool, bool>);
impl MOSCXTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MOSCXTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOSCXTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOSCXTEN` writer - Main Crystal Oscillator Enable"]
pub struct MOSCXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSCXTEN_W<'a> {
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
#[doc = "Field `MOSCXTBY` reader - Main Crystal Oscillator Bypass"]
pub struct MOSCXTBY_R(crate::FieldReader<bool, bool>);
impl MOSCXTBY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MOSCXTBY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOSCXTBY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOSCXTBY` writer - Main Crystal Oscillator Bypass"]
pub struct MOSCXTBY_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSCXTBY_W<'a> {
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
#[doc = "Field `WAITMODE` reader - Wait Mode Command (Write-only)"]
pub struct WAITMODE_R(crate::FieldReader<bool, bool>);
impl WAITMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAITMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAITMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAITMODE` writer - Wait Mode Command (Write-only)"]
pub struct WAITMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITMODE_W<'a> {
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
#[doc = "Field `MOSCRCEN` reader - Main RC Oscillator Enable"]
pub struct MOSCRCEN_R(crate::FieldReader<bool, bool>);
impl MOSCRCEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MOSCRCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOSCRCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOSCRCEN` writer - Main RC Oscillator Enable"]
pub struct MOSCRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSCRCEN_W<'a> {
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
#[doc = "Main RC Oscillator Frequency Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MOSCRCF_A {
    #[doc = "0: The RC oscillator frequency is at 4 MHz"]
    _4_MHZ = 0,
    #[doc = "1: The RC oscillator frequency is at 8 MHz"]
    _8_MHZ = 1,
    #[doc = "2: The RC oscillator frequency is at 12 MHz"]
    _12_MHZ = 2,
}
impl From<MOSCRCF_A> for u8 {
    #[inline(always)]
    fn from(variant: MOSCRCF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MOSCRCF` reader - Main RC Oscillator Frequency Selection"]
pub struct MOSCRCF_R(crate::FieldReader<u8, MOSCRCF_A>);
impl MOSCRCF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MOSCRCF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MOSCRCF_A> {
        match self.bits {
            0 => Some(MOSCRCF_A::_4_MHZ),
            1 => Some(MOSCRCF_A::_8_MHZ),
            2 => Some(MOSCRCF_A::_12_MHZ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_4_MHZ`"]
    #[inline(always)]
    pub fn is_4_mhz(&self) -> bool {
        **self == MOSCRCF_A::_4_MHZ
    }
    #[doc = "Checks if the value of the field is `_8_MHZ`"]
    #[inline(always)]
    pub fn is_8_mhz(&self) -> bool {
        **self == MOSCRCF_A::_8_MHZ
    }
    #[doc = "Checks if the value of the field is `_12_MHZ`"]
    #[inline(always)]
    pub fn is_12_mhz(&self) -> bool {
        **self == MOSCRCF_A::_12_MHZ
    }
}
impl core::ops::Deref for MOSCRCF_R {
    type Target = crate::FieldReader<u8, MOSCRCF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOSCRCF` writer - Main RC Oscillator Frequency Selection"]
pub struct MOSCRCF_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSCRCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOSCRCF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The RC oscillator frequency is at 4 MHz"]
    #[inline(always)]
    pub fn _4_mhz(self) -> &'a mut W {
        self.variant(MOSCRCF_A::_4_MHZ)
    }
    #[doc = "The RC oscillator frequency is at 8 MHz"]
    #[inline(always)]
    pub fn _8_mhz(self) -> &'a mut W {
        self.variant(MOSCRCF_A::_8_MHZ)
    }
    #[doc = "The RC oscillator frequency is at 12 MHz"]
    #[inline(always)]
    pub fn _12_mhz(self) -> &'a mut W {
        self.variant(MOSCRCF_A::_12_MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `MOSCXTST` reader - Main Crystal Oscillator Startup Time"]
pub struct MOSCXTST_R(crate::FieldReader<u8, u8>);
impl MOSCXTST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MOSCXTST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOSCXTST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOSCXTST` writer - Main Crystal Oscillator Startup Time"]
pub struct MOSCXTST_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSCXTST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Write Access Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KEY_A {
    #[doc = "55: Writing any other value in this field aborts the write operation.Always reads as 0."]
    PASSWD = 55,
}
impl From<KEY_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY` reader - Write Access Password"]
pub struct KEY_R(crate::FieldReader<u8, KEY_A>);
impl KEY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KEY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEY_A> {
        match self.bits {
            55 => Some(KEY_A::PASSWD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PASSWD`"]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        **self == KEY_A::PASSWD
    }
}
impl core::ops::Deref for KEY_R {
    type Target = crate::FieldReader<u8, KEY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY` writer - Write Access Password"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(KEY_A::PASSWD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `MOSCSEL` reader - Main Clock Oscillator Selection"]
pub struct MOSCSEL_R(crate::FieldReader<bool, bool>);
impl MOSCSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MOSCSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOSCSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOSCSEL` writer - Main Clock Oscillator Selection"]
pub struct MOSCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSCSEL_W<'a> {
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
#[doc = "Field `CFDEN` reader - Clock Failure Detector Enable"]
pub struct CFDEN_R(crate::FieldReader<bool, bool>);
impl CFDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFDEN` writer - Clock Failure Detector Enable"]
pub struct CFDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CFDEN_W<'a> {
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
#[doc = "Field `XT32KFME` reader - 32.768 kHz Crystal Oscillator Frequency Monitoring Enable"]
pub struct XT32KFME_R(crate::FieldReader<bool, bool>);
impl XT32KFME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XT32KFME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XT32KFME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XT32KFME` writer - 32.768 kHz Crystal Oscillator Frequency Monitoring Enable"]
pub struct XT32KFME_W<'a> {
    w: &'a mut W,
}
impl<'a> XT32KFME_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Main Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn moscxten(&self) -> MOSCXTEN_R {
        MOSCXTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Main Crystal Oscillator Bypass"]
    #[inline(always)]
    pub fn moscxtby(&self) -> MOSCXTBY_R {
        MOSCXTBY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wait Mode Command (Write-only)"]
    #[inline(always)]
    pub fn waitmode(&self) -> WAITMODE_R {
        WAITMODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Main RC Oscillator Enable"]
    #[inline(always)]
    pub fn moscrcen(&self) -> MOSCRCEN_R {
        MOSCRCEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Main RC Oscillator Frequency Selection"]
    #[inline(always)]
    pub fn moscrcf(&self) -> MOSCRCF_R {
        MOSCRCF_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:15 - Main Crystal Oscillator Startup Time"]
    #[inline(always)]
    pub fn moscxtst(&self) -> MOSCXTST_R {
        MOSCXTST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Write Access Password"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Main Clock Oscillator Selection"]
    #[inline(always)]
    pub fn moscsel(&self) -> MOSCSEL_R {
        MOSCSEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CFDEN_R {
        CFDEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 32.768 kHz Crystal Oscillator Frequency Monitoring Enable"]
    #[inline(always)]
    pub fn xt32kfme(&self) -> XT32KFME_R {
        XT32KFME_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Main Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn moscxten(&mut self) -> MOSCXTEN_W {
        MOSCXTEN_W { w: self }
    }
    #[doc = "Bit 1 - Main Crystal Oscillator Bypass"]
    #[inline(always)]
    pub fn moscxtby(&mut self) -> MOSCXTBY_W {
        MOSCXTBY_W { w: self }
    }
    #[doc = "Bit 2 - Wait Mode Command (Write-only)"]
    #[inline(always)]
    pub fn waitmode(&mut self) -> WAITMODE_W {
        WAITMODE_W { w: self }
    }
    #[doc = "Bit 3 - Main RC Oscillator Enable"]
    #[inline(always)]
    pub fn moscrcen(&mut self) -> MOSCRCEN_W {
        MOSCRCEN_W { w: self }
    }
    #[doc = "Bits 4:6 - Main RC Oscillator Frequency Selection"]
    #[inline(always)]
    pub fn moscrcf(&mut self) -> MOSCRCF_W {
        MOSCRCF_W { w: self }
    }
    #[doc = "Bits 8:15 - Main Crystal Oscillator Startup Time"]
    #[inline(always)]
    pub fn moscxtst(&mut self) -> MOSCXTST_W {
        MOSCXTST_W { w: self }
    }
    #[doc = "Bits 16:23 - Write Access Password"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
    #[doc = "Bit 24 - Main Clock Oscillator Selection"]
    #[inline(always)]
    pub fn moscsel(&mut self) -> MOSCSEL_W {
        MOSCSEL_W { w: self }
    }
    #[doc = "Bit 25 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&mut self) -> CFDEN_W {
        CFDEN_W { w: self }
    }
    #[doc = "Bit 26 - 32.768 kHz Crystal Oscillator Frequency Monitoring Enable"]
    #[inline(always)]
    pub fn xt32kfme(&mut self) -> XT32KFME_W {
        XT32KFME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Oscillator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckgr_mor](index.html) module"]
pub struct CKGR_MOR_SPEC;
impl crate::RegisterSpec for CKGR_MOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckgr_mor::R](R) reader structure"]
impl crate::Readable for CKGR_MOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckgr_mor::W](W) writer structure"]
impl crate::Writable for CKGR_MOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CKGR_MOR to value 0"]
impl crate::Resettable for CKGR_MOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
