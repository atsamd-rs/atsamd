#[doc = "Register `MEN` reader"]
pub type R = crate::R<MEN_SPEC>;
#[doc = "Register `MEN` writer"]
pub type W = crate::W<MEN_SPEC>;
#[doc = "Field `MENABLE` reader - Cache Controller Monitor Enable"]
pub type MENABLE_R = crate::BitReader;
#[doc = "Field `MENABLE` writer - Cache Controller Monitor Enable"]
pub type MENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Cache Controller Monitor Enable"]
    #[inline(always)]
    pub fn menable(&self) -> MENABLE_R {
        MENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache Controller Monitor Enable"]
    #[inline(always)]
    #[must_use]
    pub fn menable(&mut self) -> MENABLE_W<MEN_SPEC, 0> {
        MENABLE_W::new(self)
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
#[doc = "Cache Monitor Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`men::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`men::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEN_SPEC;
impl crate::RegisterSpec for MEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`men::R`](R) reader structure"]
impl crate::Readable for MEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`men::W`](W) writer structure"]
impl crate::Writable for MEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEN to value 0"]
impl crate::Resettable for MEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
