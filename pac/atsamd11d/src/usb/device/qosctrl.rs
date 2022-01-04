#[doc = "Register `QOSCTRL` reader"]
pub struct R(crate::R<QOSCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QOSCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QOSCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QOSCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QOSCTRL` writer"]
pub struct W(crate::W<QOSCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QOSCTRL_SPEC>;
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
impl From<crate::W<QOSCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QOSCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CQOS` reader - Configuration Quality of Service"]
pub struct CQOS_R(crate::FieldReader<u8, u8>);
impl CQOS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CQOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CQOS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CQOS` writer - Configuration Quality of Service"]
pub struct CQOS_W<'a> {
    w: &'a mut W,
}
impl<'a> CQOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u8 & 0x03);
        self.w
    }
}
#[doc = "Field `DQOS` reader - Data Quality of Service"]
pub struct DQOS_R(crate::FieldReader<u8, u8>);
impl DQOS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DQOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DQOS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQOS` writer - Data Quality of Service"]
pub struct DQOS_W<'a> {
    w: &'a mut W,
}
impl<'a> DQOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u8 & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Configuration Quality of Service"]
    #[inline(always)]
    pub fn cqos(&self) -> CQOS_R {
        CQOS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Data Quality of Service"]
    #[inline(always)]
    pub fn dqos(&self) -> DQOS_R {
        DQOS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configuration Quality of Service"]
    #[inline(always)]
    pub fn cqos(&mut self) -> CQOS_W {
        CQOS_W { w: self }
    }
    #[doc = "Bits 2:3 - Data Quality of Service"]
    #[inline(always)]
    pub fn dqos(&mut self) -> DQOS_W {
        DQOS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Quality Of Service\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qosctrl](index.html) module"]
pub struct QOSCTRL_SPEC;
impl crate::RegisterSpec for QOSCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [qosctrl::R](R) reader structure"]
impl crate::Readable for QOSCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qosctrl::W](W) writer structure"]
impl crate::Writable for QOSCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QOSCTRL to value 0x05"]
impl crate::Resettable for QOSCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x05
    }
}
