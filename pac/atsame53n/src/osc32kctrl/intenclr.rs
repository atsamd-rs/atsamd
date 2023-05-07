#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XOSC32KRDY` reader - XOSC32K Ready Interrupt Enable"]
pub type XOSC32KRDY_R = crate::BitReader<bool>;
#[doc = "Field `XOSC32KRDY` writer - XOSC32K Ready Interrupt Enable"]
pub type XOSC32KRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, O>;
#[doc = "Field `XOSC32KFAIL` reader - XOSC32K Clock Failure Detector Interrupt Enable"]
pub type XOSC32KFAIL_R = crate::BitReader<bool>;
#[doc = "Field `XOSC32KFAIL` writer - XOSC32K Clock Failure Detector Interrupt Enable"]
pub type XOSC32KFAIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - XOSC32K Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xosc32krdy(&self) -> XOSC32KRDY_R {
        XOSC32KRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - XOSC32K Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    pub fn xosc32kfail(&self) -> XOSC32KFAIL_R {
        XOSC32KFAIL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XOSC32K Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xosc32krdy(&mut self) -> XOSC32KRDY_W<0> {
        XOSC32KRDY_W::new(self)
    }
    #[doc = "Bit 2 - XOSC32K Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xosc32kfail(&mut self) -> XOSC32KFAIL_W<2> {
        XOSC32KFAIL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
