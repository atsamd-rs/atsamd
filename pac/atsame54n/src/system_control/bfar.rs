#[doc = "Register `BFAR` reader"]
pub struct R(crate::R<BFAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BFAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BFAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BFAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BFAR` writer"]
pub struct W(crate::W<BFAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BFAR_SPEC>;
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
impl From<crate::W<BFAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BFAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRESS` reader - Address that generated the BusFault"]
pub type ADDRESS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDRESS` writer - Address that generated the BusFault"]
pub type ADDRESS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BFAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Address that generated the BusFault"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address that generated the BusFault"]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<0> {
        ADDRESS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BusFault Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bfar](index.html) module"]
pub struct BFAR_SPEC;
impl crate::RegisterSpec for BFAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bfar::R](R) reader structure"]
impl crate::Readable for BFAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bfar::W](W) writer structure"]
impl crate::Writable for BFAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BFAR to value 0"]
impl crate::Resettable for BFAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
