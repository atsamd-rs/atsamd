#[doc = "Reader of register STATUSC"]
pub type R = crate::R<u32, super::STATUSC>;
#[doc = "Reader of field `TCC2_`"]
pub type TCC2__R = crate::R<bool, bool>;
#[doc = "Reader of field `PDEC_`"]
pub type PDEC__R = crate::R<bool, bool>;
#[doc = "Reader of field `AC_`"]
pub type AC__R = crate::R<bool, bool>;
#[doc = "Reader of field `AES_`"]
pub type AES__R = crate::R<bool, bool>;
#[doc = "Reader of field `TRNG_`"]
pub type TRNG__R = crate::R<bool, bool>;
#[doc = "Reader of field `ICM_`"]
pub type ICM__R = crate::R<bool, bool>;
#[doc = "Reader of field `PUKCC_`"]
pub type PUKCC__R = crate::R<bool, bool>;
#[doc = "Reader of field `QSPI_`"]
pub type QSPI__R = crate::R<bool, bool>;
#[doc = "Reader of field `CCL_`"]
pub type CCL__R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 3 - TCC2 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc2_(&self) -> TCC2__R {
        TCC2__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PDEC APB Protect Enable"]
    #[inline(always)]
    pub fn pdec_(&self) -> PDEC__R {
        PDEC__R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - AC APB Protect Enable"]
    #[inline(always)]
    pub fn ac_(&self) -> AC__R {
        AC__R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - AES APB Protect Enable"]
    #[inline(always)]
    pub fn aes_(&self) -> AES__R {
        AES__R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TRNG APB Protect Enable"]
    #[inline(always)]
    pub fn trng_(&self) -> TRNG__R {
        TRNG__R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ICM APB Protect Enable"]
    #[inline(always)]
    pub fn icm_(&self) -> ICM__R {
        ICM__R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PUKCC APB Protect Enable"]
    #[inline(always)]
    pub fn pukcc_(&self) -> PUKCC__R {
        PUKCC__R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - QSPI APB Protect Enable"]
    #[inline(always)]
    pub fn qspi_(&self) -> QSPI__R {
        QSPI__R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CCL APB Protect Enable"]
    #[inline(always)]
    pub fn ccl_(&self) -> CCL__R {
        CCL__R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
