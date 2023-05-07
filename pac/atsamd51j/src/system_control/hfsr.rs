#[doc = "Register `HFSR` reader"]
pub struct R(crate::R<HFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFSR` writer"]
pub struct W(crate::W<HFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFSR_SPEC>;
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
impl From<crate::W<HFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VECTTBL` reader - BusFault on a Vector Table read during exception processing"]
pub type VECTTBL_R = crate::BitReader<bool>;
#[doc = "Field `VECTTBL` writer - BusFault on a Vector Table read during exception processing"]
pub type VECTTBL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFSR_SPEC, bool, O>;
#[doc = "Field `FORCED` reader - Forced Hard Fault"]
pub type FORCED_R = crate::BitReader<bool>;
#[doc = "Field `FORCED` writer - Forced Hard Fault"]
pub type FORCED_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFSR_SPEC, bool, O>;
#[doc = "Field `DEBUGEVT` reader - Debug: always write 0"]
pub type DEBUGEVT_R = crate::BitReader<bool>;
#[doc = "Field `DEBUGEVT` writer - Debug: always write 0"]
pub type DEBUGEVT_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - BusFault on a Vector Table read during exception processing"]
    #[inline(always)]
    pub fn vecttbl(&self) -> VECTTBL_R {
        VECTTBL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 30 - Forced Hard Fault"]
    #[inline(always)]
    pub fn forced(&self) -> FORCED_R {
        FORCED_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Debug: always write 0"]
    #[inline(always)]
    pub fn debugevt(&self) -> DEBUGEVT_R {
        DEBUGEVT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - BusFault on a Vector Table read during exception processing"]
    #[inline(always)]
    #[must_use]
    pub fn vecttbl(&mut self) -> VECTTBL_W<1> {
        VECTTBL_W::new(self)
    }
    #[doc = "Bit 30 - Forced Hard Fault"]
    #[inline(always)]
    #[must_use]
    pub fn forced(&mut self) -> FORCED_W<30> {
        FORCED_W::new(self)
    }
    #[doc = "Bit 31 - Debug: always write 0"]
    #[inline(always)]
    #[must_use]
    pub fn debugevt(&mut self) -> DEBUGEVT_W<31> {
        DEBUGEVT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HardFault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfsr](index.html) module"]
pub struct HFSR_SPEC;
impl crate::RegisterSpec for HFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfsr::R](R) reader structure"]
impl crate::Readable for HFSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfsr::W](W) writer structure"]
impl crate::Writable for HFSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HFSR to value 0"]
impl crate::Resettable for HFSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
