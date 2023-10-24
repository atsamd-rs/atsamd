#[doc = "Register `DFLLSYNC` reader"]
pub type R = crate::R<DFLLSYNC_SPEC>;
#[doc = "Register `DFLLSYNC` writer"]
pub type W = crate::W<DFLLSYNC_SPEC>;
#[doc = "Field `READREQ` writer - Read Request"]
pub type READREQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 7 - Read Request"]
    #[inline(always)]
    #[must_use]
    pub fn readreq(&mut self) -> READREQ_W<DFLLSYNC_SPEC, 7> {
        READREQ_W::new(self)
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
