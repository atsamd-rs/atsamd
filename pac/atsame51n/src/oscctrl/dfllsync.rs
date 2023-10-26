#[doc = "Register `DFLLSYNC` reader"]
pub type R = crate::R<DFLLSYNC_SPEC>;
#[doc = "Register `DFLLSYNC` writer"]
pub type W = crate::W<DFLLSYNC_SPEC>;
#[doc = "Field `ENABLE` reader - ENABLE Synchronization Busy"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - ENABLE Synchronization Busy"]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DFLLCTRLB` reader - DFLLCTRLB Synchronization Busy"]
pub type DFLLCTRLB_R = crate::BitReader;
#[doc = "Field `DFLLCTRLB` writer - DFLLCTRLB Synchronization Busy"]
pub type DFLLCTRLB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DFLLVAL` reader - DFLLVAL Synchronization Busy"]
pub type DFLLVAL_R = crate::BitReader;
#[doc = "Field `DFLLVAL` writer - DFLLVAL Synchronization Busy"]
pub type DFLLVAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DFLLMUL` reader - DFLLMUL Synchronization Busy"]
pub type DFLLMUL_R = crate::BitReader;
#[doc = "Field `DFLLMUL` writer - DFLLMUL Synchronization Busy"]
pub type DFLLMUL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - ENABLE Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DFLLCTRLB Synchronization Busy"]
    #[inline(always)]
    pub fn dfllctrlb(&self) -> DFLLCTRLB_R {
        DFLLCTRLB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DFLLVAL Synchronization Busy"]
    #[inline(always)]
    pub fn dfllval(&self) -> DFLLVAL_R {
        DFLLVAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DFLLMUL Synchronization Busy"]
    #[inline(always)]
    pub fn dfllmul(&self) -> DFLLMUL_R {
        DFLLMUL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - ENABLE Synchronization Busy"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<DFLLSYNC_SPEC, 1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - DFLLCTRLB Synchronization Busy"]
    #[inline(always)]
    #[must_use]
    pub fn dfllctrlb(&mut self) -> DFLLCTRLB_W<DFLLSYNC_SPEC, 2> {
        DFLLCTRLB_W::new(self)
    }
    #[doc = "Bit 3 - DFLLVAL Synchronization Busy"]
    #[inline(always)]
    #[must_use]
    pub fn dfllval(&mut self) -> DFLLVAL_W<DFLLSYNC_SPEC, 3> {
        DFLLVAL_W::new(self)
    }
    #[doc = "Bit 4 - DFLLMUL Synchronization Busy"]
    #[inline(always)]
    #[must_use]
    pub fn dfllmul(&mut self) -> DFLLMUL_W<DFLLSYNC_SPEC, 4> {
        DFLLMUL_W::new(self)
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
#[doc = "DFLL48M Synchronization\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfllsync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfllsync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFLLSYNC_SPEC;
impl crate::RegisterSpec for DFLLSYNC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dfllsync::R`](R) reader structure"]
impl crate::Readable for DFLLSYNC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dfllsync::W`](W) writer structure"]
impl crate::Writable for DFLLSYNC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFLLSYNC to value 0"]
impl crate::Resettable for DFLLSYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
