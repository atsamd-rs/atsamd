#[doc = "Register `SSC_SR` reader"]
pub struct R(crate::R<SSC_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSC_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSC_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSC_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXRDY` reader - Transmit Ready"]
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
#[doc = "Field `TXEMPTY` reader - Transmit Empty"]
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
#[doc = "Field `RXRDY` reader - Receive Ready"]
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
#[doc = "Field `OVRUN` reader - Receive Overrun"]
pub struct OVRUN_R(crate::FieldReader<bool, bool>);
impl OVRUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVRUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CP0` reader - Compare 0"]
pub struct CP0_R(crate::FieldReader<bool, bool>);
impl CP0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CP0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CP1` reader - Compare 1"]
pub struct CP1_R(crate::FieldReader<bool, bool>);
impl CP1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSYN` reader - Transmit Sync"]
pub struct TXSYN_R(crate::FieldReader<bool, bool>);
impl TXSYN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXSYN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSYN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXSYN` reader - Receive Sync"]
pub struct RXSYN_R(crate::FieldReader<bool, bool>);
impl RXSYN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXSYN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXSYN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXEN` reader - Transmit Enable"]
pub struct TXEN_R(crate::FieldReader<bool, bool>);
impl TXEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXEN` reader - Receive Enable"]
pub struct RXEN_R(crate::FieldReader<bool, bool>);
impl RXEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Ready"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Empty"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive Ready"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive Overrun"]
    #[inline(always)]
    pub fn ovrun(&self) -> OVRUN_R {
        OVRUN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Compare 0"]
    #[inline(always)]
    pub fn cp0(&self) -> CP0_R {
        CP0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Compare 1"]
    #[inline(always)]
    pub fn cp1(&self) -> CP1_R {
        CP1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmit Sync"]
    #[inline(always)]
    pub fn txsyn(&self) -> TXSYN_R {
        TXSYN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Receive Sync"]
    #[inline(always)]
    pub fn rxsyn(&self) -> RXSYN_R {
        RXSYN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Transmit Enable"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Receive Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssc_sr](index.html) module"]
pub struct SSC_SR_SPEC;
impl crate::RegisterSpec for SSC_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssc_sr::R](R) reader structure"]
impl crate::Readable for SSC_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SSC_SR to value 0"]
impl crate::Resettable for SSC_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
