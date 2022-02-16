#[doc = "Register `RTT_AR` reader"]
pub struct R(crate::R<RTT_AR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTT_AR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTT_AR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTT_AR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTT_AR` writer"]
pub struct W(crate::W<RTT_AR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTT_AR_SPEC>;
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
impl From<crate::W<RTT_AR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTT_AR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALMV` reader - Alarm Value"]
pub struct ALMV_R(crate::FieldReader<u32, u32>);
impl ALMV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ALMV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALMV_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALMV` writer - Alarm Value"]
pub struct ALMV_W<'a> {
    w: &'a mut W,
}
impl<'a> ALMV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Alarm Value"]
    #[inline(always)]
    pub fn almv(&self) -> ALMV_R {
        ALMV_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Alarm Value"]
    #[inline(always)]
    pub fn almv(&mut self) -> ALMV_W {
        ALMV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtt_ar](index.html) module"]
pub struct RTT_AR_SPEC;
impl crate::RegisterSpec for RTT_AR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtt_ar::R](R) reader structure"]
impl crate::Readable for RTT_AR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtt_ar::W](W) writer structure"]
impl crate::Writable for RTT_AR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTT_AR to value 0"]
impl crate::Resettable for RTT_AR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
