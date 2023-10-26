#[doc = "Register `DFSR` reader"]
pub type R = crate::R<DFSR_SPEC>;
#[doc = "Register `DFSR` writer"]
pub type W = crate::W<DFSR_SPEC>;
#[doc = "Field `HALTED` reader - "]
pub type HALTED_R = crate::BitReader;
#[doc = "Field `HALTED` writer - "]
pub type HALTED_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BKPT` reader - "]
pub type BKPT_R = crate::BitReader;
#[doc = "Field `BKPT` writer - "]
pub type BKPT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DWTTRAP` reader - "]
pub type DWTTRAP_R = crate::BitReader;
#[doc = "Field `DWTTRAP` writer - "]
pub type DWTTRAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VCATCH` reader - "]
pub type VCATCH_R = crate::BitReader;
#[doc = "Field `VCATCH` writer - "]
pub type VCATCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTERNAL` reader - "]
pub type EXTERNAL_R = crate::BitReader;
#[doc = "Field `EXTERNAL` writer - "]
pub type EXTERNAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn halted(&mut self) -> HALTED_W<DFSR_SPEC, 0> {
        HALTED_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn bkpt(&mut self) -> BKPT_W<DFSR_SPEC, 1> {
        BKPT_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn dwttrap(&mut self) -> DWTTRAP_W<DFSR_SPEC, 2> {
        DWTTRAP_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn vcatch(&mut self) -> VCATCH_W<DFSR_SPEC, 3> {
        VCATCH_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn external(&mut self) -> EXTERNAL_W<DFSR_SPEC, 4> {
        EXTERNAL_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Debug Fault Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSR_SPEC;
impl crate::RegisterSpec for DFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsr::R`](R) reader structure"]
impl crate::Readable for DFSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dfsr::W`](W) writer structure"]
impl crate::Writable for DFSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFSR to value 0"]
impl crate::Resettable for DFSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
