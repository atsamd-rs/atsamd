#[doc = "Register `GMAC_CBSCR` reader"]
pub struct R(crate::R<GMAC_CBSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_CBSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_CBSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_CBSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GMAC_CBSCR` writer"]
pub struct W(crate::W<GMAC_CBSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GMAC_CBSCR_SPEC>;
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
impl From<crate::W<GMAC_CBSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GMAC_CBSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QBE` reader - Queue B CBS Enable"]
pub struct QBE_R(crate::FieldReader<bool, bool>);
impl QBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QBE` writer - Queue B CBS Enable"]
pub struct QBE_W<'a> {
    w: &'a mut W,
}
impl<'a> QBE_W<'a> {
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
#[doc = "Field `QAE` reader - Queue A CBS Enable"]
pub struct QAE_R(crate::FieldReader<bool, bool>);
impl QAE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QAE` writer - Queue A CBS Enable"]
pub struct QAE_W<'a> {
    w: &'a mut W,
}
impl<'a> QAE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Queue B CBS Enable"]
    #[inline(always)]
    pub fn qbe(&self) -> QBE_R {
        QBE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Queue A CBS Enable"]
    #[inline(always)]
    pub fn qae(&self) -> QAE_R {
        QAE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Queue B CBS Enable"]
    #[inline(always)]
    pub fn qbe(&mut self) -> QBE_W {
        QBE_W { w: self }
    }
    #[doc = "Bit 1 - Queue A CBS Enable"]
    #[inline(always)]
    pub fn qae(&mut self) -> QAE_W {
        QAE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Credit-Based Shaping Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_cbscr](index.html) module"]
pub struct GMAC_CBSCR_SPEC;
impl crate::RegisterSpec for GMAC_CBSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_cbscr::R](R) reader structure"]
impl crate::Readable for GMAC_CBSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gmac_cbscr::W](W) writer structure"]
impl crate::Writable for GMAC_CBSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GMAC_CBSCR to value 0"]
impl crate::Resettable for GMAC_CBSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
