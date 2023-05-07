#[doc = "Register `TSL` reader"]
pub struct R(crate::R<TSL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSL` writer"]
pub struct W(crate::W<TSL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSL_SPEC>;
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
impl From<crate::W<TSL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCS` reader - Timer Count in Seconds"]
pub type TCS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TCS` writer - Timer Count in Seconds"]
pub type TCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Timer Count in Seconds"]
    #[inline(always)]
    pub fn tcs(&self) -> TCS_R {
        TCS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer Count in Seconds"]
    #[inline(always)]
    #[must_use]
    pub fn tcs(&mut self) -> TCS_W<0> {
        TCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1588 Timer Seconds \\[31:0\\]
Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsl](index.html) module"]
pub struct TSL_SPEC;
impl crate::RegisterSpec for TSL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsl::R](R) reader structure"]
impl crate::Readable for TSL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsl::W](W) writer structure"]
impl crate::Writable for TSL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSL to value 0"]
impl crate::Resettable for TSL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
