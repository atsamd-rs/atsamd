#[doc = "Register `TWIHS_SR` reader"]
pub struct R(crate::R<TWIHS_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWIHS_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWIHS_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWIHS_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXCOMP` reader - Transmission Completed (cleared by writing TWIHS_THR)"]
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
#[doc = "Field `RXRDY` reader - Receive Holding Register Ready (cleared by reading TWIHS_RHR)"]
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
#[doc = "Field `TXRDY` reader - Transmit Holding Register Ready (cleared by writing TWIHS_THR)"]
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
#[doc = "Field `SVREAD` reader - Slave Read"]
pub struct SVREAD_R(crate::FieldReader<bool, bool>);
impl SVREAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SVREAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SVREAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVACC` reader - Slave Access"]
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
#[doc = "Field `GACC` reader - General Call Access (cleared on read)"]
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
#[doc = "Field `OVRE` reader - Overrun Error (cleared on read)"]
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
#[doc = "Field `UNRE` reader - Underrun Error (cleared on read)"]
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
#[doc = "Field `NACK` reader - Not Acknowledged (cleared on read)"]
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
#[doc = "Field `ARBLST` reader - Arbitration Lost (cleared on read)"]
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
#[doc = "Field `SCLWS` reader - Clock Wait State"]
pub struct SCLWS_R(crate::FieldReader<bool, bool>);
impl SCLWS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCLWS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLWS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOSACC` reader - End Of Slave Access (cleared on read)"]
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
#[doc = "Field `MCACK` reader - Master Code Acknowledge (cleared on read)"]
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
#[doc = "Field `TOUT` reader - Timeout Error (cleared on read)"]
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
#[doc = "Field `PECERR` reader - PEC Error (cleared on read)"]
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
#[doc = "Field `SMBDAM` reader - SMBus Default Address Match (cleared on read)"]
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
#[doc = "Field `SMBHHM` reader - SMBus Host Header Address Match (cleared on read)"]
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
#[doc = "Field `SCL` reader - SCL Line Value"]
pub struct SCL_R(crate::FieldReader<bool, bool>);
impl SCL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDA` reader - SDA Line Value"]
pub struct SDA_R(crate::FieldReader<bool, bool>);
impl SDA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Transmission Completed (cleared by writing TWIHS_THR)"]
    #[inline(always)]
    pub fn txcomp(&self) -> TXCOMP_R {
        TXCOMP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Holding Register Ready (cleared by reading TWIHS_RHR)"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit Holding Register Ready (cleared by writing TWIHS_THR)"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Slave Read"]
    #[inline(always)]
    pub fn svread(&self) -> SVREAD_R {
        SVREAD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Slave Access"]
    #[inline(always)]
    pub fn svacc(&self) -> SVACC_R {
        SVACC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - General Call Access (cleared on read)"]
    #[inline(always)]
    pub fn gacc(&self) -> GACC_R {
        GACC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Overrun Error (cleared on read)"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Underrun Error (cleared on read)"]
    #[inline(always)]
    pub fn unre(&self) -> UNRE_R {
        UNRE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Not Acknowledged (cleared on read)"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Arbitration Lost (cleared on read)"]
    #[inline(always)]
    pub fn arblst(&self) -> ARBLST_R {
        ARBLST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Clock Wait State"]
    #[inline(always)]
    pub fn sclws(&self) -> SCLWS_R {
        SCLWS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - End Of Slave Access (cleared on read)"]
    #[inline(always)]
    pub fn eosacc(&self) -> EOSACC_R {
        EOSACC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Master Code Acknowledge (cleared on read)"]
    #[inline(always)]
    pub fn mcack(&self) -> MCACK_R {
        MCACK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Timeout Error (cleared on read)"]
    #[inline(always)]
    pub fn tout(&self) -> TOUT_R {
        TOUT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PEC Error (cleared on read)"]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - SMBus Default Address Match (cleared on read)"]
    #[inline(always)]
    pub fn smbdam(&self) -> SMBDAM_R {
        SMBDAM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SMBus Host Header Address Match (cleared on read)"]
    #[inline(always)]
    pub fn smbhhm(&self) -> SMBHHM_R {
        SMBHHM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - SCL Line Value"]
    #[inline(always)]
    pub fn scl(&self) -> SCL_R {
        SCL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - SDA Line Value"]
    #[inline(always)]
    pub fn sda(&self) -> SDA_R {
        SDA_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_sr](index.html) module"]
pub struct TWIHS_SR_SPEC;
impl crate::RegisterSpec for TWIHS_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twihs_sr::R](R) reader structure"]
impl crate::Readable for TWIHS_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TWIHS_SR to value 0"]
impl crate::Resettable for TWIHS_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
