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
pub type LSBTIR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LSBTIR` writer - Lower Significant Bits of Timer Increment"]
pub type LSBTIR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TISUBN_SPEC, u16, u16, 16, O>;
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
    #[must_use]
    pub fn lsbtir(&mut self) -> LSBTIR_W<0> {
        LSBTIR_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TISUBN to value 0"]
impl crate::Resettable for TISUBN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
