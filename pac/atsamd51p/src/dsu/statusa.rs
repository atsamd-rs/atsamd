#[doc = "Register `STATUSA` reader"]
pub type R = crate::R<STATUSA_SPEC>;
#[doc = "Register `STATUSA` writer"]
pub type W = crate::W<STATUSA_SPEC>;
#[doc = "Field `DONE` reader - Done"]
pub type DONE_R = crate::BitReader;
#[doc = "Field `DONE` writer - Done"]
pub type DONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRSTEXT` reader - CPU Reset Phase Extension"]
pub type CRSTEXT_R = crate::BitReader;
#[doc = "Field `CRSTEXT` writer - CPU Reset Phase Extension"]
pub type CRSTEXT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BERR` reader - Bus Error"]
pub type BERR_R = crate::BitReader;
#[doc = "Field `BERR` writer - Bus Error"]
pub type BERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAIL` reader - Failure"]
pub type FAIL_R = crate::BitReader;
#[doc = "Field `FAIL` writer - Failure"]
pub type FAIL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PERR` reader - Protection Error"]
pub type PERR_R = crate::BitReader;
#[doc = "Field `PERR` writer - Protection Error"]
pub type PERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Done"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU Reset Phase Extension"]
    #[inline(always)]
    pub fn crstext(&self) -> CRSTEXT_R {
        CRSTEXT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bus Error"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Failure"]
    #[inline(always)]
    pub fn fail(&self) -> FAIL_R {
        FAIL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Protection Error"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Done"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<STATUSA_SPEC, 0> {
        DONE_W::new(self)
    }
    #[doc = "Bit 1 - CPU Reset Phase Extension"]
    #[inline(always)]
    #[must_use]
    pub fn crstext(&mut self) -> CRSTEXT_W<STATUSA_SPEC, 1> {
        CRSTEXT_W::new(self)
    }
    #[doc = "Bit 2 - Bus Error"]
    #[inline(always)]
    #[must_use]
    pub fn berr(&mut self) -> BERR_W<STATUSA_SPEC, 2> {
        BERR_W::new(self)
    }
    #[doc = "Bit 3 - Failure"]
    #[inline(always)]
    #[must_use]
    pub fn fail(&mut self) -> FAIL_W<STATUSA_SPEC, 3> {
        FAIL_W::new(self)
    }
    #[doc = "Bit 4 - Protection Error"]
    #[inline(always)]
    #[must_use]
    pub fn perr(&mut self) -> PERR_W<STATUSA_SPEC, 4> {
        PERR_W::new(self)
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
#[doc = "Status A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statusa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`statusa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUSA_SPEC;
impl crate::RegisterSpec for STATUSA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`statusa::R`](R) reader structure"]
impl crate::Readable for STATUSA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`statusa::W`](W) writer structure"]
impl crate::Writable for STATUSA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUSA to value 0"]
impl crate::Resettable for STATUSA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
