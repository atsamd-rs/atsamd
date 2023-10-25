#[doc = "Register `TN` reader"]
pub type R = crate::R<TN_SPEC>;
#[doc = "Register `TN` writer"]
pub type W = crate::W<TN_SPEC>;
#[doc = "Field `TNS` reader - Timer Count in Nanoseconds"]
pub type TNS_R = crate::FieldReader<u32>;
#[doc = "Field `TNS` writer - Timer Count in Nanoseconds"]
pub type TNS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 30, O, u32>;
impl R {
    #[doc = "Bits 0:29 - Timer Count in Nanoseconds"]
    #[inline(always)]
    pub fn tns(&self) -> TNS_R {
        TNS_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Timer Count in Nanoseconds"]
    #[inline(always)]
    #[must_use]
    pub fn tns(&mut self) -> TNS_W<TN_SPEC, 0> {
        TNS_W::new(self)
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
#[doc = "1588 Timer Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TN_SPEC;
impl crate::RegisterSpec for TN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tn::R`](R) reader structure"]
impl crate::Readable for TN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tn::W`](W) writer structure"]
impl crate::Writable for TN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TN to value 0"]
impl crate::Resettable for TN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
