#[doc = "Register `STATUSB` reader"]
pub struct R(crate::R<STATUSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USB_` reader - USB APB Protect Enable"]
pub type USB__R = crate::BitReader<bool>;
#[doc = "Field `DSU_` reader - DSU APB Protect Enable"]
pub type DSU__R = crate::BitReader<bool>;
#[doc = "Field `NVMCTRL_` reader - NVMCTRL APB Protect Enable"]
pub type NVMCTRL__R = crate::BitReader<bool>;
#[doc = "Field `CMCC_` reader - CMCC APB Protect Enable"]
pub type CMCC__R = crate::BitReader<bool>;
#[doc = "Field `PORT_` reader - PORT APB Protect Enable"]
pub type PORT__R = crate::BitReader<bool>;
#[doc = "Field `DMAC_` reader - DMAC APB Protect Enable"]
pub type DMAC__R = crate::BitReader<bool>;
#[doc = "Field `HMATRIX_` reader - HMATRIX APB Protect Enable"]
pub type HMATRIX__R = crate::BitReader<bool>;
#[doc = "Field `EVSYS_` reader - EVSYS APB Protect Enable"]
pub type EVSYS__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM2_` reader - SERCOM2 APB Protect Enable"]
pub type SERCOM2__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM3_` reader - SERCOM3 APB Protect Enable"]
pub type SERCOM3__R = crate::BitReader<bool>;
#[doc = "Field `TCC0_` reader - TCC0 APB Protect Enable"]
pub type TCC0__R = crate::BitReader<bool>;
#[doc = "Field `TCC1_` reader - TCC1 APB Protect Enable"]
pub type TCC1__R = crate::BitReader<bool>;
#[doc = "Field `TC2_` reader - TC2 APB Protect Enable"]
pub type TC2__R = crate::BitReader<bool>;
#[doc = "Field `TC3_` reader - TC3 APB Protect Enable"]
pub type TC3__R = crate::BitReader<bool>;
#[doc = "Field `RAMECC_` reader - RAMECC APB Protect Enable"]
pub type RAMECC__R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - USB APB Protect Enable"]
    #[inline(always)]
    pub fn usb_(&self) -> USB__R {
        USB__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DSU APB Protect Enable"]
    #[inline(always)]
    pub fn dsu_(&self) -> DSU__R {
        DSU__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NVMCTRL APB Protect Enable"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> NVMCTRL__R {
        NVMCTRL__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMCC APB Protect Enable"]
    #[inline(always)]
    pub fn cmcc_(&self) -> CMCC__R {
        CMCC__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PORT APB Protect Enable"]
    #[inline(always)]
    pub fn port_(&self) -> PORT__R {
        PORT__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMAC APB Protect Enable"]
    #[inline(always)]
    pub fn dmac_(&self) -> DMAC__R {
        DMAC__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HMATRIX APB Protect Enable"]
    #[inline(always)]
    pub fn hmatrix_(&self) -> HMATRIX__R {
        HMATRIX__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EVSYS APB Protect Enable"]
    #[inline(always)]
    pub fn evsys_(&self) -> EVSYS__R {
        EVSYS__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - SERCOM2 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom2_(&self) -> SERCOM2__R {
        SERCOM2__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SERCOM3 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom3_(&self) -> SERCOM3__R {
        SERCOM3__R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TCC0 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc0_(&self) -> TCC0__R {
        TCC0__R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TCC1 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc1_(&self) -> TCC1__R {
        TCC1__R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TC2 APB Protect Enable"]
    #[inline(always)]
    pub fn tc2_(&self) -> TC2__R {
        TC2__R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TC3 APB Protect Enable"]
    #[inline(always)]
    pub fn tc3_(&self) -> TC3__R {
        TC3__R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - RAMECC APB Protect Enable"]
    #[inline(always)]
    pub fn ramecc_(&self) -> RAMECC__R {
        RAMECC__R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Peripheral write protection status - Bridge B\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusb](index.html) module"]
pub struct STATUSB_SPEC;
impl crate::RegisterSpec for STATUSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statusb::R](R) reader structure"]
impl crate::Readable for STATUSB_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUSB to value 0x02"]
impl crate::Resettable for STATUSB_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
