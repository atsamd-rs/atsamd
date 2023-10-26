#[doc = "Register `STATUSC` reader"]
pub type R = crate::R<STATUSC_SPEC>;
#[doc = "Field `CAN0_` reader - CAN0 APB Protect Enable"]
pub type CAN0__R = crate::BitReader;
#[doc = "Field `CAN1_` reader - CAN1 APB Protect Enable"]
pub type CAN1__R = crate::BitReader;
#[doc = "Field `GMAC_` reader - GMAC APB Protect Enable"]
pub type GMAC__R = crate::BitReader;
#[doc = "Field `TCC2_` reader - TCC2 APB Protect Enable"]
pub type TCC2__R = crate::BitReader;
#[doc = "Field `TCC3_` reader - TCC3 APB Protect Enable"]
pub type TCC3__R = crate::BitReader;
#[doc = "Field `TC4_` reader - TC4 APB Protect Enable"]
pub type TC4__R = crate::BitReader;
#[doc = "Field `TC5_` reader - TC5 APB Protect Enable"]
pub type TC5__R = crate::BitReader;
#[doc = "Field `PDEC_` reader - PDEC APB Protect Enable"]
pub type PDEC__R = crate::BitReader;
#[doc = "Field `AC_` reader - AC APB Protect Enable"]
pub type AC__R = crate::BitReader;
#[doc = "Field `AES_` reader - AES APB Protect Enable"]
pub type AES__R = crate::BitReader;
#[doc = "Field `TRNG_` reader - TRNG APB Protect Enable"]
pub type TRNG__R = crate::BitReader;
#[doc = "Field `ICM_` reader - ICM APB Protect Enable"]
pub type ICM__R = crate::BitReader;
#[doc = "Field `PUKCC_` reader - PUKCC APB Protect Enable"]
pub type PUKCC__R = crate::BitReader;
#[doc = "Field `QSPI_` reader - QSPI APB Protect Enable"]
pub type QSPI__R = crate::BitReader;
#[doc = "Field `CCL_` reader - CCL APB Protect Enable"]
pub type CCL__R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CAN0 APB Protect Enable"]
    #[inline(always)]
    pub fn can0_(&self) -> CAN0__R {
        CAN0__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CAN1 APB Protect Enable"]
    #[inline(always)]
    pub fn can1_(&self) -> CAN1__R {
        CAN1__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GMAC APB Protect Enable"]
    #[inline(always)]
    pub fn gmac_(&self) -> GMAC__R {
        GMAC__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TCC2 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc2_(&self) -> TCC2__R {
        TCC2__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TCC3 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc3_(&self) -> TCC3__R {
        TCC3__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TC4 APB Protect Enable"]
    #[inline(always)]
    pub fn tc4_(&self) -> TC4__R {
        TC4__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TC5 APB Protect Enable"]
    #[inline(always)]
    pub fn tc5_(&self) -> TC5__R {
        TC5__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PDEC APB Protect Enable"]
    #[inline(always)]
    pub fn pdec_(&self) -> PDEC__R {
        PDEC__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AC APB Protect Enable"]
    #[inline(always)]
    pub fn ac_(&self) -> AC__R {
        AC__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AES APB Protect Enable"]
    #[inline(always)]
    pub fn aes_(&self) -> AES__R {
        AES__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TRNG APB Protect Enable"]
    #[inline(always)]
    pub fn trng_(&self) -> TRNG__R {
        TRNG__R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ICM APB Protect Enable"]
    #[inline(always)]
    pub fn icm_(&self) -> ICM__R {
        ICM__R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PUKCC APB Protect Enable"]
    #[inline(always)]
    pub fn pukcc_(&self) -> PUKCC__R {
        PUKCC__R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - QSPI APB Protect Enable"]
    #[inline(always)]
    pub fn qspi_(&self) -> QSPI__R {
        QSPI__R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CCL APB Protect Enable"]
    #[inline(always)]
    pub fn ccl_(&self) -> CCL__R {
        CCL__R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "Peripheral write protection status - Bridge C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statusc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUSC_SPEC;
impl crate::RegisterSpec for STATUSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusc::R`](R) reader structure"]
impl crate::Readable for STATUSC_SPEC {}
#[doc = "`reset()` method sets STATUSC to value 0"]
impl crate::Resettable for STATUSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
