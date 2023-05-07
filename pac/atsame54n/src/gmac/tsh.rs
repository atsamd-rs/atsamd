#[doc = "Register `TSH` reader"]
pub struct R(crate::R<TSH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSH` writer"]
pub struct W(crate::W<TSH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSH_SPEC>;
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
impl From<crate::W<TSH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCS` reader - Timer Count in Seconds"]
pub type TCS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TCS` writer - Timer Count in Seconds"]
pub type TCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSH_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Timer Count in Seconds"]
    #[inline(always)]
    pub fn tcs(&self) -> TCS_R {
        TCS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer Count in Seconds"]
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
#[doc = "1588 Timer Seconds High \\[15:0\\]
Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsh](index.html) module"]
pub struct TSH_SPEC;
impl crate::RegisterSpec for TSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsh::R](R) reader structure"]
impl crate::Readable for TSH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsh::W](W) writer structure"]
impl crate::Writable for TSH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSH to value 0"]
impl crate::Resettable for TSH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
