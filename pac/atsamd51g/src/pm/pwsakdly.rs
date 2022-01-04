#[doc = "Register `PWSAKDLY` reader"]
pub struct R(crate::R<PWSAKDLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWSAKDLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWSAKDLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWSAKDLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWSAKDLY` writer"]
pub struct W(crate::W<PWSAKDLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWSAKDLY_SPEC>;
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
impl From<crate::W<PWSAKDLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWSAKDLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLYVAL` reader - Delay Value"]
pub struct DLYVAL_R(crate::FieldReader<u8, u8>);
impl DLYVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DLYVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLYVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYVAL` writer - Delay Value"]
pub struct DLYVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u8 & 0x7f);
        self.w
    }
}
#[doc = "Field `IGNACK` reader - Ignore Acknowledge"]
pub struct IGNACK_R(crate::FieldReader<bool, bool>);
impl IGNACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IGNACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IGNACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IGNACK` writer - Ignore Acknowledge"]
pub struct IGNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> IGNACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Delay Value"]
    #[inline(always)]
    pub fn dlyval(&self) -> DLYVAL_R {
        DLYVAL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Ignore Acknowledge"]
    #[inline(always)]
    pub fn ignack(&self) -> IGNACK_R {
        IGNACK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Delay Value"]
    #[inline(always)]
    pub fn dlyval(&mut self) -> DLYVAL_W {
        DLYVAL_W { w: self }
    }
    #[doc = "Bit 7 - Ignore Acknowledge"]
    #[inline(always)]
    pub fn ignack(&mut self) -> IGNACK_W {
        IGNACK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Switch Acknowledge Delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwsakdly](index.html) module"]
pub struct PWSAKDLY_SPEC;
impl crate::RegisterSpec for PWSAKDLY_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pwsakdly::R](R) reader structure"]
impl crate::Readable for PWSAKDLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwsakdly::W](W) writer structure"]
impl crate::Writable for PWSAKDLY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWSAKDLY to value 0"]
impl crate::Resettable for PWSAKDLY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
