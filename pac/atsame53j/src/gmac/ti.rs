#[doc = "Register `TI` reader"]
pub type R = crate::R<TI_SPEC>;
#[doc = "Register `TI` writer"]
pub type W = crate::W<TI_SPEC>;
#[doc = "Field `CNS` reader - Count Nanoseconds"]
pub type CNS_R = crate::FieldReader;
#[doc = "Field `CNS` writer - Count Nanoseconds"]
pub type CNS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `ACNS` reader - Alternative Count Nanoseconds"]
pub type ACNS_R = crate::FieldReader;
#[doc = "Field `ACNS` writer - Alternative Count Nanoseconds"]
pub type ACNS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `NIT` reader - Number of Increments"]
pub type NIT_R = crate::FieldReader;
#[doc = "Field `NIT` writer - Number of Increments"]
pub type NIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Count Nanoseconds"]
    #[inline(always)]
    pub fn cns(&self) -> CNS_R {
        CNS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Alternative Count Nanoseconds"]
    #[inline(always)]
    pub fn acns(&self) -> ACNS_R {
        ACNS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of Increments"]
    #[inline(always)]
    pub fn nit(&self) -> NIT_R {
        NIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Count Nanoseconds"]
    #[inline(always)]
    #[must_use]
    pub fn cns(&mut self) -> CNS_W<TI_SPEC, 0> {
        CNS_W::new(self)
    }
    #[doc = "Bits 8:15 - Alternative Count Nanoseconds"]
    #[inline(always)]
    #[must_use]
    pub fn acns(&mut self) -> ACNS_W<TI_SPEC, 8> {
        ACNS_W::new(self)
    }
    #[doc = "Bits 16:23 - Number of Increments"]
    #[inline(always)]
    #[must_use]
    pub fn nit(&mut self) -> NIT_W<TI_SPEC, 16> {
        NIT_W::new(self)
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
#[doc = "1588 Timer Increment Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ti::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ti::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TI_SPEC;
impl crate::RegisterSpec for TI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ti::R`](R) reader structure"]
impl crate::Readable for TI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ti::W`](W) writer structure"]
impl crate::Writable for TI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TI to value 0"]
impl crate::Resettable for TI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
