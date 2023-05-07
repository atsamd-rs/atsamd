#[doc = "Register `LCKWAY` reader"]
pub struct R(crate::R<LCKWAY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCKWAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCKWAY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCKWAY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCKWAY` writer"]
pub struct W(crate::W<LCKWAY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCKWAY_SPEC>;
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
impl From<crate::W<LCKWAY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCKWAY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCKWAY` reader - Lockdown way Register"]
pub type LCKWAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LCKWAY` writer - Lockdown way Register"]
pub type LCKWAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCKWAY_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Lockdown way Register"]
    #[inline(always)]
    pub fn lckway(&self) -> LCKWAY_R {
        LCKWAY_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Lockdown way Register"]
    #[inline(always)]
    #[must_use]
    pub fn lckway(&mut self) -> LCKWAY_W<0> {
        LCKWAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Lock per Way Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lckway](index.html) module"]
pub struct LCKWAY_SPEC;
impl crate::RegisterSpec for LCKWAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lckway::R](R) reader structure"]
impl crate::Readable for LCKWAY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lckway::W](W) writer structure"]
impl crate::Writable for LCKWAY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCKWAY to value 0"]
impl crate::Resettable for LCKWAY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
