#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<INTFLAG_SPEC>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<INTFLAG_SPEC>;
#[doc = "Field `ENCCMP` reader - Encryption Complete"]
pub type ENCCMP_R = crate::BitReader;
#[doc = "Field `ENCCMP` writer - Encryption Complete"]
pub type ENCCMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GFMCMP` reader - GF Multiplication Complete"]
pub type GFMCMP_R = crate::BitReader;
#[doc = "Field `GFMCMP` writer - GF Multiplication Complete"]
pub type GFMCMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Encryption Complete"]
    #[inline(always)]
    pub fn enccmp(&self) -> ENCCMP_R {
        ENCCMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GF Multiplication Complete"]
    #[inline(always)]
    pub fn gfmcmp(&self) -> GFMCMP_R {
        GFMCMP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Encryption Complete"]
    #[inline(always)]
    #[must_use]
    pub fn enccmp(&mut self) -> ENCCMP_W<INTFLAG_SPEC, 0> {
        ENCCMP_W::new(self)
    }
    #[doc = "Bit 1 - GF Multiplication Complete"]
    #[inline(always)]
    #[must_use]
    pub fn gfmcmp(&mut self) -> GFMCMP_W<INTFLAG_SPEC, 1> {
        GFMCMP_W::new(self)
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
#[doc = "Interrupt Flag Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTFLAG_SPEC;
impl crate::RegisterSpec for INTFLAG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for INTFLAG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for INTFLAG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for INTFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
