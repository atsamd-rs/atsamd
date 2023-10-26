#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<INTFLAG_SPEC>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<INTFLAG_SPEC>;
#[doc = "Field `HSOF` reader - Host Start Of Frame"]
pub type HSOF_R = crate::BitReader;
#[doc = "Field `HSOF` writer - Host Start Of Frame"]
pub type HSOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RST` reader - Bus Reset"]
pub type RST_R = crate::BitReader;
#[doc = "Field `RST` writer - Bus Reset"]
pub type RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAKEUP` reader - Wake Up"]
pub type WAKEUP_R = crate::BitReader;
#[doc = "Field `WAKEUP` writer - Wake Up"]
pub type WAKEUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DNRSM` reader - Downstream"]
pub type DNRSM_R = crate::BitReader;
#[doc = "Field `DNRSM` writer - Downstream"]
pub type DNRSM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPRSM` reader - Upstream Resume from the Device"]
pub type UPRSM_R = crate::BitReader;
#[doc = "Field `UPRSM` writer - Upstream Resume from the Device"]
pub type UPRSM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RAMACER` reader - Ram Access"]
pub type RAMACER_R = crate::BitReader;
#[doc = "Field `RAMACER` writer - Ram Access"]
pub type RAMACER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DCONN` reader - Device Connection"]
pub type DCONN_R = crate::BitReader;
#[doc = "Field `DCONN` writer - Device Connection"]
pub type DCONN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DDISC` reader - Device Disconnection"]
pub type DDISC_R = crate::BitReader;
#[doc = "Field `DDISC` writer - Device Disconnection"]
pub type DDISC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 2 - Host Start Of Frame"]
    #[inline(always)]
    pub fn hsof(&self) -> HSOF_R {
        HSOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bus Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake Up"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Downstream"]
    #[inline(always)]
    pub fn dnrsm(&self) -> DNRSM_R {
        DNRSM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Upstream Resume from the Device"]
    #[inline(always)]
    pub fn uprsm(&self) -> UPRSM_R {
        UPRSM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Ram Access"]
    #[inline(always)]
    pub fn ramacer(&self) -> RAMACER_R {
        RAMACER_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Device Connection"]
    #[inline(always)]
    pub fn dconn(&self) -> DCONN_R {
        DCONN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Device Disconnection"]
    #[inline(always)]
    pub fn ddisc(&self) -> DDISC_R {
        DDISC_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Host Start Of Frame"]
    #[inline(always)]
    #[must_use]
    pub fn hsof(&mut self) -> HSOF_W<INTFLAG_SPEC, 2> {
        HSOF_W::new(self)
    }
    #[doc = "Bit 3 - Bus Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<INTFLAG_SPEC, 3> {
        RST_W::new(self)
    }
    #[doc = "Bit 4 - Wake Up"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup(&mut self) -> WAKEUP_W<INTFLAG_SPEC, 4> {
        WAKEUP_W::new(self)
    }
    #[doc = "Bit 5 - Downstream"]
    #[inline(always)]
    #[must_use]
    pub fn dnrsm(&mut self) -> DNRSM_W<INTFLAG_SPEC, 5> {
        DNRSM_W::new(self)
    }
    #[doc = "Bit 6 - Upstream Resume from the Device"]
    #[inline(always)]
    #[must_use]
    pub fn uprsm(&mut self) -> UPRSM_W<INTFLAG_SPEC, 6> {
        UPRSM_W::new(self)
    }
    #[doc = "Bit 7 - Ram Access"]
    #[inline(always)]
    #[must_use]
    pub fn ramacer(&mut self) -> RAMACER_W<INTFLAG_SPEC, 7> {
        RAMACER_W::new(self)
    }
    #[doc = "Bit 8 - Device Connection"]
    #[inline(always)]
    #[must_use]
    pub fn dconn(&mut self) -> DCONN_W<INTFLAG_SPEC, 8> {
        DCONN_W::new(self)
    }
    #[doc = "Bit 9 - Device Disconnection"]
    #[inline(always)]
    #[must_use]
    pub fn ddisc(&mut self) -> DDISC_W<INTFLAG_SPEC, 9> {
        DDISC_W::new(self)
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
#[doc = "HOST Host Interrupt Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
