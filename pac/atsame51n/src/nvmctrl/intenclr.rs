#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<INTENCLR_SPEC>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<INTENCLR_SPEC>;
#[doc = "Field `DONE` reader - Command Done Interrupt Clear"]
pub type DONE_R = crate::BitReader;
#[doc = "Field `DONE` writer - Command Done Interrupt Clear"]
pub type DONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADDRE` reader - Address Error"]
pub type ADDRE_R = crate::BitReader;
#[doc = "Field `ADDRE` writer - Address Error"]
pub type ADDRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PROGE` reader - Programming Error Interrupt Clear"]
pub type PROGE_R = crate::BitReader;
#[doc = "Field `PROGE` writer - Programming Error Interrupt Clear"]
pub type PROGE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LOCKE` reader - Lock Error Interrupt Clear"]
pub type LOCKE_R = crate::BitReader;
#[doc = "Field `LOCKE` writer - Lock Error Interrupt Clear"]
pub type LOCKE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ECCSE` reader - ECC Single Error Interrupt Clear"]
pub type ECCSE_R = crate::BitReader;
#[doc = "Field `ECCSE` writer - ECC Single Error Interrupt Clear"]
pub type ECCSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ECCDE` reader - ECC Dual Error Interrupt Clear"]
pub type ECCDE_R = crate::BitReader;
#[doc = "Field `ECCDE` writer - ECC Dual Error Interrupt Clear"]
pub type ECCDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NVME` reader - NVM Error Interrupt Clear"]
pub type NVME_R = crate::BitReader;
#[doc = "Field `NVME` writer - NVM Error Interrupt Clear"]
pub type NVME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SUSP` reader - Suspended Write Or Erase Interrupt Clear"]
pub type SUSP_R = crate::BitReader;
#[doc = "Field `SUSP` writer - Suspended Write Or Erase Interrupt Clear"]
pub type SUSP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SEESFULL` reader - Active SEES Full Interrupt Clear"]
pub type SEESFULL_R = crate::BitReader;
#[doc = "Field `SEESFULL` writer - Active SEES Full Interrupt Clear"]
pub type SEESFULL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SEESOVF` reader - Active SEES Overflow Interrupt Clear"]
pub type SEESOVF_R = crate::BitReader;
#[doc = "Field `SEESOVF` writer - Active SEES Overflow Interrupt Clear"]
pub type SEESOVF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SEEWRC` reader - SEE Write Completed Interrupt Clear"]
pub type SEEWRC_R = crate::BitReader;
#[doc = "Field `SEEWRC` writer - SEE Write Completed Interrupt Clear"]
pub type SEEWRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Command Done Interrupt Clear"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address Error"]
    #[inline(always)]
    pub fn addre(&self) -> ADDRE_R {
        ADDRE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Programming Error Interrupt Clear"]
    #[inline(always)]
    pub fn proge(&self) -> PROGE_R {
        PROGE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Lock Error Interrupt Clear"]
    #[inline(always)]
    pub fn locke(&self) -> LOCKE_R {
        LOCKE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ECC Single Error Interrupt Clear"]
    #[inline(always)]
    pub fn eccse(&self) -> ECCSE_R {
        ECCSE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ECC Dual Error Interrupt Clear"]
    #[inline(always)]
    pub fn eccde(&self) -> ECCDE_R {
        ECCDE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NVM Error Interrupt Clear"]
    #[inline(always)]
    pub fn nvme(&self) -> NVME_R {
        NVME_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Suspended Write Or Erase Interrupt Clear"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Active SEES Full Interrupt Clear"]
    #[inline(always)]
    pub fn seesfull(&self) -> SEESFULL_R {
        SEESFULL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Active SEES Overflow Interrupt Clear"]
    #[inline(always)]
    pub fn seesovf(&self) -> SEESOVF_R {
        SEESOVF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SEE Write Completed Interrupt Clear"]
    #[inline(always)]
    pub fn seewrc(&self) -> SEEWRC_R {
        SEEWRC_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Done Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<INTENCLR_SPEC, 0> {
        DONE_W::new(self)
    }
    #[doc = "Bit 1 - Address Error"]
    #[inline(always)]
    #[must_use]
    pub fn addre(&mut self) -> ADDRE_W<INTENCLR_SPEC, 1> {
        ADDRE_W::new(self)
    }
    #[doc = "Bit 2 - Programming Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn proge(&mut self) -> PROGE_W<INTENCLR_SPEC, 2> {
        PROGE_W::new(self)
    }
    #[doc = "Bit 3 - Lock Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn locke(&mut self) -> LOCKE_W<INTENCLR_SPEC, 3> {
        LOCKE_W::new(self)
    }
    #[doc = "Bit 4 - ECC Single Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn eccse(&mut self) -> ECCSE_W<INTENCLR_SPEC, 4> {
        ECCSE_W::new(self)
    }
    #[doc = "Bit 5 - ECC Dual Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn eccde(&mut self) -> ECCDE_W<INTENCLR_SPEC, 5> {
        ECCDE_W::new(self)
    }
    #[doc = "Bit 6 - NVM Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nvme(&mut self) -> NVME_W<INTENCLR_SPEC, 6> {
        NVME_W::new(self)
    }
    #[doc = "Bit 7 - Suspended Write Or Erase Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<INTENCLR_SPEC, 7> {
        SUSP_W::new(self)
    }
    #[doc = "Bit 8 - Active SEES Full Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn seesfull(&mut self) -> SEESFULL_W<INTENCLR_SPEC, 8> {
        SEESFULL_W::new(self)
    }
    #[doc = "Bit 9 - Active SEES Overflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn seesovf(&mut self) -> SEESOVF_W<INTENCLR_SPEC, 9> {
        SEESOVF_W::new(self)
    }
    #[doc = "Bit 10 - SEE Write Completed Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn seewrc(&mut self) -> SEEWRC_W<INTENCLR_SPEC, 10> {
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
#[doc = "Interrupt Enable Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
