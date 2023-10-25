#[doc = "Register `BCR` reader"]
pub type R = crate::R<BCR_SPEC>;
#[doc = "Register `BCR` writer"]
pub type W = crate::W<BCR_SPEC>;
#[doc = "Field `BCNT` reader - Blocks Count for Current Transfer"]
pub type BCNT_R = crate::FieldReader<u16>;
#[doc = "Field `BCNT` writer - Blocks Count for Current Transfer"]
pub type BCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Blocks Count for Current Transfer"]
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Blocks Count for Current Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn bcnt(&mut self) -> BCNT_W<BCR_SPEC, 0> {
        BCNT_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Block Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCR_SPEC;
impl crate::RegisterSpec for BCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bcr::R`](R) reader structure"]
impl crate::Readable for BCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bcr::W`](W) writer structure"]
impl crate::Writable for BCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCR to value 0"]
impl crate::Resettable for BCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
