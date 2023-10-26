#[doc = "Register `INTFLAGB` reader"]
pub type R = crate::R<INTFLAGB_SPEC>;
#[doc = "Register `INTFLAGB` writer"]
pub type W = crate::W<INTFLAGB_SPEC>;
#[doc = "Field `USB_` reader - USB"]
pub type USB__R = crate::BitReader;
#[doc = "Field `USB_` writer - USB"]
pub type USB__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DSU_` reader - DSU"]
pub type DSU__R = crate::BitReader;
#[doc = "Field `DSU_` writer - DSU"]
pub type DSU__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NVMCTRL_` reader - NVMCTRL"]
pub type NVMCTRL__R = crate::BitReader;
#[doc = "Field `NVMCTRL_` writer - NVMCTRL"]
pub type NVMCTRL__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMCC_` reader - CMCC"]
pub type CMCC__R = crate::BitReader;
#[doc = "Field `CMCC_` writer - CMCC"]
pub type CMCC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PORT_` reader - PORT"]
pub type PORT__R = crate::BitReader;
#[doc = "Field `PORT_` writer - PORT"]
pub type PORT__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMAC_` reader - DMAC"]
pub type DMAC__R = crate::BitReader;
#[doc = "Field `DMAC_` writer - DMAC"]
pub type DMAC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HMATRIX_` reader - HMATRIX"]
pub type HMATRIX__R = crate::BitReader;
#[doc = "Field `HMATRIX_` writer - HMATRIX"]
pub type HMATRIX__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EVSYS_` reader - EVSYS"]
pub type EVSYS__R = crate::BitReader;
#[doc = "Field `EVSYS_` writer - EVSYS"]
pub type EVSYS__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SERCOM2_` reader - SERCOM2"]
pub type SERCOM2__R = crate::BitReader;
#[doc = "Field `SERCOM2_` writer - SERCOM2"]
pub type SERCOM2__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SERCOM3_` reader - SERCOM3"]
pub type SERCOM3__R = crate::BitReader;
#[doc = "Field `SERCOM3_` writer - SERCOM3"]
pub type SERCOM3__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCC0_` reader - TCC0"]
pub type TCC0__R = crate::BitReader;
#[doc = "Field `TCC0_` writer - TCC0"]
pub type TCC0__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCC1_` reader - TCC1"]
pub type TCC1__R = crate::BitReader;
#[doc = "Field `TCC1_` writer - TCC1"]
pub type TCC1__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC2_` reader - TC2"]
pub type TC2__R = crate::BitReader;
#[doc = "Field `TC2_` writer - TC2"]
pub type TC2__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC3_` reader - TC3"]
pub type TC3__R = crate::BitReader;
#[doc = "Field `TC3_` writer - TC3"]
pub type TC3__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RAMECC_` reader - RAMECC"]
pub type RAMECC__R = crate::BitReader;
#[doc = "Field `RAMECC_` writer - RAMECC"]
pub type RAMECC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - USB"]
    #[inline(always)]
    pub fn usb_(&self) -> USB__R {
        USB__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DSU"]
    #[inline(always)]
    pub fn dsu_(&self) -> DSU__R {
        DSU__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NVMCTRL"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> NVMCTRL__R {
        NVMCTRL__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMCC"]
    #[inline(always)]
    pub fn cmcc_(&self) -> CMCC__R {
        CMCC__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PORT"]
    #[inline(always)]
    pub fn port_(&self) -> PORT__R {
        PORT__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMAC"]
    #[inline(always)]
    pub fn dmac_(&self) -> DMAC__R {
        DMAC__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HMATRIX"]
    #[inline(always)]
    pub fn hmatrix_(&self) -> HMATRIX__R {
        HMATRIX__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EVSYS"]
    #[inline(always)]
    pub fn evsys_(&self) -> EVSYS__R {
        EVSYS__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - SERCOM2"]
    #[inline(always)]
    pub fn sercom2_(&self) -> SERCOM2__R {
        SERCOM2__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SERCOM3"]
    #[inline(always)]
    pub fn sercom3_(&self) -> SERCOM3__R {
        SERCOM3__R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TCC0"]
    #[inline(always)]
    pub fn tcc0_(&self) -> TCC0__R {
        TCC0__R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TCC1"]
    #[inline(always)]
    pub fn tcc1_(&self) -> TCC1__R {
        TCC1__R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TC2"]
    #[inline(always)]
    pub fn tc2_(&self) -> TC2__R {
        TC2__R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TC3"]
    #[inline(always)]
    pub fn tc3_(&self) -> TC3__R {
        TC3__R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - RAMECC"]
    #[inline(always)]
    pub fn ramecc_(&self) -> RAMECC__R {
        RAMECC__R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB"]
    #[inline(always)]
    #[must_use]
    pub fn usb_(&mut self) -> USB__W<INTFLAGB_SPEC, 0> {
        USB__W::new(self)
    }
    #[doc = "Bit 1 - DSU"]
    #[inline(always)]
    #[must_use]
    pub fn dsu_(&mut self) -> DSU__W<INTFLAGB_SPEC, 1> {
        DSU__W::new(self)
    }
    #[doc = "Bit 2 - NVMCTRL"]
    #[inline(always)]
    #[must_use]
    pub fn nvmctrl_(&mut self) -> NVMCTRL__W<INTFLAGB_SPEC, 2> {
        NVMCTRL__W::new(self)
    }
    #[doc = "Bit 3 - CMCC"]
    #[inline(always)]
    #[must_use]
    pub fn cmcc_(&mut self) -> CMCC__W<INTFLAGB_SPEC, 3> {
        CMCC__W::new(self)
    }
    #[doc = "Bit 4 - PORT"]
    #[inline(always)]
    #[must_use]
    pub fn port_(&mut self) -> PORT__W<INTFLAGB_SPEC, 4> {
        PORT__W::new(self)
    }
    #[doc = "Bit 5 - DMAC"]
    #[inline(always)]
    #[must_use]
    pub fn dmac_(&mut self) -> DMAC__W<INTFLAGB_SPEC, 5> {
        DMAC__W::new(self)
    }
    #[doc = "Bit 6 - HMATRIX"]
    #[inline(always)]
    #[must_use]
    pub fn hmatrix_(&mut self) -> HMATRIX__W<INTFLAGB_SPEC, 6> {
        HMATRIX__W::new(self)
    }
    #[doc = "Bit 7 - EVSYS"]
    #[inline(always)]
    #[must_use]
    pub fn evsys_(&mut self) -> EVSYS__W<INTFLAGB_SPEC, 7> {
        EVSYS__W::new(self)
    }
    #[doc = "Bit 9 - SERCOM2"]
    #[inline(always)]
    #[must_use]
    pub fn sercom2_(&mut self) -> SERCOM2__W<INTFLAGB_SPEC, 9> {
        SERCOM2__W::new(self)
    }
    #[doc = "Bit 10 - SERCOM3"]
    #[inline(always)]
    #[must_use]
    pub fn sercom3_(&mut self) -> SERCOM3__W<INTFLAGB_SPEC, 10> {
        SERCOM3__W::new(self)
    }
    #[doc = "Bit 11 - TCC0"]
    #[inline(always)]
    #[must_use]
    pub fn tcc0_(&mut self) -> TCC0__W<INTFLAGB_SPEC, 11> {
        TCC0__W::new(self)
    }
    #[doc = "Bit 12 - TCC1"]
    #[inline(always)]
    #[must_use]
    pub fn tcc1_(&mut self) -> TCC1__W<INTFLAGB_SPEC, 12> {
        TCC1__W::new(self)
    }
    #[doc = "Bit 13 - TC2"]
    #[inline(always)]
    #[must_use]
    pub fn tc2_(&mut self) -> TC2__W<INTFLAGB_SPEC, 13> {
        TC2__W::new(self)
    }
    #[doc = "Bit 14 - TC3"]
    #[inline(always)]
    #[must_use]
    pub fn tc3_(&mut self) -> TC3__W<INTFLAGB_SPEC, 14> {
        TC3__W::new(self)
    }
    #[doc = "Bit 16 - RAMECC"]
    #[inline(always)]
    #[must_use]
    pub fn ramecc_(&mut self) -> RAMECC__W<INTFLAGB_SPEC, 16> {
        RAMECC__W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Peripheral interrupt flag status - Bridge B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflagb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflagb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTFLAGB_SPEC;
impl crate::RegisterSpec for INTFLAGB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflagb::R`](R) reader structure"]
impl crate::Readable for INTFLAGB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intflagb::W`](W) writer structure"]
impl crate::Writable for INTFLAGB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAGB to value 0"]
impl crate::Resettable for INTFLAGB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
