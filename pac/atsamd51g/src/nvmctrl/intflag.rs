#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<INTFLAG_SPEC>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<INTFLAG_SPEC>;
#[doc = "Field `DONE` reader - Command Done"]
pub type DONE_R = crate::BitReader;
#[doc = "Field `DONE` writer - Command Done"]
pub type DONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADDRE` reader - Address Error"]
pub type ADDRE_R = crate::BitReader;
#[doc = "Field `ADDRE` writer - Address Error"]
pub type ADDRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PROGE` reader - Programming Error"]
pub type PROGE_R = crate::BitReader;
#[doc = "Field `PROGE` writer - Programming Error"]
pub type PROGE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LOCKE` reader - Lock Error"]
pub type LOCKE_R = crate::BitReader;
#[doc = "Field `LOCKE` writer - Lock Error"]
pub type LOCKE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ECCSE` reader - ECC Single Error"]
pub type ECCSE_R = crate::BitReader;
#[doc = "Field `ECCSE` writer - ECC Single Error"]
pub type ECCSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ECCDE` reader - ECC Dual Error"]
pub type ECCDE_R = crate::BitReader;
#[doc = "Field `ECCDE` writer - ECC Dual Error"]
pub type ECCDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NVME` reader - NVM Error"]
pub type NVME_R = crate::BitReader;
#[doc = "Field `NVME` writer - NVM Error"]
pub type NVME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SUSP` reader - Suspended Write Or Erase Operation"]
pub type SUSP_R = crate::BitReader;
#[doc = "Field `SUSP` writer - Suspended Write Or Erase Operation"]
pub type SUSP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SEESFULL` reader - Active SEES Full"]
pub type SEESFULL_R = crate::BitReader;
#[doc = "Field `SEESFULL` writer - Active SEES Full"]
pub type SEESFULL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SEESOVF` reader - Active SEES Overflow"]
pub type SEESOVF_R = crate::BitReader;
#[doc = "Field `SEESOVF` writer - Active SEES Overflow"]
pub type SEESOVF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SEEWRC` reader - SEE Write Completed"]
pub type SEEWRC_R = crate::BitReader;
#[doc = "Field `SEEWRC` writer - SEE Write Completed"]
pub type SEEWRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Command Done"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address Error"]
    #[inline(always)]
    pub fn addre(&self) -> ADDRE_R {
        ADDRE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Programming Error"]
    #[inline(always)]
    pub fn proge(&self) -> PROGE_R {
        PROGE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Lock Error"]
    #[inline(always)]
    pub fn locke(&self) -> LOCKE_R {
        LOCKE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ECC Single Error"]
    #[inline(always)]
    pub fn eccse(&self) -> ECCSE_R {
        ECCSE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ECC Dual Error"]
    #[inline(always)]
    pub fn eccde(&self) -> ECCDE_R {
        ECCDE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NVM Error"]
    #[inline(always)]
    pub fn nvme(&self) -> NVME_R {
        NVME_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Suspended Write Or Erase Operation"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Active SEES Full"]
    #[inline(always)]
    pub fn seesfull(&self) -> SEESFULL_R {
        SEESFULL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Active SEES Overflow"]
    #[inline(always)]
    pub fn seesovf(&self) -> SEESOVF_R {
        SEESOVF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SEE Write Completed"]
    #[inline(always)]
    pub fn seewrc(&self) -> SEEWRC_R {
        SEEWRC_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Done"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<INTFLAG_SPEC, 0> {
        DONE_W::new(self)
    }
    #[doc = "Bit 1 - Address Error"]
    #[inline(always)]
    #[must_use]
    pub fn addre(&mut self) -> ADDRE_W<INTFLAG_SPEC, 1> {
        ADDRE_W::new(self)
    }
    #[doc = "Bit 2 - Programming Error"]
    #[inline(always)]
    #[must_use]
    pub fn proge(&mut self) -> PROGE_W<INTFLAG_SPEC, 2> {
        PROGE_W::new(self)
    }
    #[doc = "Bit 3 - Lock Error"]
    #[inline(always)]
    #[must_use]
    pub fn locke(&mut self) -> LOCKE_W<INTFLAG_SPEC, 3> {
        LOCKE_W::new(self)
    }
    #[doc = "Bit 4 - ECC Single Error"]
    #[inline(always)]
    #[must_use]
    pub fn eccse(&mut self) -> ECCSE_W<INTFLAG_SPEC, 4> {
        ECCSE_W::new(self)
    }
    #[doc = "Bit 5 - ECC Dual Error"]
    #[inline(always)]
    #[must_use]
    pub fn eccde(&mut self) -> ECCDE_W<INTFLAG_SPEC, 5> {
        ECCDE_W::new(self)
    }
    #[doc = "Bit 6 - NVM Error"]
    #[inline(always)]
    #[must_use]
    pub fn nvme(&mut self) -> NVME_W<INTFLAG_SPEC, 6> {
        NVME_W::new(self)
    }
    #[doc = "Bit 7 - Suspended Write Or Erase Operation"]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<INTFLAG_SPEC, 7> {
        SUSP_W::new(self)
    }
    #[doc = "Bit 8 - Active SEES Full"]
    #[inline(always)]
    #[must_use]
    pub fn seesfull(&mut self) -> SEESFULL_W<INTFLAG_SPEC, 8> {
        SEESFULL_W::new(self)
    }
    #[doc = "Bit 9 - Active SEES Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn seesovf(&mut self) -> SEESOVF_W<INTFLAG_SPEC, 9> {
        SEESOVF_W::new(self)
    }
    #[doc = "Bit 10 - SEE Write Completed"]
    #[inline(always)]
    #[must_use]
    pub fn seewrc(&mut self) -> SEEWRC_W<INTFLAG_SPEC, 10> {
        SEEWRC_W::new(self)
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
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTFLAG_SPEC;
impl crate::RegisterSpec for INTFLAG_SPEC {
    type Ux = u16;
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
