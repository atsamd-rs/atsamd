#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `PRM` reader - Power Reduction Mode"]
pub type PRM_R = crate::BitReader;
#[doc = "Field `LOAD` reader - NVM Page Buffer Active Loading"]
pub type LOAD_R = crate::BitReader;
#[doc = "Field `LOAD` writer - NVM Page Buffer Active Loading"]
pub type LOAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PROGE` reader - Programming Error Status"]
pub type PROGE_R = crate::BitReader;
#[doc = "Field `PROGE` writer - Programming Error Status"]
pub type PROGE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LOCKE` reader - Lock Error Status"]
pub type LOCKE_R = crate::BitReader;
#[doc = "Field `LOCKE` writer - Lock Error Status"]
pub type LOCKE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NVME` reader - NVM Error"]
pub type NVME_R = crate::BitReader;
#[doc = "Field `NVME` writer - NVM Error"]
pub type NVME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SB` reader - Security Bit Status"]
pub type SB_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Power Reduction Mode"]
    #[inline(always)]
    pub fn prm(&self) -> PRM_R {
        PRM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NVM Page Buffer Active Loading"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Programming Error Status"]
    #[inline(always)]
    pub fn proge(&self) -> PROGE_R {
        PROGE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Lock Error Status"]
    #[inline(always)]
    pub fn locke(&self) -> LOCKE_R {
        LOCKE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NVM Error"]
    #[inline(always)]
    pub fn nvme(&self) -> NVME_R {
        NVME_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Security Bit Status"]
    #[inline(always)]
    pub fn sb(&self) -> SB_R {
        SB_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - NVM Page Buffer Active Loading"]
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LOAD_W<STATUS_SPEC, 1> {
        LOAD_W::new(self)
    }
    #[doc = "Bit 2 - Programming Error Status"]
    #[inline(always)]
    #[must_use]
    pub fn proge(&mut self) -> PROGE_W<STATUS_SPEC, 2> {
        PROGE_W::new(self)
    }
    #[doc = "Bit 3 - Lock Error Status"]
    #[inline(always)]
    #[must_use]
    pub fn locke(&mut self) -> LOCKE_W<STATUS_SPEC, 3> {
        LOCKE_W::new(self)
    }
    #[doc = "Bit 4 - NVM Error"]
    #[inline(always)]
    #[must_use]
    pub fn nvme(&mut self) -> NVME_W<STATUS_SPEC, 4> {
        NVME_W::new(self)
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
#[doc = "Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
