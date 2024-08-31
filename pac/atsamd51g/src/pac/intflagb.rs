#[doc = "Register `INTFLAGB` reader"]
pub type R = crate::R<IntflagbSpec>;
#[doc = "Register `INTFLAGB` writer"]
pub type W = crate::W<IntflagbSpec>;
#[doc = "Field `USB_` reader - USB"]
pub type Usb_R = crate::BitReader;
#[doc = "Field `USB_` writer - USB"]
pub type Usb_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSU_` reader - DSU"]
pub type Dsu_R = crate::BitReader;
#[doc = "Field `DSU_` writer - DSU"]
pub type Dsu_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NVMCTRL_` reader - NVMCTRL"]
pub type Nvmctrl_R = crate::BitReader;
#[doc = "Field `NVMCTRL_` writer - NVMCTRL"]
pub type Nvmctrl_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMCC_` reader - CMCC"]
pub type Cmcc_R = crate::BitReader;
#[doc = "Field `CMCC_` writer - CMCC"]
pub type Cmcc_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_` reader - PORT"]
pub type Port_R = crate::BitReader;
#[doc = "Field `PORT_` writer - PORT"]
pub type Port_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAC_` reader - DMAC"]
pub type Dmac_R = crate::BitReader;
#[doc = "Field `DMAC_` writer - DMAC"]
pub type Dmac_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HMATRIX_` reader - HMATRIX"]
pub type Hmatrix_R = crate::BitReader;
#[doc = "Field `HMATRIX_` writer - HMATRIX"]
pub type Hmatrix_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVSYS_` reader - EVSYS"]
pub type Evsys_R = crate::BitReader;
#[doc = "Field `EVSYS_` writer - EVSYS"]
pub type Evsys_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERCOM2_` reader - SERCOM2"]
pub type Sercom2_R = crate::BitReader;
#[doc = "Field `SERCOM2_` writer - SERCOM2"]
pub type Sercom2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERCOM3_` reader - SERCOM3"]
pub type Sercom3_R = crate::BitReader;
#[doc = "Field `SERCOM3_` writer - SERCOM3"]
pub type Sercom3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC0_` reader - TCC0"]
pub type Tcc0_R = crate::BitReader;
#[doc = "Field `TCC0_` writer - TCC0"]
pub type Tcc0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC1_` reader - TCC1"]
pub type Tcc1_R = crate::BitReader;
#[doc = "Field `TCC1_` writer - TCC1"]
pub type Tcc1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC2_` reader - TC2"]
pub type Tc2_R = crate::BitReader;
#[doc = "Field `TC2_` writer - TC2"]
pub type Tc2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC3_` reader - TC3"]
pub type Tc3_R = crate::BitReader;
#[doc = "Field `TC3_` writer - TC3"]
pub type Tc3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMECC_` reader - RAMECC"]
pub type Ramecc_R = crate::BitReader;
#[doc = "Field `RAMECC_` writer - RAMECC"]
pub type Ramecc_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB"]
    #[inline(always)]
    pub fn usb_(&self) -> Usb_R {
        Usb_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DSU"]
    #[inline(always)]
    pub fn dsu_(&self) -> Dsu_R {
        Dsu_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NVMCTRL"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> Nvmctrl_R {
        Nvmctrl_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMCC"]
    #[inline(always)]
    pub fn cmcc_(&self) -> Cmcc_R {
        Cmcc_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PORT"]
    #[inline(always)]
    pub fn port_(&self) -> Port_R {
        Port_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMAC"]
    #[inline(always)]
    pub fn dmac_(&self) -> Dmac_R {
        Dmac_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HMATRIX"]
    #[inline(always)]
    pub fn hmatrix_(&self) -> Hmatrix_R {
        Hmatrix_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EVSYS"]
    #[inline(always)]
    pub fn evsys_(&self) -> Evsys_R {
        Evsys_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - SERCOM2"]
    #[inline(always)]
    pub fn sercom2_(&self) -> Sercom2_R {
        Sercom2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SERCOM3"]
    #[inline(always)]
    pub fn sercom3_(&self) -> Sercom3_R {
        Sercom3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TCC0"]
    #[inline(always)]
    pub fn tcc0_(&self) -> Tcc0_R {
        Tcc0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TCC1"]
    #[inline(always)]
    pub fn tcc1_(&self) -> Tcc1_R {
        Tcc1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TC2"]
    #[inline(always)]
    pub fn tc2_(&self) -> Tc2_R {
        Tc2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TC3"]
    #[inline(always)]
    pub fn tc3_(&self) -> Tc3_R {
        Tc3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - RAMECC"]
    #[inline(always)]
    pub fn ramecc_(&self) -> Ramecc_R {
        Ramecc_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB"]
    #[inline(always)]
    #[must_use]
    pub fn usb_(&mut self) -> Usb_W<IntflagbSpec> {
        Usb_W::new(self, 0)
    }
    #[doc = "Bit 1 - DSU"]
    #[inline(always)]
    #[must_use]
    pub fn dsu_(&mut self) -> Dsu_W<IntflagbSpec> {
        Dsu_W::new(self, 1)
    }
    #[doc = "Bit 2 - NVMCTRL"]
    #[inline(always)]
    #[must_use]
    pub fn nvmctrl_(&mut self) -> Nvmctrl_W<IntflagbSpec> {
        Nvmctrl_W::new(self, 2)
    }
    #[doc = "Bit 3 - CMCC"]
    #[inline(always)]
    #[must_use]
    pub fn cmcc_(&mut self) -> Cmcc_W<IntflagbSpec> {
        Cmcc_W::new(self, 3)
    }
    #[doc = "Bit 4 - PORT"]
    #[inline(always)]
    #[must_use]
    pub fn port_(&mut self) -> Port_W<IntflagbSpec> {
        Port_W::new(self, 4)
    }
    #[doc = "Bit 5 - DMAC"]
    #[inline(always)]
    #[must_use]
    pub fn dmac_(&mut self) -> Dmac_W<IntflagbSpec> {
        Dmac_W::new(self, 5)
    }
    #[doc = "Bit 6 - HMATRIX"]
    #[inline(always)]
    #[must_use]
    pub fn hmatrix_(&mut self) -> Hmatrix_W<IntflagbSpec> {
        Hmatrix_W::new(self, 6)
    }
    #[doc = "Bit 7 - EVSYS"]
    #[inline(always)]
    #[must_use]
    pub fn evsys_(&mut self) -> Evsys_W<IntflagbSpec> {
        Evsys_W::new(self, 7)
    }
    #[doc = "Bit 9 - SERCOM2"]
    #[inline(always)]
    #[must_use]
    pub fn sercom2_(&mut self) -> Sercom2_W<IntflagbSpec> {
        Sercom2_W::new(self, 9)
    }
    #[doc = "Bit 10 - SERCOM3"]
    #[inline(always)]
    #[must_use]
    pub fn sercom3_(&mut self) -> Sercom3_W<IntflagbSpec> {
        Sercom3_W::new(self, 10)
    }
    #[doc = "Bit 11 - TCC0"]
    #[inline(always)]
    #[must_use]
    pub fn tcc0_(&mut self) -> Tcc0_W<IntflagbSpec> {
        Tcc0_W::new(self, 11)
    }
    #[doc = "Bit 12 - TCC1"]
    #[inline(always)]
    #[must_use]
    pub fn tcc1_(&mut self) -> Tcc1_W<IntflagbSpec> {
        Tcc1_W::new(self, 12)
    }
    #[doc = "Bit 13 - TC2"]
    #[inline(always)]
    #[must_use]
    pub fn tc2_(&mut self) -> Tc2_W<IntflagbSpec> {
        Tc2_W::new(self, 13)
    }
    #[doc = "Bit 14 - TC3"]
    #[inline(always)]
    #[must_use]
    pub fn tc3_(&mut self) -> Tc3_W<IntflagbSpec> {
        Tc3_W::new(self, 14)
    }
    #[doc = "Bit 16 - RAMECC"]
    #[inline(always)]
    #[must_use]
    pub fn ramecc_(&mut self) -> Ramecc_W<IntflagbSpec> {
        Ramecc_W::new(self, 16)
    }
}
#[doc = "Peripheral interrupt flag status - Bridge B\n\nYou can [`read`](crate::Reg::read) this register and get [`intflagb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflagb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagbSpec;
impl crate::RegisterSpec for IntflagbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflagb::R`](R) reader structure"]
impl crate::Readable for IntflagbSpec {}
#[doc = "`write(|w| ..)` method takes [`intflagb::W`](W) writer structure"]
impl crate::Writable for IntflagbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTFLAGB to value 0"]
impl crate::Resettable for IntflagbSpec {
    const RESET_VALUE: u32 = 0;
}
