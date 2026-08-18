#[doc = "Register `BASEADDR` reader"]
pub struct R(crate::R<BASEADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BASEADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BASEADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BASEADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BASEADDR` writer"]
pub struct W(crate::W<BASEADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BASEADDR_SPEC>;
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
impl From<crate::W<BASEADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BASEADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASEADDR` reader - Descriptor Memory Base Address"]
pub struct BASEADDR_R(crate::FieldReader<u32, u32>);
impl BASEADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BASEADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BASEADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BASEADDR` writer - Descriptor Memory Base Address"]
pub struct BASEADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> BASEADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Descriptor Memory Base Address"]
    #[inline(always)]
    pub fn baseaddr(&self) -> BASEADDR_R {
        BASEADDR_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Descriptor Memory Base Address"]
    #[inline(always)]
    pub fn baseaddr(&mut self) -> BASEADDR_W {
        BASEADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Descriptor Memory Section Base Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baseaddr](index.html) module"]
pub struct BASEADDR_SPEC;
impl crate::RegisterSpec for BASEADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [baseaddr::R](R) reader structure"]
impl crate::Readable for BASEADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [baseaddr::W](W) writer structure"]
impl crate::Writable for BASEADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BASEADDR to value 0"]
impl crate::Resettable for BASEADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
