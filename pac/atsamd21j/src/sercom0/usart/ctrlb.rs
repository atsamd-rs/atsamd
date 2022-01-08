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
#[doc = "Field `CHSIZE` reader - Character Size"]
pub struct CHSIZE_R(crate::FieldReader<u8, u8>);
impl CHSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSIZE` writer - Character Size"]
pub struct CHSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `SBMODE` reader - Stop Bit Mode"]
pub struct SBMODE_R(crate::FieldReader<bool, bool>);
impl SBMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SBMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBMODE` writer - Stop Bit Mode"]
pub struct SBMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SBMODE_W<'a> {
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
#[doc = "Field `COLDEN` reader - Collision Detection Enable"]
pub struct COLDEN_R(crate::FieldReader<bool, bool>);
impl COLDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COLDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COLDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COLDEN` writer - Collision Detection Enable"]
pub struct COLDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> COLDEN_W<'a> {
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
#[doc = "Field `SFDE` reader - Start of Frame Detection Enable"]
pub struct SFDE_R(crate::FieldReader<bool, bool>);
impl SFDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SFDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SFDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFDE` writer - Start of Frame Detection Enable"]
pub struct SFDE_W<'a> {
    w: &'a mut W,
}
impl<'a> SFDE_W<'a> {
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
#[doc = "Field `ENC` reader - Encoding Format"]
pub struct ENC_R(crate::FieldReader<bool, bool>);
impl ENC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENC` writer - Encoding Format"]
pub struct ENC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENC_W<'a> {
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
#[doc = "Field `PMODE` reader - Parity Mode"]
pub struct PMODE_R(crate::FieldReader<bool, bool>);
impl PMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMODE` writer - Parity Mode"]
pub struct PMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PMODE_W<'a> {
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
#[doc = "Field `TXEN` reader - Transmitter Enable"]
pub struct TXEN_R(crate::FieldReader<bool, bool>);
impl TXEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub struct TXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEN_W<'a> {
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
#[doc = "Field `RXEN` reader - Receiver Enable"]
pub struct RXEN_R(crate::FieldReader<bool, bool>);
impl RXEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub struct RXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEN_W<'a> {
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
    #[doc = "Bits 0:2 - Character Size"]
    #[inline(always)]
    pub fn chsize(&self) -> CHSIZE_R {
        CHSIZE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 6 - Stop Bit Mode"]
    #[inline(always)]
    pub fn sbmode(&self) -> SBMODE_R {
        SBMODE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Collision Detection Enable"]
    #[inline(always)]
    pub fn colden(&self) -> COLDEN_R {
        COLDEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Start of Frame Detection Enable"]
    #[inline(always)]
    pub fn sfde(&self) -> SFDE_R {
        SFDE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Encoding Format"]
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Parity Mode"]
    #[inline(always)]
    pub fn pmode(&self) -> PMODE_R {
        PMODE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Character Size"]
    #[inline(always)]
    pub fn chsize(&mut self) -> CHSIZE_W {
        CHSIZE_W { w: self }
    }
    #[doc = "Bit 6 - Stop Bit Mode"]
    #[inline(always)]
    pub fn sbmode(&mut self) -> SBMODE_W {
        SBMODE_W { w: self }
    }
    #[doc = "Bit 8 - Collision Detection Enable"]
    #[inline(always)]
    pub fn colden(&mut self) -> COLDEN_W {
        COLDEN_W { w: self }
    }
    #[doc = "Bit 9 - Start of Frame Detection Enable"]
    #[inline(always)]
    pub fn sfde(&mut self) -> SFDE_W {
        SFDE_W { w: self }
    }
    #[doc = "Bit 10 - Encoding Format"]
    #[inline(always)]
    pub fn enc(&mut self) -> ENC_W {
        ENC_W { w: self }
    }
    #[doc = "Bit 13 - Parity Mode"]
    #[inline(always)]
    pub fn pmode(&mut self) -> PMODE_W {
        PMODE_W { w: self }
    }
    #[doc = "Bit 16 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen(&mut self) -> TXEN_W {
        TXEN_W { w: self }
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RXEN_W {
        RXEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u32;
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
