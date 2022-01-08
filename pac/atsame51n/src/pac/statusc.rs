#[doc = "Register `STATUSC` reader"]
pub struct R(crate::R<STATUSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAN0_` reader - CAN0 APB Protect Enable"]
pub struct CAN0__R(crate::FieldReader<bool, bool>);
impl CAN0__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAN0__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAN0__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAN1_` reader - CAN1 APB Protect Enable"]
pub struct CAN1__R(crate::FieldReader<bool, bool>);
impl CAN1__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAN1__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAN1__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCC2_` reader - TCC2 APB Protect Enable"]
pub struct TCC2__R(crate::FieldReader<bool, bool>);
impl TCC2__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCC2__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCC2__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCC3_` reader - TCC3 APB Protect Enable"]
pub struct TCC3__R(crate::FieldReader<bool, bool>);
impl TCC3__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCC3__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCC3__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC4_` reader - TC4 APB Protect Enable"]
pub struct TC4__R(crate::FieldReader<bool, bool>);
impl TC4__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC4__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC4__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC5_` reader - TC5 APB Protect Enable"]
pub struct TC5__R(crate::FieldReader<bool, bool>);
impl TC5__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC5__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC5__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDEC_` reader - PDEC APB Protect Enable"]
pub struct PDEC__R(crate::FieldReader<bool, bool>);
impl PDEC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AC_` reader - AC APB Protect Enable"]
pub struct AC__R(crate::FieldReader<bool, bool>);
impl AC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AES_` reader - AES APB Protect Enable"]
pub struct AES__R(crate::FieldReader<bool, bool>);
impl AES__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AES__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AES__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRNG_` reader - TRNG APB Protect Enable"]
pub struct TRNG__R(crate::FieldReader<bool, bool>);
impl TRNG__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRNG__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRNG__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICM_` reader - ICM APB Protect Enable"]
pub struct ICM__R(crate::FieldReader<bool, bool>);
impl ICM__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ICM__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICM__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUKCC_` reader - PUKCC APB Protect Enable"]
pub struct PUKCC__R(crate::FieldReader<bool, bool>);
impl PUKCC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PUKCC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUKCC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QSPI_` reader - QSPI APB Protect Enable"]
pub struct QSPI__R(crate::FieldReader<bool, bool>);
impl QSPI__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QSPI__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QSPI__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCL_` reader - CCL APB Protect Enable"]
pub struct CCL__R(crate::FieldReader<bool, bool>);
impl CCL__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCL__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCL__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - CAN0 APB Protect Enable"]
    #[inline(always)]
    pub fn can0_(&self) -> CAN0__R {
        CAN0__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CAN1 APB Protect Enable"]
    #[inline(always)]
    pub fn can1_(&self) -> CAN1__R {
        CAN1__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TCC2 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc2_(&self) -> TCC2__R {
        TCC2__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TCC3 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc3_(&self) -> TCC3__R {
        TCC3__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TC4 APB Protect Enable"]
    #[inline(always)]
    pub fn tc4_(&self) -> TC4__R {
        TC4__R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TC5 APB Protect Enable"]
    #[inline(always)]
    pub fn tc5_(&self) -> TC5__R {
        TC5__R::new(((self.bits >> 6) & 0x01) != 0)
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
#[doc = "Peripheral write protection status - Bridge C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusc](index.html) module"]
pub struct STATUSC_SPEC;
impl crate::RegisterSpec for STATUSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statusc::R](R) reader structure"]
impl crate::Readable for STATUSC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUSC to value 0"]
impl crate::Resettable for STATUSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
