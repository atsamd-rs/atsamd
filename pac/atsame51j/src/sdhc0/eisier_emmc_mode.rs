#[doc = "Register `EISIER_EMMC_MODE` reader"]
pub struct R(crate::R<EISIER_EMMC_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EISIER_EMMC_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EISIER_EMMC_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EISIER_EMMC_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EISIER_EMMC_MODE` writer"]
pub struct W(crate::W<EISIER_EMMC_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EISIER_EMMC_MODE_SPEC>;
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
impl From<crate::W<EISIER_EMMC_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EISIER_EMMC_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Command Timeout Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDTEO_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<CMDTEO_A> for bool {
    #[inline(always)]
    fn from(variant: CMDTEO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDTEO` reader - Command Timeout Error Signal Enable"]
pub struct CMDTEO_R(crate::FieldReader<bool, CMDTEO_A>);
impl CMDTEO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMDTEO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDTEO_A {
        match self.bits {
            false => CMDTEO_A::MASKED,
            true => CMDTEO_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == CMDTEO_A::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CMDTEO_A::ENABLED
    }
}
impl core::ops::Deref for CMDTEO_R {
    type Target = crate::FieldReader<bool, CMDTEO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDTEO` writer - Command Timeout Error Signal Enable"]
pub struct CMDTEO_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDTEO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDTEO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(CMDTEO_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMDTEO_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Command CRC Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDCRC_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<CMDCRC_A> for bool {
    #[inline(always)]
    fn from(variant: CMDCRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDCRC` reader - Command CRC Error Signal Enable"]
pub struct CMDCRC_R(crate::FieldReader<bool, CMDCRC_A>);
impl CMDCRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMDCRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDCRC_A {
        match self.bits {
            false => CMDCRC_A::MASKED,
            true => CMDCRC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == CMDCRC_A::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CMDCRC_A::ENABLED
    }
}
impl core::ops::Deref for CMDCRC_R {
    type Target = crate::FieldReader<bool, CMDCRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDCRC` writer - Command CRC Error Signal Enable"]
pub struct CMDCRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDCRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDCRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(CMDCRC_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMDCRC_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Command End Bit Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDEND_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<CMDEND_A> for bool {
    #[inline(always)]
    fn from(variant: CMDEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDEND` reader - Command End Bit Error Signal Enable"]
pub struct CMDEND_R(crate::FieldReader<bool, CMDEND_A>);
impl CMDEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMDEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDEND_A {
        match self.bits {
            false => CMDEND_A::MASKED,
            true => CMDEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == CMDEND_A::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CMDEND_A::ENABLED
    }
}
impl core::ops::Deref for CMDEND_R {
    type Target = crate::FieldReader<bool, CMDEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDEND` writer - Command End Bit Error Signal Enable"]
pub struct CMDEND_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDEND_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(CMDEND_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMDEND_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Command Index Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDIDX_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<CMDIDX_A> for bool {
    #[inline(always)]
    fn from(variant: CMDIDX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDIDX` reader - Command Index Error Signal Enable"]
pub struct CMDIDX_R(crate::FieldReader<bool, CMDIDX_A>);
impl CMDIDX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMDIDX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDIDX_A {
        match self.bits {
            false => CMDIDX_A::MASKED,
            true => CMDIDX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == CMDIDX_A::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CMDIDX_A::ENABLED
    }
}
impl core::ops::Deref for CMDIDX_R {
    type Target = crate::FieldReader<bool, CMDIDX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDIDX` writer - Command Index Error Signal Enable"]
pub struct CMDIDX_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDIDX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDIDX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(CMDIDX_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMDIDX_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
#[doc = "Data Timeout Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATTEO_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<DATTEO_A> for bool {
    #[inline(always)]
    fn from(variant: DATTEO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATTEO` reader - Data Timeout Error Signal Enable"]
pub struct DATTEO_R(crate::FieldReader<bool, DATTEO_A>);
impl DATTEO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATTEO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATTEO_A {
        match self.bits {
            false => DATTEO_A::MASKED,
            true => DATTEO_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == DATTEO_A::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DATTEO_A::ENABLED
    }
}
impl core::ops::Deref for DATTEO_R {
    type Target = crate::FieldReader<bool, DATTEO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATTEO` writer - Data Timeout Error Signal Enable"]
pub struct DATTEO_W<'a> {
    w: &'a mut W,
}
impl<'a> DATTEO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATTEO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(DATTEO_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DATTEO_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Data CRC Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATCRC_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<DATCRC_A> for bool {
    #[inline(always)]
    fn from(variant: DATCRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATCRC` reader - Data CRC Error Signal Enable"]
pub struct DATCRC_R(crate::FieldReader<bool, DATCRC_A>);
impl DATCRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATCRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATCRC_A {
        match self.bits {
            false => DATCRC_A::MASKED,
            true => DATCRC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == DATCRC_A::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DATCRC_A::ENABLED
    }
}
impl core::ops::Deref for DATCRC_R {
    type Target = crate::FieldReader<bool, DATCRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATCRC` writer - Data CRC Error Signal Enable"]
pub struct DATCRC_W<'a> {
    w: &'a mut W,
}
impl<'a> DATCRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATCRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(DATCRC_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DATCRC_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
        self.w
    }
}
#[doc = "Data End Bit Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATEND_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<DATEND_A> for bool {
    #[inline(always)]
    fn from(variant: DATEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATEND` reader - Data End Bit Error Signal Enable"]
pub struct DATEND_R(crate::FieldReader<bool, DATEND_A>);
impl DATEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATEND_A {
        match self.bits {
            false => DATEND_A::MASKED,
            true => DATEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == DATEND_A::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DATEND_A::ENABLED
    }
}
impl core::ops::Deref for DATEND_R {
    type Target = crate::FieldReader<bool, DATEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATEND` writer - Data End Bit Error Signal Enable"]
pub struct DATEND_W<'a> {
    w: &'a mut W,
}
impl<'a> DATEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATEND_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(DATEND_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DATEND_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
#[doc = "Current Limit Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CURLIM_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<CURLIM_A> for bool {
    #[inline(always)]
    fn from(variant: CURLIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CURLIM` reader - Current Limit Error Signal Enable"]
pub struct CURLIM_R(crate::FieldReader<bool, CURLIM_A>);
impl CURLIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CURLIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CURLIM_A {
        match self.bits {
            false => CURLIM_A::MASKED,
            true => CURLIM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == CURLIM_A::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CURLIM_A::ENABLED
    }
}
impl core::ops::Deref for CURLIM_R {
    type Target = crate::FieldReader<bool, CURLIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CURLIM` writer - Current Limit Error Signal Enable"]
pub struct CURLIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CURLIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CURLIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(CURLIM_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CURLIM_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
#[doc = "Auto CMD Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMD_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ACMD_A> for bool {
    #[inline(always)]
    fn from(variant: ACMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMD` reader - Auto CMD Error Signal Enable"]
pub struct ACMD_R(crate::FieldReader<bool, ACMD_A>);
impl ACMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACMD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMD_A {
        match self.bits {
            false => ACMD_A::MASKED,
            true => ACMD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == ACMD_A::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ACMD_A::ENABLED
    }
}
impl core::ops::Deref for ACMD_R {
    type Target = crate::FieldReader<bool, ACMD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACMD` writer - Auto CMD Error Signal Enable"]
pub struct ACMD_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(ACMD_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ACMD_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
#[doc = "ADMA Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADMA_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ADMA_A> for bool {
    #[inline(always)]
    fn from(variant: ADMA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADMA` reader - ADMA Error Signal Enable"]
pub struct ADMA_R(crate::FieldReader<bool, ADMA_A>);
impl ADMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADMA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADMA_A {
        match self.bits {
            false => ADMA_A::MASKED,
            true => ADMA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == ADMA_A::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ADMA_A::ENABLED
    }
}
impl core::ops::Deref for ADMA_R {
    type Target = crate::FieldReader<bool, ADMA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADMA` writer - ADMA Error Signal Enable"]
pub struct ADMA_W<'a> {
    w: &'a mut W,
}
impl<'a> ADMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADMA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(ADMA_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADMA_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u16 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `BOOTAE` reader - Boot Acknowledge Error Signal Enable"]
pub struct BOOTAE_R(crate::FieldReader<bool, bool>);
impl BOOTAE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BOOTAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOTAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOTAE` writer - Boot Acknowledge Error Signal Enable"]
pub struct BOOTAE_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOTAE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u16 & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Command Timeout Error Signal Enable"]
    #[inline(always)]
    pub fn cmdteo(&self) -> CMDTEO_R {
        CMDTEO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Command CRC Error Signal Enable"]
    #[inline(always)]
    pub fn cmdcrc(&self) -> CMDCRC_R {
        CMDCRC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Command End Bit Error Signal Enable"]
    #[inline(always)]
    pub fn cmdend(&self) -> CMDEND_R {
        CMDEND_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Command Index Error Signal Enable"]
    #[inline(always)]
    pub fn cmdidx(&self) -> CMDIDX_R {
        CMDIDX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Data Timeout Error Signal Enable"]
    #[inline(always)]
    pub fn datteo(&self) -> DATTEO_R {
        DATTEO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Data CRC Error Signal Enable"]
    #[inline(always)]
    pub fn datcrc(&self) -> DATCRC_R {
        DATCRC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Data End Bit Error Signal Enable"]
    #[inline(always)]
    pub fn datend(&self) -> DATEND_R {
        DATEND_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Current Limit Error Signal Enable"]
    #[inline(always)]
    pub fn curlim(&self) -> CURLIM_R {
        CURLIM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Auto CMD Error Signal Enable"]
    #[inline(always)]
    pub fn acmd(&self) -> ACMD_R {
        ACMD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADMA Error Signal Enable"]
    #[inline(always)]
    pub fn adma(&self) -> ADMA_R {
        ADMA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Boot Acknowledge Error Signal Enable"]
    #[inline(always)]
    pub fn bootae(&self) -> BOOTAE_R {
        BOOTAE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Timeout Error Signal Enable"]
    #[inline(always)]
    pub fn cmdteo(&mut self) -> CMDTEO_W {
        CMDTEO_W { w: self }
    }
    #[doc = "Bit 1 - Command CRC Error Signal Enable"]
    #[inline(always)]
    pub fn cmdcrc(&mut self) -> CMDCRC_W {
        CMDCRC_W { w: self }
    }
    #[doc = "Bit 2 - Command End Bit Error Signal Enable"]
    #[inline(always)]
    pub fn cmdend(&mut self) -> CMDEND_W {
        CMDEND_W { w: self }
    }
    #[doc = "Bit 3 - Command Index Error Signal Enable"]
    #[inline(always)]
    pub fn cmdidx(&mut self) -> CMDIDX_W {
        CMDIDX_W { w: self }
    }
    #[doc = "Bit 4 - Data Timeout Error Signal Enable"]
    #[inline(always)]
    pub fn datteo(&mut self) -> DATTEO_W {
        DATTEO_W { w: self }
    }
    #[doc = "Bit 5 - Data CRC Error Signal Enable"]
    #[inline(always)]
    pub fn datcrc(&mut self) -> DATCRC_W {
        DATCRC_W { w: self }
    }
    #[doc = "Bit 6 - Data End Bit Error Signal Enable"]
    #[inline(always)]
    pub fn datend(&mut self) -> DATEND_W {
        DATEND_W { w: self }
    }
    #[doc = "Bit 7 - Current Limit Error Signal Enable"]
    #[inline(always)]
    pub fn curlim(&mut self) -> CURLIM_W {
        CURLIM_W { w: self }
    }
    #[doc = "Bit 8 - Auto CMD Error Signal Enable"]
    #[inline(always)]
    pub fn acmd(&mut self) -> ACMD_W {
        ACMD_W { w: self }
    }
    #[doc = "Bit 9 - ADMA Error Signal Enable"]
    #[inline(always)]
    pub fn adma(&mut self) -> ADMA_W {
        ADMA_W { w: self }
    }
    #[doc = "Bit 12 - Boot Acknowledge Error Signal Enable"]
    #[inline(always)]
    pub fn bootae(&mut self) -> BOOTAE_W {
        BOOTAE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Interrupt Signal Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eisier_emmc_mode](index.html) module"]
pub struct EISIER_EMMC_MODE_SPEC;
impl crate::RegisterSpec for EISIER_EMMC_MODE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [eisier_emmc_mode::R](R) reader structure"]
impl crate::Readable for EISIER_EMMC_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eisier_emmc_mode::W](W) writer structure"]
impl crate::Writable for EISIER_EMMC_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EISIER_EMMC_MODE to value 0"]
impl crate::Resettable for EISIER_EMMC_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
