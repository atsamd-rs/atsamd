#[doc = "Register `SCL` reader"]
pub type R = crate::R<SCL_SPEC>;
#[doc = "Register `SCL` writer"]
pub type W = crate::W<SCL_SPEC>;
#[doc = "Field `SEC` reader - 1588 Timer Second comparison value"]
pub type SEC_R = crate::FieldReader<u32>;
#[doc = "Field `SEC` writer - 1588 Timer Second comparison value"]
pub type SEC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - 1588 Timer Second comparison value"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1588 Timer Second comparison value"]
    #[inline(always)]
    #[must_use]
    pub fn sec(&mut self) -> SEC_W<SCL_SPEC, 0> {
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
#[doc = "Tsu timer second comparison Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCL_SPEC;
impl crate::RegisterSpec for SCL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl::R`](R) reader structure"]
impl crate::Readable for SCL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scl::W`](W) writer structure"]
impl crate::Writable for SCL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCL to value 0"]
impl crate::Resettable for SCL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
