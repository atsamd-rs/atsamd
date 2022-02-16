#[doc = "Register `US_IDTRX` reader"]
pub struct R(crate::R<US_IDTRX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_IDTRX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_IDTRX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_IDTRX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_IDTRX` writer"]
pub struct W(crate::W<US_IDTRX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_IDTRX_SPEC>;
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
impl From<crate::W<US_IDTRX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_IDTRX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDTRX` reader - LON Indeterminate Time after Reception (comm_type = 1 mode only)"]
pub struct IDTRX_R(crate::FieldReader<u32, u32>);
impl IDTRX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        IDTRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDTRX_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDTRX` writer - LON Indeterminate Time after Reception (comm_type = 1 mode only)"]
pub struct IDTRX_W<'a> {
    w: &'a mut W,
}
impl<'a> IDTRX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - LON Indeterminate Time after Reception (comm_type = 1 mode only)"]
    #[inline(always)]
    pub fn idtrx(&self) -> IDTRX_R {
        IDTRX_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - LON Indeterminate Time after Reception (comm_type = 1 mode only)"]
    #[inline(always)]
    pub fn idtrx(&mut self) -> IDTRX_W {
        IDTRX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LON IDT Rx Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_idtrx](index.html) module"]
pub struct US_IDTRX_SPEC;
impl crate::RegisterSpec for US_IDTRX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_idtrx::R](R) reader structure"]
impl crate::Readable for US_IDTRX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_idtrx::W](W) writer structure"]
impl crate::Writable for US_IDTRX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets US_IDTRX to value 0"]
impl crate::Resettable for US_IDTRX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
