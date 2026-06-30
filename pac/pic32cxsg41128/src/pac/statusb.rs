#[doc = "Register `STATUSB` reader"]
pub type R = crate::R<StatusbSpec>;
#[doc = "Field `USB_` reader - USB APB Protect Enable"]
pub type Usb_R = crate::BitReader;
#[doc = "Field `DSU_` reader - DSU APB Protect Enable"]
pub type Dsu_R = crate::BitReader;
#[doc = "Field `NVMCTRL_` reader - NVMCTRL APB Protect Enable"]
pub type Nvmctrl_R = crate::BitReader;
#[doc = "Field `CMCC_` reader - CMCC APB Protect Enable"]
pub type Cmcc_R = crate::BitReader;
#[doc = "Field `PORT_` reader - PORT APB Protect Enable"]
pub type Port_R = crate::BitReader;
#[doc = "Field `DMAC_` reader - DMAC APB Protect Enable"]
pub type Dmac_R = crate::BitReader;
#[doc = "Field `EVSYS_` reader - EVSYS APB Protect Enable"]
pub type Evsys_R = crate::BitReader;
#[doc = "Field `SERCOM2_` reader - SERCOM2 APB Protect Enable"]
pub type Sercom2_R = crate::BitReader;
#[doc = "Field `SERCOM3_` reader - SERCOM3 APB Protect Enable"]
pub type Sercom3_R = crate::BitReader;
#[doc = "Field `TCC0_` reader - TCC0 APB Protect Enable"]
pub type Tcc0_R = crate::BitReader;
#[doc = "Field `TCC1_` reader - TCC1 APB Protect Enable"]
pub type Tcc1_R = crate::BitReader;
#[doc = "Field `TC2_` reader - TC2 APB Protect Enable"]
pub type Tc2_R = crate::BitReader;
#[doc = "Field `TC3_` reader - TC3 APB Protect Enable"]
pub type Tc3_R = crate::BitReader;
#[doc = "Field `RAMECC_` reader - RAMECC APB Protect Enable"]
pub type Ramecc_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - USB APB Protect Enable"]
    #[inline(always)]
    pub fn usb_(&self) -> Usb_R {
        Usb_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DSU APB Protect Enable"]
    #[inline(always)]
    pub fn dsu_(&self) -> Dsu_R {
        Dsu_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NVMCTRL APB Protect Enable"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> Nvmctrl_R {
        Nvmctrl_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMCC APB Protect Enable"]
    #[inline(always)]
    pub fn cmcc_(&self) -> Cmcc_R {
        Cmcc_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PORT APB Protect Enable"]
    #[inline(always)]
    pub fn port_(&self) -> Port_R {
        Port_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMAC APB Protect Enable"]
    #[inline(always)]
    pub fn dmac_(&self) -> Dmac_R {
        Dmac_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - EVSYS APB Protect Enable"]
    #[inline(always)]
    pub fn evsys_(&self) -> Evsys_R {
        Evsys_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - SERCOM2 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom2_(&self) -> Sercom2_R {
        Sercom2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SERCOM3 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom3_(&self) -> Sercom3_R {
        Sercom3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TCC0 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc0_(&self) -> Tcc0_R {
        Tcc0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TCC1 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc1_(&self) -> Tcc1_R {
        Tcc1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TC2 APB Protect Enable"]
    #[inline(always)]
    pub fn tc2_(&self) -> Tc2_R {
        Tc2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TC3 APB Protect Enable"]
    #[inline(always)]
    pub fn tc3_(&self) -> Tc3_R {
        Tc3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - RAMECC APB Protect Enable"]
    #[inline(always)]
    pub fn ramecc_(&self) -> Ramecc_R {
        Ramecc_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Peripheral write protection status - Bridge B\n\nYou can [`read`](crate::Reg::read) this register and get [`statusb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusbSpec;
impl crate::RegisterSpec for StatusbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusb::R`](R) reader structure"]
impl crate::Readable for StatusbSpec {}
#[doc = "`reset()` method sets STATUSB to value 0x02"]
impl crate::Resettable for StatusbSpec {
    const RESET_VALUE: u32 = 0x02;
}
