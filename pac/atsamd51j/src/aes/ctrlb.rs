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
#[doc = "Field `START` reader - Start Encryption/Decryption"]
pub struct START_R(crate::FieldReader<bool, bool>);
impl START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START` writer - Start Encryption/Decryption"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `NEWMSG` reader - New message"]
pub struct NEWMSG_R(crate::FieldReader<bool, bool>);
impl NEWMSG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NEWMSG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NEWMSG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NEWMSG` writer - New message"]
pub struct NEWMSG_W<'a> {
    w: &'a mut W,
}
impl<'a> NEWMSG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `EOM` reader - End of message"]
pub struct EOM_R(crate::FieldReader<bool, bool>);
impl EOM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOM` writer - End of message"]
pub struct EOM_W<'a> {
    w: &'a mut W,
}
impl<'a> EOM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `GFMUL` reader - GF Multiplication"]
pub struct GFMUL_R(crate::FieldReader<bool, bool>);
impl GFMUL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GFMUL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GFMUL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GFMUL` writer - GF Multiplication"]
pub struct GFMUL_W<'a> {
    w: &'a mut W,
}
impl<'a> GFMUL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Start Encryption/Decryption"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - New message"]
    #[inline(always)]
    pub fn newmsg(&self) -> NEWMSG_R {
        NEWMSG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - End of message"]
    #[inline(always)]
    pub fn eom(&self) -> EOM_R {
        EOM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GF Multiplication"]
    #[inline(always)]
    pub fn gfmul(&self) -> GFMUL_R {
        GFMUL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start Encryption/Decryption"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 1 - New message"]
    #[inline(always)]
    pub fn newmsg(&mut self) -> NEWMSG_W {
        NEWMSG_W { w: self }
    }
    #[doc = "Bit 2 - End of message"]
    #[inline(always)]
    pub fn eom(&mut self) -> EOM_W {
        EOM_W { w: self }
    }
    #[doc = "Bit 3 - GF Multiplication"]
    #[inline(always)]
    pub fn gfmul(&mut self) -> GFMUL_W {
        GFMUL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u8;
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
