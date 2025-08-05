#[doc = "Register `CFGA` reader"]
pub struct R(crate::R<CFGA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGA` writer"]
pub struct W(crate::W<CFGA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGA_SPEC>;
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
impl From<crate::W<CFGA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REFNUM` reader - Number of Reference Clock Cycles"]
pub struct REFNUM_R(crate::FieldReader<u8, u8>);
impl REFNUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REFNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFNUM` writer - Number of Reference Clock Cycles"]
pub struct REFNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> REFNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Number of Reference Clock Cycles"]
    #[inline(always)]
    pub fn refnum(&self) -> REFNUM_R {
        REFNUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of Reference Clock Cycles"]
    #[inline(always)]
    pub fn refnum(&mut self) -> REFNUM_W {
        REFNUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Config A register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfga](index.html) module"]
pub struct CFGA_SPEC;
impl crate::RegisterSpec for CFGA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cfga::R](R) reader structure"]
impl crate::Readable for CFGA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfga::W](W) writer structure"]
impl crate::Writable for CFGA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGA to value 0"]
impl crate::Resettable for CFGA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
