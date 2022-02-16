#[doc = "Register `RSTC_SR` reader"]
pub struct R(crate::R<RSTC_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTC_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTC_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTC_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `URSTS` reader - User Reset Status"]
pub struct URSTS_R(crate::FieldReader<bool, bool>);
impl URSTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        URSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Reset Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RSTTYP_A {
    #[doc = "0: First power-up reset"]
    GENERAL_RST = 0,
    #[doc = "1: Return from Backup Mode"]
    BACKUP_RST = 1,
    #[doc = "2: Watchdog fault occurred"]
    WDT_RST = 2,
    #[doc = "3: Processor reset required by the software"]
    SOFT_RST = 3,
    #[doc = "4: NRST pin detected low"]
    USER_RST = 4,
}
impl From<RSTTYP_A> for u8 {
    #[inline(always)]
    fn from(variant: RSTTYP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RSTTYP` reader - Reset Type"]
pub struct RSTTYP_R(crate::FieldReader<u8, RSTTYP_A>);
impl RSTTYP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RSTTYP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RSTTYP_A> {
        match self.bits {
            0 => Some(RSTTYP_A::GENERAL_RST),
            1 => Some(RSTTYP_A::BACKUP_RST),
            2 => Some(RSTTYP_A::WDT_RST),
            3 => Some(RSTTYP_A::SOFT_RST),
            4 => Some(RSTTYP_A::USER_RST),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GENERAL_RST`"]
    #[inline(always)]
    pub fn is_general_rst(&self) -> bool {
        **self == RSTTYP_A::GENERAL_RST
    }
    #[doc = "Checks if the value of the field is `BACKUP_RST`"]
    #[inline(always)]
    pub fn is_backup_rst(&self) -> bool {
        **self == RSTTYP_A::BACKUP_RST
    }
    #[doc = "Checks if the value of the field is `WDT_RST`"]
    #[inline(always)]
    pub fn is_wdt_rst(&self) -> bool {
        **self == RSTTYP_A::WDT_RST
    }
    #[doc = "Checks if the value of the field is `SOFT_RST`"]
    #[inline(always)]
    pub fn is_soft_rst(&self) -> bool {
        **self == RSTTYP_A::SOFT_RST
    }
    #[doc = "Checks if the value of the field is `USER_RST`"]
    #[inline(always)]
    pub fn is_user_rst(&self) -> bool {
        **self == RSTTYP_A::USER_RST
    }
}
impl core::ops::Deref for RSTTYP_R {
    type Target = crate::FieldReader<u8, RSTTYP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRSTL` reader - NRST Pin Level"]
pub struct NRSTL_R(crate::FieldReader<bool, bool>);
impl NRSTL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRSTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRSTL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRCMP` reader - Software Reset Command in Progress"]
pub struct SRCMP_R(crate::FieldReader<bool, bool>);
impl SRCMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRCMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRCMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - User Reset Status"]
    #[inline(always)]
    pub fn ursts(&self) -> URSTS_R {
        URSTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Reset Type"]
    #[inline(always)]
    pub fn rsttyp(&self) -> RSTTYP_R {
        RSTTYP_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 16 - NRST Pin Level"]
    #[inline(always)]
    pub fn nrstl(&self) -> NRSTL_R {
        NRSTL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Software Reset Command in Progress"]
    #[inline(always)]
    pub fn srcmp(&self) -> SRCMP_R {
        SRCMP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstc_sr](index.html) module"]
pub struct RSTC_SR_SPEC;
impl crate::RegisterSpec for RSTC_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstc_sr::R](R) reader structure"]
impl crate::Readable for RSTC_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSTC_SR to value 0"]
impl crate::Resettable for RSTC_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
