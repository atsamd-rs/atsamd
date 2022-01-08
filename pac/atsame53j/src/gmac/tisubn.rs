#[doc = "Register `TISUBN` reader"]
pub struct R(crate::R<TISUBN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TISUBN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TISUBN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TISUBN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TISUBN` writer"]
pub struct W(crate::W<TISUBN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TISUBN_SPEC>;
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
impl From<crate::W<TISUBN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TISUBN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSBTIR` reader - Lower Significant Bits of Timer Increment"]
pub struct LSBTIR_R(crate::FieldReader<u16, u16>);
impl LSBTIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LSBTIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSBTIR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSBTIR` writer - Lower Significant Bits of Timer Increment"]
pub struct LSBTIR_W<'a> {
    w: &'a mut W,
}
impl<'a> LSBTIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Lower Significant Bits of Timer Increment"]
    #[inline(always)]
    pub fn lsbtir(&self) -> LSBTIR_R {
        LSBTIR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Lower Significant Bits of Timer Increment"]
    #[inline(always)]
    pub fn lsbtir(&mut self) -> LSBTIR_W {
        LSBTIR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1588 Timer Increment \\[15:0\\]
Sub-Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tisubn](index.html) module"]
pub struct TISUBN_SPEC;
impl crate::RegisterSpec for TISUBN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tisubn::R](R) reader structure"]
impl crate::Readable for TISUBN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tisubn::W](W) writer structure"]
impl crate::Writable for TISUBN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TISUBN to value 0"]
impl crate::Resettable for TISUBN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
