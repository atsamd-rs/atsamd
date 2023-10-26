#[doc = "Register `APBBMASK` reader"]
pub type R = crate::R<APBBMASK_SPEC>;
#[doc = "Register `APBBMASK` writer"]
pub type W = crate::W<APBBMASK_SPEC>;
#[doc = "Field `PAC1_` reader - PAC1 APB Clock Enable"]
pub type PAC1__R = crate::BitReader;
#[doc = "Field `PAC1_` writer - PAC1 APB Clock Enable"]
pub type PAC1__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DSU_` reader - DSU APB Clock Enable"]
pub type DSU__R = crate::BitReader;
#[doc = "Field `DSU_` writer - DSU APB Clock Enable"]
pub type DSU__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NVMCTRL_` reader - NVMCTRL APB Clock Enable"]
pub type NVMCTRL__R = crate::BitReader;
#[doc = "Field `NVMCTRL_` writer - NVMCTRL APB Clock Enable"]
pub type NVMCTRL__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PORT_` reader - PORT APB Clock Enable"]
pub type PORT__R = crate::BitReader;
#[doc = "Field `PORT_` writer - PORT APB Clock Enable"]
pub type PORT__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMAC_` reader - DMAC APB Clock Enable"]
pub type DMAC__R = crate::BitReader;
#[doc = "Field `DMAC_` writer - DMAC APB Clock Enable"]
pub type DMAC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_` reader - USB APB Clock Enable"]
pub type USB__R = crate::BitReader;
#[doc = "Field `USB_` writer - USB APB Clock Enable"]
pub type USB__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HMATRIX_` reader - HMATRIX APB Clock Enable"]
pub type HMATRIX__R = crate::BitReader;
#[doc = "Field `HMATRIX_` writer - HMATRIX APB Clock Enable"]
pub type HMATRIX__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - PAC1 APB Clock Enable"]
    #[inline(always)]
    pub fn pac1_(&self) -> PAC1__R {
        PAC1__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DSU APB Clock Enable"]
    #[inline(always)]
    pub fn dsu_(&self) -> DSU__R {
        DSU__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NVMCTRL APB Clock Enable"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> NVMCTRL__R {
        NVMCTRL__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PORT APB Clock Enable"]
    #[inline(always)]
    pub fn port_(&self) -> PORT__R {
        PORT__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMAC APB Clock Enable"]
    #[inline(always)]
    pub fn dmac_(&self) -> DMAC__R {
        DMAC__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB APB Clock Enable"]
    #[inline(always)]
    pub fn usb_(&self) -> USB__R {
        USB__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HMATRIX APB Clock Enable"]
    #[inline(always)]
    pub fn hmatrix_(&self) -> HMATRIX__R {
        HMATRIX__R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PAC1 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pac1_(&mut self) -> PAC1__W<APBBMASK_SPEC, 0> {
        PAC1__W::new(self)
    }
    #[doc = "Bit 1 - DSU APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsu_(&mut self) -> DSU__W<APBBMASK_SPEC, 1> {
        DSU__W::new(self)
    }
    #[doc = "Bit 2 - NVMCTRL APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nvmctrl_(&mut self) -> NVMCTRL__W<APBBMASK_SPEC, 2> {
        NVMCTRL__W::new(self)
    }
    #[doc = "Bit 3 - PORT APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn port_(&mut self) -> PORT__W<APBBMASK_SPEC, 3> {
        PORT__W::new(self)
    }
    #[doc = "Bit 4 - DMAC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmac_(&mut self) -> DMAC__W<APBBMASK_SPEC, 4> {
        DMAC__W::new(self)
    }
    #[doc = "Bit 5 - USB APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_(&mut self) -> USB__W<APBBMASK_SPEC, 5> {
        USB__W::new(self)
    }
    #[doc = "Bit 6 - HMATRIX APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hmatrix_(&mut self) -> HMATRIX__W<APBBMASK_SPEC, 6> {
        HMATRIX__W::new(self)
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
#[doc = "APBB Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbbmask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbbmask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APBBMASK_SPEC;
impl crate::RegisterSpec for APBBMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbbmask::R`](R) reader structure"]
impl crate::Readable for APBBMASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apbbmask::W`](W) writer structure"]
impl crate::Writable for APBBMASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBBMASK to value 0x7f"]
impl crate::Resettable for APBBMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
