#[doc = "Register `MAN` reader"]
pub struct R(crate::R<MAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAN` writer"]
pub struct W(crate::W<MAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAN_SPEC>;
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
impl From<crate::W<MAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - PHY Data"]
pub struct DATA_R(crate::FieldReader<u16, u16>);
impl DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA` writer - PHY Data"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `WTN` reader - Write Ten"]
pub struct WTN_R(crate::FieldReader<u8, u8>);
impl WTN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WTN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WTN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WTN` writer - Write Ten"]
pub struct WTN_W<'a> {
    w: &'a mut W,
}
impl<'a> WTN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `REGA` reader - Register Address"]
pub struct REGA_R(crate::FieldReader<u8, u8>);
impl REGA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REGA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGA` writer - Register Address"]
pub struct REGA_W<'a> {
    w: &'a mut W,
}
impl<'a> REGA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 18)) | ((value as u32 & 0x1f) << 18);
        self.w
    }
}
#[doc = "Field `PHYA` reader - PHY Address"]
pub struct PHYA_R(crate::FieldReader<u8, u8>);
impl PHYA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PHYA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHYA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHYA` writer - PHY Address"]
pub struct PHYA_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 23)) | ((value as u32 & 0x1f) << 23);
        self.w
    }
}
#[doc = "Field `OP` reader - Operation"]
pub struct OP_R(crate::FieldReader<u8, u8>);
impl OP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OP` writer - Operation"]
pub struct OP_W<'a> {
    w: &'a mut W,
}
impl<'a> OP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `CLTTO` reader - Clause 22 Operation"]
pub struct CLTTO_R(crate::FieldReader<bool, bool>);
impl CLTTO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLTTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLTTO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLTTO` writer - Clause 22 Operation"]
pub struct CLTTO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLTTO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `WZO` reader - Write ZERO"]
pub struct WZO_R(crate::FieldReader<bool, bool>);
impl WZO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WZO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WZO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WZO` writer - Write ZERO"]
pub struct WZO_W<'a> {
    w: &'a mut W,
}
impl<'a> WZO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PHY Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - Write Ten"]
    #[inline(always)]
    pub fn wtn(&self) -> WTN_R {
        WTN_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:22 - Register Address"]
    #[inline(always)]
    pub fn rega(&self) -> REGA_R {
        REGA_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:27 - PHY Address"]
    #[inline(always)]
    pub fn phya(&self) -> PHYA_R {
        PHYA_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29 - Operation"]
    #[inline(always)]
    pub fn op(&self) -> OP_R {
        OP_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 30 - Clause 22 Operation"]
    #[inline(always)]
    pub fn cltto(&self) -> CLTTO_R {
        CLTTO_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Write ZERO"]
    #[inline(always)]
    pub fn wzo(&self) -> WZO_R {
        WZO_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - PHY Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Bits 16:17 - Write Ten"]
    #[inline(always)]
    pub fn wtn(&mut self) -> WTN_W {
        WTN_W { w: self }
    }
    #[doc = "Bits 18:22 - Register Address"]
    #[inline(always)]
    pub fn rega(&mut self) -> REGA_W {
        REGA_W { w: self }
    }
    #[doc = "Bits 23:27 - PHY Address"]
    #[inline(always)]
    pub fn phya(&mut self) -> PHYA_W {
        PHYA_W { w: self }
    }
    #[doc = "Bits 28:29 - Operation"]
    #[inline(always)]
    pub fn op(&mut self) -> OP_W {
        OP_W { w: self }
    }
    #[doc = "Bit 30 - Clause 22 Operation"]
    #[inline(always)]
    pub fn cltto(&mut self) -> CLTTO_W {
        CLTTO_W { w: self }
    }
    #[doc = "Bit 31 - Write ZERO"]
    #[inline(always)]
    pub fn wzo(&mut self) -> WZO_W {
        WZO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Maintenance Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [man](index.html) module"]
pub struct MAN_SPEC;
impl crate::RegisterSpec for MAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [man::R](R) reader structure"]
impl crate::Readable for MAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [man::W](W) writer structure"]
impl crate::Writable for MAN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAN to value 0"]
impl crate::Resettable for MAN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
