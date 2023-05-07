#[doc = "Register `PMUX[%s]` reader"]
pub struct R(crate::R<PMUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMUX[%s]` writer"]
pub struct W(crate::W<PMUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMUX_SPEC>;
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
impl From<crate::W<PMUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMUXE` reader - Peripheral Multiplexing for Even-Numbered Pin"]
pub type PMUXE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PMUXE` writer - Peripheral Multiplexing for Even-Numbered Pin"]
pub type PMUXE_W<'a, const O: u8> = crate::FieldWriter<'a, u8, PMUX_SPEC, u8, u8, 4, O>;
#[doc = "Field `PMUXO` reader - Peripheral Multiplexing for Odd-Numbered Pin"]
pub type PMUXO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PMUXO` writer - Peripheral Multiplexing for Odd-Numbered Pin"]
pub type PMUXO_W<'a, const O: u8> = crate::FieldWriter<'a, u8, PMUX_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Peripheral Multiplexing for Even-Numbered Pin"]
    #[inline(always)]
    pub fn pmuxe(&self) -> PMUXE_R {
        PMUXE_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Peripheral Multiplexing for Odd-Numbered Pin"]
    #[inline(always)]
    pub fn pmuxo(&self) -> PMUXO_R {
        PMUXO_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Peripheral Multiplexing for Even-Numbered Pin"]
    #[inline(always)]
    #[must_use]
    pub fn pmuxe(&mut self) -> PMUXE_W<0> {
        PMUXE_W::new(self)
    }
    #[doc = "Bits 4:7 - Peripheral Multiplexing for Odd-Numbered Pin"]
    #[inline(always)]
    #[must_use]
    pub fn pmuxo(&mut self) -> PMUXO_W<4> {
        PMUXO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Multiplexing\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmux](index.html) module"]
pub struct PMUX_SPEC;
impl crate::RegisterSpec for PMUX_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pmux::R](R) reader structure"]
impl crate::Readable for PMUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmux::W](W) writer structure"]
impl crate::Writable for PMUX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMUX[%s]
to value 0"]
impl crate::Resettable for PMUX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
