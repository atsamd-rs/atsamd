#[doc = "Register `QSPI_IAR` reader"]
pub struct R(crate::R<QSPI_IAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_IAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_IAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_IAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_IAR` writer"]
pub struct W(crate::W<QSPI_IAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_IAR_SPEC>;
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
impl From<crate::W<QSPI_IAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_IAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Address"]
pub struct ADDR_R(crate::FieldReader<u32, u32>);
impl ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR` writer - Address"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Instruction Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_iar](index.html) module"]
pub struct QSPI_IAR_SPEC;
impl crate::RegisterSpec for QSPI_IAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_iar::R](R) reader structure"]
impl crate::Readable for QSPI_IAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_iar::W](W) writer structure"]
impl crate::Writable for QSPI_IAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_IAR to value 0"]
impl crate::Resettable for QSPI_IAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
