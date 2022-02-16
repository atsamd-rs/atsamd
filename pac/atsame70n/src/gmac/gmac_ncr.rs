#[doc = "Register `GMAC_NCR` reader"]
pub struct R(crate::R<GMAC_NCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_NCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_NCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_NCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GMAC_NCR` writer"]
pub struct W(crate::W<GMAC_NCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GMAC_NCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GMAC_NCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GMAC_NCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LBL` reader - Loop Back Local"]
pub struct LBL_R(crate::FieldReader<bool, bool>);
impl LBL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LBL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LBL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LBL` writer - Loop Back Local"]
pub struct LBL_W<'a> {
    w: &'a mut W,
}
impl<'a> LBL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
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
#[doc = "Field `RXEN` writer - Receive Enable"]
pub struct RXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
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
#[doc = "Field `TXEN` writer - Transmit Enable"]
pub struct TXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `MPE` reader - Management Port Enable"]
pub struct MPE_R(crate::FieldReader<bool, bool>);
impl MPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPE` writer - Management Port Enable"]
pub struct MPE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CLRSTAT` reader - Clear Statistics Registers"]
pub struct CLRSTAT_R(crate::FieldReader<bool, bool>);
impl CLRSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLRSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRSTAT` writer - Clear Statistics Registers"]
pub struct CLRSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRSTAT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `INCSTAT` reader - Increment Statistics Registers"]
pub struct INCSTAT_R(crate::FieldReader<bool, bool>);
impl INCSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INCSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INCSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INCSTAT` writer - Increment Statistics Registers"]
pub struct INCSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> INCSTAT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `WESTAT` reader - Write Enable for Statistics Registers"]
pub struct WESTAT_R(crate::FieldReader<bool, bool>);
impl WESTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WESTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WESTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WESTAT` writer - Write Enable for Statistics Registers"]
pub struct WESTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> WESTAT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `BP` reader - Back pressure"]
pub struct BP_R(crate::FieldReader<bool, bool>);
impl BP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BP` writer - Back pressure"]
pub struct BP_W<'a> {
    w: &'a mut W,
}
impl<'a> BP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `TSTART` reader - Start Transmission"]
pub struct TSTART_R(crate::FieldReader<bool, bool>);
impl TSTART_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTART` writer - Start Transmission"]
pub struct TSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTART_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `THALT` reader - Transmit Halt"]
pub struct THALT_R(crate::FieldReader<bool, bool>);
impl THALT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        THALT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THALT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THALT` writer - Transmit Halt"]
pub struct THALT_W<'a> {
    w: &'a mut W,
}
impl<'a> THALT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `TXPF` reader - Transmit Pause Frame"]
pub struct TXPF_R(crate::FieldReader<bool, bool>);
impl TXPF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXPF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXPF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXPF` writer - Transmit Pause Frame"]
pub struct TXPF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `TXZQPF` reader - Transmit Zero Quantum Pause Frame"]
pub struct TXZQPF_R(crate::FieldReader<bool, bool>);
impl TXZQPF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXZQPF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXZQPF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXZQPF` writer - Transmit Zero Quantum Pause Frame"]
pub struct TXZQPF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXZQPF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `SRTSM` reader - Store Receive Time Stamp to Memory"]
pub struct SRTSM_R(crate::FieldReader<bool, bool>);
impl SRTSM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRTSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRTSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRTSM` writer - Store Receive Time Stamp to Memory"]
pub struct SRTSM_W<'a> {
    w: &'a mut W,
}
impl<'a> SRTSM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `ENPBPR` reader - Enable PFC Priority-based Pause Reception"]
pub struct ENPBPR_R(crate::FieldReader<bool, bool>);
impl ENPBPR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENPBPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENPBPR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENPBPR` writer - Enable PFC Priority-based Pause Reception"]
pub struct ENPBPR_W<'a> {
    w: &'a mut W,
}
impl<'a> ENPBPR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `TXPBPF` reader - Transmit PFC Priority-based Pause Frame"]
pub struct TXPBPF_R(crate::FieldReader<bool, bool>);
impl TXPBPF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXPBPF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXPBPF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXPBPF` writer - Transmit PFC Priority-based Pause Frame"]
pub struct TXPBPF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPBPF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `FNP` reader - Flush Next Packet"]
pub struct FNP_R(crate::FieldReader<bool, bool>);
impl FNP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FNP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FNP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FNP` writer - Flush Next Packet"]
pub struct FNP_W<'a> {
    w: &'a mut W,
}
impl<'a> FNP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `TXLPIEN` reader - Enable LPI Transmission"]
pub struct TXLPIEN_R(crate::FieldReader<bool, bool>);
impl TXLPIEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXLPIEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXLPIEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXLPIEN` writer - Enable LPI Transmission"]
pub struct TXLPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXLPIEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Loop Back Local"]
    #[inline(always)]
    pub fn lbl(&self) -> LBL_R {
        LBL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit Enable"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Management Port Enable"]
    #[inline(always)]
    pub fn mpe(&self) -> MPE_R {
        MPE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clear Statistics Registers"]
    #[inline(always)]
    pub fn clrstat(&self) -> CLRSTAT_R {
        CLRSTAT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Increment Statistics Registers"]
    #[inline(always)]
    pub fn incstat(&self) -> INCSTAT_R {
        INCSTAT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write Enable for Statistics Registers"]
    #[inline(always)]
    pub fn westat(&self) -> WESTAT_R {
        WESTAT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Back pressure"]
    #[inline(always)]
    pub fn bp(&self) -> BP_R {
        BP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Start Transmission"]
    #[inline(always)]
    pub fn tstart(&self) -> TSTART_R {
        TSTART_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmit Halt"]
    #[inline(always)]
    pub fn thalt(&self) -> THALT_R {
        THALT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transmit Pause Frame"]
    #[inline(always)]
    pub fn txpf(&self) -> TXPF_R {
        TXPF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Transmit Zero Quantum Pause Frame"]
    #[inline(always)]
    pub fn txzqpf(&self) -> TXZQPF_R {
        TXZQPF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Store Receive Time Stamp to Memory"]
    #[inline(always)]
    pub fn srtsm(&self) -> SRTSM_R {
        SRTSM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable PFC Priority-based Pause Reception"]
    #[inline(always)]
    pub fn enpbpr(&self) -> ENPBPR_R {
        ENPBPR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Transmit PFC Priority-based Pause Frame"]
    #[inline(always)]
    pub fn txpbpf(&self) -> TXPBPF_R {
        TXPBPF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Flush Next Packet"]
    #[inline(always)]
    pub fn fnp(&self) -> FNP_R {
        FNP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable LPI Transmission"]
    #[inline(always)]
    pub fn txlpien(&self) -> TXLPIEN_R {
        TXLPIEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Loop Back Local"]
    #[inline(always)]
    pub fn lbl(&mut self) -> LBL_W {
        LBL_W { w: self }
    }
    #[doc = "Bit 2 - Receive Enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RXEN_W {
        RXEN_W { w: self }
    }
    #[doc = "Bit 3 - Transmit Enable"]
    #[inline(always)]
    pub fn txen(&mut self) -> TXEN_W {
        TXEN_W { w: self }
    }
    #[doc = "Bit 4 - Management Port Enable"]
    #[inline(always)]
    pub fn mpe(&mut self) -> MPE_W {
        MPE_W { w: self }
    }
    #[doc = "Bit 5 - Clear Statistics Registers"]
    #[inline(always)]
    pub fn clrstat(&mut self) -> CLRSTAT_W {
        CLRSTAT_W { w: self }
    }
    #[doc = "Bit 6 - Increment Statistics Registers"]
    #[inline(always)]
    pub fn incstat(&mut self) -> INCSTAT_W {
        INCSTAT_W { w: self }
    }
    #[doc = "Bit 7 - Write Enable for Statistics Registers"]
    #[inline(always)]
    pub fn westat(&mut self) -> WESTAT_W {
        WESTAT_W { w: self }
    }
    #[doc = "Bit 8 - Back pressure"]
    #[inline(always)]
    pub fn bp(&mut self) -> BP_W {
        BP_W { w: self }
    }
    #[doc = "Bit 9 - Start Transmission"]
    #[inline(always)]
    pub fn tstart(&mut self) -> TSTART_W {
        TSTART_W { w: self }
    }
    #[doc = "Bit 10 - Transmit Halt"]
    #[inline(always)]
    pub fn thalt(&mut self) -> THALT_W {
        THALT_W { w: self }
    }
    #[doc = "Bit 11 - Transmit Pause Frame"]
    #[inline(always)]
    pub fn txpf(&mut self) -> TXPF_W {
        TXPF_W { w: self }
    }
    #[doc = "Bit 12 - Transmit Zero Quantum Pause Frame"]
    #[inline(always)]
    pub fn txzqpf(&mut self) -> TXZQPF_W {
        TXZQPF_W { w: self }
    }
    #[doc = "Bit 15 - Store Receive Time Stamp to Memory"]
    #[inline(always)]
    pub fn srtsm(&mut self) -> SRTSM_W {
        SRTSM_W { w: self }
    }
    #[doc = "Bit 16 - Enable PFC Priority-based Pause Reception"]
    #[inline(always)]
    pub fn enpbpr(&mut self) -> ENPBPR_W {
        ENPBPR_W { w: self }
    }
    #[doc = "Bit 17 - Transmit PFC Priority-based Pause Frame"]
    #[inline(always)]
    pub fn txpbpf(&mut self) -> TXPBPF_W {
        TXPBPF_W { w: self }
    }
    #[doc = "Bit 18 - Flush Next Packet"]
    #[inline(always)]
    pub fn fnp(&mut self) -> FNP_W {
        FNP_W { w: self }
    }
    #[doc = "Bit 19 - Enable LPI Transmission"]
    #[inline(always)]
    pub fn txlpien(&mut self) -> TXLPIEN_W {
        TXLPIEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Network Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_ncr](index.html) module"]
pub struct GMAC_NCR_SPEC;
impl crate::RegisterSpec for GMAC_NCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_ncr::R](R) reader structure"]
impl crate::Readable for GMAC_NCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gmac_ncr::W](W) writer structure"]
impl crate::Writable for GMAC_NCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GMAC_NCR to value 0"]
impl crate::Resettable for GMAC_NCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
