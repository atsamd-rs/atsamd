#[doc = "Register `CRCSTATUS` reader"]
pub struct R(crate::R<CRCSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCSTATUS` writer"]
pub struct W(crate::W<CRCSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCSTATUS_SPEC>;
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
impl From<crate::W<CRCSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCBUSY` reader - CRC Module Busy"]
pub struct CRCBUSY_R(crate::FieldReader<bool, bool>);
impl CRCBUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRCBUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCBUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCBUSY` writer - CRC Module Busy"]
pub struct CRCBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCBUSY_W<'a> {
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
#[doc = "Field `CRCZERO` reader - CRC Zero"]
pub struct CRCZERO_R(crate::FieldReader<bool, bool>);
impl CRCZERO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRCZERO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCZERO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - CRC Module Busy"]
    #[inline(always)]
    pub fn crcbusy(&self) -> CRCBUSY_R {
        CRCBUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CRC Zero"]
    #[inline(always)]
    pub fn crczero(&self) -> CRCZERO_R {
        CRCZERO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Module Busy"]
    #[inline(always)]
    pub fn crcbusy(&mut self) -> CRCBUSY_W {
        CRCBUSY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcstatus](index.html) module"]
pub struct CRCSTATUS_SPEC;
impl crate::RegisterSpec for CRCSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [crcstatus::R](R) reader structure"]
impl crate::Readable for CRCSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crcstatus::W](W) writer structure"]
impl crate::Writable for CRCSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRCSTATUS to value 0"]
impl crate::Resettable for CRCSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
