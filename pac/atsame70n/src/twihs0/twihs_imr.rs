#[doc = "Register `TWIHS_IMR` reader"]
pub struct R(crate::R<TWIHS_IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWIHS_IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWIHS_IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWIHS_IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXCOMP` reader - Transmission Completed Interrupt Mask"]
pub struct TXCOMP_R(crate::FieldReader<bool, bool>);
impl TXCOMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXCOMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXCOMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRDY` reader - Receive Holding Register Ready Interrupt Mask"]
pub struct RXRDY_R(crate::FieldReader<bool, bool>);
impl RXRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRDY` reader - Transmit Holding Register Ready Interrupt Mask"]
pub struct TXRDY_R(crate::FieldReader<bool, bool>);
impl TXRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVACC` reader - Slave Access Interrupt Mask"]
pub struct SVACC_R(crate::FieldReader<bool, bool>);
impl SVACC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SVACC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SVACC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GACC` reader - General Call Access Interrupt Mask"]
pub struct GACC_R(crate::FieldReader<bool, bool>);
impl GACC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GACC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GACC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRE` reader - Overrun Error Interrupt Mask"]
pub struct OVRE_R(crate::FieldReader<bool, bool>);
impl OVRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNRE` reader - Underrun Error Interrupt Mask"]
pub struct UNRE_R(crate::FieldReader<bool, bool>);
impl UNRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UNRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NACK` reader - Not Acknowledge Interrupt Mask"]
pub struct NACK_R(crate::FieldReader<bool, bool>);
impl NACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARBLST` reader - Arbitration Lost Interrupt Mask"]
pub struct ARBLST_R(crate::FieldReader<bool, bool>);
impl ARBLST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARBLST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARBLST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCL_WS` reader - Clock Wait State Interrupt Mask"]
pub struct SCL_WS_R(crate::FieldReader<bool, bool>);
impl SCL_WS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCL_WS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCL_WS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOSACC` reader - End Of Slave Access Interrupt Mask"]
pub struct EOSACC_R(crate::FieldReader<bool, bool>);
impl EOSACC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EOSACC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOSACC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCACK` reader - Master Code Acknowledge Interrupt Mask"]
pub struct MCACK_R(crate::FieldReader<bool, bool>);
impl MCACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUT` reader - Timeout Error Interrupt Mask"]
pub struct TOUT_R(crate::FieldReader<bool, bool>);
impl TOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PECERR` reader - PEC Error Interrupt Mask"]
pub struct PECERR_R(crate::FieldReader<bool, bool>);
impl PECERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PECERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PECERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMBDAM` reader - SMBus Default Address Match Interrupt Mask"]
pub struct SMBDAM_R(crate::FieldReader<bool, bool>);
impl SMBDAM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMBDAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMBDAM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMBHHM` reader - SMBus Host Header Address Match Interrupt Mask"]
pub struct SMBHHM_R(crate::FieldReader<bool, bool>);
impl SMBHHM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMBHHM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMBHHM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Transmission Completed Interrupt Mask"]
    #[inline(always)]
    pub fn txcomp(&self) -> TXCOMP_R {
        TXCOMP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Holding Register Ready Interrupt Mask"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit Holding Register Ready Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Slave Access Interrupt Mask"]
    #[inline(always)]
    pub fn svacc(&self) -> SVACC_R {
        SVACC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - General Call Access Interrupt Mask"]
    #[inline(always)]
    pub fn gacc(&self) -> GACC_R {
        GACC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Underrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn unre(&self) -> UNRE_R {
        UNRE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Not Acknowledge Interrupt Mask"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Arbitration Lost Interrupt Mask"]
    #[inline(always)]
    pub fn arblst(&self) -> ARBLST_R {
        ARBLST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Clock Wait State Interrupt Mask"]
    #[inline(always)]
    pub fn scl_ws(&self) -> SCL_WS_R {
        SCL_WS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - End Of Slave Access Interrupt Mask"]
    #[inline(always)]
    pub fn eosacc(&self) -> EOSACC_R {
        EOSACC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Master Code Acknowledge Interrupt Mask"]
    #[inline(always)]
    pub fn mcack(&self) -> MCACK_R {
        MCACK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Timeout Error Interrupt Mask"]
    #[inline(always)]
    pub fn tout(&self) -> TOUT_R {
        TOUT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PEC Error Interrupt Mask"]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - SMBus Default Address Match Interrupt Mask"]
    #[inline(always)]
    pub fn smbdam(&self) -> SMBDAM_R {
        SMBDAM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SMBus Host Header Address Match Interrupt Mask"]
    #[inline(always)]
    pub fn smbhhm(&self) -> SMBHHM_R {
        SMBHHM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_imr](index.html) module"]
pub struct TWIHS_IMR_SPEC;
impl crate::RegisterSpec for TWIHS_IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twihs_imr::R](R) reader structure"]
impl crate::Readable for TWIHS_IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TWIHS_IMR to value 0"]
impl crate::Resettable for TWIHS_IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
