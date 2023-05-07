#[doc = "Register `DFSR` reader"]
pub struct R(crate::R<DFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFSR` writer"]
pub struct W(crate::W<DFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFSR_SPEC>;
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
impl From<crate::W<DFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HALTED` reader - "]
pub type HALTED_R = crate::BitReader<bool>;
#[doc = "Field `HALTED` writer - "]
pub type HALTED_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSR_SPEC, bool, O>;
#[doc = "Field `BKPT` reader - "]
pub type BKPT_R = crate::BitReader<bool>;
#[doc = "Field `BKPT` writer - "]
pub type BKPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSR_SPEC, bool, O>;
#[doc = "Field `DWTTRAP` reader - "]
pub type DWTTRAP_R = crate::BitReader<bool>;
#[doc = "Field `DWTTRAP` writer - "]
pub type DWTTRAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSR_SPEC, bool, O>;
#[doc = "Field `VCATCH` reader - "]
pub type VCATCH_R = crate::BitReader<bool>;
#[doc = "Field `VCATCH` writer - "]
pub type VCATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSR_SPEC, bool, O>;
#[doc = "Field `EXTERNAL` reader - "]
pub type EXTERNAL_R = crate::BitReader<bool>;
#[doc = "Field `EXTERNAL` writer - "]
pub type EXTERNAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn halted(&self) -> HALTED_R {
        HALTED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bkpt(&self) -> BKPT_R {
        BKPT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dwttrap(&self) -> DWTTRAP_R {
        DWTTRAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn vcatch(&self) -> VCATCH_R {
        VCATCH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn external(&self) -> EXTERNAL_R {
        EXTERNAL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn halted(&mut self) -> HALTED_W<0> {
        HALTED_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn bkpt(&mut self) -> BKPT_W<1> {
        BKPT_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn dwttrap(&mut self) -> DWTTRAP_W<2> {
        DWTTRAP_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn vcatch(&mut self) -> VCATCH_W<3> {
        VCATCH_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn external(&mut self) -> EXTERNAL_W<4> {
        EXTERNAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Fault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsr](index.html) module"]
pub struct DFSR_SPEC;
impl crate::RegisterSpec for DFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsr::R](R) reader structure"]
impl crate::Readable for DFSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfsr::W](W) writer structure"]
impl crate::Writable for DFSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFSR to value 0"]
impl crate::Resettable for DFSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
