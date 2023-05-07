#[doc = "Register `NMIFLAG` reader"]
pub struct R(crate::R<NMIFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NMIFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NMIFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NMIFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NMIFLAG` writer"]
pub struct W(crate::W<NMIFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NMIFLAG_SPEC>;
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
impl From<crate::W<NMIFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NMIFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NMI` reader - Non-Maskable Interrupt"]
pub type NMI_R = crate::BitReader<bool>;
#[doc = "Field `NMI` writer - Non-Maskable Interrupt"]
pub type NMI_W<'a, const O: u8> = crate::BitWriter<'a, u8, NMIFLAG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Non-Maskable Interrupt"]
    #[inline(always)]
    pub fn nmi(&self) -> NMI_R {
        NMI_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non-Maskable Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nmi(&mut self) -> NMI_W<0> {
        NMI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Non-Maskable Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmiflag](index.html) module"]
pub struct NMIFLAG_SPEC;
impl crate::RegisterSpec for NMIFLAG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [nmiflag::R](R) reader structure"]
impl crate::Readable for NMIFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nmiflag::W](W) writer structure"]
impl crate::Writable for NMIFLAG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NMIFLAG to value 0"]
impl crate::Resettable for NMIFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
