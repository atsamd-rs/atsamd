#[doc = "Register `CCBUF[%s]` reader"]
pub struct R(crate::R<CCBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCBUF[%s]` writer"]
pub struct W(crate::W<CCBUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCBUF_SPEC>;
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
impl From<crate::W<CCBUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCBUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCBUF` reader - Channel Compare Buffer Value"]
pub struct CCBUF_R(crate::FieldReader<u16, u16>);
impl CCBUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CCBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCBUF_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCBUF` writer - Channel Compare Buffer Value"]
pub struct CCBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> CCBUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Channel Compare Buffer Value"]
    #[inline(always)]
    pub fn ccbuf(&self) -> CCBUF_R {
        CCBUF_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel Compare Buffer Value"]
    #[inline(always)]
    pub fn ccbuf(&mut self) -> CCBUF_W {
        CCBUF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Compare Buffer Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccbuf](index.html) module"]
pub struct CCBUF_SPEC;
impl crate::RegisterSpec for CCBUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccbuf::R](R) reader structure"]
impl crate::Readable for CCBUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccbuf::W](W) writer structure"]
impl crate::Writable for CCBUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCBUF[%s]
to value 0"]
impl crate::Resettable for CCBUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
