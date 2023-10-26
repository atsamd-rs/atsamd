#[doc = "Register `EISIER_EMMC_MODE` reader"]
pub type R = crate::R<EISIER_EMMC_MODE_SPEC>;
#[doc = "Register `EISIER_EMMC_MODE` writer"]
pub type W = crate::W<EISIER_EMMC_MODE_SPEC>;
#[doc = "Field `CMDTEO` reader - Command Timeout Error Signal Enable"]
pub type CMDTEO_R = crate::BitReader<CMDTEOSELECT_A>;
#[doc = "Command Timeout Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDTEOSELECT_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<CMDTEOSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CMDTEOSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDTEO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMDTEOSELECT_A {
        match self.bits {
            false => CMDTEOSELECT_A::MASKED,
            true => CMDTEOSELECT_A::ENABLED,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == CMDTEOSELECT_A::MASKED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMDTEOSELECT_A::ENABLED
    }
}
#[doc = "Field `CMDTEO` writer - Command Timeout Error Signal Enable"]
pub type CMDTEO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMDTEOSELECT_A>;
impl<'a, REG, const O: u8> CMDTEO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(CMDTEOSELECT_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMDTEOSELECT_A::ENABLED)
    }
}
#[doc = "Field `CMDCRC` reader - Command CRC Error Signal Enable"]
pub type CMDCRC_R = crate::BitReader<CMDCRCSELECT_A>;
#[doc = "Command CRC Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDCRCSELECT_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<CMDCRCSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CMDCRCSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDCRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMDCRCSELECT_A {
        match self.bits {
            false => CMDCRCSELECT_A::MASKED,
            true => CMDCRCSELECT_A::ENABLED,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == CMDCRCSELECT_A::MASKED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMDCRCSELECT_A::ENABLED
    }
}
#[doc = "Field `CMDCRC` writer - Command CRC Error Signal Enable"]
pub type CMDCRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMDCRCSELECT_A>;
impl<'a, REG, const O: u8> CMDCRC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(CMDCRCSELECT_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMDCRCSELECT_A::ENABLED)
    }
}
#[doc = "Field `CMDEND` reader - Command End Bit Error Signal Enable"]
pub type CMDEND_R = crate::BitReader<CMDENDSELECT_A>;
#[doc = "Command End Bit Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDENDSELECT_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<CMDENDSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CMDENDSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMDENDSELECT_A {
        match self.bits {
            false => CMDENDSELECT_A::MASKED,
            true => CMDENDSELECT_A::ENABLED,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == CMDENDSELECT_A::MASKED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMDENDSELECT_A::ENABLED
    }
}
#[doc = "Field `CMDEND` writer - Command End Bit Error Signal Enable"]
pub type CMDEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMDENDSELECT_A>;
impl<'a, REG, const O: u8> CMDEND_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(CMDENDSELECT_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMDENDSELECT_A::ENABLED)
    }
}
#[doc = "Field `CMDIDX` reader - Command Index Error Signal Enable"]
pub type CMDIDX_R = crate::BitReader<CMDIDXSELECT_A>;
#[doc = "Command Index Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDIDXSELECT_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<CMDIDXSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CMDIDXSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDIDX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMDIDXSELECT_A {
        match self.bits {
            false => CMDIDXSELECT_A::MASKED,
            true => CMDIDXSELECT_A::ENABLED,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == CMDIDXSELECT_A::MASKED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMDIDXSELECT_A::ENABLED
    }
}
#[doc = "Field `CMDIDX` writer - Command Index Error Signal Enable"]
pub type CMDIDX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMDIDXSELECT_A>;
impl<'a, REG, const O: u8> CMDIDX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(CMDIDXSELECT_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMDIDXSELECT_A::ENABLED)
    }
}
#[doc = "Field `DATTEO` reader - Data Timeout Error Signal Enable"]
pub type DATTEO_R = crate::BitReader<DATTEOSELECT_A>;
#[doc = "Data Timeout Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATTEOSELECT_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<DATTEOSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DATTEOSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DATTEO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DATTEOSELECT_A {
        match self.bits {
            false => DATTEOSELECT_A::MASKED,
            true => DATTEOSELECT_A::ENABLED,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == DATTEOSELECT_A::MASKED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DATTEOSELECT_A::ENABLED
    }
}
#[doc = "Field `DATTEO` writer - Data Timeout Error Signal Enable"]
pub type DATTEO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DATTEOSELECT_A>;
impl<'a, REG, const O: u8> DATTEO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(DATTEOSELECT_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DATTEOSELECT_A::ENABLED)
    }
}
#[doc = "Field `DATCRC` reader - Data CRC Error Signal Enable"]
pub type DATCRC_R = crate::BitReader<DATCRCSELECT_A>;
#[doc = "Data CRC Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATCRCSELECT_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<DATCRCSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DATCRCSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DATCRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DATCRCSELECT_A {
        match self.bits {
            false => DATCRCSELECT_A::MASKED,
            true => DATCRCSELECT_A::ENABLED,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == DATCRCSELECT_A::MASKED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DATCRCSELECT_A::ENABLED
    }
}
#[doc = "Field `DATCRC` writer - Data CRC Error Signal Enable"]
pub type DATCRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DATCRCSELECT_A>;
impl<'a, REG, const O: u8> DATCRC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(DATCRCSELECT_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DATCRCSELECT_A::ENABLED)
    }
}
#[doc = "Field `DATEND` reader - Data End Bit Error Signal Enable"]
pub type DATEND_R = crate::BitReader<DATENDSELECT_A>;
#[doc = "Data End Bit Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATENDSELECT_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<DATENDSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DATENDSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DATEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DATENDSELECT_A {
        match self.bits {
            false => DATENDSELECT_A::MASKED,
            true => DATENDSELECT_A::ENABLED,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == DATENDSELECT_A::MASKED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DATENDSELECT_A::ENABLED
    }
}
#[doc = "Field `DATEND` writer - Data End Bit Error Signal Enable"]
pub type DATEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DATENDSELECT_A>;
impl<'a, REG, const O: u8> DATEND_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(DATENDSELECT_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DATENDSELECT_A::ENABLED)
    }
}
#[doc = "Field `CURLIM` reader - Current Limit Error Signal Enable"]
pub type CURLIM_R = crate::BitReader<CURLIMSELECT_A>;
#[doc = "Current Limit Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CURLIMSELECT_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<CURLIMSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CURLIMSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CURLIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CURLIMSELECT_A {
        match self.bits {
            false => CURLIMSELECT_A::MASKED,
            true => CURLIMSELECT_A::ENABLED,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == CURLIMSELECT_A::MASKED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CURLIMSELECT_A::ENABLED
    }
}
#[doc = "Field `CURLIM` writer - Current Limit Error Signal Enable"]
pub type CURLIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CURLIMSELECT_A>;
impl<'a, REG, const O: u8> CURLIM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(CURLIMSELECT_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CURLIMSELECT_A::ENABLED)
    }
}
#[doc = "Field `ACMD` reader - Auto CMD Error Signal Enable"]
pub type ACMD_R = crate::BitReader<ACMDSELECT_A>;
#[doc = "Auto CMD Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMDSELECT_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ACMDSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ACMDSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACMDSELECT_A {
        match self.bits {
            false => ACMDSELECT_A::MASKED,
            true => ACMDSELECT_A::ENABLED,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == ACMDSELECT_A::MASKED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ACMDSELECT_A::ENABLED
    }
}
#[doc = "Field `ACMD` writer - Auto CMD Error Signal Enable"]
pub type ACMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ACMDSELECT_A>;
impl<'a, REG, const O: u8> ACMD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(ACMDSELECT_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ACMDSELECT_A::ENABLED)
    }
}
#[doc = "Field `ADMA` reader - ADMA Error Signal Enable"]
pub type ADMA_R = crate::BitReader<ADMASELECT_A>;
#[doc = "ADMA Error Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADMASELECT_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ADMASELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ADMASELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ADMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADMASELECT_A {
        match self.bits {
            false => ADMASELECT_A::MASKED,
            true => ADMASELECT_A::ENABLED,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == ADMASELECT_A::MASKED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADMASELECT_A::ENABLED
    }
}
#[doc = "Field `ADMA` writer - ADMA Error Signal Enable"]
pub type ADMA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADMASELECT_A>;
impl<'a, REG, const O: u8> ADMA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(ADMASELECT_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADMASELECT_A::ENABLED)
    }
}
#[doc = "Field `BOOTAE` reader - Boot Acknowledge Error Signal Enable"]
pub type BOOTAE_R = crate::BitReader;
#[doc = "Field `BOOTAE` writer - Boot Acknowledge Error Signal Enable"]
pub type BOOTAE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Command Timeout Error Signal Enable"]
    #[inline(always)]
    pub fn cmdteo(&self) -> CMDTEO_R {
        CMDTEO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command CRC Error Signal Enable"]
    #[inline(always)]
    pub fn cmdcrc(&self) -> CMDCRC_R {
        CMDCRC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command End Bit Error Signal Enable"]
    #[inline(always)]
    pub fn cmdend(&self) -> CMDEND_R {
        CMDEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Command Index Error Signal Enable"]
    #[inline(always)]
    pub fn cmdidx(&self) -> CMDIDX_R {
        CMDIDX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Timeout Error Signal Enable"]
    #[inline(always)]
    pub fn datteo(&self) -> DATTEO_R {
        DATTEO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data CRC Error Signal Enable"]
    #[inline(always)]
    pub fn datcrc(&self) -> DATCRC_R {
        DATCRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data End Bit Error Signal Enable"]
    #[inline(always)]
    pub fn datend(&self) -> DATEND_R {
        DATEND_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Current Limit Error Signal Enable"]
    #[inline(always)]
    pub fn curlim(&self) -> CURLIM_R {
        CURLIM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Auto CMD Error Signal Enable"]
    #[inline(always)]
    pub fn acmd(&self) -> ACMD_R {
        ACMD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADMA Error Signal Enable"]
    #[inline(always)]
    pub fn adma(&self) -> ADMA_R {
        ADMA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Boot Acknowledge Error Signal Enable"]
    #[inline(always)]
    pub fn bootae(&self) -> BOOTAE_R {
        BOOTAE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Timeout Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdteo(&mut self) -> CMDTEO_W<EISIER_EMMC_MODE_SPEC, 0> {
        CMDTEO_W::new(self)
    }
    #[doc = "Bit 1 - Command CRC Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdcrc(&mut self) -> CMDCRC_W<EISIER_EMMC_MODE_SPEC, 1> {
        CMDCRC_W::new(self)
    }
    #[doc = "Bit 2 - Command End Bit Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdend(&mut self) -> CMDEND_W<EISIER_EMMC_MODE_SPEC, 2> {
        CMDEND_W::new(self)
    }
    #[doc = "Bit 3 - Command Index Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdidx(&mut self) -> CMDIDX_W<EISIER_EMMC_MODE_SPEC, 3> {
        CMDIDX_W::new(self)
    }
    #[doc = "Bit 4 - Data Timeout Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn datteo(&mut self) -> DATTEO_W<EISIER_EMMC_MODE_SPEC, 4> {
        DATTEO_W::new(self)
    }
    #[doc = "Bit 5 - Data CRC Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn datcrc(&mut self) -> DATCRC_W<EISIER_EMMC_MODE_SPEC, 5> {
        DATCRC_W::new(self)
    }
    #[doc = "Bit 6 - Data End Bit Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn datend(&mut self) -> DATEND_W<EISIER_EMMC_MODE_SPEC, 6> {
        DATEND_W::new(self)
    }
    #[doc = "Bit 7 - Current Limit Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn curlim(&mut self) -> CURLIM_W<EISIER_EMMC_MODE_SPEC, 7> {
        CURLIM_W::new(self)
    }
    #[doc = "Bit 8 - Auto CMD Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acmd(&mut self) -> ACMD_W<EISIER_EMMC_MODE_SPEC, 8> {
        ACMD_W::new(self)
    }
    #[doc = "Bit 9 - ADMA Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adma(&mut self) -> ADMA_W<EISIER_EMMC_MODE_SPEC, 9> {
        ADMA_W::new(self)
    }
    #[doc = "Bit 12 - Boot Acknowledge Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bootae(&mut self) -> BOOTAE_W<EISIER_EMMC_MODE_SPEC, 12> {
        BOOTAE_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Error Interrupt Signal Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eisier_emmc_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eisier_emmc_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EISIER_EMMC_MODE_SPEC;
impl crate::RegisterSpec for EISIER_EMMC_MODE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`eisier_emmc_mode::R`](R) reader structure"]
impl crate::Readable for EISIER_EMMC_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eisier_emmc_mode::W`](W) writer structure"]
impl crate::Writable for EISIER_EMMC_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EISIER_EMMC_MODE to value 0"]
impl crate::Resettable for EISIER_EMMC_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
