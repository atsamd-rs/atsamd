#[doc = "Register `INTFLAG` reader"]
pub struct R(crate::R<INTFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAG` writer"]
pub struct W(crate::W<INTFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INTFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DONE` reader - Command Done"]
pub type DONE_R = crate::BitReader<bool>;
#[doc = "Field `DONE` writer - Command Done"]
pub type DONE_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTFLAG_SPEC, bool, O>;
#[doc = "Field `ADDRE` reader - Address Error"]
pub type ADDRE_R = crate::BitReader<bool>;
#[doc = "Field `ADDRE` writer - Address Error"]
pub type ADDRE_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTFLAG_SPEC, bool, O>;
#[doc = "Field `PROGE` reader - Programming Error"]
pub type PROGE_R = crate::BitReader<bool>;
#[doc = "Field `PROGE` writer - Programming Error"]
pub type PROGE_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTFLAG_SPEC, bool, O>;
#[doc = "Field `LOCKE` reader - Lock Error"]
pub type LOCKE_R = crate::BitReader<bool>;
#[doc = "Field `LOCKE` writer - Lock Error"]
pub type LOCKE_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTFLAG_SPEC, bool, O>;
#[doc = "Field `ECCSE` reader - ECC Single Error"]
pub type ECCSE_R = crate::BitReader<bool>;
#[doc = "Field `ECCSE` writer - ECC Single Error"]
pub type ECCSE_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTFLAG_SPEC, bool, O>;
#[doc = "Field `ECCDE` reader - ECC Dual Error"]
pub type ECCDE_R = crate::BitReader<bool>;
#[doc = "Field `ECCDE` writer - ECC Dual Error"]
pub type ECCDE_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTFLAG_SPEC, bool, O>;
#[doc = "Field `NVME` reader - NVM Error"]
pub type NVME_R = crate::BitReader<bool>;
#[doc = "Field `NVME` writer - NVM Error"]
pub type NVME_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTFLAG_SPEC, bool, O>;
#[doc = "Field `SUSP` reader - Suspended Write Or Erase Operation"]
pub type SUSP_R = crate::BitReader<bool>;
#[doc = "Field `SUSP` writer - Suspended Write Or Erase Operation"]
pub type SUSP_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTFLAG_SPEC, bool, O>;
#[doc = "Field `SEESFULL` reader - Active SEES Full"]
pub type SEESFULL_R = crate::BitReader<bool>;
#[doc = "Field `SEESFULL` writer - Active SEES Full"]
pub type SEESFULL_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTFLAG_SPEC, bool, O>;
#[doc = "Field `SEESOVF` reader - Active SEES Overflow"]
pub type SEESOVF_R = crate::BitReader<bool>;
#[doc = "Field `SEESOVF` writer - Active SEES Overflow"]
pub type SEESOVF_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTFLAG_SPEC, bool, O>;
#[doc = "Field `SEEWRC` reader - SEE Write Completed"]
pub type SEEWRC_R = crate::BitReader<bool>;
#[doc = "Field `SEEWRC` writer - SEE Write Completed"]
pub type SEEWRC_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTFLAG_SPEC, bool, O>;
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
    pub fn done(&mut self) -> DONE_W<0> {
        DONE_W::new(self)
    }
    #[doc = "Bit 1 - Address Error"]
    #[inline(always)]
    #[must_use]
    pub fn addre(&mut self) -> ADDRE_W<1> {
        ADDRE_W::new(self)
    }
    #[doc = "Bit 2 - Programming Error"]
    #[inline(always)]
    #[must_use]
    pub fn proge(&mut self) -> PROGE_W<2> {
        PROGE_W::new(self)
    }
    #[doc = "Bit 3 - Lock Error"]
    #[inline(always)]
    #[must_use]
    pub fn locke(&mut self) -> LOCKE_W<3> {
        LOCKE_W::new(self)
    }
    #[doc = "Bit 4 - ECC Single Error"]
    #[inline(always)]
    #[must_use]
    pub fn eccse(&mut self) -> ECCSE_W<4> {
        ECCSE_W::new(self)
    }
    #[doc = "Bit 5 - ECC Dual Error"]
    #[inline(always)]
    #[must_use]
    pub fn eccde(&mut self) -> ECCDE_W<5> {
        ECCDE_W::new(self)
    }
    #[doc = "Bit 6 - NVM Error"]
    #[inline(always)]
    #[must_use]
    pub fn nvme(&mut self) -> NVME_W<6> {
        NVME_W::new(self)
    }
    #[doc = "Bit 7 - Suspended Write Or Erase Operation"]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<7> {
        SUSP_W::new(self)
    }
    #[doc = "Bit 8 - Active SEES Full"]
    #[inline(always)]
    #[must_use]
    pub fn seesfull(&mut self) -> SEESFULL_W<8> {
        SEESFULL_W::new(self)
    }
    #[doc = "Bit 9 - Active SEES Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn seesovf(&mut self) -> SEESOVF_W<9> {
        SEESOVF_W::new(self)
    }
    #[doc = "Bit 10 - SEE Write Completed"]
    #[inline(always)]
    #[must_use]
    pub fn seewrc(&mut self) -> SEEWRC_W<10> {
        SEEWRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflag](index.html) module"]
pub struct INTFLAG_SPEC;
impl crate::RegisterSpec for INTFLAG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [intflag::R](R) reader structure"]
impl crate::Readable for INTFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflag::W](W) writer structure"]
impl crate::Writable for INTFLAG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for INTFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
