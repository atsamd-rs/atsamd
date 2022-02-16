#[doc = "Register `US_IMR_LIN_MODE` reader"]
pub struct R(crate::R<US_IMR_LIN_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_IMR_LIN_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_IMR_LIN_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_IMR_LIN_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXRDY` reader - RXRDY Interrupt Mask"]
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
#[doc = "Field `TXRDY` reader - TXRDY Interrupt Mask"]
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
#[doc = "Field `TXEMPTY` reader - TXEMPTY Interrupt Mask"]
pub struct TXEMPTY_R(crate::FieldReader<bool, bool>);
impl TXEMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXEMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXEMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RIIC` reader - Ring Indicator Input Change Mask"]
pub struct RIIC_R(crate::FieldReader<bool, bool>);
impl RIIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RIIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RIIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSRIC` reader - Data Set Ready Input Change Mask"]
pub struct DSRIC_R(crate::FieldReader<bool, bool>);
impl DSRIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSRIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSRIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDIC` reader - Data Carrier Detect Input Change Interrupt Mask"]
pub struct DCDIC_R(crate::FieldReader<bool, bool>);
impl DCDIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCDIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINBK` reader - LIN Break Sent or LIN Break Received Interrupt Mask"]
pub struct LINBK_R(crate::FieldReader<bool, bool>);
impl LINBK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINBK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINBK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINID` reader - LIN Identifier Sent or LIN Identifier Received Interrupt Mask"]
pub struct LINID_R(crate::FieldReader<bool, bool>);
impl LINID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINTC` reader - LIN Transfer Completed Interrupt Mask"]
pub struct LINTC_R(crate::FieldReader<bool, bool>);
impl LINTC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINBE` reader - LIN Bus Error Interrupt Mask"]
pub struct LINBE_R(crate::FieldReader<bool, bool>);
impl LINBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINISFE` reader - LIN Inconsistent Synch Field Error Interrupt Mask"]
pub struct LINISFE_R(crate::FieldReader<bool, bool>);
impl LINISFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINISFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINISFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINIPE` reader - LIN Identifier Parity Interrupt Mask"]
pub struct LINIPE_R(crate::FieldReader<bool, bool>);
impl LINIPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINIPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINIPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINCE` reader - LIN Checksum Error Interrupt Mask"]
pub struct LINCE_R(crate::FieldReader<bool, bool>);
impl LINCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINSNRE` reader - LIN Slave Not Responding Error Interrupt Mask"]
pub struct LINSNRE_R(crate::FieldReader<bool, bool>);
impl LINSNRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINSNRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINSNRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINSTE` reader - LIN Synch Tolerance Error Interrupt Mask"]
pub struct LINSTE_R(crate::FieldReader<bool, bool>);
impl LINSTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINSTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINSTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINHTE` reader - LIN Header Timeout Error Interrupt Mask"]
pub struct LINHTE_R(crate::FieldReader<bool, bool>);
impl LINHTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINHTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINHTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - RXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Ring Indicator Input Change Mask"]
    #[inline(always)]
    pub fn riic(&self) -> RIIC_R {
        RIIC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Data Set Ready Input Change Mask"]
    #[inline(always)]
    pub fn dsric(&self) -> DSRIC_R {
        DSRIC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Data Carrier Detect Input Change Interrupt Mask"]
    #[inline(always)]
    pub fn dcdic(&self) -> DCDIC_R {
        DCDIC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 13 - LIN Break Sent or LIN Break Received Interrupt Mask"]
    #[inline(always)]
    pub fn linbk(&self) -> LINBK_R {
        LINBK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - LIN Identifier Sent or LIN Identifier Received Interrupt Mask"]
    #[inline(always)]
    pub fn linid(&self) -> LINID_R {
        LINID_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LIN Transfer Completed Interrupt Mask"]
    #[inline(always)]
    pub fn lintc(&self) -> LINTC_R {
        LINTC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 25 - LIN Bus Error Interrupt Mask"]
    #[inline(always)]
    pub fn linbe(&self) -> LINBE_R {
        LINBE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - LIN Inconsistent Synch Field Error Interrupt Mask"]
    #[inline(always)]
    pub fn linisfe(&self) -> LINISFE_R {
        LINISFE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - LIN Identifier Parity Interrupt Mask"]
    #[inline(always)]
    pub fn linipe(&self) -> LINIPE_R {
        LINIPE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - LIN Checksum Error Interrupt Mask"]
    #[inline(always)]
    pub fn lince(&self) -> LINCE_R {
        LINCE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - LIN Slave Not Responding Error Interrupt Mask"]
    #[inline(always)]
    pub fn linsnre(&self) -> LINSNRE_R {
        LINSNRE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - LIN Synch Tolerance Error Interrupt Mask"]
    #[inline(always)]
    pub fn linste(&self) -> LINSTE_R {
        LINSTE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - LIN Header Timeout Error Interrupt Mask"]
    #[inline(always)]
    pub fn linhte(&self) -> LINHTE_R {
        LINHTE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_imr_lin_mode](index.html) module"]
pub struct US_IMR_LIN_MODE_SPEC;
impl crate::RegisterSpec for US_IMR_LIN_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_imr_lin_mode::R](R) reader structure"]
impl crate::Readable for US_IMR_LIN_MODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets US_IMR_LIN_MODE to value 0"]
impl crate::Resettable for US_IMR_LIN_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
