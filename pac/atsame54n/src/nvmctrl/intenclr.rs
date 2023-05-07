#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DONE` reader - Command Done Interrupt Clear"]
pub type DONE_R = crate::BitReader<bool>;
#[doc = "Field `DONE` writer - Command Done Interrupt Clear"]
pub type DONE_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTENCLR_SPEC, bool, O>;
#[doc = "Field `ADDRE` reader - Address Error"]
pub type ADDRE_R = crate::BitReader<bool>;
#[doc = "Field `ADDRE` writer - Address Error"]
pub type ADDRE_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTENCLR_SPEC, bool, O>;
#[doc = "Field `PROGE` reader - Programming Error Interrupt Clear"]
pub type PROGE_R = crate::BitReader<bool>;
#[doc = "Field `PROGE` writer - Programming Error Interrupt Clear"]
pub type PROGE_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTENCLR_SPEC, bool, O>;
#[doc = "Field `LOCKE` reader - Lock Error Interrupt Clear"]
pub type LOCKE_R = crate::BitReader<bool>;
#[doc = "Field `LOCKE` writer - Lock Error Interrupt Clear"]
pub type LOCKE_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTENCLR_SPEC, bool, O>;
#[doc = "Field `ECCSE` reader - ECC Single Error Interrupt Clear"]
pub type ECCSE_R = crate::BitReader<bool>;
#[doc = "Field `ECCSE` writer - ECC Single Error Interrupt Clear"]
pub type ECCSE_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTENCLR_SPEC, bool, O>;
#[doc = "Field `ECCDE` reader - ECC Dual Error Interrupt Clear"]
pub type ECCDE_R = crate::BitReader<bool>;
#[doc = "Field `ECCDE` writer - ECC Dual Error Interrupt Clear"]
pub type ECCDE_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTENCLR_SPEC, bool, O>;
#[doc = "Field `NVME` reader - NVM Error Interrupt Clear"]
pub type NVME_R = crate::BitReader<bool>;
#[doc = "Field `NVME` writer - NVM Error Interrupt Clear"]
pub type NVME_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTENCLR_SPEC, bool, O>;
#[doc = "Field `SUSP` reader - Suspended Write Or Erase Interrupt Clear"]
pub type SUSP_R = crate::BitReader<bool>;
#[doc = "Field `SUSP` writer - Suspended Write Or Erase Interrupt Clear"]
pub type SUSP_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTENCLR_SPEC, bool, O>;
#[doc = "Field `SEESFULL` reader - Active SEES Full Interrupt Clear"]
pub type SEESFULL_R = crate::BitReader<bool>;
#[doc = "Field `SEESFULL` writer - Active SEES Full Interrupt Clear"]
pub type SEESFULL_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTENCLR_SPEC, bool, O>;
#[doc = "Field `SEESOVF` reader - Active SEES Overflow Interrupt Clear"]
pub type SEESOVF_R = crate::BitReader<bool>;
#[doc = "Field `SEESOVF` writer - Active SEES Overflow Interrupt Clear"]
pub type SEESOVF_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTENCLR_SPEC, bool, O>;
#[doc = "Field `SEEWRC` reader - SEE Write Completed Interrupt Clear"]
pub type SEEWRC_R = crate::BitReader<bool>;
#[doc = "Field `SEEWRC` writer - SEE Write Completed Interrupt Clear"]
pub type SEEWRC_W<'a, const O: u8> = crate::BitWriter<'a, u16, INTENCLR_SPEC, bool, O>;
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
    pub fn done(&mut self) -> DONE_W<0> {
        DONE_W::new(self)
    }
    #[doc = "Bit 1 - Address Error"]
    #[inline(always)]
    #[must_use]
    pub fn addre(&mut self) -> ADDRE_W<1> {
        ADDRE_W::new(self)
    }
    #[doc = "Bit 2 - Programming Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn proge(&mut self) -> PROGE_W<2> {
        PROGE_W::new(self)
    }
    #[doc = "Bit 3 - Lock Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn locke(&mut self) -> LOCKE_W<3> {
        LOCKE_W::new(self)
    }
    #[doc = "Bit 4 - ECC Single Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn eccse(&mut self) -> ECCSE_W<4> {
        ECCSE_W::new(self)
    }
    #[doc = "Bit 5 - ECC Dual Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn eccde(&mut self) -> ECCDE_W<5> {
        ECCDE_W::new(self)
    }
    #[doc = "Bit 6 - NVM Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nvme(&mut self) -> NVME_W<6> {
        NVME_W::new(self)
    }
    #[doc = "Bit 7 - Suspended Write Or Erase Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<7> {
        SUSP_W::new(self)
    }
    #[doc = "Bit 8 - Active SEES Full Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn seesfull(&mut self) -> SEESFULL_W<8> {
        SEESFULL_W::new(self)
    }
    #[doc = "Bit 9 - Active SEES Overflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn seesovf(&mut self) -> SEESOVF_W<9> {
        SEESOVF_W::new(self)
    }
    #[doc = "Bit 10 - SEE Write Completed Interrupt Clear"]
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
#[doc = "Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
