#[doc = "Register `US_IDTTX` reader"]
pub struct R(crate::R<US_IDTTX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_IDTTX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_IDTTX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_IDTTX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_IDTTX` writer"]
pub struct W(crate::W<US_IDTTX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_IDTTX_SPEC>;
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
impl From<crate::W<US_IDTTX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_IDTTX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDTTX` reader - LON Indeterminate Time after Transmission (comm_type = 1 mode only)"]
pub struct IDTTX_R(crate::FieldReader<u32, u32>);
impl IDTTX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        IDTTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDTTX_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDTTX` writer - LON Indeterminate Time after Transmission (comm_type = 1 mode only)"]
pub struct IDTTX_W<'a> {
    w: &'a mut W,
}
impl<'a> IDTTX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - LON Indeterminate Time after Transmission (comm_type = 1 mode only)"]
    #[inline(always)]
    pub fn idttx(&self) -> IDTTX_R {
        IDTTX_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - LON Indeterminate Time after Transmission (comm_type = 1 mode only)"]
    #[inline(always)]
    pub fn idttx(&mut self) -> IDTTX_W {
        IDTTX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LON IDT Tx Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_idttx](index.html) module"]
pub struct US_IDTTX_SPEC;
impl crate::RegisterSpec for US_IDTTX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_idttx::R](R) reader structure"]
impl crate::Readable for US_IDTTX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_idttx::W](W) writer structure"]
impl crate::Writable for US_IDTTX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets US_IDTTX to value 0"]
impl crate::Resettable for US_IDTTX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
