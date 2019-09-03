#[doc = "Reader of register STATUSB"]
pub type R = crate::R<u32, super::STATUSB>;
#[doc = "Reader of field `USB_`"]
pub type USB__R = crate::R<bool, bool>;
#[doc = "Reader of field `DSU_`"]
pub type DSU__R = crate::R<bool, bool>;
#[doc = "Reader of field `NVMCTRL_`"]
pub type NVMCTRL__R = crate::R<bool, bool>;
#[doc = "Reader of field `CMCC_`"]
pub type CMCC__R = crate::R<bool, bool>;
#[doc = "Reader of field `PORT_`"]
pub type PORT__R = crate::R<bool, bool>;
#[doc = "Reader of field `DMAC_`"]
pub type DMAC__R = crate::R<bool, bool>;
#[doc = "Reader of field `HMATRIX_`"]
pub type HMATRIX__R = crate::R<bool, bool>;
#[doc = "Reader of field `EVSYS_`"]
pub type EVSYS__R = crate::R<bool, bool>;
#[doc = "Reader of field `SERCOM2_`"]
pub type SERCOM2__R = crate::R<bool, bool>;
#[doc = "Reader of field `SERCOM3_`"]
pub type SERCOM3__R = crate::R<bool, bool>;
#[doc = "Reader of field `TCC0_`"]
pub type TCC0__R = crate::R<bool, bool>;
#[doc = "Reader of field `TCC1_`"]
pub type TCC1__R = crate::R<bool, bool>;
#[doc = "Reader of field `TC2_`"]
pub type TC2__R = crate::R<bool, bool>;
#[doc = "Reader of field `TC3_`"]
pub type TC3__R = crate::R<bool, bool>;
#[doc = "Reader of field `RAMECC_`"]
pub type RAMECC__R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - USB APB Protect Enable"]
    #[inline(always)]
    pub fn usb_(&self) -> USB__R {
        USB__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DSU APB Protect Enable"]
    #[inline(always)]
    pub fn dsu_(&self) -> DSU__R {
        DSU__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - NVMCTRL APB Protect Enable"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> NVMCTRL__R {
        NVMCTRL__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CMCC APB Protect Enable"]
    #[inline(always)]
    pub fn cmcc_(&self) -> CMCC__R {
        CMCC__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PORT APB Protect Enable"]
    #[inline(always)]
    pub fn port_(&self) -> PORT__R {
        PORT__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DMAC APB Protect Enable"]
    #[inline(always)]
    pub fn dmac_(&self) -> DMAC__R {
        DMAC__R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - HMATRIX APB Protect Enable"]
    #[inline(always)]
    pub fn hmatrix_(&self) -> HMATRIX__R {
        HMATRIX__R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - EVSYS APB Protect Enable"]
    #[inline(always)]
    pub fn evsys_(&self) -> EVSYS__R {
        EVSYS__R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SERCOM2 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom2_(&self) -> SERCOM2__R {
        SERCOM2__R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SERCOM3 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom3_(&self) -> SERCOM3__R {
        SERCOM3__R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TCC0 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc0_(&self) -> TCC0__R {
        TCC0__R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TCC1 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc1_(&self) -> TCC1__R {
        TCC1__R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TC2 APB Protect Enable"]
    #[inline(always)]
    pub fn tc2_(&self) -> TC2__R {
        TC2__R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TC3 APB Protect Enable"]
    #[inline(always)]
    pub fn tc3_(&self) -> TC3__R {
        TC3__R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RAMECC APB Protect Enable"]
    #[inline(always)]
    pub fn ramecc_(&self) -> RAMECC__R {
        RAMECC__R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
