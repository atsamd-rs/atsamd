#[doc = "Register `TSSSL` reader"]
pub struct R(crate::R<TSSSL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSSSL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSSSL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSSSL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSSSL` writer"]
pub struct W(crate::W<TSSSL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSSSL_SPEC>;
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
impl From<crate::W<TSSSL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSSSL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VTS` reader - Value of Timer Seconds Register Capture"]
pub struct VTS_R(crate::FieldReader<u32, u32>);
impl VTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        VTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTS` writer - Value of Timer Seconds Register Capture"]
pub struct VTS_W<'a> {
    w: &'a mut W,
}
impl<'a> VTS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Value of Timer Seconds Register Capture"]
    #[inline(always)]
    pub fn vts(&self) -> VTS_R {
        VTS_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value of Timer Seconds Register Capture"]
    #[inline(always)]
    pub fn vts(&mut self) -> VTS_W {
        VTS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1588 Timer Sync Strobe Seconds \\[31:0\\]
Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsssl](index.html) module"]
pub struct TSSSL_SPEC;
impl crate::RegisterSpec for TSSSL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsssl::R](R) reader structure"]
impl crate::Readable for TSSSL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsssl::W](W) writer structure"]
impl crate::Writable for TSSSL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSSSL to value 0"]
impl crate::Resettable for TSSSL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
