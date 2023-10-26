#[doc = "Register `DFLLMUL` reader"]
pub type R = crate::R<DFLLMUL_SPEC>;
#[doc = "Register `DFLLMUL` writer"]
pub type W = crate::W<DFLLMUL_SPEC>;
#[doc = "Field `MUL` reader - DFLL Multiply Factor"]
pub type MUL_R = crate::FieldReader<u16>;
#[doc = "Field `MUL` writer - DFLL Multiply Factor"]
pub type MUL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `FSTEP` reader - Fine Maximum Step"]
pub type FSTEP_R = crate::FieldReader;
#[doc = "Field `FSTEP` writer - Fine Maximum Step"]
pub type FSTEP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CSTEP` reader - Coarse Maximum Step"]
pub type CSTEP_R = crate::FieldReader;
#[doc = "Field `CSTEP` writer - Coarse Maximum Step"]
pub type CSTEP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:15 - DFLL Multiply Factor"]
    #[inline(always)]
    pub fn mul(&self) -> MUL_R {
        MUL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Fine Maximum Step"]
    #[inline(always)]
    pub fn fstep(&self) -> FSTEP_R {
        FSTEP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 26:31 - Coarse Maximum Step"]
    #[inline(always)]
    pub fn cstep(&self) -> CSTEP_R {
        CSTEP_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - DFLL Multiply Factor"]
    #[inline(always)]
    #[must_use]
    pub fn mul(&mut self) -> MUL_W<DFLLMUL_SPEC, 0> {
        MUL_W::new(self)
    }
    #[doc = "Bits 16:23 - Fine Maximum Step"]
    #[inline(always)]
    #[must_use]
    pub fn fstep(&mut self) -> FSTEP_W<DFLLMUL_SPEC, 16> {
        FSTEP_W::new(self)
    }
    #[doc = "Bits 26:31 - Coarse Maximum Step"]
    #[inline(always)]
    #[must_use]
    pub fn cstep(&mut self) -> CSTEP_W<DFLLMUL_SPEC, 26> {
        CSTEP_W::new(self)
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
#[doc = "DFLL48M Multiplier\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfllmul::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfllmul::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFLLMUL_SPEC;
impl crate::RegisterSpec for DFLLMUL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfllmul::R`](R) reader structure"]
impl crate::Readable for DFLLMUL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dfllmul::W`](W) writer structure"]
impl crate::Writable for DFLLMUL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFLLMUL to value 0"]
impl crate::Resettable for DFLLMUL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
