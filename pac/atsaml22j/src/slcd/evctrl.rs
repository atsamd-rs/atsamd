#[doc = "Register `EVCTRL` reader"]
pub struct R(crate::R<EVCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVCTRL` writer"]
pub struct W(crate::W<EVCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVCTRL_SPEC>;
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
impl From<crate::W<EVCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FC0OEO` reader - Frame Counter 0 Overflow Event Output Enable"]
pub type FC0OEO_R = crate::BitReader<bool>;
#[doc = "Field `FC0OEO` writer - Frame Counter 0 Overflow Event Output Enable"]
pub type FC0OEO_W<'a, const O: u8> = crate::BitWriter<'a, u8, EVCTRL_SPEC, bool, O>;
#[doc = "Field `FC1OEO` reader - Frame Counter 1 Overflow Event Output Enable"]
pub type FC1OEO_R = crate::BitReader<bool>;
#[doc = "Field `FC1OEO` writer - Frame Counter 1 Overflow Event Output Enable"]
pub type FC1OEO_W<'a, const O: u8> = crate::BitWriter<'a, u8, EVCTRL_SPEC, bool, O>;
#[doc = "Field `FC2OEO` reader - Frame Counter 2 Overflow Event Output Enable"]
pub type FC2OEO_R = crate::BitReader<bool>;
#[doc = "Field `FC2OEO` writer - Frame Counter 2 Overflow Event Output Enable"]
pub type FC2OEO_W<'a, const O: u8> = crate::BitWriter<'a, u8, EVCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Frame Counter 0 Overflow Event Output Enable"]
    #[inline(always)]
    pub fn fc0oeo(&self) -> FC0OEO_R {
        FC0OEO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame Counter 1 Overflow Event Output Enable"]
    #[inline(always)]
    pub fn fc1oeo(&self) -> FC1OEO_R {
        FC1OEO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Frame Counter 2 Overflow Event Output Enable"]
    #[inline(always)]
    pub fn fc2oeo(&self) -> FC2OEO_R {
        FC2OEO_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frame Counter 0 Overflow Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fc0oeo(&mut self) -> FC0OEO_W<0> {
        FC0OEO_W::new(self)
    }
    #[doc = "Bit 1 - Frame Counter 1 Overflow Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fc1oeo(&mut self) -> FC1OEO_W<1> {
        FC1OEO_W::new(self)
    }
    #[doc = "Bit 2 - Frame Counter 2 Overflow Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fc2oeo(&mut self) -> FC2OEO_W<2> {
        FC2OEO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evctrl](index.html) module"]
pub struct EVCTRL_SPEC;
impl crate::RegisterSpec for EVCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [evctrl::R](R) reader structure"]
impl crate::Readable for EVCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evctrl::W](W) writer structure"]
impl crate::Writable for EVCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EVCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
