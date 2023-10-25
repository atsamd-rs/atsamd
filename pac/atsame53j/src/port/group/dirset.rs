#[doc = "Register `DIRSET` reader"]
pub type R = crate::R<DIRSET_SPEC>;
#[doc = "Register `DIRSET` writer"]
pub type W = crate::W<DIRSET_SPEC>;
#[doc = "Field `DIRSET` reader - Port Data Direction Set"]
pub type DIRSET_R = crate::FieldReader<u32>;
#[doc = "Field `DIRSET` writer - Port Data Direction Set"]
pub type DIRSET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Port Data Direction Set"]
    #[inline(always)]
    pub fn dirset(&self) -> DIRSET_R {
        DIRSET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Direction Set"]
    #[inline(always)]
    #[must_use]
    pub fn dirset(&mut self) -> DIRSET_W<DIRSET_SPEC, 0> {
        DIRSET_W::new(self)
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
#[doc = "Data Direction Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dirset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dirset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIRSET_SPEC;
impl crate::RegisterSpec for DIRSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dirset::R`](R) reader structure"]
impl crate::Readable for DIRSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dirset::W`](W) writer structure"]
impl crate::Writable for DIRSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIRSET to value 0"]
impl crate::Resettable for DIRSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
