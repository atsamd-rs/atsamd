#[doc = "Register `OUTSET%s` reader"]
pub type R = crate::R<OUTSET_SPEC>;
#[doc = "Register `OUTSET%s` writer"]
pub type W = crate::W<OUTSET_SPEC>;
#[doc = "Field `OUTSET` reader - Port Data Output Value Set"]
pub type OUTSET_R = crate::FieldReader<u32>;
#[doc = "Field `OUTSET` writer - Port Data Output Value Set"]
pub type OUTSET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Port Data Output Value Set"]
    #[inline(always)]
    pub fn outset(&self) -> OUTSET_R {
        OUTSET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Output Value Set"]
    #[inline(always)]
    #[must_use]
    pub fn outset(&mut self) -> OUTSET_W<OUTSET_SPEC, 0> {
        OUTSET_W::new(self)
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
#[doc = "Data Output Value Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUTSET_SPEC;
impl crate::RegisterSpec for OUTSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outset::R`](R) reader structure"]
impl crate::Readable for OUTSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`outset::W`](W) writer structure"]
impl crate::Writable for OUTSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTSET%s to value 0"]
impl crate::Resettable for OUTSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
