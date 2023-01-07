#[doc = "Register `WRBADDR` reader"]
pub struct R(crate::R<WRBADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRBADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRBADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRBADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRBADDR` writer"]
pub struct W(crate::W<WRBADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRBADDR_SPEC>;
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
impl From<crate::W<WRBADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRBADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRBADDR` reader - Write-Back Memory Base Address"]
pub struct WRBADDR_R(crate::FieldReader<u32, u32>);
impl WRBADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        WRBADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRBADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRBADDR` writer - Write-Back Memory Base Address"]
pub struct WRBADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> WRBADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Write-Back Memory Base Address"]
    #[inline(always)]
    pub fn wrbaddr(&self) -> WRBADDR_R {
        WRBADDR_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write-Back Memory Base Address"]
    #[inline(always)]
    pub fn wrbaddr(&mut self) -> WRBADDR_W {
        WRBADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write-Back Memory Section Base Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrbaddr](index.html) module"]
pub struct WRBADDR_SPEC;
impl crate::RegisterSpec for WRBADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrbaddr::R](R) reader structure"]
impl crate::Readable for WRBADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrbaddr::W](W) writer structure"]
impl crate::Writable for WRBADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WRBADDR to value 0"]
impl crate::Resettable for WRBADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
