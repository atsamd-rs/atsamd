#[doc = "Register `SCH` reader"]
pub type R = crate::R<SCH_SPEC>;
#[doc = "Register `SCH` writer"]
pub type W = crate::W<SCH_SPEC>;
#[doc = "Field `SEC` reader - 1588 Timer Second comparison value"]
pub type SEC_R = crate::FieldReader<u16>;
#[doc = "Field `SEC` writer - 1588 Timer Second comparison value"]
pub type SEC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - 1588 Timer Second comparison value"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 1588 Timer Second comparison value"]
    #[inline(always)]
    #[must_use]
    pub fn sec(&mut self) -> SEC_W<SCH_SPEC, 0> {
        SEC_W::new(self)
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
#[doc = "Tsu timer second comparison Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCH_SPEC;
impl crate::RegisterSpec for SCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sch::R`](R) reader structure"]
impl crate::Readable for SCH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sch::W`](W) writer structure"]
impl crate::Writable for SCH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCH to value 0"]
impl crate::Resettable for SCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
