#[doc = "Register `FFCR` reader"]
pub struct R(crate::R<FFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FFCR` writer"]
pub struct W(crate::W<FFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFCR_SPEC>;
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
impl From<crate::W<FFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EnFCont` reader - "]
pub type EN_FCONT_R = crate::BitReader<bool>;
#[doc = "Field `EnFCont` writer - "]
pub type EN_FCONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFCR_SPEC, bool, O>;
#[doc = "Field `TrigIn` reader - "]
pub type TRIG_IN_R = crate::BitReader<bool>;
#[doc = "Field `TrigIn` writer - "]
pub type TRIG_IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn en_fcont(&self) -> EN_FCONT_R {
        EN_FCONT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn trig_in(&self) -> TRIG_IN_R {
        TRIG_IN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn en_fcont(&mut self) -> EN_FCONT_W<1> {
        EN_FCONT_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn trig_in(&mut self) -> TRIG_IN_W<8> {
        TRIG_IN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Formatter and Flush Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffcr](index.html) module"]
pub struct FFCR_SPEC;
impl crate::RegisterSpec for FFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffcr::R](R) reader structure"]
impl crate::Readable for FFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ffcr::W](W) writer structure"]
impl crate::Writable for FFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FFCR to value 0"]
impl crate::Resettable for FFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
