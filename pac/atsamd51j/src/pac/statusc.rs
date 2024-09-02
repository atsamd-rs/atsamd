#[doc = "Register `STATUSC` reader"]
pub type R = crate::R<StatuscSpec>;
#[doc = "Field `TCC2_` reader - TCC2 APB Protect Enable"]
pub type Tcc2_R = crate::BitReader;
#[doc = "Field `TCC3_` reader - TCC3 APB Protect Enable"]
pub type Tcc3_R = crate::BitReader;
#[doc = "Field `TC4_` reader - TC4 APB Protect Enable"]
pub type Tc4_R = crate::BitReader;
#[doc = "Field `TC5_` reader - TC5 APB Protect Enable"]
pub type Tc5_R = crate::BitReader;
#[doc = "Field `PDEC_` reader - PDEC APB Protect Enable"]
pub type Pdec_R = crate::BitReader;
#[doc = "Field `AC_` reader - AC APB Protect Enable"]
pub type Ac_R = crate::BitReader;
#[doc = "Field `AES_` reader - AES APB Protect Enable"]
pub type Aes_R = crate::BitReader;
#[doc = "Field `TRNG_` reader - TRNG APB Protect Enable"]
pub type Trng_R = crate::BitReader;
#[doc = "Field `ICM_` reader - ICM APB Protect Enable"]
pub type Icm_R = crate::BitReader;
#[doc = "Field `PUKCC_` reader - PUKCC APB Protect Enable"]
pub type Pukcc_R = crate::BitReader;
#[doc = "Field `QSPI_` reader - QSPI APB Protect Enable"]
pub type Qspi_R = crate::BitReader;
#[doc = "Field `CCL_` reader - CCL APB Protect Enable"]
pub type Ccl_R = crate::BitReader;
impl R {
    #[doc = "Bit 3 - TCC2 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc2_(&self) -> Tcc2_R {
        Tcc2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TCC3 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc3_(&self) -> Tcc3_R {
        Tcc3_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TC4 APB Protect Enable"]
    #[inline(always)]
    pub fn tc4_(&self) -> Tc4_R {
        Tc4_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TC5 APB Protect Enable"]
    #[inline(always)]
    pub fn tc5_(&self) -> Tc5_R {
        Tc5_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PDEC APB Protect Enable"]
    #[inline(always)]
    pub fn pdec_(&self) -> Pdec_R {
        Pdec_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AC APB Protect Enable"]
    #[inline(always)]
    pub fn ac_(&self) -> Ac_R {
        Ac_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AES APB Protect Enable"]
    #[inline(always)]
    pub fn aes_(&self) -> Aes_R {
        Aes_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TRNG APB Protect Enable"]
    #[inline(always)]
    pub fn trng_(&self) -> Trng_R {
        Trng_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ICM APB Protect Enable"]
    #[inline(always)]
    pub fn icm_(&self) -> Icm_R {
        Icm_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PUKCC APB Protect Enable"]
    #[inline(always)]
    pub fn pukcc_(&self) -> Pukcc_R {
        Pukcc_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - QSPI APB Protect Enable"]
    #[inline(always)]
    pub fn qspi_(&self) -> Qspi_R {
        Qspi_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CCL APB Protect Enable"]
    #[inline(always)]
    pub fn ccl_(&self) -> Ccl_R {
        Ccl_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "Peripheral write protection status - Bridge C\n\nYou can [`read`](crate::Reg::read) this register and get [`statusc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatuscSpec;
impl crate::RegisterSpec for StatuscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusc::R`](R) reader structure"]
impl crate::Readable for StatuscSpec {}
#[doc = "`reset()` method sets STATUSC to value 0"]
impl crate::Resettable for StatuscSpec {
    const RESET_VALUE: u32 = 0;
}
