#[doc = "Register `CHSTATUS` reader"]
pub type R = crate::R<CHSTATUS_SPEC>;
#[doc = "Register `CHSTATUS` writer"]
pub type W = crate::W<CHSTATUS_SPEC>;
#[doc = "Field `PEND` reader - Channel Pending"]
pub type PEND_R = crate::BitReader;
#[doc = "Field `PEND` writer - Channel Pending"]
pub type PEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BUSY` reader - Channel Busy"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `BUSY` writer - Channel Busy"]
pub type BUSY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FERR` reader - Channel Fetch Error"]
pub type FERR_R = crate::BitReader;
#[doc = "Field `FERR` writer - Channel Fetch Error"]
pub type FERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRCERR` reader - Channel CRC Error"]
pub type CRCERR_R = crate::BitReader;
#[doc = "Field `CRCERR` writer - Channel CRC Error"]
pub type CRCERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Channel Pending"]
    #[inline(always)]
    pub fn pend(&self) -> PEND_R {
        PEND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel Fetch Error"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel CRC Error"]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Pending"]
    #[inline(always)]
    #[must_use]
    pub fn pend(&mut self) -> PEND_W<CHSTATUS_SPEC, 0> {
        PEND_W::new(self)
    }
    #[doc = "Bit 1 - Channel Busy"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<CHSTATUS_SPEC, 1> {
        BUSY_W::new(self)
    }
    #[doc = "Bit 2 - Channel Fetch Error"]
    #[inline(always)]
    #[must_use]
    pub fn ferr(&mut self) -> FERR_W<CHSTATUS_SPEC, 2> {
        FERR_W::new(self)
    }
    #[doc = "Bit 3 - Channel CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn crcerr(&mut self) -> CRCERR_W<CHSTATUS_SPEC, 3> {
        CRCERR_W::new(self)
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
#[doc = "Channel n Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHSTATUS_SPEC;
impl crate::RegisterSpec for CHSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`chstatus::R`](R) reader structure"]
impl crate::Readable for CHSTATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chstatus::W`](W) writer structure"]
impl crate::Writable for CHSTATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHSTATUS to value 0"]
impl crate::Resettable for CHSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
