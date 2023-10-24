#[doc = "Register `DFLLVAL` reader"]
pub type R = crate::R<DFLLVAL_SPEC>;
#[doc = "Register `DFLLVAL` writer"]
pub type W = crate::W<DFLLVAL_SPEC>;
#[doc = "Field `FINE` reader - Fine Value"]
pub type FINE_R = crate::FieldReader;
#[doc = "Field `FINE` writer - Fine Value"]
pub type FINE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `COARSE` reader - Coarse Value"]
pub type COARSE_R = crate::FieldReader;
#[doc = "Field `COARSE` writer - Coarse Value"]
pub type COARSE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `DIFF` reader - Multiplication Ratio Difference"]
pub type DIFF_R = crate::FieldReader<u16>;
#[doc = "Field `DIFF` writer - Multiplication Ratio Difference"]
pub type DIFF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:7 - Fine Value"]
    #[inline(always)]
    pub fn fine(&self) -> FINE_R {
        FINE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 10:15 - Coarse Value"]
    #[inline(always)]
    pub fn coarse(&self) -> COARSE_R {
        COARSE_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - Multiplication Ratio Difference"]
    #[inline(always)]
    pub fn diff(&self) -> DIFF_R {
        DIFF_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fine Value"]
    #[inline(always)]
    #[must_use]
    pub fn fine(&mut self) -> FINE_W<DFLLVAL_SPEC, 0> {
        FINE_W::new(self)
    }
    #[doc = "Bits 10:15 - Coarse Value"]
    #[inline(always)]
    #[must_use]
    pub fn coarse(&mut self) -> COARSE_W<DFLLVAL_SPEC, 10> {
        COARSE_W::new(self)
    }
    #[doc = "Bits 16:31 - Multiplication Ratio Difference"]
    #[inline(always)]
    #[must_use]
    pub fn diff(&mut self) -> DIFF_W<DFLLVAL_SPEC, 16> {
        DIFF_W::new(self)
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
#[doc = "DFLL48M Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfllval::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfllval::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFLLVAL_SPEC;
impl crate::RegisterSpec for DFLLVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfllval::R`](R) reader structure"]
impl crate::Readable for DFLLVAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dfllval::W`](W) writer structure"]
impl crate::Writable for DFLLVAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFLLVAL to value 0"]
impl crate::Resettable for DFLLVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
