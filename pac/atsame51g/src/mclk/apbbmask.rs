#[doc = "Register `APBBMASK` reader"]
pub type R = crate::R<ApbbmaskSpec>;
#[doc = "Register `APBBMASK` writer"]
pub type W = crate::W<ApbbmaskSpec>;
#[doc = "Field `USB_` reader - USB APB Clock Enable"]
pub type Usb_R = crate::BitReader;
#[doc = "Field `USB_` writer - USB APB Clock Enable"]
pub type Usb_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSU_` reader - DSU APB Clock Enable"]
pub type Dsu_R = crate::BitReader;
#[doc = "Field `DSU_` writer - DSU APB Clock Enable"]
pub type Dsu_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NVMCTRL_` reader - NVMCTRL APB Clock Enable"]
pub type Nvmctrl_R = crate::BitReader;
#[doc = "Field `NVMCTRL_` writer - NVMCTRL APB Clock Enable"]
pub type Nvmctrl_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_` reader - PORT APB Clock Enable"]
pub type Port_R = crate::BitReader;
#[doc = "Field `PORT_` writer - PORT APB Clock Enable"]
pub type Port_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HMATRIX_` reader - HMATRIX APB Clock Enable"]
pub type Hmatrix_R = crate::BitReader;
#[doc = "Field `HMATRIX_` writer - HMATRIX APB Clock Enable"]
pub type Hmatrix_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVSYS_` reader - EVSYS APB Clock Enable"]
pub type Evsys_R = crate::BitReader;
#[doc = "Field `EVSYS_` writer - EVSYS APB Clock Enable"]
pub type Evsys_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERCOM2_` reader - SERCOM2 APB Clock Enable"]
pub type Sercom2_R = crate::BitReader;
#[doc = "Field `SERCOM2_` writer - SERCOM2 APB Clock Enable"]
pub type Sercom2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERCOM3_` reader - SERCOM3 APB Clock Enable"]
pub type Sercom3_R = crate::BitReader;
#[doc = "Field `SERCOM3_` writer - SERCOM3 APB Clock Enable"]
pub type Sercom3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC0_` reader - TCC0 APB Clock Enable"]
pub type Tcc0_R = crate::BitReader;
#[doc = "Field `TCC0_` writer - TCC0 APB Clock Enable"]
pub type Tcc0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC1_` reader - TCC1 APB Clock Enable"]
pub type Tcc1_R = crate::BitReader;
#[doc = "Field `TCC1_` writer - TCC1 APB Clock Enable"]
pub type Tcc1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC2_` reader - TC2 APB Clock Enable"]
pub type Tc2_R = crate::BitReader;
#[doc = "Field `TC2_` writer - TC2 APB Clock Enable"]
pub type Tc2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC3_` reader - TC3 APB Clock Enable"]
pub type Tc3_R = crate::BitReader;
#[doc = "Field `TC3_` writer - TC3 APB Clock Enable"]
pub type Tc3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMECC_` reader - RAMECC APB Clock Enable"]
pub type Ramecc_R = crate::BitReader;
#[doc = "Field `RAMECC_` writer - RAMECC APB Clock Enable"]
pub type Ramecc_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB APB Clock Enable"]
    #[inline(always)]
    pub fn usb_(&self) -> Usb_R {
        Usb_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DSU APB Clock Enable"]
    #[inline(always)]
    pub fn dsu_(&self) -> Dsu_R {
        Dsu_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NVMCTRL APB Clock Enable"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> Nvmctrl_R {
        Nvmctrl_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - PORT APB Clock Enable"]
    #[inline(always)]
    pub fn port_(&self) -> Port_R {
        Port_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - HMATRIX APB Clock Enable"]
    #[inline(always)]
    pub fn hmatrix_(&self) -> Hmatrix_R {
        Hmatrix_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EVSYS APB Clock Enable"]
    #[inline(always)]
    pub fn evsys_(&self) -> Evsys_R {
        Evsys_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - SERCOM2 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom2_(&self) -> Sercom2_R {
        Sercom2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SERCOM3 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom3_(&self) -> Sercom3_R {
        Sercom3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TCC0 APB Clock Enable"]
    #[inline(always)]
    pub fn tcc0_(&self) -> Tcc0_R {
        Tcc0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TCC1 APB Clock Enable"]
    #[inline(always)]
    pub fn tcc1_(&self) -> Tcc1_R {
        Tcc1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TC2 APB Clock Enable"]
    #[inline(always)]
    pub fn tc2_(&self) -> Tc2_R {
        Tc2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TC3 APB Clock Enable"]
    #[inline(always)]
    pub fn tc3_(&self) -> Tc3_R {
        Tc3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - RAMECC APB Clock Enable"]
    #[inline(always)]
    pub fn ramecc_(&self) -> Ramecc_R {
        Ramecc_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_(&mut self) -> Usb_W<ApbbmaskSpec> {
        Usb_W::new(self, 0)
    }
    #[doc = "Bit 1 - DSU APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsu_(&mut self) -> Dsu_W<ApbbmaskSpec> {
        Dsu_W::new(self, 1)
    }
    #[doc = "Bit 2 - NVMCTRL APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nvmctrl_(&mut self) -> Nvmctrl_W<ApbbmaskSpec> {
        Nvmctrl_W::new(self, 2)
    }
    #[doc = "Bit 4 - PORT APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn port_(&mut self) -> Port_W<ApbbmaskSpec> {
        Port_W::new(self, 4)
    }
    #[doc = "Bit 6 - HMATRIX APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hmatrix_(&mut self) -> Hmatrix_W<ApbbmaskSpec> {
        Hmatrix_W::new(self, 6)
    }
    #[doc = "Bit 7 - EVSYS APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evsys_(&mut self) -> Evsys_W<ApbbmaskSpec> {
        Evsys_W::new(self, 7)
    }
    #[doc = "Bit 9 - SERCOM2 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom2_(&mut self) -> Sercom2_W<ApbbmaskSpec> {
        Sercom2_W::new(self, 9)
    }
    #[doc = "Bit 10 - SERCOM3 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom3_(&mut self) -> Sercom3_W<ApbbmaskSpec> {
        Sercom3_W::new(self, 10)
    }
    #[doc = "Bit 11 - TCC0 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcc0_(&mut self) -> Tcc0_W<ApbbmaskSpec> {
        Tcc0_W::new(self, 11)
    }
    #[doc = "Bit 12 - TCC1 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcc1_(&mut self) -> Tcc1_W<ApbbmaskSpec> {
        Tcc1_W::new(self, 12)
    }
    #[doc = "Bit 13 - TC2 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc2_(&mut self) -> Tc2_W<ApbbmaskSpec> {
        Tc2_W::new(self, 13)
    }
    #[doc = "Bit 14 - TC3 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc3_(&mut self) -> Tc3_W<ApbbmaskSpec> {
        Tc3_W::new(self, 14)
    }
    #[doc = "Bit 16 - RAMECC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ramecc_(&mut self) -> Ramecc_W<ApbbmaskSpec> {
        Ramecc_W::new(self, 16)
    }
}
#[doc = "APBB Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`apbbmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbbmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbbmaskSpec;
impl crate::RegisterSpec for ApbbmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbbmask::R`](R) reader structure"]
impl crate::Readable for ApbbmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`apbbmask::W`](W) writer structure"]
impl crate::Writable for ApbbmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBBMASK to value 0x0001_8056"]
impl crate::Resettable for ApbbmaskSpec {
    const RESET_VALUE: u32 = 0x0001_8056;
}
