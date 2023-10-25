#[doc = "Register `COUNT` reader"]
pub type R = crate::R<COUNT_SPEC>;
#[doc = "Register `COUNT` writer"]
pub type W = crate::W<COUNT_SPEC>;
#[doc = "Field `COUNT` reader - Counter Value"]
pub type COUNT_R = crate::FieldReader;
#[doc = "Field `COUNT` writer - Counter Value"]
pub type COUNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Counter Value"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<COUNT_SPEC, 0> {
        COUNT_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "COUNT8 Counter Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COUNT_SPEC;
impl crate::RegisterSpec for COUNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`count::R`](R) reader structure"]
impl crate::Readable for COUNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`count::W`](W) writer structure"]
impl crate::Writable for COUNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COUNT to value 0"]
impl crate::Resettable for COUNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
